use crate::terra_datetime_format;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct NodeIDIPPort {
    pub id: String,
    pub ip: String,
    pub port: usize,
}
impl ToString for NodeIDIPPort {
    fn to_string(&self) -> String {
        format!("{}@{}:{}", self.id, self.ip, self.port)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NodeAddr {
    pub addr: NodeIDIPPort,
    pub src: NodeIDIPPort,
    pub buckets: Vec<usize>,
    pub attempts: usize,
    #[serde(with = "terra_datetime_format")]
    pub last_attempt: DateTime<Utc>,
    #[serde(with = "terra_datetime_format")]
    pub last_success: DateTime<Utc>,
    #[serde(with = "terra_datetime_format")]
    pub last_ban_time: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AddressBook {
    pub key: String,
    pub addrs: Vec<NodeAddr>,
}
