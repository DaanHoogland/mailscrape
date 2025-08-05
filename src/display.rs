use crate::models::{AnalyzedStats, MailingListStats};
use crate::analysis::find_unanswered_emails;

#[derive(Default, Debug)]
pub struct DisplayConfig {
    pub show_header: bool,
    pub show_emails: bool,
    pub show_threads: bool,
    pub show_daily_activity: bool,
    pub show_averages: bool,
    pub show_summary: bool,
    pub show_unanswered: bool,
    pub verbose: bool,
}

impl DisplayConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_header(mut self, show: bool) -> Self {
        self.show_header = show;
        self
    }

    pub fn with_emails(mut self, show: bool) -> Self {
        self.show_emails = show;
        self
    }

    pub fn with_threads(mut self, show: bool) -> Self {
        self.show_threads = show;
        self
    }

    pub fn with_daily_activity(mut self, show: bool) -> Self {
        self.show_daily_activity = show;
        self
    }

    pub fn with_averages(mut self, show: bool) -> Self {
        self.show_averages = show;
        self
    }

    pub fn with_summary(mut self, show: bool) -> Self {
        self.show_summary = show;
        self
    }

    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }

    pub fn with_unanswered_emails(mut self, show: bool) -> Self {
        self.show_unanswered = show;
        self
    }
}

pub fn display_analysis(analyzed_stats: &AnalyzedStats, stats: &MailingListStats, config: &DisplayConfig) {
    log::debug!("Starting to display analysis with config: {:?}", config);

    if config.show_header {
        println!("\nMailing List Statistics Summary");
        println!("==============================");
        println!(
            "List: {} ",
            analyzed_stats.list_info.list_name
        );
        println!(
            "Period: {} to {}",
            analyzed_stats.list_info.period_from,
            analyzed_stats.list_info.period_to
        );
    }

    if config.show_emails {
        println!("\nEmails:");
        println!("-------");
        for email in &stats.emails {
            if config.verbose {
                println!("Subject: {}", email.subject);
                println!("From: {}", email.from);
                println!("Date: {:?}", email.date);
                println!("Message-ID: {}", email.message_id);
                println!();
            } else {
                println!("- {} (from: {})", email.subject, email.from);
            }
        }
    }

    if config.show_threads {
        println!("\nThreads:");
        println!("--------");
        for thread in stats.thread_struct.iter() {
            if config.verbose {
                println!("Thread: {}", thread.subject);
                println!("Depth: {}", thread.nest);
                println!();
            } else {
                println!("- {}", thread.subject);
            }
        }
    }

    if config.show_daily_activity {
        println!("\nDaily Activity:");
        println!("{:<12} {:>8} {:>14} {:>10}", "Date", "Emails", "Participants", "Threads");
        println!("{}", "-".repeat(46));

        for (date, count) in &stats.active_months {
            println!("{:<12} {:>8}", date, count);
        }

        println!("{}", "-".repeat(46));
        println!(
            "Totals:{:>9} {:>14} {:>10}",
            analyzed_stats.total_emails,
            analyzed_stats.total_participants,
            analyzed_stats.total_threads
        );
    }

    if config.show_averages {
        println!("\nAverages per day:");
        println!(
            "Emails: {:.2}\nParticipants: {:.2}\nThreads: {:.2}",
            analyzed_stats.avg_emails,
            analyzed_stats.avg_participants,
            analyzed_stats.avg_threads
        );
    }

    if config.show_summary {
        println!("{}", "-".repeat(32));
        println!(
            "Summary: {} emails in {} threads",
            analyzed_stats.total_emails,
            analyzed_stats.total_threads
        );
    }

    if config.show_unanswered {
        println!("\nUnanswered Emails:");
        println!("-----------------");
        let unanswered = find_unanswered_emails(&stats.emails);
        if unanswered.is_empty() {
            println!("No unanswered emails found.");
        } else {
            let len = unanswered.len();
            for email in unanswered {
                if config.verbose {
                    println!("Subject: {}", email.subject);
                    println!("From: {}", email.from);
                    println!("Date: {:?}", email.date);
                    println!("Message-ID: {}", email.message_id);
                    println!();
                } else {
                    println!("- {} (from: {}, date: {:?})", email.subject, email.from, email.date);
                }
            }
            println!("\nTotal unanswered emails: {}", len);
        }
    }

    log::debug!("Finished displaying analysis");
}
