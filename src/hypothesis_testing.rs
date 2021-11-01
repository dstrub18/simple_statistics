/* 
Thoughts:
    - trait "config" for different test modes
    - functions for different tests

    For: one-tailed upper, 2-tailed, population mean (un-)known, 
*/

#[allow(unused)]
enum ZtestKind
{
    OneTailed,
    TwoTailed
}

#[allow(unused)]
struct ZTest
{
    kind: ZtestKind,
    alpha_level: f64
}

impl ZTest
{
    #[allow(unused)]
    fn new(kind: ZtestKind) -> ZTest
    {
        match kind
        {
            ZtestKind::OneTailed => ZTest {kind: ZtestKind::OneTailed, alpha_level: 0.05f64},
            ZtestKind::TwoTailed => ZTest {kind: ZtestKind::TwoTailed, alpha_level: 0.10f64}
        }
    }
}

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