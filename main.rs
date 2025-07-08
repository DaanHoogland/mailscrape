use clap::Parser;
use serde::Deserialize;
use std::error::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Start date in YYYY-MM-DD format
    #[arg(short, long)]
    start_date: String,

    /// End date in YYYY-MM-DD format
    #[arg(short, long)]
    end_date: String,

    #[arg(short, long, default_value = "dev")]
    list: String,

    #[arg(short, long, default_value = "cloudstack.apache.org")]
    domain: String,
}

#[derive(Debug, Deserialize)]
struct MailingListStats {
    activity: Vec<DailyActivity>,
    list: String,
    domain: String,
    period: Period,
}

#[derive(Debug, Deserialize)]
struct DailyActivity {
    date: String,
    emails: i32,
    participants: i32,
    threads: i32,
}

#[derive(Debug, Deserialize)]
struct Period {
    from: String,
    to: String,
}

#[derive(Debug)]
struct AnalyzedStats {
    total_emails: i32,
    total_participants: i32,
    total_threads: i32,
    avg_emails: f64,
    avg_participants: f64,
    avg_threads: f64,
    daily_stats: Vec<DailyActivity>,
    list_info: ListInfo,
}

#[derive(Debug)]
struct ListInfo {
    list_name: String,
    domain: String,
    period_from: String,
    period_to: String,
}

fn fetch_mailing_list_data(args: &Args) -> Result<String, Box<dyn Error>> {
    let url = format!(
        "https://lists.apache.org/api/stats.lua?list={}&domain={}&d=dfr={}|dto={}",
        args.list, args.domain, args.start_date, args.end_date
    );

    Ok(reqwest::blocking::get(&url)?.text()?)
}

fn parse_mailing_list_data(json_data: &str) -> Result<MailingListStats, serde_json::Error> {
    serde_json::from_str(json_data)
}

fn analyze_stats(stats: MailingListStats) -> AnalyzedStats {
    let total_emails: i32 = stats.activity.iter().map(|day| day.emails).sum();
    let total_participants: i32 = stats.activity.iter().map(|day| day.participants).sum();
    let total_threads: i32 = stats.activity.iter().map(|day| day.threads).sum();
    let days = stats.activity.len() as f64;

    AnalyzedStats {
        total_emails,
        total_participants,
        total_threads,
        avg_emails: total_emails as f64 / days,
        avg_participants: total_participants as f64 / days,
        avg_threads: total_threads as f64 / days,
        daily_stats: stats.activity,
        list_info: ListInfo {
            list_name: stats.list,
            domain: stats.domain,
            period_from: stats.period.from,
            period_to: stats.period.to,
        },
    }
}

fn display_analysis(analyzed_stats: &AnalyzedStats) {
    println!("\nMailing List Statistics Summary");
    println!("==============================");
    println!(
        "List: {} @ {}",
        analyzed_stats.list_info.list_name, analyzed_stats.list_info.domain
    );
    println!(
        "Period: {} to {}",
        analyzed_stats.list_info.period_from, analyzed_stats.list_info.period_to
    );

    println!("\nDaily Activity:");
    println!("{:<12} {:>8} {:>14} {:>10}", "Date", "Emails", "Participants", "Threads");
    println!("{}", "-".repeat(46));

    for day in &analyzed_stats.daily_stats {
        println!(
            "{:<12} {:>8} {:>14} {:>10}",
            day.date, day.emails, day.participants, day.threads
        );
    }

    println!("{}", "-".repeat(46));
    println!(
        "Totals:{:>9} {:>14} {:>10}",
        analyzed_stats.total_emails,
        analyzed_stats.total_participants,
        analyzed_stats.total_threads
    );

    println!("\nAverages per day:");
    println!(
        "Emails: {:.2}\nParticipants: {:.2}\nThreads: {:.2}",
        analyzed_stats.avg_emails,
        analyzed_stats.avg_participants,
        analyzed_stats.avg_threads
    );
}

fn main() {
    let args = Args::parse();

    let json_data = match fetch_mailing_list_data(&args) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error fetching data: {}", e);
            return;
        }
    };

    let stats = match parse_mailing_list_data(&json_data) {
        Ok(stats) => stats,
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
            eprintln!("Raw response: {}", json_data);
            return;
        }
    };

    let analyzed_stats = analyze_stats(stats);

    display_analysis(&analyzed_stats);
}