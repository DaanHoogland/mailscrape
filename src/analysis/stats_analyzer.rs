use crate::models::*;

pub fn analyze_stats(stats: &MailingListStats) -> AnalyzedStats {
    let total_emails: i32 = stats.total_emails;
    let total_participants: i32 = stats.total_participants as i32;
    let total_threads: i32 = stats.total_threads;
    let days = stats.active_months.len() as f64 * 30.44;

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
            domain: stats.domain.clone(),
            period_from: stats.period_start.clone(),
            period_to: stats.period_end.clone(),
        },
    }
}
