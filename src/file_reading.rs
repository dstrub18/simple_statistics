
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