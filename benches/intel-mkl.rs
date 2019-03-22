#[macro_use]
extern crate criterion;
extern crate blas;
extern crate blas_src;
extern crate rand;
use rand::Rng;
use blas::*;
use criterion::Criterion;

fn blas_src_gemm() {
    let mut rng = rand::thread_rng();
    let mut big: Vec<f32> = Vec::new();
    for _ in 0..36 {
        big.push(rng.gen::<f32>())
    }
    criterion::black_box(unsafe {
        sgemm(b'n', b'n', 6, 6, 6, 0.3f32, &big, 6, &big, 6, -1.2f32, &mut big.clone(), 6);
    })
}

fn blas_src_gemm_big() {
    let mut rng = rand::thread_rng();
    let mut big: Vec<f32> = Vec::new();
    for _ in 0..1_000_000 {
        big.push(rng.gen::<f32>())
    }
    criterion::black_box(unsafe {
        sgemm(b'n', b'n', 1000, 1000, 10, 0.3f32, &big, 1000, &big, 1000, -1.2f32, &mut big.clone(), 1000);
    })
}


fn blas_src_axpy() {
    let mut rng = rand::thread_rng();
    let mut big: Vec<f32> = Vec::new();
    for _ in 0..10 {
        big.push(rng.gen::<f32>())
    }
    criterion::black_box(unsafe {
        saxpy(10, 23.0f32, &big, 1, &mut big.clone(), 1);
    })
}

fn blas_src_axpy_big() {
    let mut rng = rand::thread_rng();
    let mut big: Vec<f32> = Vec::new();
    for _ in 0..1_000_000 {
        big.push(rng.gen::<f32>())
    }
    criterion::black_box(unsafe {
        saxpy(1_000_000, 23.0f32, &big, -1, &mut big.clone(), -1);
    })
}

fn axpy() {
    let mut rng = rand::thread_rng();
    let mut big: Vec<f32> = Vec::new();
    for _ in 0..10 {
        big.push(rng.gen::<f32>())
    }
    criterion::black_box(libblas::level1::axpy(10, 23.0f32, &big, 1, &mut big.clone(), 1));
}

fn axpy_big() {
    let mut rng = rand::thread_rng();
    let mut big: Vec<f32> = Vec::new();
    for _ in 0..1_000_000 {
        big.push(rng.gen::<f32>())
    }
    criterion::black_box(libblas::level1::axpy(1_000_000, 23.0f32, &big, 1, &mut big.clone(), 1));
}

fn gemm() {
    let mut rng = rand::thread_rng();
    let mut big: Vec<f32> = Vec::new();
    for _ in 0..36 {
        big.push(rng.gen::<f32>())
    }
    criterion::black_box(libblas::level3::gemm(
        'n',
        'n',
        6,
        6,
        6,
        0.3f32,
        &big,
        6,
        &big,
        6,
        -1.2f32,
        &mut big.clone(),
        6,
    ));
}

fn gemm_big() {
    let mut rng = rand::thread_rng();
    let mut big: Vec<f32> = Vec::new();
    for _ in 0..1_000_000 {
        big.push(rng.gen::<f32>())
    }
    criterion::black_box(libblas::level3::gemm(
        'n',
        'n',
        1000,
        1000,
        10,
        0.3f32,
        &big,
        1000,
        &big,
        1000,
        -1.2f32,
        &mut big.clone(),
        1000,
    ));
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("libblas::level3::gemm", |b| b.iter(|| gemm()));
    c.bench_function("libblas::level3::gemm big", |b| b.iter(|| gemm_big()));
    c.bench_function("libblas::level1::axpy", |b| b.iter(|| axpy()));
    c.bench_function("libblas::level1::axpy-big", |b| b.iter(|| axpy_big()));
    c.bench_function("blas-src intel-mkl sgemm", |b| b.iter(|| blas_src_gemm()));
    c.bench_function("blas-src intel-mkl sgemm big", |b| b.iter(|| blas_src_gemm_big()));
    c.bench_function("blas-src intel-mkl saxpy", |b| b.iter(|| blas_src_axpy()));
    c.bench_function("blas-src intel-mkl saxpy big", |b| { b.iter(|| blas_src_axpy_big())});
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
