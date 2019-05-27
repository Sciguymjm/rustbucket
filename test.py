# -*- coding: utf-8 -*-
from cffi import FFI
import numpy as np
ffi = FFI()
# ffi.cdef("""
#     void print_text(const char*);
# typedef struct {
#         int header_size, header_crc, major_version, minor_version;
#  } ReplayObj;
#
# ReplayObj parse(const char*);
# """)
with open(r"target\debug\build\rustbucket-71f604b9f2ce340b\out\my_lib_ffi.h", "r") as f:
    text = f.readlines()
    text = "\n".join([l for l in text if not l.startswith("#include")])
    ffi.cdef(text)
    # example:
    """
    
typedef struct {
  int32_t header_size;
  int32_t header_crc;
  int32_t major_version;
  int32_t minor_version;
} ReplayObj;

int32_t dub(int32_t x);

ReplayObj *parse(const char *text);

void print_text(const char *text);

    """
C = ffi.dlopen(r"target\debug\rustbucket.dll")


def to_cstring(text):
    return ffi.new("char[]", text.encode("utf-8"))

replay = C.parse(to_cstring("C:\\Users\\Matt\\PycharmProjects\\boxcars\\assets\\edbb.replay"))

print("Python Major Version {}".format(replay.major_version))
float_out = ffi.new("float[3]")

nnums = np.ones(3, dtype=np.float64)
print(np.frombuffer(ffi.buffer(replay.arr.data, 3*8), dtype=np.float64))