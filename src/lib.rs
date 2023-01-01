use std::collections::HashMap;

use const_format::concatcp;
use serde::{Deserialize, Serialize};

pub struct Vndb {
    client: reqwest::Client,
    token: Option<String>,
}

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
    pub lengthvotes: usize,
    pub lengthvotes_sum: usize,
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

const BASE_URL: &str = "https://api.vndb.org/kana";

pub const POST_ULIST_URL: &str = concatcp!(BASE_URL, "/ulist");
pub const POST_TRAIT_URL: &str = concatcp!(BASE_URL, "/trait");
pub const POST_TAG_URL: &str = concatcp!(BASE_URL, "/tag");
pub const POST_STAFF_URL: &str = concatcp!(BASE_URL, "/staff");
pub const POST_CHARACTER_URL: &str = concatcp!(BASE_URL, "/character");
pub const POST_PRODUCER_URL: &str = concatcp!(BASE_URL, "/producer");
pub const POST_RELEASE_URL: &str = concatcp!(BASE_URL, "/release");
pub const POST_VN_URL: &str = concatcp!(BASE_URL, "/vn");

impl Vndb {
    fn new(client: reqwest::Client, token: Option<String>) -> Self {
        Self { client, token }
    }

    fn token(&self) -> Result<&str, Error> {
        match &self.token {
            Some(x) => Ok(&x),
            None => Err(Error::TokenNotPresent),
        }
    }

    pub fn simple() -> Self {
        Self::new(reqwest::Client::new(), None)
    }

    pub fn with_token(token: String) -> Self {
        Self::new(reqwest::Client::new(), Some(token))
    }

    pub fn set_token(&mut self, token: String) {
        self.token = Some(token)
    }

    pub async fn vndbstats(&self) -> Result<VndbStats, reqwest::Error> {
        const STATS_URL: &str = concatcp!(BASE_URL, "/stats");
        self.client.get(STATS_URL).send().await?.json().await
    }

    pub async fn users_stats(&self, id_or_name: &[&str]) -> Result<UserStats, reqwest::Error> {
        const USER_STATS_URL: &str = concatcp!(BASE_URL, "/user");

        self.client
            .get(USER_STATS_URL)
            .query(
                &id_or_name
                    .iter()
                    .map(|s| ("q", *s))
                    .collect::<Vec<(&str, &str)>>(),
            )
            .query(&[("fields", "lengthvotes,lengthvotes_sum")])
            .send()
            .await?
            .json()
            .await
    }

    pub async fn user_stats(&self, id_or_name: &str) -> Result<Option<UserStat>, reqwest::Error> {
        Ok(self
            .users_stats(&[id_or_name])
            .await?
            .get(id_or_name)
            .unwrap()
            .clone())
    }

    pub async fn authinfo(&self) -> Result<AuthInfo, Error> {
        let token = self.token()?;
        const AUTHINFO_URL: &str = concatcp!(BASE_URL, "/authinfo");
        Ok(self
            .client
            .get(AUTHINFO_URL)
            .header("Authorization", format!("token {}", token))
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn ulist_labels(&self, user: Option<&str>) -> Result<UlistItems, Error> {
        const ULIST_URL: &str = concatcp!(BASE_URL, "/ulist_labels");
        let req = self.client.get(ULIST_URL).query(&[("fields", "count")]);
        let req = match user {
            Some(x) => req.query(&[("user", x)]),
            None => req.header("Authorization", format!("token {}", self.token()?)),
        };
        Ok(req.send().await?.json().await?)
    }

    pub async fn post_request<'a>(
        &'a self,
        url: &'a str,
        body: &'a QueryFormat<'a>,
    ) -> Result<ResponseFormat, reqwest::Error> {
        let req = self.client.post(url).json(&body);
        let req = match self.token() {
            Ok(token) => req.header("Authorization", format!("token {}", token)),
            Err(_) => req,
        };
        req.send().await?.json().await
    }
}

#[derive(Debug)]
pub enum Error {
    ReqwestError(reqwest::Error),
    TokenNotPresent,
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::ReqwestError(e)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryFormat<'a> {
    pub filters: FilterItems,
    pub fields: &'a str,
    pub sort: &'a str,
    pub reverse: bool,
    pub results: usize,
    pub page: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<&'a str>,
    pub count: bool,
    pub compact_filters: bool,
    pub normalized_filters: bool,
}

impl<'a> QueryFormat<'a> {
    pub fn builder() -> QueryFormatBuilder<'a> {
        QueryFormatBuilder::new()
    }
}

pub struct QueryFormatBuilder<'a> {
    pub filters: FilterItems,
    pub fields: &'a str,
    pub sort: &'a str,
    pub reverse: bool,
    pub results: usize,
    pub page: usize,
    pub user: Option<&'a str>,
    pub count: bool,
    pub compact_filters: bool,
    pub normalized_filters: bool,
}

impl<'a> QueryFormatBuilder<'a> {
    fn new() -> Self {
        Self {
            filters: Vec::new(),
            fields: "",
            sort: "id",
            reverse: false,
            results: 10,
            page: 1,
            user: None,
            count: false,
            compact_filters: false,
            normalized_filters: false,
        }
    }

    pub fn filters(mut self, val: FilterItems) -> Self {
        self.filters = val;
        self
    }

    pub fn fields(mut self, val: &'a str) -> Self {
        self.fields = val;
        self
    }

    pub fn sort(mut self, val: &'a str) -> Self {
        self.sort = val;
        self
    }

    pub fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }

    pub fn results(mut self, val: usize) -> Self {
        self.results = val;
        self
    }

    pub fn page(mut self, val: usize) -> Self {
        self.page = val;
        self
    }

    pub fn user(mut self, val: &'a str) -> Self {
        self.user = Some(val);
        self
    }

    pub fn count(mut self) -> Self {
        self.count = true;
        self
    }

    pub fn compact_filters(mut self) -> Self {
        self.compact_filters = true;
        self
    }

    pub fn normalized_filters(mut self) -> Self {
        self.normalized_filters = true;
        self
    }

    pub fn build(self) -> QueryFormat<'a> {
        QueryFormat {
            filters: self.filters,
            fields: self.fields,
            sort: self.sort,
            reverse: self.reverse,
            results: self.results,
            page: self.page,
            user: self.user,
            count: self.count,
            compact_filters: self.compact_filters,
            normalized_filters: self.normalized_filters,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseFormat {
    pub results: serde_json::Value,
    pub more: bool,
    pub count: Option<usize>,
    pub compact_filters: Option<String>,
    pub normalized_filters: Option<FilterItems>,
}

pub type FilterItems = Vec<FilterItem>;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum FilterItem {
    S(String),
    V(FilterItems),
}

impl From<FilterItems> for FilterItem {
    fn from(value: FilterItems) -> Self {
        Self::V(value)
    }
}

impl From<String> for FilterItem {
    fn from(value: String) -> Self {
        Self::S(value)
    }
}

impl From<&str> for FilterItem {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}

impl<T, const N: usize> From<[T; N]> for FilterItem
where
    T: Into<FilterItem> + Clone,
{
    fn from(value: [T; N]) -> Self {
        Self::from(value.as_slice())
    }
}

impl<T> From<&[T]> for FilterItem
where
    T: Into<FilterItem> + Clone,
{
    fn from(value: &[T]) -> Self {
        Self::from(
            value
                .iter()
                .cloned()
                .map(|x| x.into())
                .collect::<FilterItems>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::FilterItem;

    #[test]
    fn simple_string_vec() {
        let ans = FilterItem::V(Vec::from([
            FilterItem::S("abc".to_string()),
            FilterItem::S("=".to_string()),
        ]));
        let test_item = FilterItem::from(vec!["abc".into(), "=".into()]);
        assert_eq!(test_item, ans);
    }

    #[test]
    fn simple_string_array() {
        let ans = FilterItem::V(Vec::from([
            FilterItem::S("abc".to_string()),
            FilterItem::S("=".to_string()),
        ]));
        let test_item = FilterItem::from(["abc", "="]);
        assert_eq!(test_item, ans);
    }

    #[test]
    fn simple_string_slice() {
        let ans = FilterItem::V(Vec::from([
            FilterItem::S("abc".to_string()),
            FilterItem::S("=".to_string()),
        ]));
        let test_item = FilterItem::from(&["abc", "="][..]);
        assert_eq!(test_item, ans);
    }
}

#[macro_export]
macro_rules! filter_items {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_filter_item: $crate::FilterItems = Vec::new();
            $(
                temp_filter_item.push($crate::FilterItem::from($x));
            )*
            temp_filter_item
        }
    };
}
