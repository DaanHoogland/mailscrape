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
