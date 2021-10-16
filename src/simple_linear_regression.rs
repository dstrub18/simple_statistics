use super::utilities::*;

#[allow(unused)]
#[derive(Debug)]
pub struct VariableTargetInfo
{
    independent_variable_info: VariableInfo,
    dependent_variable_info: VariableInfo,

    correlation_coefficient: f64,
    covariance: f64,
    slope: f64,
    intercept: f64,
    predictions: ndarray::Array1<f64>,
    sst: f64,
    sse: f64,
    ssr: f64,
}

#[allow(unused)]
pub fn get_variable_target_info(independent_variable: &ndarray::Array1<f64>, dependent_variable: &ndarray::Array1<f64>) -> VariableTargetInfo
{
        let slope = get_best_fitting_slope(independent_variable, dependent_variable).unwrap();
        let intercept = get_best_fitting_intercept(independent_variable, dependent_variable).unwrap();
        let predictions = get_predictions(independent_variable, slope, intercept).unwrap();
        
        VariableTargetInfo
        {
            independent_variable_info: get_variable_info(independent_variable),
            dependent_variable_info: get_variable_info(dependent_variable),

            correlation_coefficient: get_correlation_coefficient(independent_variable, dependent_variable).unwrap(),
            covariance: get_population_covariance(independent_variable, dependent_variable).unwrap(),
            
            sst: get_sst(dependent_variable).unwrap(),
            sse: get_sse(&predictions, dependent_variable).unwrap(),
            ssr: get_ssr(&predictions, dependent_variable).unwrap(),
            
            slope: slope,
            intercept: intercept,
            predictions: predictions,
        }
}
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