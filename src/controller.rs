use crate::model::{Project, parse_float, parse_int, try_parse_date}; // imports Project and functions from model
use crate::view::{report1, report2, report3, summary};  // imports reports and summary from view
use chrono::Datelike; // imports Datelike trait for date operations (especially .year())
use csv::ReaderBuilder; // enables CSV reading
use std::collections::HashMap;
use std::error::Error;  // allows Result<(), Box<dyn Error>> (error handling)
use std::io::Write;
use std::sync::OnceLock; // allows storing of projects globally once

// ----- NOTE: GLOBAL VARIABLES -----
const MIN_YEAR: i32 = 2021;
const MAX_YEAR: i32 = 2023;

static PROJECTS: OnceLock<Vec<Project>> = OnceLock::new();
// ----- NOTE: GLOBAL VARIABLES -----

// ----- Load File -----
pub fn load_file() -> Result<(), Box<dyn Error>> {
    print!("Processing dataset...");
    std::io::stdout().flush()?; // ensure prompt prints immediately

    let mut rdr = ReaderBuilder::new()  // reads content of dpwh_flood_control_projects.csv
        .flexible(true) // allows rows with different number of fields/columns
        .from_path("dpwh_flood_control_projects.csv")?;

    let headers = rdr.headers()?.clone();   // reads and copies header row (first row)
    let header_map: HashMap<String, usize> =
        headers.iter().enumerate().map(|(i, h)| (h.to_lowercase(), i)).collect();   // iterates through headers, lowercases them, and maps to their index
                                                                                    // .collect creates a HashMap from the iterator (Hashmaop<String, usize>)

    let mut projects: Vec<Project> = Vec::new();    // initializes empty vector to store projects

    // ----- Read Each Row -----
    for result in rdr.records() /* iterates over each row*/ {
        let record = result?;
        let mut p = Project::new();

        let get = |name: &str| -> String {
            header_map
                .get(&name.to_lowercase())  // looks up lowercased column name in header_map
                .and_then(|&i| record.get(i))   // gets value at that index from current row
                .unwrap_or("")  // returns empty string if column name not found or index out of bounds
                .trim()
                .to_string() // converts from &str to String
        };

        // Assignment of values
        p.project_id = Some(get("ProjectId"));  // p.project_id is assigned a value under the column "ProjectId" from the current row
                                                // Some() wraps the String in an Option (valu may be empty or may not) 
        p.funding_year = parse_int(&get("FundingYear"));    // &get borrows the String to avoid moving it (used for parse functions to not give them ownership and only provide reference)
        p.region = Some(get("Region"));
        p.main_island = Some(get("MainIsland"));
        p.province = Some(get("Province"));
        p.contractor = Some(get("Contractor"));
        p.type_of_work = Some(get("TypeOfWork"));

        p.approved_budget_for_contract = parse_float(&get("ApprovedBudgetForContract"));
        p.contract_cost = parse_float(&get("ContractCost"));

        p.start_date = try_parse_date(&get("StartDate"));
        p.actual_completion_date = try_parse_date(&get("ActualCompletionDate"));

        // Compute cost savings
        if let (Some(a), Some(c)) =
            (p.approved_budget_for_contract, p.contract_cost)
        {
            p.cost_savings = Some(a - c);
        }

        // Compute completion delay days
        if let (Some(s), Some(e)) = (p.start_date, p.actual_completion_date) {
            p.completion_delay_days = Some((e - s).num_days()); // .num_dats() gets difference in days between two dates in days. Returns i64
        }   

        // Debug printing
        if p.completion_delay_days.is_none() {
            println!(
                "No delay data for: {}",
                p.project_id.as_deref().unwrap_or("Unknown ID") // .as_deref() converts Option<String> to Option<&str> and returns value or "Unknown ID" if None
            );
        }

        projects.push(p); // adds project to projects vector
    }

    let total = projects.len();

    // ----- Filter Projects by Year -----
    projects.retain(|p| {
        if let Some(date) = p.start_date {
            let year = date.year();
            (MIN_YEAR..=MAX_YEAR).contains(&year)
        } else {
            false
        }
    }); // retains only projects whose start_date year is between MIN_YEAR and MAX_YEAR

    let filtered = projects.len();
    println!(" ({total} rows loaded, {filtered} filtered for {MIN_YEAR}–{MAX_YEAR})");

    if PROJECTS.set(projects).is_err() {
        println!("File already loaded. Use option 2 to generate reports.");
    }

    Ok(())
}

// ----- Generate Reports -----
pub fn generate_reports() -> Result<(), Box<dyn Error>> {
    if let Some(projects) = PROJECTS.get() {
        println!("\nGenerating reports...");

        report1::report_regional_efficiency(projects)?;
        report2::report_top_contractors(projects)?;
        report3::report_annual_trends(projects)?;
        summary::summary_json(projects)?;

        println!("\nAll reports generated.");
    } else {
        println!("\nNo data loaded yet. Please choose [1] first.");
    }

    Ok(())
}
