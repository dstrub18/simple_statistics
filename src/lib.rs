pub mod sampling
{
    #[allow(unused)]
    pub fn get_sample (array_to_sample: &ndarray::Array2<f64>, num_elements_in_sample: usize) -> ndarray::Array2<f64>
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
        array_to_return
    }
}

pub mod file_reading
{
    use csv::{ReaderBuilder};
    use ndarray::{Array2};
    use ndarray_csv::{Array2Reader};
    #[allow(unused)]
    pub fn read_csv_to_array(path_to_file:&str) -> Result<Array2<f64>, ndarray_csv::ReadError>
    {
        let file = std::fs::File::open(path_to_file).unwrap();
        let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
        reader.deserialize_array2_dynamic()
    }
}

pub mod counting 
{
    use super::utilities::compute_factorial;

    #[allow(unused)]
    pub fn compute_combinations(n: u64, r: u64) -> Result<u128, String> 
    {
        if n <= 0 || r <= 0
        {
            Err(String::from ("n and r must be equal or greater than 0!"))
        }
        else
        {
            Ok(compute_factorial(n) / (compute_factorial(r) * compute_factorial(n - r)))
        }
    }

    #[allow(unused)]
    pub fn compute_permutations(n: u64, r: u64) -> Result<u128, String> 
    {
        if n <= 0 || r <= 0
        {
            Err(String::from("n and r must be equal or greater than 0!"))
        }
        else
        {
            Ok(compute_factorial(n) / compute_factorial(n - r))
        }
    }
} //  End counting

/// Linear regression with multiple variables
mod multiple_regression 
{
    #[allow(unused)]
    struct VariableRelationshipInfo
    {

    }
}

/// Linear regression with 1 independent variable.
pub mod simple_linear_regression 
{
    use super::utilities::*;

    #[allow(unused)]
    pub fn get_slope_and_intercept(x: &ndarray::Array1<f64>, y: &ndarray::Array1<f64>) -> Result<(f64, f64), String> 
    {
            Ok((compute_best_fitting_slope(x, y)?,
                compute_best_fitting_intercept(x, y)?))
    }

    #[allow(unused)]
    pub fn compute_best_fitting_intercept(
        independent_variable: &ndarray::Array1<f64>,
        dependent_variable: &ndarray::Array1<f64>,
    ) -> Result<f64, String>
    {
        Ok (compute_mean(dependent_variable)?
            - compute_best_fitting_slope(independent_variable, dependent_variable)?
            * compute_mean(independent_variable)?)
    }

    #[allow(unused)]
    pub fn compute_best_fitting_slope(independent_variable: &ndarray::Array1<f64>, dependent_variable: &ndarray::Array1<f64>) -> Result<f64, String> 
    {
        let mean_independent_variable = compute_mean(independent_variable)?;
        let mean_dependent_variable = compute_mean(dependent_variable)?;
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

    #[allow(unused)]
    pub fn compute_correlation_coefficient(
        independent_variable: &ndarray::Array1<f64>,
        dependent_variable: &ndarray::Array1<f64>,
    ) -> Result<f64, String> 
    {
        Ok (compute_best_fitting_slope(independent_variable, dependent_variable)?
            * (compute_standard_deviation(independent_variable)?
                / compute_standard_deviation(dependent_variable)?))
    }
} // End mod linear regression

pub mod utilities 
{
    #[allow(unused)]
    pub fn compute_standard_error(x: &ndarray::Array1<f64>, y: &ndarray::Array1<f64>) -> Result<f64, String>
    {
        Ok(compute_mse(x, y)?.sqrt())
    }

    #[allow(unused)]
    pub fn compute_mse(x: &ndarray::Array1<f64>, y: &ndarray::Array1<f64>) -> Result<f64, String>
    {
        let degrees_of_freedom = 2;
        Ok(compute_sse(x, y)? / ((x.len() - degrees_of_freedom) as f64))
    }

    #[allow(unused)]
    pub fn compute_coefficient_of_determination(predictions: &ndarray::Array1<f64>,observations: &ndarray::Array1<f64>) -> Result<f64, String>
    {
        Ok(compute_ssr(predictions, observations)? / compute_sst(observations))
    }

    #[allow(unused)]
    pub fn compute_ssr(predictions: &ndarray::Array1<f64>, observations: &ndarray::Array1<f64>) -> Result<f64, String>
    {
        Ok(compute_sst(observations) - compute_sse(predictions, observations)?)
    }

    #[allow(unused)]
    pub fn compute_sst(observations: &ndarray::Array1<f64>) -> f64 
    {
        let mean = compute_mean(observations).unwrap();
        observations
            .into_iter()
            .map(|x| {
                let error = x - mean;
                error.powi(2)
            })
            .sum()
    }

    #[allow(unused)]
    pub fn compute_sse(predictions: &ndarray::Array1<f64>, observations: &ndarray::Array1<f64>) -> Result<f64, String >
    {
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
    pub fn compute_predictions(input_vector: &ndarray::Array1<f64>, slope: f64, intercept: f64) -> Result<ndarray::Array1<f64>, String>
    {
        let input_vector = check_vector_for_nans(input_vector)?;
        Ok(input_vector
            .into_iter()
            .map(|x| slope * x + intercept)
            .collect())
    }

    #[allow(unused)]
    pub fn compute_standard_deviation(input_vector: &ndarray::Array1<f64>) -> Result<f64, String> 
    {
        Ok(compute_variance(input_vector)?.sqrt())
    }

    #[allow(unused)]
    pub fn compute_variance(input_vector: &ndarray::Array1<f64>) -> Result<f64, String>
    {
        Ok (compute_sum_of_squares(input_vector)? / (input_vector.len() as f64 - 1.0))
    }

    #[allow(unused)]
    pub fn compute_sum_of_squares(input_vector: &ndarray::Array1<f64>) -> Result<f64, String>
    {
        let mean = compute_mean(input_vector)?;
        
        Ok(input_vector.into_iter().map(
                                        |element|
                                        {(element - mean).powi(2)}).sum::<f64>() as f64)
    }

    #[allow(unused)]
    pub fn compute_mean(input_vector: &ndarray::Array1<f64>) -> Result<f64, String> 
    {
        let input_vector = check_vector_for_nans(input_vector)?;
        match input_vector.len() {
            0 => Err(String::from("Vector cannot be empty")),
            1 => Ok(input_vector[0]),
            _ => Ok(input_vector.iter().sum::<f64>() / input_vector.len() as f64),
        }
    }

    pub fn compute_factorial(n: u64) -> u128 
    {
        let mut result: u128 = 1;
        for i in (2..n + 1).rev() 
        {
            result = result * i as u128;
        }
        result
    }

    pub fn check_vector_for_nans(input_vector: &ndarray::Array1<f64>) -> Result<&ndarray::Array1<f64>, String> 
    {
        if !input_vector.iter().any(|&x| x.is_nan()) {
            Ok(input_vector)
        } else {
            Err(String::from("Vector must not contain nans!"))
        }
    }
} // End mod utilities

#[cfg(test)]
mod tests 
{
    use math::round::*;
    use crate::counting;
    use crate::simple_linear_regression::*;
    use crate::utilities;
    use assert_approx_eq;

    static NUM_DECIMAL_DIGITS: i8 = 3;

    #[test]
    fn test_mean_empty_vector() 
    {
        let expected_result = Err(String::from("Vector cannot be empty"));
        let vector = ndarray::arr1(&[]);
        assert_eq!(utilities::compute_mean(&vector), expected_result);
    }

    #[test]
    fn test_mean_containing_nan() 
    {
        let expected_result = Err(String::from("Vector must not contain nans!"));
        let vector = ndarray::arr1(&[2.0, f64::NAN]);
        assert_eq!(utilities::compute_mean(&vector), expected_result);
    }

    #[allow(unused)]
    #[test]
    fn test_nan_check_for_error() 
    {
        let vector = ndarray::arr1(&[1.0, f64::NAN]);

        let expected_result = Err(String::from("Vector must not contain nans!"));
        assert_eq!(utilities::check_vector_for_nans(&vector), expected_result);
    }

    #[test]
    fn test_nan_check_for_success() 
    {
        let vector = ndarray::arr1(&[1.0, 2.0]);
        let expected_result = Ok(&vector);
        assert_eq!(utilities::check_vector_for_nans(&vector), expected_result);
    }

    #[test]
    fn test_permutations() 
    {
        let n = 10;
        let r = 3;
        let result = counting::compute_permutations(n, r);

        let expected_result: Result<u128, String> = Ok(720);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_combinations() 
    {
        let n = 10;
        let r = 3;
        let result = counting::compute_combinations(n, r);

        let expected_result: Result<u128, String> = Ok(120);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_factorial() 
    {
        let n = 4;
        let result = utilities::compute_factorial(n);
        
        let expected_result: u128 = 24;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_compute_standard_error() 
    {
        let x = ndarray::arr1(&[4.1512, 14.9715, 8.5378, 12.0471, 13.6555, 6.6369]);
        let observations = ndarray::arr1(&[5.0, 17.0, 11.0, 8.0, 14.0, 5.0]);
        let result = half_away_from_zero(utilities::compute_standard_error(&x, &observations).unwrap(), NUM_DECIMAL_DIGITS);
        let expected_result = 2.742;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_compute_mse() 
    {
        let x = ndarray::arr1(&[4.1512, 14.9715, 8.5378, 12.0471, 13.6555, 6.6369]);
        let observations = ndarray::arr1(&[5.0, 17.0, 11.0, 8.0, 14.0, 5.0]);
        let result = half_away_from_zero( utilities::compute_mse(&x, &observations).unwrap(),NUM_DECIMAL_DIGITS);
        let expected_result = 7.519;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_compute_coefficient_of_determination() 
    {
        let x = ndarray::arr1(&[34.0, 108.0, 64.0, 88.0, 99.0, 51.0]);
        let observations = ndarray::arr1(&[5.0, 17.0, 11.0, 8.0, 14.0, 5.0]);
        let slope = compute_best_fitting_slope(&x, &observations).unwrap();
        let intercept = compute_best_fitting_intercept(&x, &observations).unwrap();
        let predictions = utilities::compute_predictions(&x, slope, intercept).unwrap();
        let result = half_away_from_zero(utilities::compute_coefficient_of_determination(&predictions, &observations).unwrap(), NUM_DECIMAL_DIGITS);
        
        let expected_result = 0.749;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_compute_ssr() 
    {
        let x = ndarray::arr1(&[34.0, 108.0, 64.0, 88.0, 99.0, 51.0]);
        let observations = ndarray::arr1(&[5.0, 17.0, 11.0, 8.0, 14.0, 5.0]);
        let slope = compute_best_fitting_slope(&x, &observations).unwrap();
        let intercept = compute_best_fitting_intercept(&x, &observations).unwrap();
        let predictions = utilities::compute_predictions(&x, slope, intercept).unwrap();
        let result = half_away_from_zero(utilities::compute_ssr(&predictions, &observations).unwrap(), NUM_DECIMAL_DIGITS);
        
        let expected_result = 89.925;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_compute_correlation_coefficient() 
    {
        let observations = ndarray::arr1(&[5.0, 17.0, 11.0, 8.0, 14.0, 5.0]);
        let x = ndarray::arr1(&[34.0, 108.0, 64.0, 88.0, 99.0, 51.0]);
        let result = half_away_from_zero(compute_correlation_coefficient(&x, &observations).unwrap(), NUM_DECIMAL_DIGITS);

        let expected_result = 0.866;
        assert_eq!(result, expected_result);
    }
    #[test]
    fn test_compute_auto_correlation() 
    {
        let x = ndarray::arr1(&[34.0, 108.0, 64.0, 88.0, 99.0, 51.0]);
        let result = half_away_from_zero(compute_correlation_coefficient(&x, &x).unwrap(), NUM_DECIMAL_DIGITS);

        let expected_result = 1.0;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_compute_sse() 
    {
        let observations = ndarray::arr1(&[5.0, 17.0, 11.0, 8.0, 14.0, 5.0]);
        let x = ndarray::arr1(&[34.0, 108.0, 64.0, 88.0, 99.0, 51.0]);
        let slope = compute_best_fitting_slope(&x, &observations).unwrap();
        let intercept = compute_best_fitting_intercept(&x, &observations).unwrap();
        let predictions = utilities::compute_predictions(&x, slope, intercept).unwrap();
        let result = half_away_from_zero (utilities::compute_sse(&predictions, &observations).unwrap(), NUM_DECIMAL_DIGITS);

        let expected_result = 30.075;
        assert_approx_eq::assert_approx_eq!(result, expected_result, 0.1);
    }

    #[test]
    fn test_compute_sst() 
    {
        let observations = ndarray::arr1(&[5.0, 17.0, 11.0, 8.0, 14.0, 5.0]);
        let result = utilities::compute_sst(&observations);

        let expected_result = 120.0;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_compute_prediction() 
    {
        let x = ndarray::arr1(&[34.0, 108.0, 64.0, 88.0, 99.0, 51.0]);
        let result = utilities::compute_predictions(&x, 0.14621968616262482, -0.8202567760342365).unwrap();

        let expected_result = ndarray::arr1(&[4.1512, 14.9715, 8.5378, 12.0471, 13.6555, 6.6369]);

        assert_eq!(
            round_vector_elements(
                &result,
                2
            ),
            round_vector_elements(&expected_result, 2)
        );
    }

    #[test]
    fn test_best_fitting_intercept() 
    {
        let x = ndarray::arr1(&[34.0, 108.0, 64.0, 88.0, 99.0, 51.0]);
        let y = ndarray::arr1(&[5.0, 17.0, 11.0, 8.0, 14.0, 5.0]);
        let result = half_away_from_zero(compute_best_fitting_intercept(&x, &y).unwrap(), NUM_DECIMAL_DIGITS);
        let expected_result = -0.820;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_best_fitting_slope() 
    {
        let x = ndarray::arr1(&[34.0, 108.0, 64.0, 88.0, 99.0, 51.0]);
        let y = ndarray::arr1(&[5.0, 17.0, 11.0, 8.0, 14.0, 5.0]);
        let result = half_away_from_zero(compute_best_fitting_slope(&x, &y).unwrap(), NUM_DECIMAL_DIGITS);
        let expected_result = 0.146;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_mean() 
    {
        let v = ndarray::arr1(&[2.0, 4.0, 6.0]);
        assert_eq!(utilities::compute_mean(&v).unwrap(), 4.0);
    }
    
    #[test]
    fn test_mean_length_1() 
    {
        let v = ndarray::arr1(&[2.0]);
        assert_eq!(utilities::compute_mean(&v).unwrap(), v[0]);
    }

    #[test]
    fn test_standard_deviation() 
    {
        let v = ndarray::arr1(&[34.0, 108.0, 64.0, 88.0, 99.0, 51.0]);
        let result = half_away_from_zero (utilities::compute_standard_deviation(&v).unwrap(), NUM_DECIMAL_DIGITS);
        let expected_result = 29.003;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_variance() 
    {
        let v = ndarray::arr1(&[2.0, 3.0, 4.0, 5.0, 6.0]);
        let result = half_away_from_zero(utilities::compute_variance(&v).unwrap(), NUM_DECIMAL_DIGITS);
        let expected_result = 2.5;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_sum_of_squares() 
    {
        let v = ndarray::arr1(&[2.0, 3.0, 4.0, 5.0, 6.0]);
        let result = half_away_from_zero(utilities::compute_sum_of_squares(&v).unwrap(), NUM_DECIMAL_DIGITS);
        let expected_result = 10.0;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_round() 
    {
        let x = 3.001;
        assert_eq!(round(&x, 2), 3.000);
    }

    // Rounding utilities
    fn round_vector_elements(x: &ndarray::Array1<f64>, decimal_precision: u32) -> Vec<f64> 
    {
        let exp = (10 ^ decimal_precision) as f64;
        x.into_iter()
        .map(|elem| {
                let scaled_element = elem * exp;
                scaled_element.round() / exp
        }).collect()
    }

    fn round(x: &f64, decimal_precision: u32) -> f64 
    {
        let exp = (10 ^ decimal_precision) as f64;
        (x * exp).round() / exp
    }
}
