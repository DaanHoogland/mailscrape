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
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ThreadStructValue {
    Map(HashMap<String, ThreadStruct>),
    Array(Vec<ThreadStruct>),
}

impl Default for ThreadStructValue {
    fn default() -> Self {
        ThreadStructValue::Array(Vec::new())
    }
}

impl ThreadStructValue {
    pub fn iter(&self) -> Box<dyn Iterator<Item = &ThreadStruct> + '_> {
        match self {
            ThreadStructValue::Map(map) => Box::new(map.values()),
            ThreadStructValue::Array(vec) => Box::new(vec.iter()),
        }
    }

    pub fn into_vec(self) -> Vec<ThreadStruct> {
        match self {
            ThreadStructValue::Map(map) => map.into_values().collect(),
            ThreadStructValue::Array(vec) => vec,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            ThreadStructValue::Map(map) => map.len(),
            ThreadStructValue::Array(vec) => vec.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            ThreadStructValue::Array(arr) => arr.is_empty(),
            ThreadStructValue::Map(map) => map.is_empty(),
        }
    }

    pub fn from_vec(vec: Vec<ThreadStruct>) -> Self {
        ThreadStructValue::Array(vec)
    }
}

impl From<Vec<ThreadStruct>> for ThreadStructValue {
    fn from(vec: Vec<ThreadStruct>) -> Self {
        ThreadStructValue::Array(vec)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ThreadStruct {
    #[serde(default)]
    pub children: Vec<ThreadStruct>,
    #[serde(default)]
    pub tid: String,
    #[serde(default)]
    pub subject: String,
    #[serde(default)]
    pub tsubject: String,
    #[serde(default)]
    pub epoch: i64,
    #[serde(default)]
    pub nest: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Participant {
    pub email: String,
    pub name: String,
    pub count: i32,
    pub gravatar: String,
}

#[cfg(test)]
mod tests {
    use super::*; // This imports everything from the parent module

    #[test]
    fn test_thread_struct_all_fields() {
        let child = ThreadStruct {
            tid: "child-123".to_string(),
            subject: "Child Subject".to_string(),
            tsubject: "Child Thread Subject".to_string(),
            epoch: 1609459300,
            nest: 1,
            children: Vec::new(),
        };

        let thread = ThreadStruct {
            tid: "parent-123".to_string(),
            subject: "Parent Subject".to_string(),
            tsubject: "Parent Thread Subject".to_string(),
            epoch: 1609459200,
            nest: 0,
            children: vec![child.clone()],
        };

        // Test all fields to ensure they're used
        assert_eq!(thread.tid, "parent-123");
        assert_eq!(thread.subject, "Parent Subject");
        assert_eq!(thread.tsubject, "Parent Thread Subject");
        assert_eq!(thread.epoch, 1609459200);
        assert_eq!(thread.nest, 0);
        assert_eq!(thread.children.len(), 1);
        assert_eq!(thread.children[0].tid, "child-123");
    }

    #[test]
    fn test_thread_struct_value_array() {
        let thread1 = ThreadStruct {
            tid: "thread1".to_string(),
            subject: "Thread 1".to_string(),
            tsubject: "Thread 1 Subject".to_string(),
            epoch: 1609459200,
            nest: 0,
            children: Vec::new(),
        };

        let thread2 = ThreadStruct {
            tid: "thread2".to_string(),
            subject: "Thread 2".to_string(),
            tsubject: "Thread 2 Subject".to_string(),
            epoch: 1609459300,
            nest: 0,
            children: Vec::new(),
        };

        let thread_value = ThreadStructValue::Array(vec![thread1.clone(), thread2.clone()]);

        // Test methods
        assert_eq!(thread_value.len(), 2);
        let threads: Vec<&ThreadStruct> = thread_value.iter().collect();
        assert_eq!(threads.len(), 2);
        assert_eq!(threads[0].tid, "thread1");
        assert_eq!(threads[1].tid, "thread2");

        let thread_vec = thread_value.into_vec();
        assert_eq!(thread_vec.len(), 2);
        assert_eq!(thread_vec[0].tid, "thread1");
        assert_eq!(thread_vec[1].tid, "thread2");
    }

    #[test]
    fn test_thread_struct_value_map() {
        let thread1 = ThreadStruct {
            tid: "thread1".to_string(),
            subject: "Thread 1".to_string(),
            tsubject: "Thread 1 Subject".to_string(),
            epoch: 1609459200,
            nest: 0,
            children: Vec::new(),
        };

        let thread2 = ThreadStruct {
            tid: "thread2".to_string(),
            subject: "Thread 2".to_string(),
            tsubject: "Thread 2 Subject".to_string(),
            epoch: 1609459300,
            nest: 0,
            children: Vec::new(),
        };

        let mut map = HashMap::new();
        map.insert("key1".to_string(), thread1.clone());
        map.insert("key2".to_string(), thread2.clone());

        let thread_value = ThreadStructValue::Map(map);

        // Test methods
        assert_eq!(thread_value.len(), 2);
        let threads: Vec<&ThreadStruct> = thread_value.iter().collect();
        assert_eq!(threads.len(), 2);
        assert!(threads.iter().any(|t| t.tid == "thread1"));
        assert!(threads.iter().any(|t| t.tid == "thread2"));

        let thread_vec = thread_value.into_vec();
        assert_eq!(thread_vec.len(), 2);
        assert!(thread_vec.iter().any(|t| t.tid == "thread1"));
        assert!(thread_vec.iter().any(|t| t.tid == "thread2"));
    }

    #[test]
    fn test_participant() {
        let participant = Participant {
            email: "user@example.com".to_string(),
            name: "Test User".to_string(),
            count: 5,
            gravatar: "gravatar-hash".to_string(),
        };

        assert_eq!(participant.email, "user@example.com");
        assert_eq!(participant.name, "Test User");
        assert_eq!(participant.count, 5);
        assert_eq!(participant.gravatar, "gravatar-hash");
    }

    #[test]
    fn test_thread_struct_value_is_empty() {
        // Create an empty Array variant
        let empty_array = ThreadStructValue::Array(vec![]);
        assert_eq!(empty_array.len(), 0);
        assert!(empty_array.is_empty());

        // Create a non-empty Array variant
        let thread = ThreadStruct {
            tid: "thread-123".to_string(),
            subject: "Thread Subject".to_string(),
            tsubject: "Thread Subject".to_string(),
            epoch: 1609459200,
            nest: 0,
            children: Vec::new(),
        };
        let non_empty_array = ThreadStructValue::Array(vec![thread]);
        assert_eq!(non_empty_array.len(), 1);
        assert!(!non_empty_array.is_empty());

        // Create an empty Map variant
        let empty_map = ThreadStructValue::Map(HashMap::new());
        assert_eq!(empty_map.len(), 0);
        assert!(empty_map.is_empty());

        // Create a non-empty Map variant
        let mut map = HashMap::new();
        map.insert(
            "key".to_string(),
            ThreadStruct {
                tid: "thread-456".to_string(),
                subject: "Another Thread".to_string(),
                tsubject: "Another Thread".to_string(),
                epoch: 1609459200,
                nest: 0,
                children: Vec::new(),
            },
        );
        let non_empty_map = ThreadStructValue::Map(map);
        assert_eq!(non_empty_map.len(), 1);
        assert!(!non_empty_map.is_empty());
    }
}
