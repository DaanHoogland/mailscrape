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
use crate::models::Email;
use std::collections::HashSet;

pub fn find_unanswered_emails(emails: &[Email]) -> Vec<&Email> {
    let mut emails_with_replies = HashSet::new();
    log::debug!("Processing {} emails for threading", emails.len());

    let mut reply_emails = HashSet::new();
    for email in emails {
        if email.subject.starts_with("Re: ") {
            let email_id = email
                .message_id
                .trim_start_matches('<')
                .trim_end_matches('>');
            reply_emails.insert(email_id.to_string());

            if !email.in_reply_to.is_empty() {
                let parent_id = email
                    .in_reply_to
                    .trim_start_matches('<')
                    .trim_end_matches('>');
                emails_with_replies.insert(parent_id.to_string());
            }
        }
    }
    log::debug!(
        "Found {} emails that have received replies and {} reply emails",
        emails_with_replies.len(),
        reply_emails.len()
    );

    for email in emails {
        if !email.in_reply_to.is_empty() {
            let parent_id = email
                .in_reply_to
                .trim_start_matches('<')
                .trim_end_matches('>');
            emails_with_replies.insert(parent_id.to_string());
            log::debug!(
                "Found that {} / {} / {} may be in reply to {}; subject: {}",
                email.id,
                email.mid,
                email.message_id,
                parent_id,
                email.subject);
        }
    }
    log::debug!(
        "Found {} emails that have received replies.",
        emails_with_replies.len()
    );

    let unanswered = emails
        .iter()
        .filter(|email| {
            let message_id = email
                .message_id
                .trim_start_matches('<')
                .trim_end_matches('>');

            // An email is considered unanswered if:
            // 1. It's not a reply itself (doesn't start with Re: and has no in_reply_to)
            // 2. No other email has replied to it
            let is_not_reply = !email.subject.starts_with("Re: "); // && we are not sure about email.in_reply_to.is_empty();
            let has_no_replies = !emails_with_replies.contains(message_id);

            let is_unanswered = is_not_reply && has_no_replies;

            if is_unanswered {
                log::debug!(
                    "Found unanswered email: '{}' with message-id: {}",
                    email.subject,
                    message_id
                );
            }

            is_unanswered
        })
        .collect::<Vec<_>>();
    log::debug!(
        "Identified {} unanswered emails out of {} total",
        unanswered.len(),
        emails.len()
    );

    unanswered
}

#[cfg(test)]
mod tests {
    use crate::models::email::Email;

    fn create_test_email(message_id: &str, subject: &str, in_reply_to: &str) -> Email {
        Email {
            message_id: message_id.to_string(),
            subject: subject.to_string(),
            in_reply_to: in_reply_to.to_string(),
            from: "test@example.com".to_string(),
            date: Option::from("2025-08-04".to_string()),
            private: false,
            attachments: vec![],
            epoch: 0,
            list: String::new(),
            gravatar: String::new(),
            list_raw: String::new(),
            id: String::new(),
            body: String::new(),
            mid: message_id.to_string(),
        }
    }

    #[test]
    fn test_create_test_email_helper() {
        // Test that the helper function creates proper email objects
        let email = create_test_email("<msg1@example.com>", "Test Subject", "<parent@example.com>");

        // Verify the email was created correctly
        assert_eq!(email.message_id, "<msg1@example.com>");
        assert_eq!(email.subject, "Test Subject");
        assert_eq!(email.in_reply_to, "<parent@example.com>");
        assert_eq!(email.from, "test@example.com");
        assert_eq!(email.date, Some("2025-08-04".to_string()));
        assert!(!email.private);
        assert_eq!(email.attachments.len(), 0);
        assert_eq!(email.mid, "<msg1@example.com>");

        // Create another email with different values
        let email2 = create_test_email("<msg2@example.com>", "Another Subject", "");

        // Verify it has the correct values
        assert_eq!(email2.message_id, "<msg2@example.com>");
        assert_eq!(email2.subject, "Another Subject");
        assert_eq!(email2.in_reply_to, "");
        assert_eq!(email2.mid, "<msg2@example.com>");
    }
}
