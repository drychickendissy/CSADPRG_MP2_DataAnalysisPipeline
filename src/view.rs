use crate::model::{Project, median, round2};
use csv::WriterBuilder;
use serde::Serialize;
use serde_json::json;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct Report1Row
{
    region: String,
    total_budget: f64,
    median_savings: f64,
    avg_delay: f64,
    efficiency_score: f64,
}

pub fn report_regional_efficiency(projects: &[Project]) -> Result<(), Box<dyn Error>>
{
    let mut by_region: HashMap<String, Vec<&Project>> = HashMap::new();
    for p in projects.iter()
    {
        let key = p.region.clone().unwrap_or_else(|| "Unknown".to_string());
        by_region.entry(key).or_insert_with(Vec::new).push(p);
    }

    let mut rows: Vec<Report1Row> = Vec::new();
    for (region, group) in by_region
    {
        let total_budget: f64 = group.iter().filter_map(|p| p.approved_budget_for_contract).sum();
        let mut savings: Vec<f64> = group.iter().filter_map(|p| p.cost_savings).collect();
        let median_savings = median(&mut savings);
        let delays: Vec<i64> = group.iter().filter_map(|p| p.completion_delay_days).collect();
        let avg_delay = if delays.is_empty() { 0.0 } else { (delays.iter().sum::<i64>() as f64) / (delays.len() as f64) };
        let efficiency_score = if avg_delay != 0.0 { (median_savings / avg_delay) * 100.0 } else { 0.0 };

        rows.push(Report1Row {
            region,
            total_budget: round2(total_budget),
            median_savings: round2(median_savings),
            avg_delay: round2(avg_delay),
            efficiency_score: round2(efficiency_score),
        });
    }

    rows.sort_by(|a, b| b.efficiency_score.partial_cmp(&a.efficiency_score).unwrap_or(Ordering::Equal));

    let mut wtr = WriterBuilder::new().from_path("report1_regional_efficiency.csv")?;
    wtr.write_record(&["Region", "TotalBudget", "MedianSavings", "AvgDelay", "EfficiencyScore"])?;
    for r in rows
    {
        wtr.write_record(&[
            &r.region,
            &format!("{:.2}", r.total_budget),
            &format!("{:.2}", r.median_savings),
            &format!("{:.2}", r.avg_delay),
            &format!("{:.2}", r.efficiency_score),
        ])?;
    }
    wtr.flush()?;
    Ok(())
}

// ---------- Report 2 ----------
pub fn report_top_contractors(projects: &[Project]) -> Result<(), Box<dyn Error>>
{
    let mut by_contractor: HashMap<String, Vec<&Project>> = HashMap::new();
    for p in projects.iter()
    {
        let key = p.contractor.clone().unwrap_or_else(|| "Unknown".to_string());
        by_contractor.entry(key).or_insert_with(Vec::new).push(p);
    }

    #[derive(Serialize)]
    struct Row
    {
        contractor: String,
        num_projects: usize,
        reliability_index: f64,
        risk_flag: String,
    }

    let mut rows: Vec<Row> = Vec::new();
    for (contractor, group) in by_contractor
    {
        if group.len() < 5 { continue; }
        let avg_delay = group.iter().filter_map(|p| p.completion_delay_days).map(|d| d as f64).sum::<f64>() / (group.len() as f64);
        let total_savings: f64 = group.iter().filter_map(|p| p.cost_savings).sum();
        let total_cost: f64 = group.iter().filter_map(|p| p.contract_cost).sum();
        let mut reliability = (1.0 - (avg_delay / 90.0)) * (total_savings / total_cost) * 100.0;
        if reliability > 100.0 { reliability = 100.0; }
        if reliability < 0.0 { reliability = 0.0; }
        let risk_flag = if reliability < 50.0 { "High Risk" } else { "OK" };

        rows.push(Row {
            contractor,
            num_projects: group.len(),
            reliability_index: round2(reliability),
            risk_flag: risk_flag.to_string(),
        });
    }

    rows.sort_by(|a, b| b.reliability_index.partial_cmp(&a.reliability_index).unwrap_or(Ordering::Equal));
    rows.truncate(15);

    let mut wtr = WriterBuilder::new().from_path("report2_top_contractors.csv")?;
    wtr.write_record(&["Contractor", "NumProjects", "ReliabilityIndex", "RiskFlag"])?;
    for r in rows
    {
        wtr.write_record(&[
            &r.contractor,
            &r.num_projects.to_string(),
            &format!("{:.2}", r.reliability_index),
            &r.risk_flag,
        ])?;
    }
    wtr.flush()?;
    Ok(())
}

// ---------- Report 3 ----------
pub fn report_annual_trends(projects: &[Project]) -> Result<(), Box<dyn Error>>
{
    let mut by_year: HashMap<i32, Vec<&Project>> = HashMap::new();
    for p in projects.iter()
    {
        if let Some(y) = p.funding_year
        {
            by_year.entry(y).or_insert_with(Vec::new).push(p);
        }
    }

    #[derive(Serialize)]
    struct Row
    {
        year: i32,
        avg_savings: f64,
    }

    let mut rows: Vec<Row> = Vec::new();
    for (year, group) in by_year
    {
        let avg_savings: f64 = group.iter().filter_map(|p| p.cost_savings).sum::<f64>() / (group.len() as f64);
        rows.push(Row { year, avg_savings: round2(avg_savings) });
    }

    rows.sort_by_key(|r| r.year);

    let mut wtr = WriterBuilder::new().from_path("report3_annual_trends.csv")?;
    wtr.write_record(&["Year", "AvgSavings"])?;
    for r in rows
    {
        wtr.write_record(&[&r.year.to_string(), &format!("{:.2}", r.avg_savings)])?;
    }
    wtr.flush()?;
    Ok(())
}

// ---------- Summary JSON ----------
pub fn summary_json(projects: &[Project]) -> Result<(), Box<dyn Error>>
{
    let total_projects = projects.len();
    let contractors: std::collections::HashSet<_> = projects.iter().filter_map(|p| p.contractor.clone()).collect();
    let total_contractors = contractors.len();
    let avg_delay = projects.iter().filter_map(|p| p.completion_delay_days).map(|d| d as f64).sum::<f64>() / (projects.len() as f64);
    let total_savings: f64 = projects.iter().filter_map(|p| p.cost_savings).sum();

    let summary = json!({
        "total_projects": total_projects,
        "total_contractors": total_contractors,
        "global_avg_delay": round2(avg_delay),
        "total_savings": round2(total_savings)
    });

    let mut f = File::create("summary.json")?;
    f.write_all(serde_json::to_string_pretty(&summary)?.as_bytes())?;
    f.flush()?;
    Ok(())
}