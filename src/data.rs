use chrono::NaiveDate;

#[derive(serde::Deserialize)]
pub struct DataEntry {
    /// The new-moon dates in this year
    new_moons: Vec<NaiveDate>,
    /// On which new-moon the new year is
    new_year: u8,
    /// The Chinese date of 1st January of this year
    initial_date: u8,
    /// The dates of solar terms, starting with xiaohan
    solar_terms: [NaiveDate; 24],
}

impl DataEntry {
    
}
