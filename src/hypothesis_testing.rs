use super::utilities::{get_mean, get_variance};
use ndarray::{Array1};
use prettytable::{Table, Attr, row, cell, color, Row, Cell};

#[allow(unused)]
pub enum ZTestKind
{
    OneTailedUpper,
    OneTailedLower,
    TwoTailed,
}

#[allow(unused)]
pub struct ZTest
{
    z_test_kind: ZTestKind,
    alpha_level: f64,
    z_critical: f64
}


#[allow(unused)]
impl ZTest
{
    pub fn perform_test(&self, hypothesized_mean: f64, population_std: f64, sample: &ndarray::Array1<f64>)
    {
        
        let z_value = self.calculate_z_score(hypothesized_mean, population_std, sample).unwrap();
        
        let mut result: bool;
        let mut style_attribute: Attr;
        
        match self.z_test_kind
        {
            ZTestKind::OneTailedUpper    =>  {result = (z_value <= self.z_critical);
                style_attribute = if result == true {Attr::ForegroundColor(color::GREEN)} else {Attr::ForegroundColor(color::RED)};
            },
            ZTestKind::OneTailedLower       =>  {result = (z_value >= self.z_critical);
                style_attribute = if result == true {Attr::ForegroundColor(color::GREEN)} else {Attr::ForegroundColor(color::RED)};
            },
            ZTestKind::TwoTailed                 =>  {result = (z_value >= -self.z_critical && z_value <= self.z_critical);
                style_attribute = if result == true {Attr::ForegroundColor(color::GREEN)} else {Attr::ForegroundColor(color::RED)};
            },
        }

        let comment = if result ==  true {"Failed to reject null hypothesis"} else {"Reject null hypothesis"};
        let result = format!("{}", &z_value);

        let mut result_table = Table::new();
        result_table.add_row(row!["Z-Critical", "Sample Mean",  "Hypothesized Mean", "Z-Value", "Comment"]);

        result_table.add_row(Row::new
                                    (vec![
                                        Cell::new(&self.z_critical.to_string()),
                                        Cell::new(&get_mean(sample).unwrap().to_string()),
                                        Cell::new(&hypothesized_mean.to_string()),
                                        Cell::new(&result.to_string()).with_style(style_attribute),
                                        Cell::new(comment)
                                        ]
                                    )
                            );
        result_table.printstd();
    }
}

#[allow(unused)]
impl ZTest
{
    pub fn calculate_z_score(&self, hypothesized_mean: f64, population_std: f64, sample: &ndarray::Array1<f64>) -> Result<f64, String>
    {
        Ok
        (
            (get_mean(sample)? - hypothesized_mean)
            /
            (population_std / (sample.len() as f64).sqrt())
        )
    }
}


#[allow(unused)]
impl ZTest
{
    pub fn new(z_test_kind: ZTestKind, alpha_level: f64) -> Result<Self, String>
    {
        let z_critical;
        match &z_test_kind
        {
            ZTestKind::OneTailedUpper 
            => match alpha_level
                {
                    x if x == 0.10      => {z_critical = 1.282;}
                    x if x == 0.05      => {z_critical = 1.645;}
                    x if x == 0.025     => {z_critical = 1.960;}
                    x if x == 0.010     => {z_critical = 2.326;}
                    x if x == 0.005     => {z_critical = 2.576;}
                    x if x == 0.001     => {z_critical = 3.090;}
                    x if x == 0.0001    => {z_critical = 3.719;}
                    _ => panic!("No suitable alpha level.")

                } // End match
            ZTestKind::OneTailedLower    
            => match alpha_level
            {
                x if x == 0.10      => {z_critical = -1.282;}
                x if x == 0.05      => {z_critical = -1.645;}
                x if x == 0.025     => {z_critical = -1.960;}
                x if x == 0.010     => {z_critical = -2.326;}
                x if x == 0.005     => {z_critical = -2.576;}
                x if x == 0.001     => {z_critical = -3.090;}
                x if x == 0.0001    => {z_critical = -3.719;}
                _ => panic!("No suitable alpha level.")

            } // End match
            ZTestKind::TwoTailed              
            => match alpha_level
            {
                x if x == 0.20      => {z_critical = 1.282;}
                x if x == 0.10      => {z_critical = 1.645;}
                x if x == 0.05      => {z_critical = 1.960;}
                x if x == 0.010     => {z_critical = 2.576;}
                x if x == 0.001     => {z_critical = 3.291;}
                x if x == 0.0001    => {z_critical = 3.819;}
                _ => panic!("No suitable alpha level.")

            } // End match
        }

        Ok
        (
            ZTest
            {
                z_test_kind: z_test_kind,
                alpha_level: alpha_level,
                z_critical: z_critical
            }
        )
    }
}

#[allow(unused)]
pub fn get_f_statistic(sample_1: &Array1<f64>, sample_2: &Array1<f64>) -> Result<f64, String>
{
    let sample_1_length = sample_1.shape()[0] - 1;
    let sample_2_length = sample_2.shape()[0] - 1;

    if sample_1_length >= sample_2_length
    {
        Ok(get_variance(sample_1)? / get_variance(sample_2)?)
    }
    else
    {
        Ok(get_variance(sample_2)? / get_variance(sample_1)?)
    }
}