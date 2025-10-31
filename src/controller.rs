use crate::model::{Project, parse_float, parse_int, try_parse_date};
use crate::view::{report1, report2, report3, summary};
use csv::ReaderBuilder;
use std::collections::HashMap;
use std::error::Error;
use std::sync::OnceLock;
use std::io::Write;

const MIN_YEAR: i32 = 2021;
const MAX_YEAR: i32 = 2023;

static PROJECTS: OnceLock<Vec<Project>> = OnceLock::new();

pub fn load_file() -> Result<(), Box<dyn Error>>
{
    print!("Processing dataset...");
    std::io::stdout().flush()?; // print inline without newline

    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .from_path("dpwh_flood_control_projects.csv")?;
    let headers = rdr.headers()?.clone();
    let header_map: HashMap<String, usize> =
        headers.iter().enumerate().map(|(i, h)| (h.to_lowercase(), i)).collect();

    let mut projects: Vec<Project> = Vec::new();
    for result in rdr.records()
    {
        let record = result?;
        let mut p = Project::new();

        let get = |name: &str| -> String {
            header_map
                .get(&name.to_lowercase())
                .and_then(|&i| record.get(i))
                .unwrap_or("")
                .trim()
                .to_string()
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

        if let (Some(a), Some(c)) = (p.approved_budget_for_contract, p.contract_cost)
        {
            p.cost_savings = Some(a - c);
        }
        if let (Some(s), Some(e)) = (p.start_date, p.actual_completion_date)
        {
            p.completion_delay_days = Some((e - s).num_days());
        }

        if p.completion_delay_days.is_none() {
    println!("No delay data for: {:?}", p.project_id);
}


        projects.push(p);
    }

    let total = projects.len();
    projects.retain(|p| p.funding_year.map(|y| y >= MIN_YEAR && y <= MAX_YEAR).unwrap_or(false));
    let filtered = projects.len();

    println!(" ({total} rows loaded, {filtered} filtered for {MIN_YEAR}–{MAX_YEAR})");

    if PROJECTS.set(projects).is_err()
    {
        println!("File already loaded. Use option 2 to generate reports.");
    }

    Ok(())
}

pub fn generate_reports() -> Result<(), Box<dyn Error>>
{
    if let Some(projects) = PROJECTS.get()
    {
        println!("\nGenerating reports...");
        report1::report_regional_efficiency(projects)?;
        report2::report_top_contractors(projects)?;
        report3::report_annual_trends(projects)?;
        summary::summary_json(projects)?;
        println!("\nAll reports generated.");
    }
    else
    {
        println!("\nNo data loaded yet. Please choose [1] first.");
    }

    Ok(())
}
