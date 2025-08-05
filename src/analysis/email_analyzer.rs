use std::collections::HashSet;
use crate::models::Email;

pub fn find_unanswered_emails<'a>(emails: &'a [Email]) -> Vec<&'a Email> {
    let mut emails_with_replies = HashSet::new();
    log::debug!("Processing {} emails for threading", emails.len());

    let mut reply_emails = HashSet::new();
    for email in emails {
        if !email.in_reply_to.is_empty() || email.subject.starts_with("Re: ") {
            let email_id = email.message_id.trim_start_matches('<').trim_end_matches('>');
            reply_emails.insert(email_id.to_string());

            if !email.in_reply_to.is_empty() {
                let parent_id = email.in_reply_to.trim_start_matches('<').trim_end_matches('>');
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
            let parent_id = email.in_reply_to.trim_start_matches('<').trim_end_matches('>');
            emails_with_replies.insert(parent_id.to_string());
            log::debug!("Found reply to: {}", parent_id);
        }
    }
    log::debug!(
        "Found {} emails that have received replies",
        emails_with_replies.len()
    );

    let unanswered = emails
        .iter()
        .filter(|email| {
            let message_id = email.message_id.trim_start_matches('<').trim_end_matches('>');

            // An email is considered unanswered if:
            // 1. It's not a reply itself (doesn't start with Re: and has no in_reply_to)
            // 2. No other email has replied to it
            let is_not_reply = !email.subject.starts_with("Re: ") && email.in_reply_to.is_empty();
            let has_no_replies = !emails_with_replies.contains(message_id);

            let is_unanswered = is_not_reply && has_no_replies;

            if is_unanswered {
                log::debug!("Found unanswered email: '{}' with message-id: {}",
                    email.subject, message_id);
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
    use super::*;

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
    fn test_single_unanswered_email() {
        let emails = vec![
            create_test_email(
                "msg1",
                "Test Subject",
                ""
            ),
        ];

        let unanswered = find_unanswered_emails(&emails);
        assert_eq!(unanswered.len(), 1);
        assert_eq!(unanswered[0].subject, "Test Subject");
    }
}
