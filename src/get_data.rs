use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VndbStats {
    pub releases: usize,
    pub producers: usize,
    pub vn: usize,
    pub tags: usize,
    pub staff: usize,
    pub traits: usize,
    pub chars: usize,
}

pub type UserStats = HashMap<String, Option<UserStat>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserStat {
    pub id: String,
    pub username: String,
    pub lengthvotes: Option<usize>,
    pub lengthvotes_sum: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthInfo {
    pub id: String,
    pub username: String,
    pub permissions: Vec<String>,
}

pub type UlistItems = HashMap<String, Vec<UlistItem>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UlistItem {
    pub id: isize,
    pub private: bool,
    pub count: usize,
    pub label: String,
}
