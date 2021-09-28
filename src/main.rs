fn main() 
{
    let something = simple_statistics::file_reading::read_csv_to_array("src/datasets/data_banknote_authentication.csv");
    println!("{:?}", something);
}
