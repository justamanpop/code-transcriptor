local ffi = require("ffi")

ffi.cdef([[
    int add(int a, int b);
]])

local lib = ffi.load("/home/anishs/development/voice_to_code/rust_client/target/release/libtranscript_processor.so")

local result = lib.add(10, 20)
print(result)
