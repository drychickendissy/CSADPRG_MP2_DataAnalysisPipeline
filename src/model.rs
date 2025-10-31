use chrono::NaiveDate; // imports NaiveDate (represents calendar date with no timezone)

#[derive(Debug, Clone)]
pub struct Project
{
    // NOTE: All fields are optional to handle missing data
    pub project_id: Option<String>,
    pub funding_year: Option<i32>,
    pub region: Option<String>,
    pub main_island: Option<String>,
    pub province: Option<String>,
    pub contractor: Option<String>,
    pub type_of_work: Option<String>,
    pub approved_budget_for_contract: Option<f64>,
    pub contract_cost: Option<f64>,
    pub start_date: Option<NaiveDate>,
    pub actual_completion_date: Option<NaiveDate>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub cost_savings: Option<f64>,
    pub completion_delay_days: Option<i64>,
}

impl Project
{
    // Constructor to create new empty Project
    pub fn new() -> Self
    {
        Self
        {
            project_id: None,
            funding_year: None,
            region: None,
            main_island: None,
            province: None,
            contractor: None,
            type_of_work: None,
            approved_budget_for_contract: None,
            contract_cost: None,
            start_date: None,
            actual_completion_date: None,
            lat: None,
            lon: None,
            cost_savings: None,
            completion_delay_days: None,
        }
    }
}

// ---------- Parsing Helpers ----------

/* Removes commas in numbers and converts number into float */
pub fn parse_float(s: &str) -> Option<f64>
{
    let trimmed = s.trim().replace(",", "");
    if trimmed.is_empty() 
    { 
        return None; 
    }
    trimmed.parse::<f64>().ok() // .ok() converts Result to Option, returning None on error
}

/* Converts text into int */
pub fn parse_int(s: &str) -> Option<i32>
{
    let trimmed = s.trim();
    if trimmed.is_empty() 
    { 
        return None; 
    }
    trimmed.parse::<i32>().ok()
}

/* Converts text of the format "%Y-%m-%d", "%d/%m/%Y", "%m/%d/%Y", "%Y/%m/%d" into NaiveDate */
pub fn try_parse_date(s: &str) -> Option<NaiveDate>
{
    let s_trim = s.trim();
    if s_trim.is_empty() 
    { 
        return None; 
    }

    if let Ok(d) = NaiveDate::parse_from_str(s_trim, "%d/%m/%Y")
    {
        return Some(d);
    }
    None
}

// ---------- Utility math ----------
/* Gets median */
pub fn median(values: &mut [f64]) -> f64
{
    if values.is_empty() 
    { 
        return 0.0; 
    }
    values.sort_by(|a, b| a.partial_cmp(b).unwrap()); // Sorts list of numbers
    let n = values.len();
    
    // actual median computation
    if n % 2 == 1
    {
        // values (at the last expression) without semicolon are automatially returned
        values[n / 2]
    }
    else
    {
        // values (at the last expression) without semicolon are automatially returned
        (values[n / 2 - 1] + values[n / 2]) / 2.0
    }
}

/* Rounds number to 2 decimal places */
pub fn round2(v: f64) -> f64
{
    // * 100 to shift decimal point two places to the right
    // .round() to round to nearest integer
    // / 100 to shift decimal point back to original position
    (v * 100.0).round() / 100.0
}