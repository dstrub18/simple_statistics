fn main()
{
    let something = simple_statistics::file_reading::read_csv_to_array("src/datasets/data_banknote_authentication.csv").unwrap();

    let some_column = something.index_axis(ndarray::Axis(1), 1).to_owned();
    let some_other_column = something.index_axis(ndarray::Axis(1), 2).to_owned();

    let some_corrcoef = simple_statistics::simple_linear_regression::compute_correlation_coefficient(&some_column, &some_other_column).unwrap();

    let _some_sample = simple_statistics::sampling::get_sample(&something, 30);
    println!("final shape is: {:?}", _some_sample.shape());

    
    let some_mean = simple_statistics::utilities::compute_mean(&some_column).unwrap();
    let some_std = simple_statistics::utilities::compute_standard_deviation(&some_column).unwrap();
    let some_variance = simple_statistics::utilities::compute_variance(&some_column).unwrap();

    println!(" mean: {:?},\n std: {:?},\n var: {:?},\n corrcoef: {:}", some_mean, some_std, some_variance, some_corrcoef);
}
