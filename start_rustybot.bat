@echo off
start python "C:\Repos\rustybot\artificialintelligence\audiototext\mic2stream_server.py"
start cmd /k "cd C:\Repos\rustybot && cargo run"
start wsl -e bash -lic "cd /mnt/c/Repos/rustybot/artificialintelligence/ ; python3 stable_diffusion_server.py; exec bash"
start wsl -e bash -lic "cd /mnt/c/Repos/rustybot/artificialintelligence/audiototext/ ; python3 stream2audio_command_client.py; exec bash"
start wsl -e bash -lic "cd /mnt/c/Repos/rustybot/artificialintelligence/audiototext/ ; python3 transcribe_ai_server.py; exec bash"
start wsl -e bash -lic "cd /mnt/c/Repos/rustybot/authserver/ ; node authserver.js; exec bash"