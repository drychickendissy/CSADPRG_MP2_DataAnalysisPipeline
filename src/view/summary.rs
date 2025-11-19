use crate::model::Project;
use serde::Serialize;
use std::error::Error;
use std::fs::File;
use std::io::Write;

fn clean_contractor(name: &str) -> Option<String> {
    let n = name.trim();
    if n.is_empty() { return None; }

    let lower = n.to_lowercase();
    if lower.contains("clustered with contract id") { return None; }
    if lower.contains("myca with project id") { return None; }

    Some(n.to_uppercase())
}

fn clean_province(name: &str) -> Option<String> {
    let n = name.trim();
    if n.is_empty() { return None; }
    Some(n.to_uppercase())
}

#[derive(Serialize)]
struct SummaryJson {
    total_projects: usize,
    total_contractors: usize,
    total_provinces: usize,
    global_avg_delay_days: f64,
    global_total_savings: f64,
}

pub fn summary_json(projects: &[Project]) -> Result<(), Box<dyn Error>> {
    println!("\nExporting summary.json ...");

    let total_projects = projects.len();

    // UNIQUE CONTRACTORS
    let mut contractor_set = std::collections::HashSet::new();
    for p in projects {
        if let Some(c) = &p.contractor {
            if let Some(clean) = clean_contractor(c) {
                contractor_set.insert(clean);
            }
        }
    }

    // UNIQUE PROVINCES
    let mut province_set = std::collections::HashSet::new();
    for p in projects {
        if let Some(pr) = &p.province {
            if let Some(clean) = clean_province(pr) {
                province_set.insert(clean);
            }
        }
    }

    // GLOBAL AVERAGE DELAY (OR MEDIAN IF YOU WANT)
    let mut delays: Vec<f64> = projects
        .iter()
        .filter_map(|p| p.completion_delay_days.map(|d| d as f64))
        .collect();

    let global_avg_delay_days = if delays.is_empty() {
        0.0
    } else {
        delays.iter().sum::<f64>() / delays.len() as f64
    };

    // TOTAL SAVINGS
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

    let json_text = serde_json::to_string_pretty(&summary)?;
    let mut file = File::create("summary.json")?;
    file.write_all(json_text.as_bytes())?;

    println!("summary.json created.\n");
    Ok(())
}
