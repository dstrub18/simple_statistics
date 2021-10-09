
use crate::utilities::*;

#[allow(unused)]
pub fn get_sample (array_to_sample: &ndarray::Array2<f64>, num_elements_in_sample: usize, set_seed: bool) -> Result<ndarray::Array2<f64>, String>
{
    if (set_seed)
    {
        fastrand::seed(42);
    }

    if (array_to_sample.raw_dim()[1] >= array_to_sample.raw_dim()[0])
    {
        return Err(String::from("Number of rows must be great than number of columns!"));
    }
    else
    if (num_elements_in_sample > array_to_sample.raw_dim()[0])
    {
        return Err(String::from("You want to sample more elements than those that are contained in the array"));
    }
    else
    {
        let mut array_to_return = ndarray::Array2::<f64>::zeros((0, array_to_sample.raw_dim()[1]));
        
        let random_indices: Vec<u32> = std::iter::repeat_with
        (|| fastrand::u32(0 .. array_to_sample.raw_dim()[0] as u32))
        .take(num_elements_in_sample).collect();
        for i in random_indices
        {
            let row_from_source = array_to_sample.index_axis(ndarray::Axis(0), i as usize);
            array_to_return.push_row(row_from_source);
        }
        Ok(array_to_return)
    }
}

#[allow(unused)]
pub fn get_t_distribution(population: &ndarray::Array1<f64>, sample: &ndarray::Array1<f64>) -> Result<f64, String>
{
    Ok(
        (get_mean(sample)? - get_mean(population)?) // Numerator
        /
        (get_standard_deviation(sample)? / (sample.len() as f64).sqrt()) // Denominator
        )
}

