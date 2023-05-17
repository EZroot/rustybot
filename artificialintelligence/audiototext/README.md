USAGE INSTRUCTIONS
generally you use the mic capture server on windows/whatever has your mic
and use the AI on linux (WSL, or whatever)


mic2stream_server (run on windows or wherever sound drivers are)
* captures mic, listens as a server

stream2audio_client (doesnt matter, i pick wsl2)
* connects to mic2stream, gathers audio
* does basic detection to try to cipher out commands
* once it ciphers a command and it saves it, REQUEST transcribe_ai_server to transcribe it

transcribe_ai_server (doesnt matter)
* convert to mp3 incase corrupted bytes
* converts and returns text
* send requests to rustybot server /derp?command={} 
//try to spell correct?
//get context? (is it a command?)
//try to get command to use and prompt