# Project Purpose
This project was created to demonstrate and benchmark one form of Python-Rust
FFI for the Colorado Springs Python user group called PySprings.

# Benchmarking
I have included results from running the benchmark in the
`benchmark_results.md` file in this repo.

## Running the benchmark
There are two steps to running the benchmarks. The first is easy but will take
a while, simply run the following commands:

```
$> virtualenv venv --python=python3
$> source venv/bin/activate
(venv) $> pip install -r benchmark.reqs.txt
(venv) $> python benchmark.py
```

After running those commands the results should be printed for you. However
these results do not contain the run for pypy. In order to benchmark pypy some
extra steps are required.

First, create a virtualenv with pypy as the interpreter:
```
$> virtualenv pypy-venv --python=$(which pypy3)
$> source pypy-venv/bin/activate
```

Now install the dependencies into this new environment. The installation of
`pillow` may require you to have the `libjpeg-dev` headers installed.
```
(pypy-venv) $> pip install -r bencmarks.reqs.txt
```

Now run `benchmark.py` with the `--python-only` flag to get your `pypy` times.

```
(pypy-venv) $> python benchmark.py --pure-python
```
