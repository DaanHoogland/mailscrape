use std::collections::HashMap;
use super::{email::Email, thread::ThreadStructValue};

#[derive(Debug)]
pub struct MailingListStats {
    pub total_emails: i32,
    pub total_participants: usize,
    pub total_threads: i32,
    pub period_start: String,
    pub period_end: String,
    pub list_name: String,
    pub domain: String,
    pub emails: Vec<Email>,
    pub thread_struct: ThreadStructValue,
    pub active_months: HashMap<String, i32>,
}

#[derive(Debug)]
pub struct AnalyzedStats {
    pub total_emails: i32,
    pub total_participants: i32,
    pub total_threads: i32,
    pub avg_emails: f64,
    pub avg_participants: f64,
    pub avg_threads: f64,
    pub daily_stats: HashMap<String, i32>,
    pub list_info: ListInfo,
}

#[derive(Debug)]
pub struct ListInfo {
    pub list_name: String,
    pub domain: String,
    pub period_from: String,
    pub period_to: String,
}
