use crate::model::round2;
use crate::model::Project;
use csv::WriterBuilder;
use serde::Serialize;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;

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

    let mut by_contractor: HashMap<String, Vec<&Project>> = HashMap::new();
    for p in projects.iter()
    {
        let key = p.contractor.clone().unwrap_or_else(|| "Unknown".to_string());
        by_contractor.entry(key).or_insert_with(Vec::new).push(p);
    }

    let mut rows: Vec<Row> = Vec::new();

    for (contractor, group) in by_contractor
    {
        if group.len() < 5 { continue; }

        let avg_delay = group
            .iter()
            .filter_map(|p| p.completion_delay_days)
            .map(|d| d as f64)
            .sum::<f64>()
            / (group.len() as f64);

        let total_savings: f64 = group.iter().filter_map(|p| p.cost_savings).sum();
        let total_cost: f64 = group.iter().filter_map(|p| p.contract_cost).sum();

        let mut reliability = (1.0 - (avg_delay / 90.0)) * (total_savings / total_cost) * 100.0;
        if reliability > 100.0 { reliability = 100.0; }
        if reliability < 0.0 { reliability = 0.0; }

        let risk_flag = if reliability < 50.0 { "High Risk" } else { "OK" };

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
    rows.sort_by(|a, b| b.total_cost.partial_cmp(&a.total_cost).unwrap_or(Ordering::Equal));
    rows.truncate(15);

    // ---------- PRINT TABLE ----------
    println!(
        "| {:<4} | {:<55} | {:>14} | {:>12} | {:>8} | {:>14} | {:>16} | {:<9} |",
        "Rank", "Contractor", "TotalCost", "NumProjects", "AvgDelay", "TotalSavings", "ReliabilityIndex", "RiskFlag"
    );
    println!(
        "|{:-<6}|{:-<57}|{:-<16}|{:-<14}|{:-<10}|{:-<16}|{:-<18}|{:-<11}|",
        "", "", "", "", "", "", "", ""
    );

    for (i, r) in rows.iter().enumerate()
    {
        println!(
            "| {:<4} | {:<55} | {:>14.0} | {:>12} | {:>8.1} | {:>14.0} | {:>16.2} | {:<9} |",
            i + 1,
            truncate(&r.contractor, 55),
            r.total_cost,
            r.num_projects,
            r.avg_delay,
            r.total_savings,
            r.reliability_index,
            r.risk_flag
        );
    }

    println!("\n(Full table exported to report2_top_contractors.csv)\n");

    // ---------- WRITE CSV ----------
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

// helper to keep super-long contractor names aligned
fn truncate(s: &str, max_len: usize) -> String
{
    if s.len() > max_len
    {
        format!("{}…", &s[..max_len - 1])
    }
    else
    {
        s.to_string()
    }
}
