import sys
import argparse

from cffi import FFI

DEBUG_PATH = "./target/debug/librandom_blending_ffi.so"
RELEASE_PATH = "./target/release/librandom_blending_ffi.so"

ffi = FFI()
ffi.cdef("""
    int random_blending_c(int, int, int*, char*);
""")

PROBABILITIES = [
    1, 8, 8,
    1, 0, 8,
    1, 1, 1,
]

probabilities = ffi.new('int[9]')

for i in PROBABILITIES:
    probabilities[i] = ffi.cast("int", PROBABILITIES[i])

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('size', type=int, help="how big the image will be")
    parser.add_argument('iterations', type=int, help='how many iterations to execute')
    parser.add_argument('--release', help="use release .so", action='store_true')
    args = parser.parse_args()

    if args.release:
        C = ffi.dlopen(RELEASE_PATH)
    else:
        C = ffi.dlopen(DEBUG_PATH)

    out_path = ffi.new("char[]", "output.png".encode())
    ret = C.random_blending_c(args.size, args.iterations, PROBABILITIES, out_path)

    if ret != 0:
        print("Generation failed")
        sys.exit(1)
    else:
        print("Generation successs")

if __name__ == '__main__':
    main()
