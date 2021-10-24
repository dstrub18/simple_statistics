
#[cfg(test)]
mod tests
{
    
    use math::round::*;
    use simple_statistics::counting;
    use simple_statistics::simple_linear_regression;
    use simple_statistics::utilities;
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
        let vector = ndarray::arr1(&[1.0f64, 2.0f64]);
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
        let slope = simple_linear_regression::get_best_fitting_slope(&x, &observations).unwrap();
        let intercept = simple_linear_regression::get_best_fitting_intercept(&x, &observations).unwrap();
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
        let slope = simple_linear_regression::get_best_fitting_slope(&x, &observations).unwrap();
        let intercept = simple_linear_regression::get_best_fitting_intercept(&x, &observations).unwrap();
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
        let slope = simple_linear_regression::get_best_fitting_slope(&x, &observations).unwrap();
        let intercept = simple_linear_regression::get_best_fitting_intercept(&x, &observations).unwrap();
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
        let result = half_away_from_zero(simple_linear_regression::get_best_fitting_intercept(&x, &y).unwrap(), NUM_DECIMAL_DIGITS);
        let expected_result = -0.820;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_best_fitting_slope() 
    {
        let x = ndarray::arr1(&[34.0, 108.0, 64.0, 88.0, 99.0, 51.0]);
        let y = ndarray::arr1(&[5.0, 17.0, 11.0, 8.0, 14.0, 5.0]);
        let result = half_away_from_zero(simple_linear_regression::get_best_fitting_slope(&x, &y).unwrap(), NUM_DECIMAL_DIGITS);
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