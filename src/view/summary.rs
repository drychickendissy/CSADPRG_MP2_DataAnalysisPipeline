use crate::model::round2;
use serde_json::json;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use crate::model::Project;

pub fn summary_json(projects: &[Project]) -> Result<(), Box<dyn Error>>
{
    let total_projects = projects.len();
    let contractors: HashSet<_> = projects.iter().filter_map(|p| p.contractor.clone()).collect();
    let total_contractors = contractors.len();
    let avg_delay = projects.iter().filter_map(|p| p.completion_delay_days).map(|d| d as f64).sum::<f64>() / (projects.len() as f64);
    let total_savings: f64 = projects.iter().filter_map(|p| p.cost_savings).sum();

    let summary = json!({
        "total_projects": total_projects,
        "total_contractors": total_contractors,
        "global_avg_delay": round2(avg_delay),
        "total_savings": round2(total_savings)
    });

    println!("Summary Stats (summary.json):\n{}\n", serde_json::to_string_pretty(&summary)?);

    let mut f = File::create("summary.json")?;
    f.write_all(serde_json::to_string_pretty(&summary)?.as_bytes())?;
    f.flush()?;
    Ok(())
}
