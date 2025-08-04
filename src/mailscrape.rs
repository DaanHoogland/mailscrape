use clap::Parser;
use std::error::Error;

use mailscrape::{
    models::stats::MailingListStats,
    api::client::fetch_mailing_list_data,
    analysis::stats_analyzer::analyze_stats,
    display::{DisplayConfig, display_analysis}
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    start_date: String,

    #[arg(short, long)]
    end_date: String,

    #[arg(short, long, default_value = "dev")]
    list: String,

    #[arg(short, long, default_value = "cloudstack.apache.org")]
    domain: String,

    #[arg(short = 'H', long, default_value = "lists.apache.org")]
    host: String,

    #[arg(long)]
    show_emails: bool,

    #[arg(long)]
    show_threads: bool,

    #[arg(long)]
    show_daily: bool,

    #[arg(long)]
    show_averages: bool,

    #[arg(long)]
    show_unanswered: bool,

    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let args = Args::parse();

    let json_data = fetch_mailing_list_data(
        &args.start_date,
        &args.end_date,
        &args.list,
        &args.domain,
    ).await?;

    let stats: MailingListStats = json_data.into();
    let analyzed_stats = analyze_stats(&stats);

    let display_config = DisplayConfig::new()
        .with_header(true)
        .with_emails(args.show_emails)
        .with_threads(args.show_threads)
        .with_daily_activity(args.show_daily)
        .with_averages(args.show_averages)
        .with_summary(true)
        .with_unanswered_emails(args.show_unanswered)
        .verbose(args.verbose);

    display_analysis(&analyzed_stats, &stats, &display_config);
    Ok(())
}
