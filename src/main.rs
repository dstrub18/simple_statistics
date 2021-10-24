use simple_statistics::{file_reading, utilities, sampling};
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

}