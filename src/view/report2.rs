/********************
Last names: Abdulrahman, Bilanes, Cruz, Nicolas
Language: JavaScript
Paradigm(s): Procedural, Object-Oriented, Functional, Data-Driven, Immutable
********************/

use crate::model::{Project, truncate, round2};   // imports Project struct and utility functions from model
use csv::WriterBuilder; // enables CSV writing
use serde::Serialize;   // enables serialization for CSVs
use std::cmp::Ordering; // enables Ordering for sorting
use std::collections::HashMap;  // enables HashMap usage
use std::error::Error;  // allows Result<(), Box<dyn Error>> (error handling)
use num_format::{ToFormattedString};    // for formatting numbers with commas

#[derive(Serialize)]
struct Row
{
    contractor: String,
    num_projects: usize,
    reliability_index: f64,
    risk_flag: String,
    total_cost: f64,
    avg_delay: f64,
    total_savings: f64,
}

pub fn report_top_contractors(projects: &[Project]) -> Result<(), Box<dyn Error>>
{
    println!("\nReport 2: Top Contractors Performance Ranking\n");
    println!("Top Contractors Performance Ranking (Top 15 by TotalCost, >=5 Projects)\n");

    // Group by Contractor
    let mut by_contractor: HashMap<String, Vec<&Project>> = HashMap::new();
    for p in projects.iter()
    {
        let key = p.contractor.clone().unwrap_or_else(|| "Unknown".to_string());    // get contractor name or "Unknown" if missing as a key
        by_contractor.entry(key).or_insert_with(Vec::new).push(p);  // adds project to group (contractor)
    }

    let mut rows: Vec<Row> = Vec::new(); // stores rows for report. Each row will hold metrics for one contractor (only contractors with >=5 projects)

    for (contractor, group) in by_contractor
    {
        if group.len() < 5 { continue; }    // skip contractors with less than 5 projects

        let avg_delay = group
            .iter()
            .filter_map(|p| p.completion_delay_days)    // gets completion_delay_days if exists
            .map(|d| d as f64)  // converts i64 to f64
            .sum::<f64>()
            / (group.len() as f64);
            // BASICALLY: avg_delay = total_delay_days / num_projects

        let total_savings: f64 = group.iter().filter_map(|p| p.cost_savings).sum(); // sums up cost_savings for all projects in group
        let total_cost: f64 = group.iter().filter_map(|p| p.contract_cost).sum();   // sums up contract_cost for all projects in group

        let mut reliability = (1.0 - (avg_delay / 90.0)) * (total_savings / total_cost) * 100.0;
        if reliability > 100.0 { 
            reliability = 100.0; 
        } 
        if reliability < 0.0 { 
            reliability = 0.0; 
        }
        // reliability index = (1 - (avg delay / 90)) * (total savings / total cost) * 100 (capped at 100)

        let risk_flag = if reliability < 50.0 { 
            "High Risk" 
        } else { 
            "OK" 
        };

        rows.push(Row
        {
            contractor,
            num_projects: group.len(),
            reliability_index: round2(reliability),
            risk_flag: risk_flag.to_string(),
            total_cost: round2(total_cost),
            avg_delay: round2(avg_delay),
            total_savings: round2(total_savings),
        });
    }

    // sort & truncate
    // Sort descending by total_cost
    rows.sort_by(|a, b| b.total_cost.partial_cmp(&a.total_cost).unwrap_or(Ordering::Equal));
    rows.truncate(15);  // keep only top 15 contractors by total_cost

    // ---------- Print Table ----------
    println!(
        "| {:<4} | {:<45} | {:>16} | {:>12} | {:>8} | {:>14} | {:>16} | {:<9} |",
        "Rank", "Contractor", "TotalCost", "NumProjects", "AvgDelay", "TotalSavings", "ReliabilityIndex", "RiskFlag"
    );
    println!(
        "|{:-<6}|{:-<47}|{:-<18}|{:-<14}|{:-<10}|{:-<16}|{:-<18}|{:-<11}|",
        "", "", "", "", "", "", "", ""
    );

    for (i, r) in rows.iter().enumerate()
    {
        let formatted_total_cost = format!(
            "{}.{:02}",
            (r.total_cost as u64).to_formatted_string(&num_format::Locale::en),
            (r.total_cost.fract() * 100.0).round() as u64
        );

        let formatted_total_savings = format!(
            "{}.{:02}",
            (r.total_savings as u64).to_formatted_string(&num_format::Locale::en),
            (r.total_savings.fract() * 100.0).round() as u64
        );
        println!(
            "| {:<4} | {:<45} | {:>14} | {:>12} | {:>8.1} | {:>14} | {:>16.2} | {:<9} |",
            i + 1,
            truncate(&r.contractor, 45),
            formatted_total_cost,
            r.num_projects, 
            r.avg_delay,
            formatted_total_savings,
            r.reliability_index,
            r.risk_flag
        );
    }

    // ----- Save CSV -----
    let mut wtr = WriterBuilder::new().from_path("report2_top_contractors.csv")?;
    wtr.write_record(&[
        "Contractor",
        "NumProjects",
        "TotalCost",
        "AvgDelay",
        "TotalSavings",
        "ReliabilityIndex",
        "RiskFlag",
    ])?;
    
    println!("(Full table exported to report2_top_contractors.csv)\n");

    for r in rows
    {
        wtr.write_record(&[
            &r.contractor,
            &r.num_projects.to_string(),
            &format!("{:.2}", r.total_cost),
            &format!("{:.2}", r.avg_delay),
            &format!("{:.2}", r.total_savings),
            &format!("{:.2}", r.reliability_index),
            &r.risk_flag,
        ])?;
    }
    wtr.flush()?;
    Ok(())
}
