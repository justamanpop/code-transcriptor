WisprFlow is not available on Linux (or android), so this is an attempt to replicate it using the open source Whisper model.

The idea is to load the model in memory and keep a daemon with the lodaed model running, ready to be called to transcribe. 

A separate program will listen to the microphone input from when user says start to till they say stop, and have that input be put in a wav file. Then it calls the daemon in step 2 pointing it to this temp wav file, getting back transcription as a result. The temp file is then deleted, and the text returned by this program.

Finally, will have to try and integrate this with an editor like neovim by making it on some hotkey, call the recording program and start listening, with another hotkey to make it stop listening. Once it gets the output, that can be written on to the screen.

Additionally, some cleaning will be required. Instead of doing it on the model itself, I thought of doing this with a program, so that it's more deterministic, requires less AI knowledge to finetune model and prompt it correctly and all, instead just replacing common programming things with the characters I want. 

Tech stack:
1. Python for Whisper daemon, since that's the only interface available apart from a CLI tool, which sadly can't keep the model in memory.
2. Rust for the recording app, I can have it record voice and call python daemon pointing to wav file with recorded voice. It's fast, and so I will go with it.
3. Neovim (and so, lua) as editor to integrate the above with, as I am familiar with how to make plugins there.
4. Rust as the language to write cleaning replacement script after receving transcription to make it programming suitable. Again because of speed.
