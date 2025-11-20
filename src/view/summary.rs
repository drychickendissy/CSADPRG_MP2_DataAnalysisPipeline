/********************
Last names: Abdulrahman, Bilanes, Cruz, Nicolas
Language: JavaScript
Paradigm(s): Procedural, Object-Oriented, Functional, Data-Driven, Immutable
********************/

use crate::model::Project;
use serde::Serialize;   // for converting structs to JSON
use std::error::Error;  // for error handling
use std::fs::File;  // for file operations
use std::io::Write;

// Structure for summary.json
#[derive(Serialize)]
struct SummaryJson {
    total_projects: usize,
    total_contractors: usize,
    total_provinces: usize,
    global_avg_delay_days: f64,
    global_total_savings: f64,
}

// Cleans contractor name by trimming, checking for invalid phrases, and uppercasing
fn clean_contractor(name: &str) -> Option<String> {
    let n = name.trim();
    if n.is_empty() { 
        return None; 
    }

    let lower = n.to_lowercase();
    if lower.contains("clustered with contract id") { 
        return None; 
    }
    if lower.contains("myca with project id") { 
        return None; 
    }

    Some(n.to_uppercase())
}

// Cleans province name by trimming and uppercasing
fn clean_province(name: &str) -> Option<String> {
    let n = name.trim();
    if n.is_empty() { 
        return None; 
    }
    Some(n.to_uppercase())
}

// Generates summary.json file with key statistics
pub fn summary_json(projects: &[Project]) -> Result<(), Box<dyn Error>> {
    println!("\nExporting summary.json ...");

    let total_projects = projects.len();

    // Unique Contractors
    let mut contractor_set = std::collections::HashSet::new();
    for p in projects {
        if let Some(c) = &p.contractor {    // if contractor exists
            if let Some(clean) = clean_contractor(c) {  // clean the name
                contractor_set.insert(clean);   //  add to set
            }
        }
    }

    // Unique Provinces
    let mut province_set = std::collections::HashSet::new();
    for p in projects {
        if let Some(pr) = &p.province { // if province exists
            if let Some(clean) = clean_province(pr) {   //  clean the name
                province_set.insert(clean); // add to set
            }
        }
    }

    // Global Average Delay Days
    let delays: Vec<f64> = projects
        .iter()
        .filter_map(|p| p.completion_delay_days.map(|d| d as f64))
        .collect();

    let global_avg_delay_days = if delays.is_empty() {
        0.0
    } else {
        delays.iter().sum::<f64>() / delays.len() as f64    //  average calculation
    };

    // Total Savings
    let global_total_savings = projects
        .iter()
        .filter_map(|p| p.cost_savings)
        .sum::<f64>();

    let summary = SummaryJson {
        total_projects,
        total_contractors: contractor_set.len(),
        total_provinces: province_set.len(),
        global_avg_delay_days,
        global_total_savings,
    };

    let json_text = serde_json::to_string_pretty(&summary)?;    // convert summary struct to pretty JSON string
    let mut file = File::create("summary.json")?;
    file.write_all(json_text.as_bytes())?;  // write JSON string to file

    println!("summary.json created.\n");
    println!("\n==================== Summary Report ====================");
    println!("Total Projects        : {}", summary.total_projects);
    println!("Unique Contractors    : {}", summary.total_contractors);
    println!("Unique Provinces      : {}", summary.total_provinces);
    println!("Avg Delay (days)      : {:.2}", summary.global_avg_delay_days);
    println!("Total Savings (₱)     : {:.2}", summary.global_total_savings);
    println!("========================================================\n");

    Ok(())
}
