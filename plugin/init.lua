local ffi = require("ffi")
local async = require("plenary.async")

ffi.cdef([[
    char* transcribe_audio(const char* audio_file_path, const char* socket_file_path);
]])
local lib = ffi.load("/home/anishs/development/voice_to_code/rust_client/target/release/libtranscript_processor.so")

function get_transcription()
	local transcript = lib.transcribe_audio("/tmp/nvim_recording.wav", "/tmp/whisper_daemon.sock")
	return ffi.string(transcript)
end

local record_cmd = {
	"arecord",
	"-f",
	"S16_LE",
	"-c",
	1,
	"-r",
	16000,
	"/tmp/nvim_recording.wav",
}

local recording_job = nil

local function start_recording()
	recording_job = vim.fn.jobstart(record_cmd)
end

local function stop_recording()
	vim.fn.jobstop(recording_job)
	recording_job = nil
end

local function on_stop_recording()
	print("recording stopped, generating transcript")
	local transcript = get_transcription()
	print("generated transcript, writing to file")
	local line_count = vim.api.nvim_buf_line_count(0)
	local lines = vim.split(transcript, "\n", { plain = true })
	vim.api.nvim_buf_set_lines(0, line_count, line_count, false, lines)
end

local function toggle_recording_and_append()
	if recording_job == nil then
		start_recording()
		print("recording started")
	else
		stop_recording()
		async.run(on_stop_recording)
	end
end

vim.api.nvim_create_user_command("VoiceToggle", toggle_recording_and_append, {})
vim.keymap.set("n", "<leader>vt", toggle_recording_and_append, { desc = "Voice: Toggle record" })
