use simple_statistics::{file_reading, utilities, sampling, simple_linear_regression};
use ndarray::{Axis};

fn main()
{
    let input_file_has_headers = false;
    let some_data = file_reading::read_csv_to_array("src/datasets/data_banknote_authentication.csv", input_file_has_headers).unwrap();

    let some_column = some_data.index_axis(Axis(1), 1).to_owned();
    let some_other_column = some_data.index_axis(Axis(1), 2).to_owned();

    let some_corrcoef = simple_linear_regression::compute_correlation_coefficient(&some_column, &some_other_column).unwrap();

    let _some_sample = sampling::get_sample(&some_data, 50);
    println!("final shape is: {:?}", _some_sample.unwrap().shape());

    
    let some_mean = utilities::compute_mean(&some_column).unwrap();
    let some_std = utilities::compute_standard_deviation(&some_column).unwrap();
    let some_variance = utilities::compute_variance(&some_column).unwrap();

    println!(" mean: {:?},\n std: {:?},\n var: {:?},\n corrcoef: {:}", some_mean, some_std, some_variance, some_corrcoef);
}