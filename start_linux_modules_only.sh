#!/bin/bash

# Run the first Python file in a new terminal window
xterm -e "python3 artificialintelligence/stable_diffusion_server.py" &

# Run the second Python file in a new terminal window
xterm -e "python3 artificialintelligence/audiototext/transcribe_ai_server.py" &

# Run the third Python file in a new terminal window
xterm -e "python3 artificialintelligence/audiototext/stream2audio_command_client.py" &

# Run the Node.js server in the current terminal window
node authserver/authserver.js
