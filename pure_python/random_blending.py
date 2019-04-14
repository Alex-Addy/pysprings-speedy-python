#!/bin/usr/env python3

import copy
import random
import argparse

from typing import List, Tuple

from PIL import Image

#
# Constants
#

PROBABILITIES = [
    1, 8, 8,
    1, 0, 8,
    1, 1, 1,
]

#
# Helper functions
#

def gen_matrix(size: int) -> List[List[int]]:
    return [[ random.randint(0, 255) for _ in range(size) ] for _ in range(size)]

def pick_from_probability() -> int:
    total = sum(PROBABILITIES)
    pick = random.randint(0, total-1)
    choice_idx = 0
    running_total = PROBABILITIES[choice_idx]
    while running_total < pick:
        choice_idx += 1
        running_total += PROBABILITIES[choice_idx]

    return choice_idx

def split_index(idx: int) -> Tuple[int, int]:
    lookup = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), (0, 0), (0, 1),
        (1, -1), (1, 0), (1, 1),
    ]
    return lookup[idx]

def pick_blend_indexes(i: int, j: int, size: int) -> Tuple[int, int]:
    while True:
        (row_diff, col_diff) = split_index(pick_from_probability())
        if 0 <= row_diff + i < size and 0 <= col_diff + j < size:
            break

    return (row_diff + i, col_diff + j)

#
# Core processing
#

def blend_into(size: int, src: List[List[int]], dst: List[List[int]]) -> None:
    for i in range(0, size):
        for j in range(0, size):
            (blend_i, blend_j) = pick_blend_indexes(i, j, size)
            dst[i][j] = (src[i][j] + src[blend_i][blend_j]) // 2

def random_blending(size: int, iterations: int) -> List[List[int]]:
    if iterations <= 0:
        raise ValueError("iterations should be greater than zero")
    if size <= 1: 
        return ValueError("the matrix size needs to be greater than one")

    first = gen_matrix(size)
    second = copy.deepcopy(first)
    swapped = False

    for iter_num in range(1, iterations+1):
        if swapped:
            blend_into(size, second, first)
            swapped = False
        else:
            blend_into(size, first, second)
            swapped = True
        print("Completed iteration {}/{}".format(iter_num, iterations))

    if swapped:
        return second
    else:
        return first

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('size', type=int, help="how big the image will be")
    parser.add_argument('iterations', type=int, help='how many iterations to execute')
    args = parser.parse_args()
    try:
        grayscale = random_blending(args.size, args.iterations)
    except ValueError as exc:
        print("Error while blending: {}".format(exc))
        return
    
    img = Image.new('L', (args.size, args.size))
    for i in range(args.size):
        for j in range(args.size):
            img.putpixel((i, j), grayscale[i][j])
    img.save("output.png")


if __name__ == '__main__':
    main()
