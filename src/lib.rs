pub mod file_reading {
    use std::error::Error;
    use std::fs::File;
    pub fn read_csv(path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(path).expect("File not found!");
        let mut rdr = csv::Reader::from_reader(file);
        println!("Worked!");
        for result in rdr.records() {
            let record = result?;
            println!("{:?}", record);
        }
        Ok(())
    }
}

mod counting {
    use super::utilities::compute_factorial;
    #[allow(unused)]
    pub fn compute_combinations(n: u64, r: u64) -> u128 {
        compute_factorial(n) / (compute_factorial(r) * compute_factorial(n - r))
    }

    #[allow(unused)]
    pub fn compute_permutations(n: u64, r: u64) -> u128 {
        compute_factorial(n) / compute_factorial(n - r)
    }
} //  End counting

/// Linear regression with multiple variables
mod multiple_regression {}

/// Linear regression with 1 independent variable.
mod simple_linear_regression {
    use super::utilities::*;

    #[allow(unused)]
    pub fn get_slope_and_intercept(x: &Vec<f64>, y: &Vec<f64>) -> (f64, f64) {
        (
            compute_best_fitting_slope(x, y),
            compute_best_fitting_intercept(x, y),
        )
    }

    #[allow(unused)]
    pub fn compute_best_fitting_intercept(
        independent_variable: &Vec<f64>,
        dependent_variable: &Vec<f64>,
    ) -> f64 {
        compute_mean(dependent_variable).unwrap()
            - compute_best_fitting_slope(independent_variable, dependent_variable)
                * compute_mean(independent_variable).unwrap()
    }

    #[allow(unused)]
    pub fn compute_best_fitting_slope(
        independent_variable: &Vec<f64>,
        dependent_variable: &Vec<f64>,
    ) -> f64 {
        let mean_independent_variable = compute_mean(independent_variable).unwrap();
        let mean_dependent_variable = compute_mean(dependent_variable).unwrap();
        let mut numerator_sum = 0.0;
        let mut denominator_sum = 0.0;

        for (x_i, y_i) in independent_variable.iter().zip(dependent_variable) {
            let x_diff_to_mean = x_i - mean_independent_variable;
            let y_diff_to_mean = y_i - mean_dependent_variable;
            numerator_sum += x_diff_to_mean * y_diff_to_mean;

            let mut x_dev_denominator = x_i - mean_independent_variable;
            x_dev_denominator = x_dev_denominator.powi(2);
            denominator_sum += x_dev_denominator;
        }
        numerator_sum / denominator_sum
    }

    #[allow(unused)]
    pub fn compute_correlation_coefficient(
        independent_variable: &Vec<f64>,
        dependent_variable: &Vec<f64>,
    ) -> f64 {
        // This dependency does not make too much sense.
        compute_best_fitting_slope(independent_variable, dependent_variable)
            * (compute_standard_deviation(independent_variable).unwrap()
                / compute_standard_deviation(dependent_variable).unwrap())
    }
} // End mod linear regression

pub mod utilities {
    #[allow(unused)]
    pub fn compute_standard_error(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
        compute_mse(x, y).sqrt()
    }

    #[allow(unused)]
    pub fn compute_mse(x: &Vec<f64>, y: &Vec<f64>) -> f64 {
        let degrees_of_freedom = 2;
        // This dependency does not make too much sense.
        compute_sse(x, y) / ((x.len() - degrees_of_freedom) as f64)
    }

    #[allow(unused)]
    pub fn compute_coefficient_of_determination(
        predictions: &Vec<f64>,
        observations: &Vec<f64>,
    ) -> f64 {
        compute_ssr(predictions, observations) / compute_sst(observations)
    }

    #[allow(unused)]
    pub fn compute_ssr(predictions: &Vec<f64>, observations: &Vec<f64>) -> f64 {
        compute_sst(observations) - compute_sse(predictions, observations)
    }

    #[allow(unused)]
    pub fn compute_sst(observations: &Vec<f64>) -> f64 {
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
    pub fn compute_sse(predictions: &Vec<f64>, observations: &Vec<f64>) -> f64 {
        predictions
            .iter()
            .zip(observations)
            .into_iter()
            .map(|zipped_element| {
                let error = zipped_element.0 - zipped_element.1;
                error.powi(2)
            })
            .into_iter()
            .sum()
    }

    #[allow(unused)]
    pub fn compute_predictions(input_vector: &Vec<f64>, slope: f64, intercept: f64) -> Vec<f64> {
        check_vector_for_nans(input_vector);
        input_vector
            .into_iter()
            .map(|x| slope * x + intercept)
            .collect()
    }
    #[allow(unused)]
    pub fn standardize_range(x: &Vec<f64>) -> Result<Vec<f64>, &str> {
        let mean = compute_mean(x)?;
        let standard_deviation = compute_standard_deviation(x)?;

        Ok(x.into_iter()
            .map(|x| (x - mean) / standard_deviation)
            .collect())
    }

    #[allow(unused)]
    pub fn compute_standard_deviation(input_vector: &Vec<f64>) -> Result<f64, &str> {
        let mean = compute_mean(&input_vector)?;
        let mut sum_of_squared_differences = 0.0;
        for i in input_vector.iter() {
            let diff_to_mean = i - mean;
            sum_of_squared_differences += diff_to_mean.powi(2);
        }
        let variance = sum_of_squared_differences / input_vector.len() as f64;
        let standard_deviation = variance.sqrt();
        Ok(standard_deviation as f64)
    }

    #[allow(unused)]
    pub fn compute_mean(input_vector: &Vec<f64>) -> Result<f64, &str> {
        match check_vector_for_nans(input_vector)?.len() {
            0 => Err("Vector cannot be empty"),
            1 => Ok(input_vector[0]),
            _ => Ok(input_vector.iter().sum::<f64>() / input_vector.len() as f64),
        }
    }

    pub fn compute_factorial(n: u64) -> u128 {
        let mut result: u128 = 1;
        for i in (2..n + 1).rev() {
            result = result * i as u128;
        }
        result
    }

    pub fn check_vector_for_nans(input_vector: &Vec<f64>) -> Result<&Vec<f64>, &str> {
        if !input_vector.iter().any(|&x| x.is_nan()) {
            Ok(input_vector)
        } else {
            Err("Vector must not contain nans!")
        }
    }
} // End mod utilities

#[cfg(test)]
mod tests {
    use crate::counting;
    use crate::simple_linear_regression::*;
    use crate::utilities;
    use assert_approx_eq;

    #[test]
    fn test_nan_check_for_error() {
        let expected_result = Err("Vector must not contain nans!");
        let vector = vec![1.0, f64::NAN];
        assert_eq!(utilities::check_vector_for_nans(&vector), expected_result);
    }
    #[test]
    fn test_nan_check_for_success() {
        let vector = vec![1.0, 2.0];
        let expected_result = Ok(&vector);
        assert_eq!(utilities::check_vector_for_nans(&vector), expected_result);
    }

    #[test]
    fn test_permutations() {
        let expected_result: u128 = 720;
        let n = 10;
        let r = 3;
        assert_eq!(counting::compute_permutations(n, r), expected_result);
    }

    #[test]
    fn test_combinations() {
        let expected_result: u128 = 120;
        let n = 10;
        let r = 3;
        assert_eq!(counting::compute_combinations(n, r), expected_result);
    }

    #[test]
    fn test_factorial() {
        let expected_result: u128 = 24;
        let n = 4;
        assert_eq!(utilities::compute_factorial(n), expected_result);
    }
    #[test]
    fn test_compute_standard_error() {
        let x = vec![4.1512, 14.9715, 8.5378, 12.0471, 13.6555, 6.6369];
        let observations = vec![5.0, 17.0, 11.0, 8.0, 14.0, 5.0];
        let result = utilities::compute_standard_error(&x, &observations);
        let expected_result = 2.7420267504165596;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_compute_mse() {
        let x = vec![4.1512, 14.9715, 8.5378, 12.0471, 13.6555, 6.6369];
        let observations = vec![5.0, 17.0, 11.0, 8.0, 14.0, 5.0];
        let result = utilities::compute_mse(&x, &observations);
        let expected_result = 7.518710699999999;
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_standardize_range() {
        let x = vec![34.0, 108.0, 64.0, 88.0, 99.0, 51.0];
        let result = utilities::standardize_range(&x);
        let expected_result = Ok(vec![
            -1.5107791492009959,
            1.2841622768208465,
            -0.37769478730024897,
            0.5287727022203486,
            0.9442369682506224,
            -0.8686980107905726,
        ] as Vec<f64>);
        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_compute_coefficient_of_determination() {
        let x = vec![34.0, 108.0, 64.0, 88.0, 99.0, 51.0];
        let observations = vec![5.0, 17.0, 11.0, 8.0, 14.0, 5.0];
        let slope = compute_best_fitting_slope(&x, &observations);
        let intercept = compute_best_fitting_intercept(&x, &observations);

        let predictions = utilities::compute_predictions(&x, slope, intercept);
        let result = round(
            &utilities::compute_coefficient_of_determination(&predictions, &observations),
            2,
        );
        assert_approx_eq::assert_approx_eq!(result, 0.7493, 0.001);
    }

    #[test]
    fn test_compute_ssr() {
        let x = vec![34.0, 108.0, 64.0, 88.0, 99.0, 51.0];
        let observations = vec![5.0, 17.0, 11.0, 8.0, 14.0, 5.0];
        let slope = compute_best_fitting_slope(&x, &observations);
        let intercept = compute_best_fitting_intercept(&x, &observations);

        let predictions = utilities::compute_predictions(&x, slope, intercept);
        let result = round(&utilities::compute_ssr(&predictions, &observations), 3);
        assert_approx_eq::assert_approx_eq!(result, 89.925, 0.1);
    }

    #[test]
    fn test_compute_correlation_coefficient() {
        let observations = vec![5.0, 17.0, 11.0, 8.0, 14.0, 5.0];
        let x = vec![34.0, 108.0, 64.0, 88.0, 99.0, 51.0];
        let expected_result = 0.8656649996294479;
        assert_eq!(
            compute_correlation_coefficient(&x, &observations),
            expected_result
        );
    }

    #[test]
    fn test_compute_sse() {
        let observations = vec![5.0, 17.0, 11.0, 8.0, 14.0, 5.0];
        let x = vec![34.0, 108.0, 64.0, 88.0, 99.0, 51.0];
        let slope = compute_best_fitting_slope(&x, &observations);
        let intercept = compute_best_fitting_intercept(&x, &observations);

        let predictions = utilities::compute_predictions(&x, slope, intercept);
        let result = round(&utilities::compute_sse(&predictions, &observations), 3);
        assert_approx_eq::assert_approx_eq!(result, 30.075, 0.1);
    }

    #[test]
    fn test_compute_sst() {
        let observations = vec![5.0, 17.0, 11.0, 8.0, 14.0, 5.0];
        assert_eq!(utilities::compute_sst(&observations), 120.0);
    }

    #[test]
    fn test_compute_prediction() {
        let x = vec![34.0, 108.0, 64.0, 88.0, 99.0, 51.0];
        let expected_result = vec![4.1512, 14.9715, 8.5378, 12.0471, 13.6555, 6.6369];

        assert_eq!(
            round_vector_elements(
                &utilities::compute_predictions(&x, 0.14621968616262482, -0.8202567760342365),
                2
            ),
            round_vector_elements(&expected_result, 2)
        );
    }

    #[test]
    fn test_best_fitting_intercept() {
        let x = vec![34.0, 108.0, 64.0, 88.0, 99.0, 51.0];
        let y = vec![5.0, 17.0, 11.0, 8.0, 14.0, 5.0];
        assert_eq!(compute_best_fitting_intercept(&x, &y), -0.8202567760342365);
    }

    #[test]
    fn test_best_fitting_slope() {
        let x = vec![34.0, 108.0, 64.0, 88.0, 99.0, 51.0];
        let y = vec![5.0, 17.0, 11.0, 8.0, 14.0, 5.0];
        assert_eq!(compute_best_fitting_slope(&x, &y), 0.14621968616262482);
    }

    #[test]
    fn test_mean() {
        let v = vec![2.0, 4.0, 6.0];
        assert_eq!(utilities::compute_mean(&v).unwrap(), 4.0);
    }
    #[test]
    fn test_mean_length_1() {
        let v = vec![2.0];
        assert_eq!(utilities::compute_mean(&v).unwrap(), 2.0);
    }

    #[test]
    fn test_standard_deviation() {
        let v = vec![2.0, 4.0, 6.0, 8.0];
        assert_eq!(
            round(&utilities::compute_standard_deviation(&v).unwrap(), 3),
            round(&2.23606797749979, 3)
        );
    }

    #[test]
    fn test_round() {
        let x = 3.001;
        assert_eq!(round(&x, 2), 3.000);
    }

    // Rounding utilities
    fn round_vector_elements(x: &Vec<f64>, decimal_precision: u32) -> Vec<f64> {
        let exp = (10 ^ decimal_precision) as f64;
        x.into_iter()
            .map(|elem| {
                let scaled_element = elem * exp;
                scaled_element.round() / exp
            })
            .collect()
    }

    fn round(x: &f64, decimal_precision: u32) -> f64 {
        let exp = (10 ^ decimal_precision) as f64;
        (x * exp).round() / exp
    }
}
