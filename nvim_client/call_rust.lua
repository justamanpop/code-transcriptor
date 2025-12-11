local ffi = require("ffi")

ffi.cdef([[
    char* transcribe_audio(const char* audio_file_path, const char* socket_file_path);
]])

local lib = ffi.load("/home/anishs/development/voice_to_code/rust_client/target/release/libtranscript_processor.so")

local result = lib.transcribe_audio("/tmp/nvim_recording.wav", "/tmp/whisper_daemon.sock")
print(ffi.string(result))
