use ndarray::{Array1, Array2};
use reqwest::blocking;
use serde_json::{Value, from_str, from_value};
use crate::error::{BootError, Kind};
use std::io::Write;
use std::time::Duration;
use std::thread::sleep;

use crate::constants::BOOT_IDS;

const PAST_GAMES: u32 = 20;
const DEFAULT_RETRY_TIME: u64 = 40;

// https://developer.riotgames.com/apis#spectator-v4/GET_getCurrentGameInfoBySummoner
pub fn get_current_match_data(name: &str, token: &str) -> Result<Array1<f32>, BootError> {
    let client = blocking::Client::new();
    let id_uri: String = format!("https://euw1.api.riotgames.com/lol/summoner/v4/summoners/by-name/{name}");
    let summoner_id: String;

    // Get the summonerID from the PUUID
    loop {
        let response = client
            .get(&id_uri)
            .header("X-Riot-Token", token)
            .send();

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    let body = res.text()?;
                    let parsed_value: Value = from_str(&body)?;
                    summoner_id = parsed_value.get("id").unwrap().as_str().unwrap().to_string();
                    break;
                } else if res.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                    if let Some(retry_after) = res.headers().get("Retry-After") {
                        if let Ok(retry_seconds) = retry_after.to_str().unwrap_or(&DEFAULT_RETRY_TIME.to_string()).parse::<u64>() {
                            // print!(" /!\\ ({:?})", retry_seconds);
                            // let _ = std::io::stdout().flush();
                            sleep(Duration::from_secs(retry_seconds));
                        }
                    } else {
                        // print!(" /!\\ ({:?})", DEFAULT_RETRY_TIME);
                        // let _ = std::io::stdout().flush();
                        sleep(Duration::from_secs(DEFAULT_RETRY_TIME));
                    }
                } else {
                    return Err(BootError {
                        kind: Kind::INTERNAL,
                        message: String::from("Invalid PUUID"),
                    });
                }
            }
            Err(e) => {
                return Err(BootError::from(e));
            }
        }
    }

    // Get the game information from the summonerID
    let game_uri = ["https://euw1.api.riotgames.com/lol/spectator/v4/active-games/by-summoner/", &summoner_id].join("");
    let game_info: Value;
    loop {
        let response = client
            .get(&game_uri)
            .header("X-Riot-Token", token)
            .send();

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    let body = res.text()?;
                    let parsed_value: Value = from_str(&body)?;
                    game_info = parsed_value;
                    break;
                } else if res.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                    if let Some(retry_after) = res.headers().get("Retry-After") {
                        if let Ok(retry_seconds) = retry_after.to_str().unwrap_or(&DEFAULT_RETRY_TIME.to_string()).parse::<u64>() {
                            // print!(" /!\\ ({:?})", retry_seconds);
                            // let _ = std::io::stdout().flush();
                            // print!(" /!\\ ({:?})", retry_seconds);
                            // let _ = std::io::stdout().flush();
                            sleep(Duration::from_secs(retry_seconds));
                        }
                    } else {
                        // print!(" /!\\ ({:?})", DEFAULT_RETRY_TIME);
                        // let _ = std::io::stdout().flush();
                        // print!(" /!\\ ({:?})", DEFAULT_RETRY_TIME);
                        // let _ = std::io::stdout().flush();
                        sleep(Duration::from_secs(DEFAULT_RETRY_TIME));
                    }
                } else if res.status() == reqwest::StatusCode::NOT_FOUND {
                    return Err(BootError {
                        kind: Kind::INTERNAL,
                        message: String::from("You are not currently in a game"),
                    });
                }
            }
            Err(e) => {
                return Err(BootError::from(e));
            }
        }
    }

    match prepare_game_data(game_info, &summoner_id) {
        Some(data) => Ok(data),
        None => Err(BootError {
            kind: Kind::INTERNAL,
            message: String::from("I can only make good predictions for ARAM games")
        })
    }
}

pub fn get_training_data(starting_names: Vec<String>, count: usize, token: &str) -> Array2<f32> {
    let games = fetch_games(starting_names, count, token).unwrap();
    prepare_training_data(games)
}

fn fetch_games(starting_names: Vec<String>, count: usize, token: &str) -> Result<Vec<Value>, BootError> {
    let mut known_puuids: Vec<String> = vec![];
    let mut new_puuids: Vec<String> = vec![];
    let mut game_ids: Vec<String> = vec![];
    let mut games: Vec<Value> = vec![];
    let mut duplicate_games: usize = 0;

    {
        let client = blocking::Client::new();
        for name in starting_names {
            let uri = format!("https://euw1.api.riotgames.com/lol/summoner/v4/summoners/by-name/{name}");
            let response = client
                .get(&uri)
                .header("X-Riot-Token", token)
                .send();

            match response {
                Ok(res) => {
                    if res.status().is_success() {
                        let body: Value = from_str(&res.text().unwrap()).unwrap();
                        let puuid: String = from_value(body.get("puuid").unwrap().clone()).unwrap();
                        new_puuids.push(puuid);
                    } else if res.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                        if let Some(retry_after) = res.headers().get("Retry-After") {
                            if let Ok(retry_seconds) = retry_after.to_str().unwrap_or(&DEFAULT_RETRY_TIME.to_string()).parse::<u64>() {
                                sleep(Duration::from_secs(retry_seconds));
                            }
                        } else {
                            sleep(Duration::from_secs(DEFAULT_RETRY_TIME));
                        }
                    } else {
                        print!("\rSummoner API returned code {:?}", res.status());
                        let _ = std::io::stdout().flush();
                        continue;
                    }
                }
                Err(_) => {
                    continue;
                }
            }
        }
    }

    if new_puuids.len() == 0 {
        return Err(BootError {
            kind: Kind::INTERNAL,
            message: String::from("No starting PUUIDS to fetch games"),
        });
    }

    println!("Downloading data from {:?} ARAM games.", count);
    while games.len() < count {
        let puuids: Vec<String> = new_puuids.clone();
        for puuid in puuids {
            // remove the puuid from the new puuids and add it to the known puuids
            new_puuids.retain(|x| x != &puuid);
            known_puuids.push(puuid.clone());
            // fetch the last ARAM games of this puuid
            let game_value = get_last_games(&puuid, token, PAST_GAMES).expect(&format!("Failed to fetch game history for {:?}", &puuid));
            let game_vec: Vec<String> = from_value(game_value)?;

            for game_id in game_vec {                
                // Print progress
                if games.len() % ((count / 400)+1) == 0 || duplicate_games % ((count / 400)+1) == 0 {
                    print_loading_bar(games.len(), count, duplicate_games);
                }
                // Do not get match info if we already have it
                if game_ids.contains(&game_id) {
                    duplicate_games += 1;
                    continue;
                }
                game_ids.push(game_id.clone());
                // Get match info and extract puuids from it
                if let Some(info) = get_match_info(&game_id, token) {
                    let participant_value = info.get("metadata")
                        .expect("game info contained no metadata")
                        .get("participants")
                        .expect("game metadata contained no participants");
                    let participant_vector: Vec<String> = from_value(participant_value.clone())?;
                    for participant in participant_vector {
                        // Add new participant if we do not already have it
                        if !new_puuids.contains(&participant) && !known_puuids.contains(&participant) {
                            new_puuids.push(participant);
                        }
                    }
                    games.push(info);
                    if games.len() >= count {
                        return Ok(games);
                    }
                }
            }
        }
    }
    Ok(games)
}

// https://developer.riotgames.com/apis#match-v5/GET_getMatchIdsByPUUID
fn get_last_games(puuid: &str, token: &str, count: u32) -> Option<Value> {
    let client = blocking::Client::new();
    let uri = ["https://europe.api.riotgames.com/lol/match/v5/matches/by-puuid/", puuid, "/ids?queue=450&start=0&count=", &count.to_string()].join("");
    
    loop {
        let response = client
            .get(&uri)
            .header("X-Riot-Token", token)
            .send();

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    let body = res.text().unwrap();
                    let games: Value = from_str(&body).unwrap();
                    return Some(games);
                } else if res.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                    if let Some(retry_after) = res.headers().get("Retry-After") {
                        if let Ok(retry_seconds) = retry_after.to_str().unwrap_or(&DEFAULT_RETRY_TIME.to_string()).parse::<u64>() {
                            // print!(" /!\\ ({:?})", retry_seconds);
                            // let _ = std::io::stdout().flush();
                            sleep(Duration::from_secs(retry_seconds));
                        }
                    } else {
                        // print!(" /!\\ ({:?})", DEFAULT_RETRY_TIME);
                        // let _ = std::io::stdout().flush();
                        sleep(Duration::from_secs(DEFAULT_RETRY_TIME));
                    }
                } else {
                    print!("\rLast Matches API returned code {:?}", res.status());
                    let _ = std::io::stdout().flush();
                    return None;
                }
            }
            Err(_) => {
                return None;
            }
        }
    }
}

// https://developer.riotgames.com/apis#match-v5/GET_getMatch
fn get_match_info(matchid: &str, token: &str) -> Option<Value> {
    let client = blocking::Client::new();
    let uri = ["https://europe.api.riotgames.com/lol/match/v5/matches/", matchid].join("");

    loop {
        let response = client
        .get(&uri)
        .header("X-Riot-Token", token)
        .send();

        match response {
            Ok(res) => {
                if res.status().is_success() {
                    let body = res.text().unwrap();
                    let games: Value = from_str(&body).unwrap();
                    return Some(games);
                } else if res.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
                    if let Some(retry_after) = res.headers().get("Retry-After") {
                        if let Ok(retry_seconds) = retry_after.to_str().unwrap_or(&DEFAULT_RETRY_TIME.to_string()).parse::<u64>() {
                            // print!(" /!\\ ({:?})", retry_seconds);
                            // let _ = std::io::stdout().flush();
                            sleep(Duration::from_secs(retry_seconds));
                        }
                    } else {
                        // print!(" /!\\ ({:?})", DEFAULT_RETRY_TIME);
                        // let _ = std::io::stdout().flush();
                        sleep(Duration::from_secs(DEFAULT_RETRY_TIME));
                    }
                } else {
                    print!("\rMatch API returned code {:?}", res.status());
                    let _ = std::io::stdout().flush();
                    return None;
                }
            }
            Err(_) => {
                return None;
            }
        }
    }
}

// We can use every game ten times as training data! Once for each player
fn prepare_training_data(games: Vec<Value>) -> Array2<f32> {
    let mut data_vec: Vec<f32> = vec![];
    let columns = &games.len() * 10;

    for game in games {
        let participants_value = game.get("info").unwrap().get("participants").unwrap();
        let participants: Vec<Value> = from_value(participants_value.clone()).unwrap();
        // Add one data row for each participant
        for i in 0..participants.len() {
            let player: &Value = &participants[i];
            let enemies: Vec<&Value> = participants.iter().filter(|p| p.get("teamId").unwrap() != player.get("teamId").unwrap()).collect();

            let enemy_champs: Vec<f32> = enemies.iter()
                .map(|e| from_value::<f32>(e.get("championId").unwrap().clone()).unwrap()).collect();


            let enemy_trees: Vec<f32> = enemies.iter()
                .map(|e| e.get("perks").unwrap().get("styles").unwrap()[0].clone())
                .map(|s| from_value::<f32>(s.get("style").unwrap().clone()).unwrap())
                .collect();

            let player_champ = from_value::<f32>(player.get("championId").unwrap().clone()).unwrap();
            let player_tree = from_value::<f32>(player.get("perks").unwrap().get("styles").unwrap().get(0).unwrap().get("style").unwrap().clone()).unwrap();
            
            let mut player_items: Vec<f32> = vec![];
            for i in 0..=6 {
                player_items.push(from_value::<f32>(player.get(format!("item{}", i)).unwrap().clone()).unwrap());
            }
            let boot_ids: Vec<f32> = BOOT_IDS.iter().map(|i| i.0).collect();
            let player_boots: f32 = player_items.iter().filter(|item| boot_ids.contains(item)).nth(0).unwrap_or(&1001.).clone();

            let mut entry: Vec<f32> = vec![
                player_champ,
                player_tree,
                enemy_champs[0],
                enemy_trees[0],
                enemy_champs[1],
                enemy_trees[1],
                enemy_champs[2],
                enemy_trees[2],
                enemy_champs[3],
                enemy_trees[3],
                enemy_champs[4],
                enemy_trees[4],
                player_boots as f32,
            ];

            let _ = data_vec.append(&mut entry);
        }
    }

    Array2::from_shape_vec((columns, 13), data_vec).unwrap()
}

fn prepare_game_data(game: Value, summoner_id: &str) -> Option<Array1<f32>> {
    let participants_value = game.get("participants").unwrap();
    let participants: Vec<Value> = from_value(participants_value.clone()).unwrap();
    for i in 0..participants.len() {
        let player: &Value = &participants[i];
        let part_id = player.get("summonerId").unwrap().as_str().unwrap();
        if part_id != summoner_id {
            continue;
        }

        let enemies: Vec<&Value> = participants.iter().filter(|p| p.get("teamId").unwrap() != player.get("teamId").unwrap()).collect();

        // Make triple-sure we are in the correct gamemode...
        let gamemode = game.get("gameMode").unwrap().as_str().unwrap();
        let queue_id = game.get("gameQueueConfigId").unwrap().as_u64().unwrap();
        if gamemode != "ARAM" || enemies.len() != 5 || queue_id != 450 {
            return None;
        }

        let enemy_champs: Vec<f32> = enemies.iter()
            .map(|e| from_value::<f32>(e.get("championId").unwrap().clone()).unwrap()).collect();


        let enemy_trees: Vec<f32> = enemies.iter()
            .map(|e| from_value::<f32>(e.get("perks").unwrap().get("perkStyle").unwrap().clone()).unwrap())
            .collect();

        let player_champ = from_value::<f32>(player.get("championId").unwrap().clone()).unwrap();
          let player_tree = from_value::<f32>(player.get("perks").unwrap().get("perkStyle").unwrap().clone()).unwrap();
            
        let player_boots: f32 = 0.; // dummy value as we do not know this

        let entry: Vec<f32> = vec![
            player_champ,
            player_tree,
            enemy_champs[0],
            enemy_trees[0],
            enemy_champs[1],
            enemy_trees[1],
            enemy_champs[2],
            enemy_trees[2],
            enemy_champs[3],
            enemy_trees[3],
            enemy_champs[4],
            enemy_trees[4],
            player_boots,
        ];
        return Some(Array1::from_shape_vec(13, entry).unwrap());
    }

    None
}

fn print_loading_bar(progress: usize, total: usize, duplicates: usize) {
    let bar_length = 20; // Adjust the length of the loading bar as needed
    let progress_percentage = (progress as f32 / total as f32) * 100.0;
    let hashes = ((progress as f32 / total as f32) * bar_length as f32) as usize;

    print!("\r[");
    for _ in 0..hashes {
        print!("#");
    }
    for _ in 0..(bar_length - hashes) {
        print!(" ");
    }
    print!("] {:.2}% ({:?} dupes)", progress_percentage, duplicates);

    // Flush the output to make the progress visible without newline
    let _ = std::io::stdout().flush();
}
