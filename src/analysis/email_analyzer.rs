use std::collections::HashSet;
use crate::models::Email;
use crate::models::thread::ThreadStruct;

pub fn find_unanswered_emails<'a>(emails: &'a [Email], threads: &[ThreadStruct]) -> Vec<&'a Email> {
    // Find thread IDs that have no children
    let thread_ids_without_replies = find_threads_without_replies(threads);

    // Map these thread IDs back to emails
    emails
        .iter()
        .filter(|email| {
            thread_ids_without_replies.contains(&email.mid)
        })
        .collect()
}

fn find_threads_without_replies(threads: &[ThreadStruct]) -> HashSet<String> {
    let mut unanswered_tids = HashSet::new();

    for thread in threads {
        collect_leaf_threads(thread, &mut unanswered_tids);
    }

    unanswered_tids
}

fn collect_leaf_threads(thread: &ThreadStruct, leaf_tids: &mut HashSet<String>) {
    if thread.children.is_empty() {
        // If this is the root thread (nest == 0) and has no children, it's unanswered
        if thread.nest == 0 {
            leaf_tids.insert(thread.tid.clone());
        }
    } else {
        // Recursively process children
        for child in &thread.children {
            collect_leaf_threads(child, leaf_tids);
        }
    }
}
