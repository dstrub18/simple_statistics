use super::utilities::get_factorial;

#[allow(unused)]
pub fn get_combinations(n: u64, r: u64) -> Result<u128, String> 
{
    if n <= 0 || r <= 0
    {
        Err(String::from ("n and r must be equal or greater than 0!"))
    }
    else
    {
        Ok(get_factorial(n) / (get_factorial(r) * get_factorial(n - r)))
    }
}

#[allow(unused)]
pub fn get_permutations(n: u64, r: u64) -> Result<u128, String> 
{
    if n <= 0 || r <= 0
    {
        Err(String::from("n and r must be equal or greater than 0!"))
    }
    else
    {
        Ok(get_factorial(n) / get_factorial(n - r))
    }
}