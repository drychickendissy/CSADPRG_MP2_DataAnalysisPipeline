# MAJOR COURSE OUTPUT #2: DATA ANALYSIS PIPELINE FOR FLOOD CONTROL PROJECTS

## INTRODUCTION

This major course output presents the development of a Data Analysis Pipeline that demonstrates fundamental concepts in programming paradigms and data processing. The application is designed to ingest a real-world CSV dataset on DPWH flood control projects, perform preprocessing, and generate three tabular reports to facilitate analysis of infrastructure trends, financial efficiencies, and performance metrics. *A key component of this project is the use of libraries or packages to process the data.*

## FUNCTIONAL SPECIFICATIONS

### MANAGING DATA INGESTION

| REQ # | DETAILS |
| ----- | ------- |
| REQ-0001 | Provision to read the CSV file dpwh_flood_control_projects.csv containing 9,800+ rows of flood mitigation projects. |
| REQ-0002 | Provision to perform basic validation: Log total row count and detect/parse errors (e.g., invalid dates or missing values). |
| REQ-0003 | Provision to filter projects from 2021-2023 (exclude 2024 entries for analysis stability). |
| REQ-0004 | Provision to compute derived fields: <ul><li>CostSavings = ApprovedBudgetForContract - ContractCost;</li><li>CompletionDelayDays = days between StartDate and ActualCompletionDate (positive if delayed).</li></ul> |
| REQ-0005 | Provision to clean data uniformly: <ul><li>Convert financial fields to floats (PHP);</li><li>parse dates or use date data types when possible;</li><li>impute or filter incomplete rows (e.g., null lat/long via provincial averages).</li></ul> |

### MANAGING REPORT GENERATION

| REQ # | DETAILS |
| ----- | ------- |
| REQ-0006 | Provision to generate Report 1: Regional Flood Mitigation Efficiency Summary. This table will have the following columns: <ul><li>aggregate total ApprovedBudgetForContract,</li><li>median CostSavings,</li><li>average CompletionDelayDays, and</li><li>percentage of projects with delays >30 days by Region and MainIsland.</li></ul><br>Include "Efficiency Score", which is computed as:<br>(median savings / average delay) * 100, normalized to 0-100.<br><br>Output as sorted CSV (descending by EfficiencyScore). |
| REQ-0007 | Provision to generate Report 2: Top Contractors Performance Ranking. Rank top 15 Contractors by total ContractCost (descending, filter >=5 projects), with columns for the following: <ul><li>number of projects,</li><li>average CompletionDelayDays,</li><li>total CostSavings,</li><li>"Reliability Index", which is computed as (1 - (avg delay / 90)) * (total savings / total cost) * 100 (capped at 100). Flag <50 as "High Risk".</li></ul><br>Output as sorted CSV. |
| REQ-0008 | Provision to generate Report 3: Annual Project Type Cost Overrun Trends. Group by FundingYear and TypeOfWork, computing the following: <ul><li>total projects</li><li>average CostSavings (negative if overrun)</li><li>overrun rate (% with negative savings)</li><li>year-over-year % change in average savings (2021 baseline).</li></ul><br>Output as sorted CSV (ascending by year, descending by AvgSavings). |
| REQ-0009 | Provision to produce a summary.json aggregating key stats across reports (e.g., total number of projects, total number of contractors, total provinces with projects, global average delay, total savings). |

## TECHNICAL SPECIFICATION

| REQ # | DETAILS |
| ----- | ------- |
| REQ-0010 | Application should be developed / built on the following programming languages: ● R ● JavaScript ● Kotlin ● Rust |
| REQ-0011 | Provision for output standardization: Generate identical CSV files for each report (comma-formatted numbers, rounded to 2 decimals); one run command per language (e.g., Rscript main.R, node index.js). |

## CHECKLIST
- [ ] REQ-0001
- [ ] REQ-0002
- [ ] REQ-0003
- [ ] REQ-0004
- [ ] REQ-0005
- [x] REQ-0006
- [x] REQ-0007
- [ ] REQ-0008
- [ ] REQ-0009
- [ ] REQ-0010
- [ ] REQ-0011
