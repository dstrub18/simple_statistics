use super::utilities::*;

#[allow(unused)]
pub fn get_best_fitting_intercept(
    independent_variable: &ndarray::Array1<f64>,
    dependent_variable: &ndarray::Array1<f64>,
) -> Result<f64, String>
{
    Ok (get_mean(dependent_variable)?
        - get_best_fitting_slope(independent_variable, dependent_variable)?
        * get_mean(independent_variable)?)
}

#[allow(unused)]
pub fn get_best_fitting_slope(independent_variable: &ndarray::Array1<f64>, dependent_variable: &ndarray::Array1<f64>) -> Result<f64, String> 
{
    let mean_independent_variable = get_mean(independent_variable)?;
    let mean_dependent_variable = get_mean(dependent_variable)?;
    let mut numerator_sum = 0.0;
    let mut denominator_sum = 0.0;

    for (x_i, y_i) in independent_variable.iter().zip(dependent_variable) 
    {
        let x_diff_to_mean = x_i - mean_independent_variable;
        let y_diff_to_mean = y_i - mean_dependent_variable;
        numerator_sum += x_diff_to_mean * y_diff_to_mean;

        let x_dev_denominator = (x_i - mean_independent_variable).powi(2);
        denominator_sum += x_dev_denominator;
    }
    Ok (numerator_sum / denominator_sum)
}