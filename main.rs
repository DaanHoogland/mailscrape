use std::collections::HashMap;
use std::fmt;
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

    #[arg(short = 'H', long, default_value = "lists.apache.org")]
    host: String,
}


#[derive(Deserialize, Debug)]
struct MailingListResponse {
    #[serde(rename = "firstYear")]
    first_year: i32,
    #[serde(rename = "lastYear")]
    last_year: i32,
    #[serde(rename = "firstMonth")]
    first_month: i32,
    #[serde(rename = "lastMonth")]
    last_month: i32,
    active_months: HashMap<String, i32>,
    hits: i32,
    numparts: i32,
    no_threads: i32,
    emails: Vec<Email>,
    participants: Vec<Participant>,
    searchlist: String,
    domain: String,
    name: String,
    list: String,
    #[serde(rename = "searchParams")]
    search_params: SearchParams,
    unixtime: i64,
    thread_struct: Vec<ThreadStruct>,
    cloud: HashMap<String, i32>,
}

// Implement Display for MailingListResponse
impl fmt::Display for MailingListResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Mailing List: {} ({} emails, {} participants, {} threads)",
               self.list,
               self.emails.len(),
               self.participants.len(),
               self.no_threads
        )
    }
}

#[derive(Debug, Deserialize, Clone)]
struct Email {
    #[serde(rename = "in-reply-to")]
    in_reply_to: String,
    private: bool,
    attachments: Vec<String>,
    subject: String,
    mid: String,
    epoch: i64,
    list: String,
    gravatar: String,
    #[serde(rename = "message-id")]
    message_id: String,
    from: String,
    list_raw: String,
    id: String,
    body: String,

    #[serde(default)]  // This will use a default value if the field is missing
    date: String,
}

#[derive(Deserialize, Debug, Clone)]
struct Participant {
    email: String,
    name: String,
    count: i32,
    gravatar: String,
}

#[derive(Deserialize, Debug)]
struct SearchParams {
    list: String,
    domain: String,
    d: String,
    full: String,
}

#[derive(Deserialize, Debug, Clone)]
struct ThreadStruct {
    children: Vec<ThreadStruct>,
    tid: String,
    subject: String,
    tsubject: String,
    epoch: i64,
    nest: i32,
}

#[derive(Debug, Deserialize)]
struct Thread {
    subject: String,
    emails: Vec<Email>,
    participants: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct MailingListStats {
    total_emails: i32,
    total_participants: usize,
    total_threads: i32,
    period_start: String,
    period_end: String,
    list_name: String,
    emails: Vec<Email>,
    thread_struct: Vec<ThreadStruct>,
    active_months: HashMap<String, i32>,
}

#[derive(Debug, Deserialize, Clone)]
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
    daily_stats: HashMap<String, i32>,
//    daily_stats: Vec<DailyActivity>,
    list_info: ListInfo,
}

#[derive(Debug)]
struct ListInfo {
    list_name: String,
    period_from: String,
    period_to: String,
}

fn fetch_mailing_list_data(args: &Args) -> Result<MailingListResponse, Box<dyn Error>> {
    let url = format!(
        "https://{}/api/stats.lua?list={}&domain={}&d=dfr={}|dto={}&full=true",
        args.host, args.list, args.domain, args.start_date, args.end_date
    );

    let response = reqwest::blocking::get(&url)?.text()?;
    let data: MailingListResponse = serde_json::from_str(&response)?;
    Ok(data)
}

fn parse_mailing_list_data(response: &MailingListResponse) -> Result<MailingListStats, Box<dyn Error>> {
    Ok(MailingListStats {
        total_emails: response.hits,
        total_participants: response.participants.len(),
        total_threads: response.no_threads,
        period_start: response.search_params.d.split('|').next()
            .and_then(|s| s.strip_prefix("dfr="))
            .unwrap_or("unknown").to_string(),
        period_end: response.search_params.d.split('|').nth(1)
            .and_then(|s| s.strip_prefix("dto="))
            .unwrap_or("unknown").to_string(),
        list_name: response.list.clone(),
        emails: response.emails.clone(),
        thread_struct: response.thread_struct.clone(),
        active_months: response.active_months.clone(),
    })
}

fn analyze_stats(stats: &MailingListStats) -> AnalyzedStats {
    let total_emails: i32 = stats.total_emails;
    let total_participants: i32 = stats.total_participants as i32;
    let total_threads: i32 = stats.total_threads;

    // Calculate the number of days based on the active months in the period
    let days = stats.active_months.len() as f64 * 30.44; // approximate average days per month

    AnalyzedStats {
        total_emails,
        total_participants,
        total_threads,
        avg_emails: total_emails as f64 / days,
        avg_participants: total_participants as f64 / days,
        avg_threads: total_threads as f64 / days,
        daily_stats: stats.active_months.clone(),
        list_info: ListInfo {
            list_name: stats.list_name.clone(),
            period_from: stats.period_start.clone(),
            period_to: stats.period_end.clone(),
        },
    }
}

fn display_analysis(analyzed_stats: &AnalyzedStats, stats: &MailingListStats) {
    println!("\nMailing List Statistics Summary");
    println!("==============================");
    println!(
        "List: {} ",
        analyzed_stats.list_info.list_name
    );

    println!("Emails:");
    println!("-------");
    for email in &stats.emails {
        println!("Subject: {}", email.subject);
        println!("From: {}", email.from);
        println!("Date: {}", email.date);
        println!("Message-ID: {}", email.message_id);
        println!();
    }

    println!("\nThreads:");
    println!("--------");
    for thread in &stats.thread_struct {
        println!("Thread: {}", thread.subject);
        println!();
    }

    println!(
        "Period: {} to {}",
        analyzed_stats.list_info.period_from, analyzed_stats.list_info.period_to
    );

    println!("\nDaily Activity:");
    println!("{:<12} {:>8} {:>14} {:>10}", "Date", "Emails", "Participants", "Threads");
    println!("{}", "-".repeat(46));

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

    // Summary statistics
    println!("{}", "-".repeat(32));
    println!(
        "Totals: {} emails in {} threads",
        analyzed_stats.total_emails,
        analyzed_stats.total_threads
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

    let analyzed_stats = analyze_stats(&stats);

    display_analysis(&analyzed_stats, &stats);
}