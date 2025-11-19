use crate::model::{Project, parse_float, parse_int, try_parse_date}; // imports Project and functions from model
use crate::view::{report1, report2, report3, summary};  // imports reports and summary from view
use chrono::Datelike; // imports Datelike trait for date operations (especially .year())
use csv::ReaderBuilder; // enables CSV reading
use std::collections::HashMap;
use std::error::Error;  // allows Result<(), Box<dyn Error>> (error handling)
use std::io::Write;
use regex::Regex;   // for regex parsing


// ----- Load File -----
pub fn load_file() -> Result<Vec<Project>, Box<dyn Error>> {
    
    // Precompile regexes for cluster/MYCA references
    let cluster_re = Regex::new(r"Clustered with Contract ID\s+([\w\-.]+)").unwrap();   // regex to match "Clustered with Contract ID <ID>"
    let myca_re = Regex::new(r"MYCA with Project ID\s+([\w\-.]+)").unwrap();    // regex to match "MYCA with Project ID <ID>"
    print!("Processing dataset...");
    std::io::stdout().flush()?; // ensure prompt prints immediately

    let mut rdr = ReaderBuilder::new()  // reads content of dpwh_flood_control_projects.csv
        .flexible(true) // allows rows with different number of fields/columns
        .from_path("dpwh_flood_control_projects.csv")?;

    let headers = rdr.headers()?.clone();   // reads and copies header row (first row)
    let header_map: HashMap<String, usize> =
        headers.iter().enumerate().map(|(i, h)| (h.to_lowercase(), i)).collect();   // iterates through headers, lowercases them, and maps to their index

    let mut projects: Vec<Project> = Vec::new();
    
    // 1. Collect contract and project lookups for cluster/MYCA
    let mut contract_budget: HashMap<String, f64> = HashMap::new();
    let mut contract_cost: HashMap<String, f64> = HashMap::new();
    let mut project_budget: HashMap<String, f64> = HashMap::new();
    let mut project_cost: HashMap<String, f64> = HashMap::new();
    let mut raw_records: Vec<csv::StringRecord> = Vec::new();
    
    for result in rdr.records() {
        let record = result?;   // read each record
        raw_records.push(record.clone());   // store raw record for second pass (2. Resolve cluster/MYCA references and build Project structs)
        
        let get = |name: &str| -> String {
            header_map.get(&name.to_lowercase()).and_then(|&i| record.get(i)).unwrap_or("").trim().to_string()
        };

        let contract_id = get("ContractId");
        let project_id = get("ProjectId");

        if !contract_id.is_empty() {
            if let Some(b) = parse_float(&get("ApprovedBudgetForContract")) {
                contract_budget.insert(contract_id.clone(), b);
            }
            if let Some(c) = parse_float(&get("ContractCost")) {
                contract_cost.insert(contract_id.clone(), c);
            }
        }

        if !project_id.is_empty() {
            if let Some(b) = parse_float(&get("ApprovedBudgetForContract")) {
                project_budget.insert(project_id.clone(), b);
            }
            if let Some(c) = parse_float(&get("ContractCost")) {
                project_cost.insert(project_id.clone(), c);
            }
        }
    }

    // 2. Resolve cluster/MYCA references and build Project structs
    for record in raw_records {
        let get = |name: &str| -> String {
            header_map.get(&name.to_lowercase()).and_then(|&i| record.get(i)).unwrap_or("").trim().to_string()
        };
        
        let mut p = Project::new();

        p.project_id = Some(get("ProjectId"));
        p.funding_year = parse_int(&get("FundingYear"));
        p.region = Some(get("Region"));
        p.main_island = Some(get("MainIsland"));
        p.province = Some(get("Province"));
        p.contractor = Some(get("Contractor"));
        p.type_of_work = Some(get("TypeOfWork"));
        p.contract_id = Some(get("ContractId"));

        // ----- Cluster/MYCA resolution for budget -----
        let raw_budget = get("ApprovedBudgetForContract");
        let mut budget = parse_float(&raw_budget);
        
        if budget.is_none() {
            if let Some(cap) = cluster_re.captures(&raw_budget) {   // check for cluster reference
                let ref_id = cap.get(1).unwrap().as_str();  // extract referenced contract ID
                budget = contract_budget.get(ref_id).copied();  // lookup budget from contract_budget map
            } 
            else if let Some(cap) = myca_re.captures(&raw_budget) {   // same logic for MYCA reference
                let ref_id = cap.get(1).unwrap().as_str();
                budget = project_budget.get(ref_id).copied();
            }
        }
        p.approved_budget_for_contract = budget;

        // ----- Cluster/MYCA resolution for cost -----
        let raw_cost = get("ContractCost");
        let mut cost = parse_float(&raw_cost);
        if cost.is_none() {
            if let Some(cap) = cluster_re.captures(&raw_cost) { // check for cluster reference
                let ref_id = cap.get(1).unwrap().as_str();  //  extract referenced contract ID
                cost = contract_cost.get(ref_id).copied();  // lookup cost from contract_cost map
            } 
            else if let Some(cap) = myca_re.captures(&raw_cost) { // same logic for MYCA reference
                let ref_id = cap.get(1).unwrap().as_str();
                cost = project_cost.get(ref_id).copied();
            }
        }
        p.contract_cost = cost;

        // ----- Parse latitude & longitude -----
        p.lat = parse_float(&get("ProjectLatitude"));
        p.lon = parse_float(&get("ProjectLongitude"));

        p.start_date = try_parse_date(&get("StartDate"));
        p.actual_completion_date = try_parse_date(&get("ActualCompletionDate"));

        // Compute cost savings
        if let (Some(a), Some(c)) = (p.approved_budget_for_contract, p.contract_cost) {
            p.cost_savings = Some(a - c);
        }

        // Compute completion delay days
        if let (Some(s), Some(e)) = (p.start_date, p.actual_completion_date) {
            p.completion_delay_days = Some((e - s).num_days());
        }

        projects.push(p);
    }

    let total = projects.len();
    // Filter projects by year 2021-2023
    let filtered_projects: Vec<Project> = projects.into_iter().filter(|p| {
        if let Some(date) = p.start_date {
            let year = date.year();
            (2021..=2023).contains(&year)
        } else {
            false
        }
    }).collect();
    let filtered = filtered_projects.len();
    println!(" ({total} rows loaded, {filtered} filtered for 2021–2023)");
    Ok(filtered_projects)
}

// ----- Generate Reports -----
pub fn generate_reports(projects: &[Project]) -> Result<(), Box<dyn Error>> {
    println!("\nGenerating reports...");
    report1::report_regional_efficiency(projects)?;
    report2::report_top_contractors(projects)?;
    report3::report_annual_trends(projects)?;
    summary::summary_json(projects)?;
    println!("\nAll reports generated.");
    Ok(())
}
