# Methodology
All results except Pypy were gathered from the same run of `benchmark.py`. Pypy
was run in the same method in a different run and different virtualenv.

# Results
Result is the average time of ten different runs. Each run is for an image of
size 1024 and 50 iterations.

 - Debug Rust: 48.01
 - Release Rust: 1.41
 - Debug Mixed: 51.01
 - Release Mixed: 1.61
 - Python: 100.59
 - Pypy: 11.56

# Environment

`Linux fe2o3 5.0.6-arch1-1-ARCH #1 SMP PREEMPT Wed Apr 3 09:35:20 UTC 2019 x86_64 GNU/Linux`

CPU: `Intel(R) Core(TM) i7-6700HQ CPU @ 2.60GHz`

Language stuff:
 - Rustc 1.33.0 (2aa4c46cf 2019-02-28)
 - Python 3.7.3
 - PyPy 7.1.0-beta0 with GCC 8.2.1 20181127
