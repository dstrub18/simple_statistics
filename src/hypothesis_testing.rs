use super::utilities::{get_mean, get_variance};
use ndarray::{Array1};

#[allow(unused)]
pub enum NullHypothesisKind
{
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,
    EqualTo,
}

#[allow(unused)]
pub struct ZTest
{
    null_hypothesis: NullHypothesisKind,
    z_table: [(f64, f64); 7]
}

#[allow(unused)]
impl ZTest
{
    pub fn perform_test(&self, hypothesized_mean: f64, population_std: f64, sample: &ndarray::Array1<f64>) -> Result<f64, String>
    {
        Ok
        (
            (get_mean(sample)? - hypothesized_mean)
            /
            (population_std / (sample.len() as f64).sqrt())
        )
    }
}

#[allow(unused)]
impl ZTest
{
    pub fn new(&self, null_hypothesis: NullHypothesisKind, alpha_level: f64) -> Result<Self, String>
    {
        let z_table:[(f64, f64);7];
        match &null_hypothesis
        {
            NullHypothesisKind::GreaterThanOrEqualTo => z_table = get_table_for_upper_one_tailed(),
            NullHypothesisKind::LessThanOrEqualTo    => z_table = get_table_for_lower_one_tailed(),
            NullHypothesisKind::EqualTo              => z_table = get_table_for_two_tailed()
        }

        Ok(ZTest{null_hypothesis: null_hypothesis, z_table: z_table})
    }
}

fn get_table_for_upper_one_tailed() -> [(f64, f64);7]
{
    [(0.10,     1.282), 
     (0.05,     1.645),
     (0.025,    1.960), 
     (0.010,    2.326),
     (0.005,    2.576), 
     (0.001,    3.090),
     (0.0001,   3.719)]
}

fn get_table_for_lower_one_tailed() -> [(f64, f64);7]
{
    [(0.10,     -1.282), 
     (0.05,     -1.645),
     (0.025,    -1.960), 
     (0.010,    -2.326),
     (0.005,    -2.576), 
     (0.001,    -3.090),
     (0.0001,   -3.719)]
}

fn get_table_for_two_tailed() -> [(f64, f64);7]
{
    [(0.20,     1.282), 
     (0.10,     1.645),
     (0.05,    1.960), 
     (0.010,    2.576),
     (0.001,    3.291),
     (0.0001,   3.819),
     (0.0,      0.0)]
}



#[allow(unused)]
pub fn get_critical_value(hypothesis_kind: NullHypothesisKind,alpha_level: f64)
{
   
}


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