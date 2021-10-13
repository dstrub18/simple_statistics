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
pub fn get_variable_target_info(x: &ndarray::Array1<f64>, y: &ndarray::Array1<f64>) -> VariableTargetInfo
{
        VariableTargetInfo
        {
            independent_variable_info: get_variable_info(x),
            dependent_variable_info: get_variable_info(y),

            correlation_coefficient: get_correlation_coefficient(x, y).unwrap(),
            covariance: get_population_covariance(x, y).unwrap(),
            slope: get_best_fitting_slope(x, y).unwrap(),
            intercept: get_best_fitting_intercept(x, y).unwrap(),
            predictions: get_predictions(x, get_best_fitting_slope(x, y).unwrap(), get_best_fitting_intercept(x, y).unwrap()).unwrap(),

            sst: get_sst(y).unwrap(),
            sse: get_sse(&get_predictions(x, get_best_fitting_slope(x, y).unwrap(), get_best_fitting_intercept(x, y).unwrap()).unwrap(), y).unwrap(),
            ssr: get_ssr(&get_predictions(x, get_best_fitting_slope(x, y).unwrap(), get_best_fitting_intercept(x, y).unwrap()).unwrap(), y).unwrap()
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