local cmd = {
	"arecord",
	"-f",
	"S16_LE",
	"-c",
	1,
	"-r",
	16000,
	"/tmp/nvim_recording.wav",
}

local clock = os.clock
local function sleep(n) -- seconds
	local t0 = clock()
	while clock() - t0 <= n do
	end
end

local recording_job = vim.fn.jobstart(cmd)
sleep(4)
vim.fn.jobstop(recording_job)
