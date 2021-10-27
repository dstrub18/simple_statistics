use super::utilities::{get_variance};
use ndarray::{Array1};

#[allow(unused)]
pub fn get_f_statistic(sample_1: &Array1<f64>, sample_2: &Array1<f64>) -> Result<f64, String>
{
    let sample_1_length = get_max_dimension(sample_1.shape()) - 1;
    let sample_2_length = get_max_dimension(sample_2.shape()) - 1;

    if sample_1_length > sample_2_length
    {
        Ok(get_variance(sample_1)? / get_variance(sample_2)?)
    }
    else
    {
        Ok(get_variance(sample_2)? / get_variance(sample_1)?)
    }
}

pub fn get_max_dimension(shape: &[usize]) -> usize
{
    shape[0].max(shape[1])
}