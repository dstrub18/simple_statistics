use simple_statistics::{file_reading, utilities, sampling, hypothesis_testing};
use ndarray::Axis;


fn main()
{
    let input_file_has_headers = false;
    let some_data = file_reading::read_csv_to_array("src/datasets/data_banknote_authentication.csv", input_file_has_headers).unwrap();

    let some_column = some_data.index_axis(Axis(1), 1).to_owned();

    let _column_info = utilities::get_variable_info(&some_column);

    let use_seed_for_sampling = true;
    let some_sample = sampling::get_sample(&some_data, 500, use_seed_for_sampling).unwrap();
    let _some_sample_column = some_sample.index_axis(Axis(1), 1).to_owned();

    let cov_mat = utilities::get_covariance_matrix(&some_data);
    println!("{:#?}", cov_mat);



    // Analysis on fish dataset
    let fish_dataset = file_reading::read_csv_to_array("src/datasets/Fish.csv", input_file_has_headers).unwrap();

    let corr_coeff_mat = utilities::get_correlation_coefficient_matrix(&fish_dataset);
    println!("{:#?}", corr_coeff_mat);

    // Example 1
    let mut sample = ndarray::Array1::<f64>::zeros(112);
    for i in 0..sample.shape()[0]
    {
        sample[i] = 72180.0;
    }
    let hypothesized_mean = 69873.0;
    let population_std = 13985.0;
    let alpha_level = 0.05;
    let ztest = hypothesis_testing::ZTest::new(hypothesis_testing::NullHypothesisKind::EqualTo, alpha_level).unwrap();

    ztest.perform_test(hypothesized_mean, population_std, &sample);

    // Example 2
    let mut sample = ndarray::Array1::<f64>::zeros(225);
    for i in 0..sample.shape()[0]
    {
        sample[i] = 3.25;
    }
    let hypothesized_mean = 3.0;
    let population_std = 1.5;
    let alpha_level = 0.01;

    let ztest = hypothesis_testing::ZTest::new(hypothesis_testing::NullHypothesisKind::GreaterThanOrEqualTo, alpha_level).unwrap();

    ztest.perform_test(hypothesized_mean, population_std, &sample);

    
}