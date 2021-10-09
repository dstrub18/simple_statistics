use simple_statistics::{file_reading, utilities, sampling};
use ndarray::{Axis};

fn main()
{
    let input_file_has_headers = false;
    let some_data = file_reading::read_csv_to_array("src/datasets/data_banknote_authentication.csv", input_file_has_headers).unwrap();

    let some_column = some_data.index_axis(Axis(1), 1).to_owned();
    let some_other_column = some_data.index_axis(Axis(1), 2).to_owned();

    let use_seed_for_sampling = true;
    let some_sample = sampling::get_sample(&some_data, 500, use_seed_for_sampling).unwrap();
    let some_sample_column = some_sample.index_axis(Axis(1), 1).to_owned();
    let some_t_value = sampling::get_t_distribution(&some_column, &some_sample_column);
    println!("t-value is: {:?}", some_t_value);

    let some_corrcoef = utilities::get_correlation_coefficient(&some_column, &some_other_column).unwrap();

    let some_mean = utilities::get_mean(&some_column).unwrap();
    let some_std = utilities::get_standard_deviation(&some_column).unwrap();
    let some_variance = utilities::get_variance(&some_column).unwrap();

    println!(" mean: {:?},\n std: {:?},\n var: {:?},\n corrcoef: {:}", some_mean, some_std, some_variance, some_corrcoef);
}