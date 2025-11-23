/********************
Last names: Abdulrahman, Bilanes, Cruz, Nicolas
Language: JavaScript
Paradigm(s): Procedural, Object-Oriented, Functional, Data-Driven, Immutable
********************/

use crate::model::{Project, median, round2};    // imports Project struct and utility functions from model
use csv::WriterBuilder; // enables CSV writing
use serde::Serialize;   // enables serialization for CSV
use std::error::Error;  // allows Result<(), Box<dyn Error>> (error handling)
use std::collections::HashMap;  // enables HashMap usage
use num_format::{Locale, ToFormattedString};    // enables number formatting with commas
use chrono::Datelike;   // allows .year() on NaiveDate

#[derive(Serialize)]
struct Report1Row
{
    region: String,
    main_island: String,
    total_budget: f64,
    median_savings: f64,
    avg_delay: f64,
    high_delay_pct: f64,
    efficiency_score: f64,
}

pub fn report_regional_efficiency(projects: &[Project]) -> Result<(), Box<dyn Error>>
{
    println!("\nRegional Flood Mitigation Efficiency Summary");
    println!("(Filtered: 2021–2023 Projects)\n");

    // Filter projects by StartDate year (2021–2023) 
    // NOTE: REMOVE THIS IF ALREADY FILTERED IN CONTROLLER
    let filtered: Vec<&Project> = projects
        .iter()
        .filter(|p| {
            if let Some(date) = p.start_date {
                let year = date.year();
                year >= 2021 && year <= 2023
            } else {
                false
            }
        })
        .collect();


    // Group by Region + MainIsland
    let mut by_group: HashMap<String, Vec<&Project>> = HashMap::new();
    for p in filtered
    {
        let project_region = p.region.clone().unwrap_or_else(|| "Unknown".to_string());
        let project_island = p.main_island.clone().unwrap_or_else(|| "Unknown".to_string());
        let key = format!("{}|{}", project_region, project_island); // combines region and island as a key
        by_group.entry(key).or_insert_with(Vec::new).push(p);   // adds project to group (region + island)
    }

    let mut rows: Vec<Report1Row> = Vec::new(); // stores rows for report. Each row will hold metrics for one region + main island group

    for (key, group) in by_group
    {
        let parts: Vec<&str> = key.split('|').collect();    // splits key back into region and island
        let region = parts[0].to_string();  // .to_string() converts &str to String
        let main_island = parts[1].to_string();

        let total_budget: f64 = group
            .iter() // iterates through projects in group
            .filter_map(|p| p.approved_budget_for_contract) // extracts approved_budget_for_contract if Some, skips if None
            .sum(); // sums up all budgets
        // same (more or less) logic for savings and delays

        let mut savings: Vec<f64> = group
            .iter()
            .filter_map(|p| p.cost_savings)
            .collect();
        let median_savings = median(&mut savings);

        let delays: Vec<f64> = group
            .iter()
            .filter_map(|p| p.completion_delay_days.map(|d| d as f64))
            .collect();

        let avg_delay = if delays.is_empty() {
            0.0
        }
        else {
            delays.iter().sum::<f64>() / delays.len() as f64    // sum of delays / count of delays = average delay in days
        };

        let high_delay_count = delays.iter().filter(|&&d| d > 30.0).count();    // counts number of delays greater than 30 days
        let high_delay_pct = if delays.is_empty() {
            0.0
        }
        else {
            (high_delay_count as f64 / delays.len() as f64) * 100.0 // (count of high delays / total delays) * 100 = percentage of high delays
        };

        // --- EfficiencyRaw
        let efficiency_raw = if avg_delay > 0.0 {
            (median_savings / avg_delay) * 100.0
        }
        else {
            0.0
        };  // (if average delay > 0) (median savings / average delay) * 100

        rows.push(Report1Row
        {
            region,
            main_island,
            total_budget: round2(total_budget), // round2(i) rounds i to 2 decimal places
            median_savings: round2(median_savings),
            avg_delay: round2(avg_delay),
            high_delay_pct: round2(high_delay_pct),
            efficiency_score: efficiency_raw, // will normalize later
        }); // adds row to rows vector
    }

    // --- Normalize efficiency scores to 0–100
    let valid_scores: Vec<f64> = rows
        .iter()
        .filter(|r| r.efficiency_score > 0.0)
        .map(|r| r.efficiency_score)
        .collect(); // collects all efficiency scores greater than 0 into valid_scores vector

    let (min_score, max_score) = if valid_scores.is_empty() {
        (0.0, 1.0)
    }   // if valid_scores is empty, set min to 0 and max to 1 to avoid division by zero
    else {
        (
            valid_scores
                .iter()
                .cloned()   // clone to get f64 values instead of references
                .fold(f64::INFINITY, f64::min), // finds minimum score in valid_scores
            valid_scores
                .iter()
                .cloned()
                .fold(f64::NEG_INFINITY, f64::max), // finds maximum score in valid_scores
        )
    };

    // NORMALIZATION OF EFFICIENCY SCORES (normalized to 0–100)
    for r in &mut rows {
        if r.efficiency_score > 0.0 && (max_score - min_score) > f64::EPSILON {
            r.efficiency_score = ((r.efficiency_score - min_score) / (max_score - min_score)) * 100.0;
        }
        else {
            r.efficiency_score = 0.0;
        }

        r.efficiency_score = round2(r.efficiency_score);
    }   // BASICALLY: normalized_score = ((raw_score - min) / (max - min)) * 100, assuming efficiency_score > 0 and max != min

    // Sort descending by efficiency
    rows.sort_by(|a, b| {
        b.efficiency_score  // sorts rows in descending order by efficiency_score
            .partial_cmp(&a.efficiency_score)   // use partial_cmp for f64 comparison (f64 can have NaN values)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    // ---------- Print Table ----------
    // :> is right align
    // :< is left align
    println!(
        "| {:<35} | {:<10} | {:>17} | {:>14} | {:>9} | {:>13} | {:>17} |",
        "Region", "MainIsland", "TotalBudget", "MedianSavings", "AvgDelay", "HighDelayPct", "EfficiencyScore"
    );
    println!(
        "|{:-<37}|{:-<12}|{:-<19}|{:-<16}|{:-<11}|{:-<15}|{:-<19}|",
        "", "", "", "", "", "", ""
    );

    for r in &rows
    {
        let formatted_budget = format!(
            "{}.{:02}",
            (r.total_budget as u64).to_formatted_string(&Locale::en),
            (r.total_budget.fract() * 100.0).round() as u64
        );
        let formatted_median_savings = format!(
            "{}.{:02}",
            (r.median_savings as u64).to_formatted_string(&Locale::en),
            (r.median_savings.fract() * 100.0).round() as u64
        );

        println!(
            "| {:<35} | {:<10} | {:>17} | {:>14} | {:>9.1} | {:>13.2} | {:>17.2} |",    // :>17 and not :>14 to match alignment
            r.region,
            r.main_island,
            formatted_budget,
            formatted_median_savings,
            r.avg_delay,
            r.high_delay_pct,
            r.efficiency_score
        );
    }

    println!("(Full table exported to report1_regional_efficiency.csv)\n");

    // ----- Save CSV -----
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

    for r in rows
    {
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