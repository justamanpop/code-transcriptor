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
end

local function toggle_recording()
	if recording_job == nil then
		start_recording()
		print("recording started")
	else
		stop_recording()
		print("recording stopped")
	end
end

vim.api.nvim_create_user_command("VoiceToggle", toggle_recording, {})
vim.keymap.set("n", "<leader>vt", toggle_recording, { desc = "Voice: Toggle record" })
