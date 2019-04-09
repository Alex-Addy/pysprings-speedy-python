from cffi import FFI

ffi = FFI()
ffi.cdef("""
    int random_blending_c(int, int, int*, char*);
""")

C = ffi.dlopen("./target/debug/librandom_blending_ffi.so")

PROBABILITIES = [
    1, 8, 8,
    1, 0, 8,
    1, 1, 1,
]

SIZE = 128
ITERATIONS = 50

probabilities = ffi.new('int[9]')

for i in PROBABILITIES:
    probabilities[i] = ffi.cast("int", PROBABILITIES[i])

out_path = ffi.new("char[]", "output.png".encode())
print(C.random_blending_c(SIZE, ITERATIONS, PROBABILITIES, out_path))
