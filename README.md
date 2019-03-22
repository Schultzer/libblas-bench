## libblas-bench
[![CircleCI](https://circleci.com/gh/Schultzer/libblas-bench.svg?style=svg)](https://circleci.com/gh/Schultzer/libblas-bench)

Looking at the benchmark running on circleci libblas is eqaul or faster then Intel-MKL.

### Intel-MKL on CircleCI

[https://circleci.com/gh/Schultzer/libblas-bench/6]
f64
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

[https://circleci.com/gh/Schultzer/libblas-bench/8]
f32
```
libblas::level3::gemm   time:   [543.37 ns 550.91 ns 560.73 ns]
                        change: [-25.004% -23.231% -21.451%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 13 outliers among 100 measurements (13.00%)
  4 (4.00%) high mild
  9 (9.00%) high severe

libblas::level3::gemm big
                        time:   [12.651 ms 12.681 ms 12.715 ms]
                        change: [-20.961% -20.657% -20.298%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high severe

libblas::level1::axpy   time:   [270.06 ns 273.36 ns 278.02 ns]
                        change: [-16.187% -15.304% -14.294%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 16 outliers among 100 measurements (16.00%)
  7 (7.00%) high mild
  9 (9.00%) high severe

libblas::level1::axpy-big
                        time:   [4.4124 ms 4.5732 ms 4.7715 ms]
                        change: [-43.800% -41.951% -40.151%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe

blas-src intel-mkl sgemm
                        time:   [816.25 ns 823.24 ns 834.70 ns]
Found 11 outliers among 100 measurements (11.00%)
  3 (3.00%) high mild
  8 (8.00%) high severe

blas-src intel-mkl sgemm big
                        time:   [10.907 ms 11.568 ms 12.259 ms]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

blas-src intel-mkl saxpy
                        time:   [293.38 ns 299.04 ns 305.96 ns]
Found 16 outliers among 100 measurements (16.00%)
  1 (1.00%) high mild
  15 (15.00%) high severe

blas-src intel-mkl saxpy big
                        time:   [11.727 ms 12.540 ms 13.382 ms]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
```