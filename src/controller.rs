use crate::model::{Project, parse_float, parse_int, try_parse_date};
use crate::view;
use chrono::NaiveDate;
use csv::ReaderBuilder;
use std::collections::HashMap;
use std::error::Error;

const MIN_YEAR: i32 = 2021;
const MAX_YEAR: i32 = 2023;

pub fn run_pipeline() -> Result<(), Box<dyn Error>>
{
    println!("Reading CSV...");
    let mut rdr = ReaderBuilder::new().flexible(true).from_path("dpwh_flood_control_projects.csv")?;
    let headers = rdr.headers()?.clone();
    let header_map: HashMap<String, usize> = headers.iter().enumerate().map(|(i, h)| (h.to_lowercase(), i)).collect();

    let mut projects: Vec<Project> = Vec::new();
    for result in rdr.records()
    {
        let record = result?;
        let mut p = Project::new();

        let get = |name: &str| -> String
        {
            header_map.get(&name.to_lowercase()).and_then(|&i| record.get(i)).unwrap_or("").trim().to_string()
        };

        p.funding_year = parse_int(&get("FundingYear"));
        p.region = Some(get("Region"));
        p.main_island = Some(get("MainIsland"));
        p.province = Some(get("Province"));
        p.contractor = Some(get("Contractor"));
        p.type_of_work = Some(get("TypeOfWork"));
        p.approved_budget_for_contract = parse_float(&get("ApprovedBudgetForContract"));
        p.contract_cost = parse_float(&get("ContractCost"));
        p.start_date = try_parse_date(&get("StartDate"));
        p.actual_completion_date = try_parse_date(&get("ActualCompletionDate"));

        // Derived fields
        if let (Some(a), Some(c)) = (p.approved_budget_for_contract, p.contract_cost)
        {
            p.cost_savings = Some(a - c);
        }
        if let (Some(s), Some(e)) = (p.start_date, p.actual_completion_date)
        {
            p.completion_delay_days = Some((e - s).num_days());
        }

        projects.push(p);
    }

    println!("Total records loaded: {}", projects.len());

    // Filter 2021–2023
    projects.retain(|p| p.funding_year.map(|y| y >= MIN_YEAR && y <= MAX_YEAR).unwrap_or(false));
    println!("After filtering ({}–{}): {}", MIN_YEAR, MAX_YEAR, projects.len());

    // Call view functions
    view::report_regional_efficiency(&projects)?;
    view::report_top_contractors(&projects)?;
    view::report_annual_trends(&projects)?;
    view::summary_json(&projects)?;

    println!("✅ All reports generated.");
    Ok(())
}