use crate::model::{Project, round2};
use csv::WriterBuilder;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;

#[derive(Serialize)]
struct Row {
    year: i32,
    type_of_work: String,
    total_projects: usize,
    avg_savings: f64,
    overrun_rate: f64,
    yoy_change: f64,
}

pub fn report_annual_trends(projects: &[Project]) -> Result<(), Box<dyn Error>> {
    println!("\nReport 3: Annual Project Type Cost Overrun Trends\n");

    // ----------- Group by (FundingYear → TypeOfWork → Vec<Project>) -----------
    let mut map: HashMap<i32, HashMap<String, Vec<&Project>>> = HashMap::new();

    for p in projects.iter() {
        if let (Some(year), Some(work)) = (p.funding_year, p.type_of_work.clone()) {
            map.entry(year)
                .or_default()
                .entry(work)
                .or_default()
                .push(p);
        }
    }

    // ----------- Compute Rows (without YoY first) -----------
    let mut rows: Vec<Row> = Vec::new();
    let mut avg_savings_map: HashMap<(i32, String), f64> = HashMap::new();

    for (year, type_map) in &map {
        for (work, group) in type_map {
            let avg_savings = group
                .iter()
                .filter_map(|p| p.cost_savings)
                .sum::<f64>()
                / (group.len() as f64);

            let overrun_rate = avg_savings / 100.0;

            avg_savings_map.insert((*year, work.clone()), avg_savings);

            rows.push(Row {
                year: *year,
                type_of_work: work.clone(),
                total_projects: group.len(),
                avg_savings: round2(avg_savings),
                overrun_rate: round2(overrun_rate),
                yoy_change: 0.0, // fill later
            });
        }
    }

    // ----------- Compute YoY (% change from previous year) -----------
    for row in rows.iter_mut() {
        let curr = row.avg_savings;

        if let Some(prev) = avg_savings_map.get(&(row.year - 1, row.type_of_work.clone())) {
            if *prev != 0.0 {
                row.yoy_change = round2(((curr - prev) / prev) * 100.0);
            } else {
                row.yoy_change = 0.0;
            }
        } else {
            row.yoy_change = 0.0; // baseline year (e.g., 2021)
        }
    }

    // Sort results by year then type_of_work
    rows.sort_by(|a, b| a.year.cmp(&b.year).then(a.type_of_work.cmp(&b.type_of_work)));

    // ----------- PRINT TABLE -----------
    println!("Annual Project Type Cost Overrun Trends (Grouped by FundingYear and TypeOfWork)\n");

    // Header
    println!(
        "| {:<6} | {:<45} | {:>14} | {:>14} | {:>14} | {:>14} |",
        "Year", "TypeOfWork", "TotalProjects", "AvgSavings", "OverrunRate", "YoYChange"
    );
    println!(
        "|{:-<8}|{:-<47}|{:-<16}|{:-<16}|{:-<16}|{:-<16}|",
        "", "", "", "", "", ""
    );

    // Rows
    for r in &rows {
        let type_of_work = if r.type_of_work.len() > 45 {
            format!("{}…", &r.type_of_work[..44])
        } else {
            r.type_of_work.clone()
        };

        println!(
            "| {:<6} | {:<45} | {:>14} | {:>14.2} | {:>14.2} | {:>14.2} |",
            r.year,
            type_of_work,
            r.total_projects,
            r.avg_savings,
            r.overrun_rate,
            r.yoy_change
        );
    }



    println!("\n(Full table exported to report3_annual_trends.csv)\n");

    // ----------- WRITE CSV -----------
    let mut wtr = WriterBuilder::new().from_path("report3_annual_trends.csv")?;

    wtr.write_record(&[
        "FundingYear",
        "TypeOfWork",
        "TotalProjects",
        "AvgSavings",
        "OverrunRate",
        "YoYChange",
    ])?;

    for r in rows {
        wtr.write_record(&[
            &r.year.to_string(),
            &r.type_of_work,
            &r.total_projects.to_string(),
            &format!("{:.2}", r.avg_savings),
            &format!("{:.2}", r.overrun_rate),
            &format!("{:.2}", r.yoy_change),
        ])?;
    }

    wtr.flush()?;
    Ok(())
}
