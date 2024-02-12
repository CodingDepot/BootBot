# requires environment variables:

RIOT_TOKEN              The token for the Riot API

DISCORD_TOKEN           The token for the Discord API

VIP_USER                Snowflake of the user that may execute configuration commands

VIP_GUILD               Snowflake of the guild that has configuration commands

GAME_VERSION            The LoL version used for the embed downloads

CONFIG_PATH             The base path for the files listed below


# needs the following files:

snowflake_puuid.txt     Maps Discord Snowflakes to Riot PUUIDs, each on a new line, separated by pipe symbols

model.bin               The DecisionTree. Can be recreated by the bot