use simple_statistics::{file_reading, utilities, sampling, simple_linear_regression};
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

    let x = ndarray::arr1(&[34.0, 108.0, 64.0, 88.0, 99.0, 51.0]);
    let y = ndarray::arr1(&[5.0, 17.0, 11.0, 8.0, 14.0, 5.0]);
    
    let info = simple_linear_regression::get_variable_target_info(&x, &y);
    println!("{:#?}", info);

}