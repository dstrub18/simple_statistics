#![feature(test)]
extern crate test;
use simple_statistics::{utilities, file_reading};


#[cfg(test)]
mod tests
{
    #[bench]
    fn bench_expensive_corrcoef(b: &mut test::Bencher)
    {
        let data  = ndarray::Array2::ones((4, 4));
        b.iter(|| simple_statistics::utilities::get_correlation_coefficient_matrix_maybe_expensive(&data));
    }

    #[bench]
    fn bench_corrcoef(b: &mut test::Bencher)
    {
        let data  = ndarray::Array2::ones((4, 4));
        b.iter(|| simple_statistics::utilities::get_correlation_coefficient_matrix(&data));
    }
    
    #[bench]
    fn bench_test_covariance(b: &mut test::Bencher)
    {
        let data  = ndarray::Array2::ones((4, 4));
        b.iter(|| simple_statistics::utilities::get_covariance_matrix(&data));
    }

    #[bench]
    fn bench_mean(b: &mut test::Bencher)
    {
        let data  = ndarray::Array1::ones(16);
        b.iter(|| simple_statistics::utilities::get_mean(&data));
    }

    #[bench]
    fn bench_std(b: &mut test::Bencher)
    {
        let data  = ndarray::Array1::ones(16);
        b.iter(|| simple_statistics::utilities::get_standard_deviation(&data));
    }
    
    #[bench]
    fn bench_variance(b: &mut test::Bencher)
    {
        let data  = ndarray::Array1::ones(16);
        b.iter(|| simple_statistics::utilities::get_variance(&data));
    }
    
    #[bench]
    fn bench_some_squares(b: &mut test::Bencher)
    {
        
        let data  = ndarray::Array1::ones(16);
        b.iter(|| simple_statistics::utilities::get_sum_of_squares(&data));
    }
}