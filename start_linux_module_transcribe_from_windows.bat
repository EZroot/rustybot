start wsl -e bash -lic "cd /mnt/c/Repos/rustybot/artificialintelligence/audiototext/ ; python3 transcribe_ai_server.py; exec bash"
start wsl -e bash -lic "cd /mnt/c/Repos/rustybot/artificialintelligence/audiototext/ ; python3 stream2audio_command_client.py; exec bash"
start python "C:\Repos\rustybot\artificialintelligence\audiototext\mic2stream_server.py"
