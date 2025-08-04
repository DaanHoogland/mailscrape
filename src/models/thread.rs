use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ThreadStruct {
    pub children: Vec<ThreadStruct>,
    pub tid: String,
    pub subject: String,
    pub tsubject: String,
    pub epoch: i64,
    pub nest: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Participant {
    pub email: String,
    pub name: String,
    pub count: i32,
    pub gravatar: String,
}
