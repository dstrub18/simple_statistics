use num_traits;
use ndarray::{Array1, Array2};

#[allow(unused)]
#[derive(Debug)]
pub struct VariableInfo
{
    mean: f64,
    standard_deviation: f64,
    variance: f64
}

#[allow(unused)]
pub fn get_variable_info(x: &Array1<f64>) -> VariableInfo
{
    VariableInfo
    {
        mean: get_mean(x).unwrap(),
        standard_deviation: get_standard_deviation(x).unwrap(),
        variance: get_variance(x).unwrap()
    }
}

pub fn get_correlation_coefficient_matrix (mat: &Array2<f64>) -> Result<ndarray::Array2<f64>, String>
{
    let dim = mat.shape()[1] - 1;
    let mut corr_coeff_mat = ndarray::Array2::<f64>::zeros((dim, dim));

    for i in 0..dim
    {
        // Diagonal is always 1
        corr_coeff_mat[[i, i]] = 1.0f64;

        // Compute upper triangle
        for j in i+1..dim
        {
            let vec_1 = mat.index_axis(ndarray::Axis(1), i).to_owned();
            let vec_2 = mat.index_axis(ndarray::Axis(1), j).to_owned();
            corr_coeff_mat[[i, j]] = get_correlation_coefficient(&vec_1, &vec_2)?;
        }

        // Compute lower triangle
        for j in 0..i
        {
            corr_coeff_mat[[i, j]] = corr_coeff_mat[[j, i]];
        }
    }
    Ok(corr_coeff_mat)
}

pub fn get_correlation_coefficient_matrix_maybe_expensive (mat: &Array2<f64>) -> Result<ndarray::Array2<f64>, String>
{
    let dim = mat.shape()[1] - 1;
    let mut corr_coeff_mat = ndarray::Array2::<f64>::zeros((dim, dim));

    for i in 0..dim
    {
        for j in 0..dim
        {
            let vec_1 = mat.index_axis(ndarray::Axis(1), i).to_owned();
            let vec_2 = mat.index_axis(ndarray::Axis(1), j).to_owned();
            corr_coeff_mat[[i, j]] = get_correlation_coefficient(&vec_1, &vec_2)?;
        }
    }
    Ok(corr_coeff_mat)
}

#[allow(unused)]
pub fn get_correlation_coefficient(independent_variable: &Array1<f64>, dependent_variable: &Array1<f64>,)
-> Result<f64, String> 
{
    Ok (get_sample_covariance(independent_variable, dependent_variable)? / (get_standard_deviation(independent_variable)? * get_standard_deviation(dependent_variable)?))
}

pub fn get_covariance_matrix(mat: &Array2<f64>) -> Result<ndarray::Array2<f64>, String>
{
    let dim = mat.shape()[1] - 1;
    let mut cov_mat = ndarray::Array2::<f64>::zeros((dim, dim));

    for i in 0..dim
    {
        // Diagonal is variance
        let vec_1 = mat.index_axis(ndarray::Axis(1), i).to_owned();
        cov_mat[[i, i]] = get_variance(&vec_1)?;

        // Compute upper triangle
        for j in i+1..dim
        {
                let vec_1 = mat.index_axis(ndarray::Axis(1), i).to_owned();
                let vec_2 = mat.index_axis(ndarray::Axis(1), j).to_owned();
                cov_mat[[i, j]] = get_population_covariance(&vec_1, &vec_2)?;
        }
        
        // Compute lower triangle
        for j in 0..i
        {
            cov_mat[[i, j]] = cov_mat[[j, i]];
        }
    }
    Ok(cov_mat)
}

#[allow(unused)]
pub fn get_sample_covariance(x: &Array1<f64>, y: &Array1<f64>) -> Result<f64, String>
{
    check_vectors_for_equal_length(x, y);

    let mut numerator = 0.0;
    for (x_i, y_i) in x.iter().zip(y)
    {
        numerator += (x_i - get_mean(x)?) * (y_i - get_mean(y)?);
    }
    Ok(numerator / (x.len() as f64 - 1.0))
}

#[allow(unused)]
pub fn get_population_covariance(x: &Array1<f64>, y: &Array1<f64>) -> Result<f64, String>
{
    check_vectors_for_equal_length(x, y);

    let mut numerator = 0.0;
    for (x_i, y_i) in x.iter().zip(y)
    {
        numerator += (x_i - get_mean(x)?) * (y_i - get_mean(y)?);
    }
    Ok(numerator / x.len() as f64)
}

#[allow(unused)]
pub fn check_vectors_for_equal_length(x: &Array1<f64>, y: &Array1<f64>)
{
    if x.len() != y.len()
    {
        panic!("Vector lengths do not match!");
    }
}

#[allow(unused)]
pub fn get_coefficient_of_variation(population: &Array1<f64>) -> Result<f64, String>
{
    Ok ((get_standard_deviation(population)? / get_mean(population)?)  * 100.0)
}

#[allow(unused)]
pub fn get_z_score(data_point: f64, population: &Array1<f64>) -> Result<f64, String>
{
    Ok ((data_point - get_mean(population)?) / get_standard_deviation(population)?)
}

#[allow(unused)]
pub fn get_standard_error(x: &Array1<f64>, y: &Array1<f64>) -> Result<f64, String>
{
    Ok(get_mse(x, y)?.sqrt())
}

#[allow(unused)]
pub fn get_mse(x: &Array1<f64>, y: &Array1<f64>) -> Result<f64, String>
{
    let degrees_of_freedom = 2;
    Ok(get_sse(x, y)? / ((x.len() - degrees_of_freedom) as f64))
}

#[allow(unused)]
pub fn get_coefficient_of_determination(predictions: &Array1<f64>,observations: &Array1<f64>) -> Result<f64, String>
{
    Ok(get_ssr(predictions, observations)? / get_sst(observations)?)
}

#[allow(unused)]
pub fn get_ssr(predictions: &Array1<f64>, observations: &Array1<f64>) -> Result<f64, String>
{
    Ok(get_sst(observations)? - get_sse(predictions, observations)?)
}

#[allow(unused)]
pub fn get_sst(observations: &Array1<f64>) -> Result<f64, String> 
{
    let mean = get_mean(observations)?;
    Ok (observations
        .into_iter()
        .map(|&x| {
            (x - mean).powi(2)
        })
        .sum()
    )
}

#[allow(unused)]
pub fn get_sse(predictions: &Array1<f64>, observations: &Array1<f64>) -> Result<f64, String >
{
    check_vectors_for_equal_length(predictions, observations);
    let predictions = check_vector_for_nans(predictions)?;
    Ok (predictions
        .iter()
        .zip(observations)
        .into_iter()
        .map(|zipped_element| {
            let error = zipped_element.0 - zipped_element.1;
            error.powi(2)
        })
        .into_iter()
        .sum()
    )
}

#[allow(unused)]
pub fn get_predictions(input_vector: &Array1<f64>, slope: f64, intercept: f64) -> Result<ndarray::Array1<f64>, String>
{
    let input_vector = check_vector_for_nans(input_vector)?;
    Ok(input_vector
        .into_iter()
        .map(|&x| slope * x + intercept)
        .collect())
}

#[allow(unused)]
pub fn get_standard_deviation(input_vector: &Array1<f64>) -> Result<f64, String> 
{
    Ok(get_variance(input_vector)?.sqrt())
}

#[allow(unused)]
pub fn get_variance(input_vector: &Array1<f64>) -> Result<f64, String>
{
    Ok (get_sum_of_squares(input_vector)? / (input_vector.len() as f64 - 1.0))
}

#[allow(unused)]
pub fn get_sum_of_squares(input_vector: &Array1<f64>) -> Result<f64, String>
{
    let mean = get_mean(input_vector)?;
    
    Ok(input_vector.into_iter().map(
                                    |&element|
                                    {(element - mean).powi(2)}).sum::<f64>() as f64)
}

#[allow(unused)]
pub fn get_mean(input_vector: &Array1<f64>) -> Result<f64, String> 
{
    let input_vector = check_vector_for_nans(input_vector)?;
    match input_vector.len() {
        0 => Err(String::from("Vector cannot be empty")),
        1 => Ok(input_vector[0]),
        _ => Ok(input_vector.iter().sum::<f64>() / input_vector.len() as f64),
    }
}

pub fn get_factorial(n: u64) -> u128 
{
    let mut result: u128 = 1;
    for i in (2..n + 1).rev() 
    {
        result = result * i as u128;
    }
    result
}

pub fn check_vector_for_nans<T>(input_vector: &Array1<T>) -> Result<&Array1<T>, String> 
where T: num_traits::float::Float
{
    if input_vector.iter().any(|&x| x.is_nan()) 
    {
        Err(String::from("Vector must not contain nans!"))
    } 
    else
    {
        Ok(input_vector)
    }
}