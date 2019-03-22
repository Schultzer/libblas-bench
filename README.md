## libblas-bench
[![CircleCI](https://circleci.com/gh/Schultzer/libblas-bench.svg?style=svg)](https://circleci.com/gh/Schultzer/libblas-bench)

Looking at the benchmark running on circleci libblas is eqaul or faster then Intel-MKL.

[https://circleci.com/gh/Schultzer/libblas-bench/6]
### Intel-MKL on CircleCI
```
libblas::level3::gemm   time:   [641.57 ns 652.71 ns 667.59 ns]
                        change: [-11.880% -10.443% -9.1000%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 18 outliers among 100 measurements (18.00%)
  6 (6.00%) high mild
  12 (12.00%) high severe

libblas::level3::gemm big
                        time:   [15.611 ms 15.626 ms 15.641 ms]
                        change: [-2.7776% -2.6176% -2.4762%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild

libblas::level1::axpy   time:   [321.50 ns 321.62 ns 321.75 ns]
                        change: [-0.6531% +0.2894% +1.5652%] (p = 0.69 > 0.05)
                        No change in performance detected.
Found 11 outliers among 100 measurements (11.00%)
  6 (6.00%) high mild
  5 (5.00%) high severe

libblas::level1::axpy-big
                        time:   [7.4474 ms 7.4861 ms 7.5310 ms]
                        change: [-8.6585% -7.7727% -6.9733%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  4 (4.00%) high severe

blas-src intel-mkl dgemm
                        time:   [973.46 ns 990.24 ns 1.0107 us]
Found 9 outliers among 100 measurements (9.00%)
  8 (8.00%) high mild
  1 (1.00%) high severe

blas-src intel-mkl dgemm big
                        time:   [13.905 ms 14.388 ms 14.916 ms]
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) low mild
  4 (4.00%) high mild
  3 (3.00%) high severe

blas-src intel-mkl daxpy
                        time:   [327.89 ns 327.95 ns 328.02 ns]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

blas-src intel-mkl daxpy big
                        time:   [15.818 ms 16.607 ms 17.553 ms]
```