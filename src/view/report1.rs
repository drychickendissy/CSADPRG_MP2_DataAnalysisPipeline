use crate::model::{Project, median, round2};
use csv::WriterBuilder;
use serde::Serialize;
use std::error::Error;
use std::collections::HashMap;
use num_format::{Locale, ToFormattedString};

#[derive(Serialize)]
struct Report1Row {
    region: String,
    main_island: String,
    total_budget: f64,
    median_savings: f64,
    avg_delay: f64,
    high_delay_pct: f64,
    efficiency_score: f64,
}

pub fn report_regional_efficiency(projects: &[Project]) -> Result<(), Box<dyn Error>> {
    println!("\nRegional Flood Mitigation Efficiency Summary");
    println!("(Filtered: 2021–2023 Projects)\n");

    // Filter projects by FundingYear
    let filtered: Vec<&Project> = projects.iter()
        .filter(|p| matches!(p.funding_year, Some(2021..=2023)))
        .collect();

    // Group by Region + MainIsland
    let mut by_group: HashMap<String, Vec<&Project>> = HashMap::new();
    for p in filtered {
        let region = p.region.clone().unwrap_or_else(|| "Unknown".to_string());
        let island = p.main_island.clone().unwrap_or_else(|| "Unknown".to_string());
        let key = format!("{}|{}", region, island);
        by_group.entry(key).or_insert_with(Vec::new).push(p);
    }

    let mut rows: Vec<Report1Row> = Vec::new();

    for (key, group) in by_group {
        let parts: Vec<&str> = key.split('|').collect();
        let region = parts[0].to_string();
        let main_island = parts[1].to_string();

        let total_budget: f64 = group.iter()
            .filter_map(|p| p.approved_budget_for_contract)
            .sum();

        let mut savings: Vec<f64> = group.iter()
            .filter_map(|p| p.cost_savings)
            .collect();
        let median_savings = median(&mut savings);

        let delays: Vec<f64> = group.iter()
            .filter_map(|p| p.completion_delay_days.map(|d| d as f64))
            .collect();
        let avg_delay = if delays.is_empty() { 0.0 } else { delays.iter().sum::<f64>() / delays.len() as f64 };
        let high_delay_count = delays.iter().filter(|&&d| d > 30.0).count();
        let high_delay_pct = if delays.is_empty() { 0.0 } else { (high_delay_count as f64 / delays.len() as f64) * 100.0 };
        let efficiency_score = if avg_delay != 0.0 { (median_savings / avg_delay) * 100.0 } else { 0.0 };

        rows.push(Report1Row {
            region,
            main_island,
            total_budget: round2(total_budget),
            median_savings: round2(median_savings),
            avg_delay: round2(avg_delay),
            high_delay_pct: round2(high_delay_pct),
            efficiency_score: round2(efficiency_score),
        });
    }

    // Sort by efficiency score (descending)
    rows.sort_by(|a, b| b.efficiency_score.partial_cmp(&a.efficiency_score).unwrap_or(std::cmp::Ordering::Equal));

    // Print table
    println!("| {:<35} | {:<10} | {:>13} | {:>14} | {:>9} | {:>13} | {:>17} |",
        "Region", "MainIsland", "TotalBudget", "MedianSavings", "AvgDelay", "HighDelayPct", "EfficiencyScore");
    println!("|{:-<37}|{:-<12}|{:-<15}|{:-<16}|{:-<11}|{:-<15}|{:-<19}|",
        "", "", "", "", "", "", "");

    for r in &rows {
        let formatted_budget = (r.total_budget.round() as u64).to_formatted_string(&Locale::en);
        println!(
            "| {:<35} | {:<10} | {:>13} | {:>14.2} | {:>9.1} | {:>13.2} | {:>17.2} |",
            r.region,
            r.main_island,
            formatted_budget,
            r.median_savings,
            r.avg_delay,
            r.high_delay_pct,
            r.efficiency_score
        );
    }

    println!("(Full table exported to report1_regional_efficiency.csv)\n");

    // Save CSV
    let mut wtr = WriterBuilder::new().from_path("report1_regional_efficiency.csv")?;
    wtr.write_record(&[
        "Region",
        "MainIsland",
        "TotalBudget",
        "MedianSavings",
        "AvgDelay",
        "HighDelayPct",
        "EfficiencyScore",
    ])?;

    for r in rows {
        wtr.write_record(&[
            &r.region,
            &r.main_island,
            &format!("{:.2}", r.total_budget),
            &format!("{:.2}", r.median_savings),
            &format!("{:.2}", r.avg_delay),
            &format!("{:.2}", r.high_delay_pct),
            &format!("{:.2}", r.efficiency_score),
        ])?;
    }

    wtr.flush()?;
    Ok(())
}
