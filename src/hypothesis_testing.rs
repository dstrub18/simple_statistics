/* 
Thoughts:
    - trait "config" for different test modes
    - functions for different tests
*/


use super::utilities::{get_variance};
use ndarray::{Array1};

#[allow(unused)]
pub fn get_f_statistic(sample_1: &Array1<f64>, sample_2: &Array1<f64>) -> Result<f64, String>
{
    let sample_1_length = sample_1.shape()[0] - 1;
    let sample_2_length = sample_2.shape()[0] - 1;

    if sample_1_length >= sample_2_length
    {
        Ok(get_variance(sample_1)? / get_variance(sample_2)?)
    }
    else
    {
        Ok(get_variance(sample_2)? / get_variance(sample_1)?)
    }
}