import requests

def clear_slash_commands(bot_token, application_id):
    headers = {
        "Authorization": f"Bot {bot_token}"
    }
    url = f"https://discord.com/api/v8/applications/{application_id}/commands"

    response = requests.get(url, headers=headers)
    if response.status_code == 200:
        commands = response.json()
        for command in commands:
            delete_url = f"{url}/{command['id']}"
            delete_response = requests.delete(delete_url, headers=headers)
            if delete_response.status_code == 204:
                print(f"Deleted command {command['name']}")
            else:
                print(f"Failed to delete command {command['name']}")
    else:
        print("Failed to retrieve commands")

# Replace 'your_bot_token' with your bot's token and 'your_application_id' with your application's ID
clear_slash_commands('Mzg3MjY5Nzc1MjY2NDE0NTky.Gqo0uW.jHWyManf4PpNJsB70Jo9OgtiMekINti_YUDhV4', '387269775266414592')
