local uv = vim.uv or vim.loop

function get_transcription(filetype)
	local ffi = require("ffi")
	ffi.cdef([[
    char* transcribe_audio(const char* audio_file_path, const char* socket_file_path, const char* filetype);
    void free_string(char* s);
]])
	local lib = ffi.load("/home/anishs/development/voice_to_code/rust_client/target/release/libtranscript_processor.so")
	local response = lib.transcribe_audio("/tmp/nvim_recording.wav", "/tmp/whisper_daemon.sock", filetype)
	local transcript = ffi.string(response)
	lib.free_string(response)
	return transcript
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

local function toggle_recording_and_append(position)
	if recording_job == nil then
		start_recording()
		print("recording started")
	else
		stop_recording()
		local cursor_row_1_indexed, _ = unpack(vim.api.nvim_win_get_cursor(0))
		print("recording stopped, generating transcription")
		uv.new_work(get_transcription, function(transcript)
			print("generated transcript, writing to file")
			vim.schedule(function()
				local lines = vim.split(transcript, "\n", { plain = true })
				if position == "end" then
					local line_count = vim.api.nvim_buf_line_count(0)
					vim.api.nvim_buf_set_lines(0, line_count, line_count, false, lines)
				else
					vim.api.nvim_buf_set_lines(0, cursor_row_1_indexed, cursor_row_1_indexed, false, lines)
				end
				vim.api.nvim_command("write")
			end)
		end):queue(vim.bo.filetype)
	end
end

vim.api.nvim_create_user_command("VoiceToggle", toggle_recording_and_append, {})
vim.keymap.set("n", "<leader>vt", function()
	toggle_recording_and_append("end")
end, { desc = "Voice: Toggle record and append to end" })
vim.keymap.set("n", "<leader>vj", function()
	toggle_recording_and_append("curr")
end, { desc = "Voice: Toggle record and append below curr line" })
