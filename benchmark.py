import sys
import time
import argparse

from typing import List
from functools import partial

try:
    import sh
    import PIL
    import cffi
except ImportError:
    print("Pillow, sh, and cffi must be installed in order to run benchmarks")
    sys.exit(1)

def run_pure_python(size: int, iterations: int) -> float:
    """
    Run the pure python implementation with the given values.

    :returns: The execution time in seconds.
    """
    start_time = time.perf_counter()
    sh.python("./pure_python/random_blending.py", size, iterations)
    stop_time = time.perf_counter()
    return stop_time - start_time

def run_pure_rust(release: bool, size: int, iterations: int) -> float:
    path = "./pure_rust/target/{}/random_blending".format("release" if release else "debug")
    cmd = sh.Command(path)
    start_time = time.perf_counter()
    cmd(size, iterations)
    stop_time = time.perf_counter()
    return stop_time - start_time

def run_mixed_impl(release: bool, size: int, iterations: int) -> float:
    if release:
        args = [size, iterations, "--release"]
    else:
        args = [size, iterations]
    start_time = time.perf_counter()
    sh.python("./mixed-impl/random_blending.py", *args)
    stop_time = time.perf_counter()
    return stop_time - start_time

def run_and_collect(name: str, fn, times: int) -> List[float]:
    """
    Run the given 0-arg function the given number of times.
    
    Expects the functions to return a float.
    :returns: A list of return values
    """
    floats = []
    for n in range(times):
        print("Running {}, exec #{}: ".format(name, n), end='')
        result = fn()
        print("{}".format(result))
        floats.append(result)
    return floats

def main(size: int, iterations: int, times: int, python_only: bool):
    if python_only:
        pure_python = partial(run_pure_python, size, iterations)
        pure_python_runs = run_and_collect("pure_python", pure_python, times)
        print("Pure python average time:", sum(pure_python_runs)/times)
        return

    # Create 0-arg fns for each scenario
    pure_python = partial(run_pure_python, size, iterations)
    dbg_pure_rust = partial(run_pure_rust, False, size, iterations)
    rls_pure_rust = partial(run_pure_rust, True, size, iterations)
    dbg_mixed_impl = partial(run_mixed_impl, False, size, iterations)
    rls_mixed_impl = partial(run_mixed_impl, True, size, iterations)

    # collect run times
    pure_python_runs = run_and_collect("pure_python", pure_python, times)
    dbg_rust_runs = run_and_collect("dbg_pure_rust", dbg_pure_rust, times)
    rls_rust_runs = run_and_collect("rls_pure_rust", rls_pure_rust, times)
    dbg_mixed_runs = run_and_collect("dbg_mixed_impl", dbg_mixed_impl, times)
    rls_mixed_runs = run_and_collect("rls_mixed_impl", rls_mixed_impl, times)

    # Calculate and print averages
    print("All units are in seconds")
    print("Pure python average time:  ", sum(pure_python_runs)/times)
    print("Debug rust average time:   ", sum(dbg_rust_runs)/times)
    print("Release rust average time: ", sum(rls_rust_runs)/times)
    print("Debug mixed average time:  ", sum(dbg_mixed_runs)/times)
    print("Release mixed average time:", sum(rls_mixed_runs)/times)

if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument("--python-only", action='store_true', default=False)
    parser.add_argument('--size', type=int, help="how big the image will be", default=1024)
    parser.add_argument('--iterations', type=int, help='how many iterations to execute', default=50)
    parser.add_argument('--times', type=int, help="how many times to execute each type", default=10)
    args = parser.parse_args()
    main(args.size, args.iterations, args.times, args.python_only)
