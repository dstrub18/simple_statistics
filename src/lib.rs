pub mod sampling
{
    use super::utilities::{get_mean, get_standard_deviation};
    
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
}

pub mod file_reading
{
    use csv::{ReaderBuilder};
    use ndarray::{Array2};
    use ndarray_csv::{Array2Reader};

    #[allow(unused)]
    pub fn read_csv_to_array(path_to_file:&str, has_headers: bool) -> Result<Array2<f64>, ndarray_csv::ReadError>
    {
        let file = std::fs::File::open(path_to_file).unwrap();
        let mut reader = ReaderBuilder::new().has_headers(has_headers).from_reader(file);
        reader.deserialize_array2_dynamic()
    }
}

pub mod counting 
{
    use super::utilities::get_factorial;

    #[allow(unused)]
    pub fn get_combinations(n: u64, r: u64) -> Result<u128, String> 
    {
        if n <= 0 || r <= 0
        {
            Err(String::from ("n and r must be equal or greater than 0!"))
        }
        else
        {
            Ok(get_factorial(n) / (get_factorial(r) * get_factorial(n - r)))
        }
    }

    #[allow(unused)]
    pub fn get_permutations(n: u64, r: u64) -> Result<u128, String> 
    {
        if n <= 0 || r <= 0
        {
            Err(String::from("n and r must be equal or greater than 0!"))
        }
        else
        {
            Ok(get_factorial(n) / get_factorial(n - r))
        }
    }
} //  End counting

/// Linear regression with multiple variables
mod multiple_regression 
{
    
}

/// Linear regression with 1 independent variable.
pub mod simple_linear_regression 
{
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
} // End mod linear regression

pub mod utilities 
{
    #[allow(unused)]
    #[derive(Debug)]
    pub struct VariableInfo
    {
        mean: f64,
        standard_deviation: f64,
        variance: f64
    }

    // Rename this at some point
    #[allow(unused)]
    pub fn get_variable_info(x: &ndarray::Array1<f64>) -> VariableInfo
    {
        VariableInfo
        {
            mean: get_mean(x).unwrap(),
            standard_deviation: get_standard_deviation(x).unwrap(),
            variance: get_variance(x).unwrap()
        }
    }

    #[allow(unused)]
    pub fn get_correlation_coefficient(independent_variable: &ndarray::Array1<f64>, dependent_variable: &ndarray::Array1<f64>,)
    -> Result<f64, String> 
    {
        Ok (get_sample_covariance(independent_variable, dependent_variable)? / (get_standard_deviation(independent_variable)? * get_standard_deviation(dependent_variable)?))
    }

    #[allow(unused)]
    pub fn get_sample_covariance(x: &ndarray::Array1<f64>, y: &ndarray::Array1<f64>) -> Result<f64, String>
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
    pub fn get_population_covariance(x: &ndarray::Array1<f64>, y: &ndarray::Array1<f64>) -> Result<f64, String>
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
    pub fn check_vectors_for_equal_length(x: &ndarray::Array1<f64>, y: &ndarray::Array1<f64>)
    {
        if x.len() != y.len()
        {
            panic!("Vector lengths do not match!");
        }
    }

    #[allow(unused)]
    pub fn get_coefficient_of_variation(population: &ndarray::Array1<f64>) -> Result<f64, String>
    {
        Ok ((get_standard_deviation(population)? / get_mean(population)?)  * 100.0)
    }

    #[allow(unused)]
    pub fn get_z_score(data_point: f64, population: &ndarray::Array1<f64>) -> Result<f64, String>
    {
        Ok ((data_point - get_mean(population)?) / get_standard_deviation(population)?)
    }

    #[allow(unused)]
    pub fn get_standard_error(x: &ndarray::Array1<f64>, y: &ndarray::Array1<f64>) -> Result<f64, String>
    {
        Ok(get_mse(x, y)?.sqrt())
    }

    #[allow(unused)]
    pub fn get_mse(x: &ndarray::Array1<f64>, y: &ndarray::Array1<f64>) -> Result<f64, String>
    {
        let degrees_of_freedom = 2;
        Ok(get_sse(x, y)? / ((x.len() - degrees_of_freedom) as f64))
    }

    #[allow(unused)]
    pub fn get_coefficient_of_determination(predictions: &ndarray::Array1<f64>,observations: &ndarray::Array1<f64>) -> Result<f64, String>
    {
        Ok(get_ssr(predictions, observations)? / get_sst(observations)?)
    }

    #[allow(unused)]
    pub fn get_ssr(predictions: &ndarray::Array1<f64>, observations: &ndarray::Array1<f64>) -> Result<f64, String>
    {
        Ok(get_sst(observations)? - get_sse(predictions, observations)?)
    }

    #[allow(unused)]
    pub fn get_sst(observations: &ndarray::Array1<f64>) -> Result<f64, String> 
    {
        let mean = get_mean(observations)?;
        Ok (observations
            .into_iter()
            .map(|x| {
                let error = x - mean;
                error.powi(2)
            })
            .sum()
        )
    }

    #[allow(unused)]
    pub fn get_sse(predictions: &ndarray::Array1<f64>, observations: &ndarray::Array1<f64>) -> Result<f64, String >
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
    pub fn get_predictions(input_vector: &ndarray::Array1<f64>, slope: f64, intercept: f64) -> Result<ndarray::Array1<f64>, String>
    {
        let input_vector = check_vector_for_nans(input_vector)?;
        Ok(input_vector
            .into_iter()
            .map(|x| slope * x + intercept)
            .collect())
    }

    #[allow(unused)]
    pub fn get_standard_deviation(input_vector: &ndarray::Array1<f64>) -> Result<f64, String> 
    {
        Ok(get_variance(input_vector)?.sqrt())
    }

    #[allow(unused)]
    pub fn get_variance(input_vector: &ndarray::Array1<f64>) -> Result<f64, String>
    {
        Ok (get_sum_of_squares(input_vector)? / (input_vector.len() as f64 - 1.0))
    }

    #[allow(unused)]
    pub fn get_sum_of_squares(input_vector: &ndarray::Array1<f64>) -> Result<f64, String>
    {
        let mean = get_mean(input_vector)?;
        
        Ok(input_vector.into_iter().map(
                                        |element|
                                        {(element - mean).powi(2)}).sum::<f64>() as f64)
    }

    #[allow(unused)]
    pub fn get_mean(input_vector: &ndarray::Array1<f64>) -> Result<f64, String> 
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

    pub fn check_vector_for_nans(input_vector: &ndarray::Array1<f64>) -> Result<&ndarray::Array1<f64>, String> 
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
    fn test_covariance()
    {
        let vector_1 = ndarray::arr1(&[12.0, 30.0, 15.0, 24.0, 14.0, 18.0, 28.0, 26.0, 19.0, 27.0]);
        let vector_2 = ndarray::arr1(&[20.0, 60.0, 27.0, 50.0, 21.0, 30.0, 61.0, 54.0, 32.0, 57.0]);

        let result = half_away_from_zero(utilities::get_sample_covariance(&vector_1, &vector_2).unwrap(), NUM_DECIMAL_DIGITS);
        let expected_result = 106.933;

        assert_eq!(result, expected_result);

    }

    #[test]
    fn test_covariance_with_self_is_variance()
    {
        let vector_1 = ndarray::arr1(&[12.0, 30.0, 15.0, 24.0, 14.0, 18.0, 28.0, 26.0, 19.0, 27.0]);

        let result = half_away_from_zero(utilities::get_sample_covariance(&vector_1, &vector_1).unwrap(), NUM_DECIMAL_DIGITS);
        let expected_result = half_away_from_zero(utilities::get_variance(&vector_1).unwrap(), NUM_DECIMAL_DIGITS);

        assert_eq!(result, expected_result);

    }

    #[test]
    fn test_coefficient_of_variation()
    {
        let vector = ndarray::arr1(&[85.0, 95.0, 75.0, 80.0, 90.0]);
        
        let result = half_away_from_zero(utilities::get_coefficient_of_variation(&vector).unwrap(), NUM_DECIMAL_DIGITS);
        let expected_result = 9.301;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_z_score() 
    {
        let vector = ndarray::arr1(&[85.0, 95.0, 75.0, 80.0, 90.0]);
        let sample = vector[vector.len() - 1].to_owned();
        let result = half_away_from_zero(utilities::get_z_score(sample, &vector).unwrap(), NUM_DECIMAL_DIGITS);
        let expected_result = 0.632;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_mean_empty_vector() 
    {
        let expected_result = Err(String::from("Vector cannot be empty"));
        let vector = ndarray::arr1(&[]);
        assert_eq!(utilities::get_mean(&vector), expected_result);
    }

    #[test]
    fn test_mean_containing_nan() 
    {
        let expected_result = Err(String::from("Vector must not contain nans!"));
        let vector = ndarray::arr1(&[2.0, f64::NAN]);
        assert_eq!(utilities::get_mean(&vector), expected_result);
    }

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
        let result = counting::get_permutations(n, r);

        let expected_result: Result<u128, String> = Ok(720);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_combinations() 
    {
        let n = 10;
        let r = 3;
        let result = counting::get_combinations(n, r);

        let expected_result: Result<u128, String> = Ok(120);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_factorial() 
    {
        let n = 4;
        let result = utilities::get_factorial(n);
        
        let expected_result: u128 = 24;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_get_standard_error() 
    {
        let x = ndarray::arr1(&[4.1512, 14.9715, 8.5378, 12.0471, 13.6555, 6.6369]);
        let observations = ndarray::arr1(&[5.0, 17.0, 11.0, 8.0, 14.0, 5.0]);
        let result = half_away_from_zero(utilities::get_standard_error(&x, &observations).unwrap(), NUM_DECIMAL_DIGITS);
        let expected_result = 2.742;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_get_mse() 
    {
        let x = ndarray::arr1(&[4.1512, 14.9715, 8.5378, 12.0471, 13.6555, 6.6369]);
        let observations = ndarray::arr1(&[5.0, 17.0, 11.0, 8.0, 14.0, 5.0]);
        let result = half_away_from_zero( utilities::get_mse(&x, &observations).unwrap(),NUM_DECIMAL_DIGITS);
        let expected_result = 7.519;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_get_coefficient_of_determination() 
    {
        let x = ndarray::arr1(&[34.0, 108.0, 64.0, 88.0, 99.0, 51.0]);
        let observations = ndarray::arr1(&[5.0, 17.0, 11.0, 8.0, 14.0, 5.0]);
        let slope = get_best_fitting_slope(&x, &observations).unwrap();
        let intercept = get_best_fitting_intercept(&x, &observations).unwrap();
        let predictions = utilities::get_predictions(&x, slope, intercept).unwrap();
        let result = half_away_from_zero(utilities::get_coefficient_of_determination(&predictions, &observations).unwrap(), NUM_DECIMAL_DIGITS);
        
        let expected_result = 0.749;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_get_ssr() 
    {
        let x = ndarray::arr1(&[34.0, 108.0, 64.0, 88.0, 99.0, 51.0]);
        let observations = ndarray::arr1(&[5.0, 17.0, 11.0, 8.0, 14.0, 5.0]);
        let slope = get_best_fitting_slope(&x, &observations).unwrap();
        let intercept = get_best_fitting_intercept(&x, &observations).unwrap();
        let predictions = utilities::get_predictions(&x, slope, intercept).unwrap();
        let result = half_away_from_zero(utilities::get_ssr(&predictions, &observations).unwrap(), NUM_DECIMAL_DIGITS);
        
        let expected_result = 89.925;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_get_correlation_coefficient() 
    {
        let observations = ndarray::arr1(&[5.0, 17.0, 11.0, 8.0, 14.0, 5.0]);
        let x = ndarray::arr1(&[34.0, 108.0, 64.0, 88.0, 99.0, 51.0]);
        let result = half_away_from_zero(utilities::get_correlation_coefficient(&x, &observations).unwrap(), NUM_DECIMAL_DIGITS);

        let expected_result = 0.866;
        assert_eq!(result, expected_result);
    }
    #[test]
    fn test_get_auto_correlation() 
    {
        let x = ndarray::arr1(&[34.0, 108.0, 64.0, 88.0, 99.0, 51.0]);
        let result = half_away_from_zero(utilities::get_correlation_coefficient(&x, &x).unwrap(), NUM_DECIMAL_DIGITS);

        let expected_result = 1.0;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_get_sse() 
    {
        let observations = ndarray::arr1(&[5.0, 17.0, 11.0, 8.0, 14.0, 5.0]);
        let x = ndarray::arr1(&[34.0, 108.0, 64.0, 88.0, 99.0, 51.0]);
        let slope = get_best_fitting_slope(&x, &observations).unwrap();
        let intercept = get_best_fitting_intercept(&x, &observations).unwrap();
        let predictions = utilities::get_predictions(&x, slope, intercept).unwrap();
        let result = half_away_from_zero (utilities::get_sse(&predictions, &observations).unwrap(), NUM_DECIMAL_DIGITS);

        let expected_result = 30.075;
        assert_approx_eq::assert_approx_eq!(result, expected_result, 0.1);
    }

    #[test]
    fn test_get_sst() 
    {
        let observations = ndarray::arr1(&[5.0, 17.0, 11.0, 8.0, 14.0, 5.0]);
        let result = utilities::get_sst(&observations).unwrap();

        let expected_result = 120.0;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_get_prediction() 
    {
        let x = ndarray::arr1(&[34.0, 108.0, 64.0, 88.0, 99.0, 51.0]);
        let result = utilities::get_predictions(&x, 0.14621968616262482, -0.8202567760342365).unwrap();

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
        let result = half_away_from_zero(get_best_fitting_intercept(&x, &y).unwrap(), NUM_DECIMAL_DIGITS);
        let expected_result = -0.820;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_best_fitting_slope() 
    {
        let x = ndarray::arr1(&[34.0, 108.0, 64.0, 88.0, 99.0, 51.0]);
        let y = ndarray::arr1(&[5.0, 17.0, 11.0, 8.0, 14.0, 5.0]);
        let result = half_away_from_zero(get_best_fitting_slope(&x, &y).unwrap(), NUM_DECIMAL_DIGITS);
        let expected_result = 0.146;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_mean() 
    {
        let v = ndarray::arr1(&[2.0, 4.0, 6.0]);
        assert_eq!(utilities::get_mean(&v).unwrap(), 4.0);
    }
    
    #[test]
    fn test_mean_length_1() 
    {
        let v = ndarray::arr1(&[2.0]);
        assert_eq!(utilities::get_mean(&v).unwrap(), v[0]);
    }

    #[test]
    #[should_panic]
    fn test_vector_length_for_panic()
    {
        let vec1 = ndarray::arr1(&[1.0, 2.0, 3.0]);
        let vec2 = ndarray::arr1(&[1.0, 2.0]);
        utilities::check_vectors_for_equal_length(&vec1, &vec2);
    }

    #[test]
    fn test_standard_deviation() 
    {
        let v = ndarray::arr1(&[34.0, 108.0, 64.0, 88.0, 99.0, 51.0]);
        let result = half_away_from_zero (utilities::get_standard_deviation(&v).unwrap(), NUM_DECIMAL_DIGITS);
        let expected_result = 29.003;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_variance() 
    {
        let v = ndarray::arr1(&[2.0, 3.0, 4.0, 5.0, 6.0]);
        let result = half_away_from_zero(utilities::get_variance(&v).unwrap(), NUM_DECIMAL_DIGITS);
        let expected_result = 2.5;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_sum_of_squares() 
    {
        let v = ndarray::arr1(&[2.0, 3.0, 4.0, 5.0, 6.0]);
        let result = half_away_from_zero(utilities::get_sum_of_squares(&v).unwrap(), NUM_DECIMAL_DIGITS);
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
