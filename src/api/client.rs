/*
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */
use crate::models::stats::MailingListStats;
use crate::models::thread::ThreadStructValue;
use crate::models::*;
use reqwest;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;

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
    log::debug!("Total emails received: {}", data.emails.len());
    for (index, email) in data.emails.iter().enumerate() {
        log::trace!(
            "Email {}: date={:?}, subject={}, from={}, in_reply_to={}",
            index,
            email.date,
            email.subject,
            email.from,
            email.in_reply_to
        );
    }
    Ok(data)
}

impl From<MailingListResponse> for MailingListStats {
    fn from(response: MailingListResponse) -> Self {
        use chrono::{TimeZone, Utc};

        let (period_start, period_end) = if let Some(search_params) = response.search_params {
            let period_dates = search_params
                .d
                .split("|")
                .map(|s| s.replace("dfr=", "").replace("dto=", ""))
                .collect::<Vec<String>>();
            (
                period_dates.first().unwrap_or(&String::new()).to_string(),
                period_dates.get(1).unwrap_or(&String::new()).to_string(),
            )
        } else {
            (String::new(), String::new())
        };

        // Create a vector of emails with dates filled in from the epoch
        let emails_with_dates: Vec<Email> = response
            .emails
            .into_iter()
            .map(|mut email| {
                // Always convert epoch to date string, whether date is None or not
                if email.epoch > 0 {
                    // Convert the epoch to a formatted date string
                    if let Some(dt) = Utc.timestamp_opt(email.epoch, 0).single() {
                        let formatted_date = dt.format("%Y-%m-%d").to_string();
                        // Log both the original date and our calculated date
                        log::trace!(
                            "Email date: original={:?}, from epoch={}, calculated={}",
                            email.date,
                            email.epoch,
                            formatted_date
                        );
                        // Always set the date field from epoch for consistency
                        email.date = Some(formatted_date);
                    } else {
                        log::debug!("Failed to convert epoch {} to date", email.epoch);
                    }
                } else {
                    // If epoch is not available and date is None, set a placeholder
                    if email.date.is_none() {
                        email.date = Some("Unknown date".to_string());
                        log::debug!("No epoch or date available, setting placeholder");
                    }
                }

                email
            })
            .collect();

        MailingListStats {
            total_emails: response.hits,
            total_participants: response.participants.len(),
            total_threads: response.no_threads,
            period_start,
            period_end,
            list_name: response.list,
            domain: response.domain,
            emails: emails_with_dates,
            thread_struct: response.thread_struct,
            active_months: response.active_months,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api::client::{MailingListResponse, SearchParams};
    use crate::models::email::Email;
    use crate::models::stats::MailingListStats;
    use crate::models::thread::{Participant, ThreadStructValue};
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
            thread_struct: ThreadStructValue::Array(vec![]), // Empty array is fine to recreate
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
