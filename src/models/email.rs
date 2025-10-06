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
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Email {
    #[serde(rename = "in-reply-to")]
    pub in_reply_to: String,
    #[serde(default)]
    pub private: bool,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
    #[serde(default)]
    pub subject: String,
    #[serde(default)]
    pub mid: String,
    #[serde(default)]
    pub epoch: i64,
    #[serde(default)]
    pub list: String,
    #[serde(default)]
    pub gravatar: String,
    #[serde(rename = "message-id", default)]
    pub message_id: String,
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub list_raw: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub date: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Attachment {
    #[serde(default)]
    pub filename: String,
    #[serde(default)]
    pub content_type: String,
    #[serde(default)]
    pub size:  i64,
    #[serde(default)]
    pub hash: String,
}

#[cfg(test)]
mod tests {
    use crate::models::email::{Email, Attachment};

    #[test]
    fn test_email_all_fields() {
        let attachment = Attachment {
            filename: "test.txt".to_string(),
            content_type: "text/plain".to_string(),
            size: 1024,
            hash: "abc123".to_string(),
        };

        let email = Email {
            message_id: "<12345@example.com>".to_string(),
            subject: "Test Subject".to_string(),
            in_reply_to: "<67890@example.com>".to_string(),
            from: "sender@example.com".to_string(),
            date: Some("2025-01-01".to_string()),
            private: true,
            attachments: vec![attachment.clone()],
            epoch: 1609459200, // 2021-01-01 00:00:00 UTC
            list: "test-list".to_string(),
            gravatar: "gravatar-hash".to_string(),
            list_raw: "test-list@example.com".to_string(),
            id: "email-id-123".to_string(),
            body: "This is the email body".to_string(),
            mid: "mid-123".to_string(),
        };

        // Test all fields to ensure they're used
        assert_eq!(email.message_id, "<12345@example.com>");
        assert_eq!(email.subject, "Test Subject");
        assert_eq!(email.in_reply_to, "<67890@example.com>");
        assert_eq!(email.from, "sender@example.com");
        assert_eq!(email.date, Some("2025-01-01".to_string()));
        assert_eq!(email.private, true);
        assert_eq!(email.attachments.len(), 1);
        assert_eq!(email.attachments[0].filename, "test.txt");
        assert_eq!(email.attachments[0].content_type, "text/plain");
        assert_eq!(email.attachments[0].size, 1024);
        assert_eq!(email.attachments[0].hash, "abc123");
        assert_eq!(email.epoch, 1609459200);
        assert_eq!(email.list, "test-list");
        assert_eq!(email.gravatar, "gravatar-hash");
        assert_eq!(email.list_raw, "test-list@example.com");
        assert_eq!(email.id, "email-id-123");
        assert_eq!(email.body, "This is the email body");
        assert_eq!(email.mid, "mid-123");
    }
}
