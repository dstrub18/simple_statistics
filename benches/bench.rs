#![feature(test)]
extern crate test;
use simple_statistics::{utilities, file_reading};


#[cfg(test)]
mod tests
{
    #[bench]
    fn bench_expensive_corrcoef(b: &mut test::Bencher)
    {
        let data  = ndarray::Array2::ones((20, 20));
        b.iter(|| simple_statistics::utilities::get_correlation_coefficient_matrix_maybe_expensive(&data));
    }
    #[bench]
    fn bench_corrcoef(b: &mut test::Bencher)
    {
        let data  = ndarray::Array2::ones((20, 20));
        b.iter(|| simple_statistics::utilities::get_correlation_coefficient_matrix(&data));
    }
}