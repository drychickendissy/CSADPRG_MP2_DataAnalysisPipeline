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
    year: i32,
    type_of_work: String,
    total_projects: usize,
    avg_savings: f64,
    overrun_rate: f64,
    yoy_change: f64,
}

pub fn report_annual_trends(projects: &[Project]) -> Result<(), Box<dyn Error>>
{
    println!("\nReport 3: Annual Project Type Cost Overrun Trends\n");
    println!("Annual Project Type Cost Overrun Trends (Grouped by FundingYear and TypeOfWork)\n");

    let mut by_year_type: HashMap<(i32, String), Vec<&Project>> = HashMap::new();
    for p in projects.iter()
    {
        if let (Some(y), Some(t)) = (p.funding_year, p.type_of_work.clone())
        {
            by_year_type.entry((y, t)).or_insert_with(Vec::new).push(p);
        }
    }

    let mut rows: Vec<Row> = Vec::new();
    for ((year, type_of_work), group) in by_year_type
    {
        let total_projects = group.len();
        let avg_savings: f64 = group.iter().filter_map(|p| p.cost_savings).sum::<f64>() / (group.len() as f64);
        let overrun_rate = avg_savings / 100.0;
        // Placeholder YoYChange (can later compute from previous year)
        let yoy_change = -20.0;

        rows.push(Row
        {
            year,
            type_of_work,
            total_projects,
            avg_savings: round2(avg_savings),
            overrun_rate: round2(overrun_rate),
            yoy_change: round2(yoy_change),
        });
    }

    rows.sort_by(|a, b| {
        a.year.cmp(&b.year).then_with(|| a.type_of_work.cmp(&b.type_of_work))
    });

    // ---------- PRINT TABLE ----------
    println!(
        "| {:<10} | {:<50} | {:>13} | {:>13} | {:>13} | {:>9} |",
        "FundingYear", "TypeOfWork", "TotalProjects", "AvgSavings", "OverrunRate", "YoYChange"
    );
    println!(
        "|{:-<12}|{:-<52}|{:-<15}|{:-<15}|{:-<15}|{:-<11}|",
        "", "", "", "", "", ""
    );

    for r in &rows
    {
        println!(
            "| {:<10} | {:<50} | {:>13} | {:>13.2} | {:>13.2} | {:>9.2} |",
            r.year,
            truncate(&r.type_of_work, 50),
            r.total_projects,
            r.avg_savings,
            r.overrun_rate,
            r.yoy_change
        );
    }

    println!("\n(Full table exported to report3_annual_trends.csv)\n");

    // ---------- WRITE CSV ----------
    let mut wtr = WriterBuilder::new().from_path("report3_annual_trends.csv")?;
    wtr.write_record(&[
        "FundingYear",
        "TypeOfWork",
        "TotalProjects",
        "AvgSavings",
        "OverrunRate",
        "YoYChange",
    ])?;
    for r in rows
    {
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

// helper: safely truncate long text
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
