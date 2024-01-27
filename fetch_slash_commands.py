import requests

def fetch_global_commands(bot_token, application_id):
    url = f"https://discord.com/api/v8/applications/{application_id}/commands"
    headers = {
        "Authorization": f"Bot {bot_token}"
    }

    response = requests.get(url, headers=headers)
    if response.status_code == 200:
        return response.json()
    else:
        return f"Error: {response.status_code}, {response.text}"

def fetch_guild_commands(bot_token, application_id, guild_id):
    url = f"https://discord.com/api/v8/applications/{application_id}/guilds/{guild_id}/commands"
    headers = {
        "Authorization": f"Bot {bot_token}"
    }

    response = requests.get(url, headers=headers)
    if response.status_code == 200:
        return response.json()
    else:
        return f"Error: {response.status_code}, {response.text}"

# Replace these with your bot's token and application ID
BOT_TOKEN = "Mzg3MjY5Nzc1MjY2NDE0NTky.Gqo0uW.jHWyManf4PpNJsB70Jo9OgtiMekINti_YUDhV4"
APPLICATION_ID = "387269775266414592"

# If you want to check guild-specific commands, also replace this with your guild ID
GUILD_ID = "308708637679812608"

# Fetch and print global commands
global_commands = fetch_global_commands(BOT_TOKEN, APPLICATION_ID)
print("Global Commands:", global_commands)

# Fetch and print guild-specific commands
# Uncomment the lines below to use
# guild_commands = fetch_guild_commands(BOT_TOKEN, APPLICATION_ID, GUILD_ID)
# print("Guild Commands:", guild_commands)
