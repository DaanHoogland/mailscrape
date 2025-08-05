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
