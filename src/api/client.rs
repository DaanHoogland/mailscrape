use std::collections::HashMap;
use reqwest;
use serde::Deserialize;
use crate::models::*;
use std::error::Error;
use crate::models::stats::MailingListStats;
use crate::models::thread::ThreadStructValue;

#[derive(Deserialize, Debug)]
pub struct MailingListResponse {
    #[serde(default)]
    pub hits: i32,
    #[serde(default)]
    pub participants: Vec<Participant>,
    #[serde(default)]
    pub no_threads: i32,
    #[serde(rename = "searchParams", default)]
    pub search_params: Option<SearchParams>,
    #[serde(default)]
    pub list: String,
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub emails: Vec<Email>,
    #[serde(default)]
    pub thread_struct: ThreadStructValue,
    #[serde(default)]
    pub active_months: HashMap<String, i32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SearchParams {
    #[serde(default)]
    pub list: String,
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub d: String,
    #[serde(default)]
    pub full: Option<String>,
}

pub async fn fetch_mailing_list_data(
    start_date: &str,
    end_date: &str,
    list: &str,
    domain: &str,
) -> Result<MailingListResponse, Box<dyn Error>> {
    let url = format!(
        "https://lists.apache.org/api/stats.lua?list={}&domain={}&d=dfr={}|dto={}",
        list, domain, start_date, end_date
    );
    
    log::debug!("Requesting URL: {}", url);
    
    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    let text = response.text().await?;
    log::debug!("Raw response: {}", text);

    let data: MailingListResponse = serde_json::from_str(&text)?;
    Ok(data)
}

impl From<MailingListResponse> for MailingListStats {
    fn from(response: MailingListResponse) -> Self {
        let (period_start, period_end) = if let Some(search_params) = response.search_params {
            let period_dates = search_params.d.split("|")
                .map(|s| s.replace("dfr=", "").replace("dto=", ""))
                .collect::<Vec<String>>();
            (
                period_dates.get(0).unwrap_or(&String::new()).to_string(),
                period_dates.get(1).unwrap_or(&String::new()).to_string()
            )
        } else {
            (String::new(), String::new())
        };

        MailingListStats {
            total_emails: response.hits,
            total_participants: response.participants.len(),
            total_threads: response.no_threads,
            period_start,
            period_end,
            list_name: response.list,
            domain: response.domain,
            emails: response.emails,
            thread_struct: response.thread_struct,
            active_months: response.active_months,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api::client::{SearchParams, MailingListResponse};
    use crate::models::thread::{ThreadStructValue, Participant};
    use crate::models::email::Email;
    use crate::models::stats::MailingListStats;
    use std::collections::HashMap;

    #[test]
    fn test_search_params() {
        let params = SearchParams {
            list: "test-list".to_string(),
            domain: "example.com".to_string(),
            d: "dfr=2025-01-01|dto=2025-01-31".to_string(),
            full: Some("true".to_string()),
        };

        assert_eq!(params.list, "test-list");
        assert_eq!(params.domain, "example.com");
        assert_eq!(params.d, "dfr=2025-01-01|dto=2025-01-31");
        assert_eq!(params.full, Some("true".to_string()));
    }

    #[test]
    fn test_mailing_list_response() {
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

        let participant = Participant {
            email: "user@example.com".to_string(),
            name: "Test User".to_string(),
            count: 5,
            gravatar: "gravatar-hash".to_string(),
        };

        let search_params = SearchParams {
            list: "test-list".to_string(),
            domain: "example.com".to_string(),
            d: "dfr=2025-01-01|dto=2025-01-31".to_string(),
            full: Some("true".to_string()),
        };

        let mut active_months = HashMap::new();
        active_months.insert("2025-01".to_string(), 10);

        let response = MailingListResponse {
            hits: 10,
            participants: vec![participant.clone()],
            no_threads: 3,
            search_params: Some(search_params.clone()),
            list: "test-list".to_string(),
            domain: "example.com".to_string(),
            emails: vec![email.clone()],
            thread_struct: ThreadStructValue::Array(vec![]),
            active_months: active_months.clone(),
        };

        assert_eq!(response.hits, 10);
        assert_eq!(response.participants.len(), 1);
        assert_eq!(response.participants[0].email, "user@example.com");
        assert_eq!(response.no_threads, 3);
        assert!(response.search_params.is_some());
        assert_eq!(response.search_params.unwrap().list, "test-list");
        assert_eq!(response.list, "test-list");
        assert_eq!(response.domain, "example.com");
        assert_eq!(response.emails.len(), 1);
        assert_eq!(response.emails[0].subject, "Test Subject");
        assert_eq!(response.thread_struct.len(), 0);
        assert_eq!(response.active_months.get("2025-01"), Some(&10));

        // Create a clone of response for conversion test

        // Create a new search_params instance for the clone
        let search_params_clone = SearchParams {
            list: "test-list".to_string(),
            domain: "example.com".to_string(),
            d: "dfr=2025-01-01|dto=2025-01-31".to_string(),
            full: Some("true".to_string()),
        };
        // This avoids moving the original response which is still being used
        let response_clone = MailingListResponse {
            hits: response.hits,
            participants: response.participants.clone(),
            no_threads: response.no_threads,
            search_params: Some(search_params_clone),
            list: response.list.clone(),
            domain: response.domain.clone(),
            emails: response.emails.clone(),
            thread_struct: ThreadStructValue::Array(vec![]),  // Empty array is fine to recreate
            active_months: response.active_months.clone(),
        };

        // Now convert the clone to stats
        let stats: MailingListStats = response_clone.into();
        assert_eq!(stats.total_emails, 10);
        assert_eq!(stats.total_participants, 1);
        assert_eq!(stats.total_threads, 3);
        assert_eq!(stats.period_start, "2025-01-01");
        assert_eq!(stats.period_end, "2025-01-31");
        assert_eq!(stats.list_name, "test-list");
        assert_eq!(stats.domain, "example.com");
    }
}
