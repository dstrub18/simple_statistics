mod linear_regression
{
    pub mod linear_least_squares
    {
        use super::utilities::*;

        #[allow(unused)]
        pub fn get_slope_and_intercept(x: &Vec<f64>, y: &Vec<f64>) ->(f64, f64)
        {
            (compute_best_fitting_slope(x, y), compute_best_fitting_intercept(x, y))
        }

        #[allow(unused)]
        pub fn compute_best_fitting_intercept(x: &Vec<f64>, y: &Vec<f64>) -> f64
        {
            compute_mean(y) - compute_best_fitting_slope(x, y) * compute_mean(x)
        }

        #[allow(unused)]
        pub fn compute_best_fitting_slope(x: &Vec<f64>, y: &Vec<f64>) -> f64
        {
            let mean_x = compute_mean(x);
            let mean_y = compute_mean(y);
            let mut numerator_sum = 0.0;
            let mut denominator_sum = 0.0;

            for (x_i, y_i) in x.iter().zip(y) 
            {
                let x_diff_to_mean = x_i - mean_x;
                let y_diff_to_mean = y_i - mean_y;
                numerator_sum += x_diff_to_mean * y_diff_to_mean;

                let mut x_dev_denominator = x_i - mean_x;
                x_dev_denominator = x_dev_denominator.powi(2);
                denominator_sum += x_dev_denominator;
            }
            numerator_sum / denominator_sum
        }
    }

    pub mod utilities
    {
        use super::linear_least_squares;

        #[allow(unused)]
        pub fn compute_standard_error(x: &Vec<f64>, y: &Vec<f64>) ->f64
        {
            compute_mse(x, y).sqrt()
        }

        #[allow(unused)]
        pub fn compute_mse(x: &Vec<f64>, y: &Vec<f64>) ->f64
        {
            let degrees_of_freedom = 2;
            // This dependency does not make too much sense.
            compute_sse(x, y) / ((x.len() - degrees_of_freedom) as f64)
        }


        #[allow(unused)]
        pub fn compute_correlation_coefficient(x: &Vec<f64>, y: &Vec<f64>) ->f64
        {
            // This dependency does not make too much sense.
            linear_least_squares::compute_best_fitting_slope(x, y) * (compute_standard_deviation(x) / compute_standard_deviation(y))
        }

        #[allow(unused)]
        pub fn compute_coefficient_of_determination(predictions: &Vec<f64>, observations: &Vec<f64>) -> f64
        {
            compute_ssr(predictions, observations) / compute_sst(observations)
        }

        #[allow(unused)]
        pub fn compute_ssr(predictions: &Vec<f64>, observations: &Vec<f64>) -> f64
        {
            compute_sst(observations) - compute_sse(predictions, observations)
        }

        #[allow(unused)]
        pub fn compute_sst(observations: &Vec<f64>) -> f64
        {
            let mean = compute_mean(observations);
            observations.into_iter().map
            (
                |x|
                {
                let error = x - mean;
                error.powi(2)
                }).sum()
        }

        #[allow(unused)]
        pub fn compute_sse(predictions: &Vec<f64>, observations: &Vec<f64>) -> f64
        {
            predictions.iter().zip(observations).into_iter().map
            (
                |zipped_element|
                {
                    let error = zipped_element.0 - zipped_element.1;
                    error.powi(2)
                }
            ).into_iter().sum()
        }

        #[allow(unused)]
        pub fn compute_predictions(input_vector:  &Vec<f64>, slope: f64, intercept: f64) -> Vec<f64>
        {
            input_vector.into_iter().map
            (
                |x|
                {
                    slope * x + intercept
                }
            ).collect()
        }
       
        #[allow(unused)]
        pub fn standardize_range(x: &Vec<f64>) -> Vec<f64>
        {
            let mean = compute_mean(x);
            let standard_deviation = compute_standard_deviation(x);

            x.into_iter().map(|x|{(x - mean) / standard_deviation}).collect()
        }

        #[allow(unused)]
        pub fn compute_standard_deviation(input_vector: &Vec<f64>) -> f64
        {
            let mean = compute_mean(&input_vector);
            let mut sum_of_squared_differences = 0.0;
            for i in input_vector.iter() 
            {
                let diff_to_mean = i - mean;
                sum_of_squared_differences += diff_to_mean.powi(2);
            }
            let variance = sum_of_squared_differences / input_vector.len() as f64;
            let standard_deviation = variance.sqrt();
            standard_deviation as f64
        }

        #[allow(unused)]
        pub fn compute_mean(input_vector: &Vec<f64>) -> f64
        {   
            input_vector.iter().sum::<f64>() / input_vector.len() as f64
        }
    } // End mod utilities
} // End mod linear regression




#[cfg(test)]
mod tests
{
    use assert_approx_eq;
    use crate::linear_regression::*;
    
    #[test]
    fn test_compute_standard_error()
    {
        let x = vec!(4.1512, 14.9715, 8.5378, 12.0471, 13.6555, 6.6369);
        let observations = vec!(5.0, 17.0, 11.0, 8.0, 14.0, 5.0);
        let result = utilities::compute_standard_error(&x, &observations);
        let expected_result = 2.7420267504165596;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_compute_mse()
    {
        let x = vec!(4.1512, 14.9715, 8.5378, 12.0471, 13.6555, 6.6369);
        let observations = vec!(5.0, 17.0, 11.0, 8.0, 14.0, 5.0);
        let result = utilities::compute_mse(&x, &observations);
        let expected_result = 7.518710699999999;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_standardize_range()
    {
        let x = vec!(34.0, 108.0, 64.0, 88.0, 99.0, 51.0);
        let result = utilities::standardize_range(&x);
        let expected_result = vec!(-1.5107791492009959, 1.2841622768208465, -0.37769478730024897, 0.5287727022203486, 0.9442369682506224, -0.8686980107905726) as Vec<f64>;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_compute_coefficient_of_determination()
    {
        let observations = vec!(5.0, 17.0, 11.0, 8.0, 14.0, 5.0);
        let x = vec!(34.0, 108.0, 64.0, 88.0, 99.0, 51.0);
        let slope = linear_least_squares::compute_best_fitting_slope(&x, &observations);
        let intercept = linear_least_squares::compute_best_fitting_intercept(&x, &observations);

        let predictions = utilities::compute_predictions(&x, slope, intercept);
        let result = round(&utilities::compute_coefficient_of_determination(&predictions, &observations), 2);
        assert_approx_eq::assert_approx_eq!(result, 0.7493, 0.001);
    }

    #[test]
    fn test_compute_ssr()
    {
        let observations = vec!(5.0, 17.0, 11.0, 8.0, 14.0, 5.0);
        let x = vec!(34.0, 108.0, 64.0, 88.0, 99.0, 51.0);
        let slope = linear_least_squares::compute_best_fitting_slope(&x, &observations);
        let intercept = linear_least_squares::compute_best_fitting_intercept(&x, &observations);

        let predictions = utilities::compute_predictions(&x, slope, intercept);
        let result = round(&utilities::compute_ssr(&predictions, &observations), 3);
        assert_approx_eq::assert_approx_eq!(result, 89.925, 0.1);
    }

    #[test]
    fn test_compute_correlation_coefficient()
    {
        let observations = vec!(5.0, 17.0, 11.0, 8.0, 14.0, 5.0);
        let x = vec!(34.0, 108.0, 64.0, 88.0, 99.0, 51.0);
        let expected_result = 0.8656649996294479;
        assert_eq!(utilities::compute_correlation_coefficient(&x, &observations), expected_result);
    }

    #[test]
    fn test_compute_sse()
    {
        let observations = vec!(5.0, 17.0, 11.0, 8.0, 14.0, 5.0);
        let x = vec!(34.0, 108.0, 64.0, 88.0, 99.0, 51.0);
        let slope = linear_least_squares::compute_best_fitting_slope(&x, &observations);
        let intercept = linear_least_squares::compute_best_fitting_intercept(&x, &observations);

        let predictions = utilities::compute_predictions(&x, slope, intercept);
        let result = round(&utilities::compute_sse(&predictions, &observations), 3);
        assert_approx_eq::assert_approx_eq!(result, 30.075, 0.1);
    }

    #[test]
    fn test_compute_sst()
    {
        let observations = vec!(5.0, 17.0, 11.0, 8.0, 14.0, 5.0);
        assert_eq!(utilities::compute_sst(&observations), 120.0);
    }


    #[test]
    fn test_compute_prediction()
    {
        let x = vec!(34.0, 108.0, 64.0, 88.0, 99.0, 51.0);
        let expected_result = vec!(4.1512, 14.9715, 8.5378, 12.0471, 13.6555, 6.6369);

        assert_eq!(round_vector_elements(&utilities::compute_predictions(&x, 0.14621968616262482, -0.8202567760342365), 2), round_vector_elements(&expected_result, 2));
    }

    #[test]
    fn test_best_fitting_intercept()
    {
        let x = vec!(34.0, 108.0, 64.0, 88.0, 99.0, 51.0);
        let y = vec!(5.0, 17.0, 11.0, 8.0, 14.0, 5.0);
        assert_eq!(linear_least_squares::compute_best_fitting_intercept(&x, &y), -0.8202567760342365);
    }

    #[test]
    fn test_best_fitting_slope()
    {
        let x = vec!(34.0, 108.0, 64.0, 88.0, 99.0, 51.0);
        let y = vec!(5.0, 17.0, 11.0, 8.0, 14.0, 5.0);
        assert_eq!(linear_least_squares::compute_best_fitting_slope(&x, &y), 0.14621968616262482);
    }

    #[test]
    fn test_mean()
    {
        let v = vec!(2.0, 4.0, 6.0);
        assert_eq!(utilities::compute_mean(&v), 4.0);
    }

    #[test]
    fn test_standard_deviation()
    {
        let v = vec!(2.0, 4.0, 6.0, 8.0);
        assert_eq!(round(&utilities::compute_standard_deviation(&v), 3), round(&2.23606797749979, 3));
    }

    #[test]
    fn test_round()
    {
        let x = 3.001;
        assert_eq!(round(&x, 2), 3.000);
    }

    // Rounding utilities
    fn round_vector_elements(x: &Vec<f64>, decimal_precision: u32) -> Vec<f64>
    {
        let exp = (10^decimal_precision) as f64;
        x.into_iter().map
        (|elem|
            {
            let scaled_element = elem * exp;
            scaled_element.round() / exp
            }
        ).collect()
    }

    fn round(x: &f64, decimal_precision: u32) -> f64
    {
        let exp = (10^decimal_precision) as f64;
        (x * exp).round() / exp
    }
}
