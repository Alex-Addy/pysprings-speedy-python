import sys
import argparse

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

probabilities = ffi.new('int[9]')

for i in PROBABILITIES:
    probabilities[i] = ffi.cast("int", PROBABILITIES[i])


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('size', type=int, help="how big the image will be")
    parser.add_argument('iterations', type=int, help='how many iterations to execute')
    args = parser.parse_args()

    out_path = ffi.new("char[]", "output.png".encode())
    ret = C.random_blending_c(args.size, args.iterations, PROBABILITIES, out_path)

    if ret != 0:
        print("Generation failed")
        sys.exit(1)
    else:
        print("Generation successs")

if __name__ == '__main__':
    main()
