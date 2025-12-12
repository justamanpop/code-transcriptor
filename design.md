The idea is to load the model in memory and keep a daemon with the loaded model running, ready to be called to transcribe. 

A separate program will listen to the microphone input from when user indicates start to till they indicate stop (via a hotkey), and have that input be put in a wav file. 

Then it calls the daemon with the Whisper model, pointing it to this temp wav file, and getting back transcription as a result. Communication between the two will happen via a UNIX socket. 

The temp file is then deleted, and the transcription is returned over the socket by the daemon.

Additionally, some cleaning of transcription output will be required, like replacing dot with ".", semi-colon with ";", and so on.
Instead of doing it on the model itself, I will be doing this with a program, so that it's more deterministic, does not require the AI knowledge of finetuning a model or prompt engineering.

The final step is to integrate this entire flow with a text editor/IDE. The plan is to have a hotkey to start and stop recording, and then have the transcription result automatically appended to the current file when recording is stopped.

Tech stack:
1. Python for Whisper daemon, since that's the only interface available to Whisper apart from CLI, which sadly can't keep the model in memory.
2. Use the `arecord` shell command for recording
3. Rust for the program that writes the file path to the UNIX socket the daemon listens on. I will call a C FFI from lua to make that happen, as that's much faster than spawning a new process and running the rust program.
4. Neovim (and so, lua) as editor to integrate the above with, as I am familiar with how to make plugins there.
5. Rust as the language for the script that cleans the received transcription to make it programming suitable. Again because of speed.
