use std::{collections::HashMap, fs::File, io::{BufRead, BufReader, Read}, time::{SystemTime, UNIX_EPOCH}, env};

use linfa::{metrics::ToConfusionMatrix, traits::{Fit, Predict}, Dataset, DatasetBase};
use linfa_trees::{DecisionTree, SplitQuality};
use ndarray::{s, Array1, Array2, ArrayBase, Axis, Dim, OwnedRepr};
use bincode;

use self::data::get_current_match_data;

pub mod data;

const MODEL_FILE_NAME: &str = "model.bin";
const GOETHE: &str = "qan7meW9JWz1XI1nmZ8yD000EXpxLGkSbirRVPaRjCwcr9WeIcg32KQOTtJV71OEyov3LwCnRq5o5Q";

fn predict_one(model: &DecisionTree<f32, String>, data: &Array1<f32>) -> String {
    let data_boxed: Array2<f32> = data.clone().into_shape((1, 13)).unwrap();

    predict_multiple(&model, &data_boxed).get(0).unwrap().clone()
}

fn predict_multiple(model: &DecisionTree<f32, String>, data: &Array2<f32>) -> Vec<String> {
    let prediciton = model.predict(data);
    prediciton.as_slice().unwrap().to_owned()
}

fn test_tree_accuracy(model: &DecisionTree<f32, String>, training_data: &Array2<f32>) -> f32 {
    let test = create_dataset(&training_data);

    let conf_matrix = model.predict(&test).confusion_matrix(&test).unwrap();
    conf_matrix.accuracy()
}

fn create_dataset(data: &Array2<f32>) -> DatasetBase<ArrayBase<OwnedRepr<f32>, Dim<[usize; 2]>>, ArrayBase<OwnedRepr<String>, Dim<[usize; 1]>>>{
    // set features and labels
    let feature_names = vec!["Champion", "Runes", "Enemy1 Champ", "Enemy1 Runes", "Enemy2 Champ", "Enemy2 Runes", "Enemy3 Champ", "Enemy3 Runes", "Enemy4 Champ", "Enemy4 Runes", "Enemy5 Champ", "Enemy5 Runes"];
    let num_features = data.len_of(Axis(1)) - 1;
    let features = data.slice(s![.., 0..num_features]).to_owned();
    let labels = data.column(num_features).to_owned();

    // create actual dataset (maps ids to String representation)
    let dataset = Dataset::new(features, labels)
        .map_targets(|x| crate::constants::BOOT_IDS.iter().filter(|b| &b.0 == x).nth(0).map(|x| x.1).unwrap_or("No Boots").to_string())
        .with_feature_names(feature_names);

    dataset
}

fn train_decision_tree(data: &Array2<f32>, sq: SplitQuality) -> DecisionTree<f32, String> {
    let dataset = create_dataset(data);

    // fit the model to the data
    let model: DecisionTree<f32, String> = DecisionTree::params()
        .split_quality(sq)
        .fit(&dataset)
        .unwrap();

    model
}

fn save_model(model: &DecisionTree<f32, String>, file_name: &str) {
    let model_file = File::create(file_name).unwrap();
    bincode::serialize_into(&model_file, &model).unwrap();
}

fn load_model(file_name: &str) -> Option<DecisionTree<f32, String>> {
    if let Ok(mut model_file) = File::open(file_name) {
        let mut buffer = Vec::new();
        model_file.read_to_end(&mut buffer).unwrap();
        let deserialized_model: DecisionTree<f32, String> = bincode::deserialize(&buffer).unwrap();
        return Some(deserialized_model)
    }
    None
}

/**
 * Time to fetch data:
 *  500     -   10 min
 *  750     -   14 min
 *  1000    -   21 min
 *  1500    -   32 min
 *  3000    -   63 min (914)
 *  6000    -   128 min (1809)  - 66.65%
 *  10000   -   [210 min]?
 *  20000   -   [420 min]?
 */
pub fn recreate_model(game_count: usize) {
    let token: &str = &env::var("RIOT_TOKEN")
        .expect("Could not fetch the Riot token");
    let snowflake_map = create_snowflake_puuid_map("snowflake_puuid.txt");
    let test_puuid = snowflake_map.values().filter(|id| id.starts_with("f7Xz")).nth(0).unwrap().clone();

    // Train a new model
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let data = data::get_training_data(vec![test_puuid.clone()], game_count, &token);
    let end_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    println!("\nPrepared all game data in {} minutes", (end_time - start_time).as_secs() / 60);
    let tree_model = train_decision_tree(&data, SplitQuality::Gini);

    // Test the accuracy of the new model
    let test_data = data::get_training_data(vec![GOETHE.to_string()], 200, &token);
    let tree_accuracy = test_tree_accuracy(&tree_model, &test_data);
    println!("Tree Accuracy: {:.2}%", 100.0 * tree_accuracy);

    save_model(&tree_model, MODEL_FILE_NAME);
}

pub fn predict(snowflake: &str) -> Option<String> {
    let token: &str = &env::var("RIOT_TOKEN")
        .expect("Could not fetch the Riot token");
    let snowflake_map = create_snowflake_puuid_map("snowflake_puuid.txt");

    if let Some(puuid) = snowflake_map.get(snowflake) {
        if let Some(model) = load_model(MODEL_FILE_NAME) {
            if let Some(data) = get_current_match_data(puuid, &token) {
                return Some(predict_one(&model, &data));
            }
        }
    }
    None
}

fn create_snowflake_puuid_map(file_name: &str) -> HashMap<String, String> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    // Create a HashMap to store the mappings
    let mut map: HashMap<String, String> = HashMap::new();

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line.unwrap();

        // Split the line into two strings based on the '|' character
        let parts: Vec<&str> = line.split('|').collect();
        map.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
    }
    map
}

#[cfg(test)]
mod tests {
    use std::fs::remove_file;
    use ndarray::array;

    use super::*;

    #[test]
    fn test_serialize_deserialize_model() {
        const TEST_FILE: &str = "test_model.bin";

        let data: Array2<f32> = array!(
            [516., 4., 111., 4., 1., 3., 53., 4., 61., 3., 25., 3., 3111.],
            [62., 4., 21., 1., 133., 1., 58., 4., 104., 2., 7., 2., 3047.],
        );
        
        // Create a sample DecisionTree for testing
        let sample_tree = train_decision_tree(&data, SplitQuality::Gini);

        // Save and load the model to a file, delete file afterwards
        save_model(&sample_tree, TEST_FILE);
        let loaded_tree = load_model(TEST_FILE).unwrap();
        let _ = remove_file(TEST_FILE);

        // Assert that the loaded model is equal to the original model
        assert_eq!(sample_tree, loaded_tree);
    }
}