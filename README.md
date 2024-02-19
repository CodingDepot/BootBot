# Boot-Bot 

![alt text](https://ddragon.leagueoflegends.com/cdn/7.22.1/img/item/2422.png "Somewhat Miraculous Shoes")

## Description

This bot helps you in solve the biggest struggle in your daily life: **Which boots should you buy in this particular game of ARAM?**

Search no further because now we can employ cutting-edge machine learning to determine your appropriate sense of fashion for every situation.
By training a model on ~~dozens~~ thousands of ARAM games in your MMR range you can ensure your choice of footwear is never out of place.
Remember: Fashion is not a choice, but a necessity.

## Setup

### How To Build

Thanks to the power of the crustacean, building is as easy as can be. Just [download cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) and run `cargo build --release`. That's all there is to it.
The repository also contains a minimal [Dockerfile](https://github.com/CodingDepot/BootBot/blob/main/Dockerfile) for all your containerization needs.

If that still sounds like a hassle, the [docker images are public](https://hub.docker.com/r/codingdepot/boot-bot). But they are for personal use so I will not take questions regarding their internal well-being.

### How To Run

After building the binary, you can run it as you would run any executable file in your OS. However, there are some things the bot needs in order to properly function:

#### Environment Variables

These variables need to be set in the execution environment:

| Name | Function | Required? |
| ---- | -------- | --------- |
| RIOT_TOKEN | The token for the Riot API | Yes |
| DISCORD_TOKEN | The token for the Discord API | Yes |
| VIP_USER | Discord Snowflake of a privileged user that may use configuration commands | Yes |
| VIP_GUILD | Discord Snowflakle of a guild that has configuration commands enabled | Yes |
| GAME_VERSION | The LoL version used for embedded item sprites | Yes |
| CONFIG_PATH | The base path for startup files | No |

If CONFIG_PATH is not set, the program expects these files to be in the same directory as the executable.

#### Startup Files

| Filename | Content | Required? |
| -------- | ------- | --------- |
| snowflake_puuid.txt | A Mapping of Discord Snowflakes to Riot PUUIDs, one line per mapping. Syntax: `<Snowflake>\|<PUUID>` | Yes |
| model.bin | A pre-trained model. If none is provided, the bot won't be able to make predictions until a model has been trained | No |

The mapping file is convenient for small, personal use. However, the desired solution is to query Discord connections to get the Riot usernames we need.
Also, even when adding a pre-trained model remember to start training a new model after starting if the data is outdated.

## Commands

| Name | Options | Function | Privileged? |
| ---- | ------- | -------- | ----------- |
| /boots | \<\[user\]\> | Suggests the best possible choice of boots for the user if they are in an ARAM match | No |
| /model | \<number of games\> | Re-trains the model with the specified number of games. The old model will continue being used until training concludes | Yes |
| /version | \<game version\> | Updates the version string for the LoL embed downlaods. Returns an error if the new version leads to an invalid download link | Yes |

## FAQ

### How do you handle Cassiopeia?

I do not. Please do not ask the bot for a suggestion when playing Cassiopeia, since it there is a slim chance it leads to the technological singularity.

### The Bot recommended \<Boots\> on \<Champion\>, why?

Do not question the bot, for it does not question you.

### I am a RIOT employee and can't decide whether to grant you a personal API token, what should I do?

Please, I currently can only use the development token for 2 hours before spending 22 hours solving the captcha to generate the next token.

### Has anyone ever actually asked these questions?
