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

#[cfg(test)]
mod tests {
    use crate::models::stats::{MailingListStats, ListInfo};
    use crate::models::thread::{ThreadStruct, ThreadStructValue};
    use crate::models::email::Email;
    use crate::analysis::stats_analyzer::analyze_stats;
    use std::collections::HashMap;

    #[test]
    fn test_mailing_list_stats_all_fields() {
        let email = Email {
            message_id: "<12345@example.com>".to_string(),
            subject: "Test Subject".to_string(),
            in_reply_to: "".to_string(),
            from: "sender@example.com".to_string(),
            date: Some("2025-01-01".to_string()),
            private: false,
            attachments: vec![],
            epoch: 1609459200,
            list: "test-list".to_string(),
            gravatar: "gravatar-hash".to_string(),
            list_raw: "test-list@example.com".to_string(),
            id: "email-id-123".to_string(),
            body: "This is the email body".to_string(),
            mid: "mid-123".to_string(),
        };

        let thread = ThreadStruct {
            tid: "thread-123".to_string(),
            subject: "Thread Subject".to_string(),
            tsubject: "Thread Subject".to_string(),
            epoch: 1609459200,
            nest: 0,
            children: Vec::new(),
        };

        let mut active_months = HashMap::new();
        active_months.insert("2025-01".to_string(), 10);

        let stats = MailingListStats {
            total_emails: 10,
            total_participants: 5,
            total_threads: 3,
            period_start: "2025-01-01".to_string(),
            period_end: "2025-01-31".to_string(),
            list_name: "test-list".to_string(),
            domain: "example.com".to_string(),
            emails: vec![email.clone()],
            thread_struct: ThreadStructValue::Array(vec![thread.clone()]),
            active_months: active_months.clone(),
        };

        // Test all fields to ensure they're used
        assert_eq!(stats.total_emails, 10);
        assert_eq!(stats.total_participants, 5);
        assert_eq!(stats.total_threads, 3);
        assert_eq!(stats.period_start, "2025-01-01");
        assert_eq!(stats.period_end, "2025-01-31");
        assert_eq!(stats.list_name, "test-list");
        assert_eq!(stats.domain, "example.com");
        assert_eq!(stats.emails.len(), 1);
        assert_eq!(stats.emails[0].subject, "Test Subject");
        assert_eq!(stats.thread_struct.len(), 1);
        assert_eq!(stats.thread_struct.iter().next().unwrap().tid, "thread-123");
        assert_eq!(stats.active_months.get("2025-01"), Some(&10));
    }

    #[test]
    fn test_analyzed_stats() {
        let mut active_months = HashMap::new();
        active_months.insert("2025-01".to_string(), 10);

        let stats = MailingListStats {
            total_emails: 10,
            total_participants: 5,
            total_threads: 3,
            period_start: "2025-01-01".to_string(),
            period_end: "2025-01-31".to_string(),
            list_name: "test-list".to_string(),
            domain: "example.com".to_string(),
            emails: vec![],
            thread_struct: ThreadStructValue::Array(vec![]),
            active_months: active_months,
        };

        let analyzed = analyze_stats(&stats);

        // Test all fields
        assert_eq!(analyzed.total_emails, 10);
        assert_eq!(analyzed.total_participants, 5);
        assert_eq!(analyzed.total_threads, 3);
        assert!(analyzed.avg_emails > 0.0);
        assert!(analyzed.avg_participants > 0.0);
        assert!(analyzed.avg_threads > 0.0);
        assert_eq!(analyzed.daily_stats.get("2025-01"), Some(&10));
        assert_eq!(analyzed.list_info.list_name, "test-list");
        assert_eq!(analyzed.list_info.domain, "example.com");
        assert_eq!(analyzed.list_info.period_from, "2025-01-01");
        assert_eq!(analyzed.list_info.period_to, "2025-01-31");
    }

    #[test]
    fn test_list_info() {
        let info = ListInfo {
            list_name: "test-list".to_string(),
            domain: "example.com".to_string(),
            period_from: "2025-01-01".to_string(),
            period_to: "2025-01-31".to_string(),
        };

        assert_eq!(info.list_name, "test-list");
        assert_eq!(info.domain, "example.com");
        assert_eq!(info.period_from, "2025-01-01");
        assert_eq!(info.period_to, "2025-01-31");
    }
}
