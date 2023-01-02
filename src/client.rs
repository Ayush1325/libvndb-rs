use serde::de::DeserializeOwned;

use crate::{
    get_data::{AuthInfo, UlistItems, UserStats, VndbStats},
    post_data::{QueryFormat, ResponseFormat},
    urls::{GET_AUTHINFO_URL, GET_STATS_URL, GET_ULIST_URL, GET_USER_URL},
};

pub struct Client {
    client: reqwest::Client,
    token: Option<String>,
}

impl Client {
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

    pub async fn vndbstats(&self) -> Result<VndbStats, Error> {
        self.get_request(GET_STATS_URL, &[], false).await
    }

    pub async fn users_stats(&self, id_or_name: &[&str]) -> Result<UserStats, Error> {
        let mut query = id_or_name
            .iter()
            .map(|s| ("q", *s))
            .collect::<Vec<(&str, &str)>>();
        query.push(("fields", "lengthvotes,lengthvotes_sum"));
        self.get_request(GET_USER_URL, &query, false).await
    }

    pub async fn authinfo(&self) -> Result<AuthInfo, Error> {
        self.get_request(GET_AUTHINFO_URL, &[], true).await
    }

    pub async fn ulist_labels(&self, user: Option<&str>) -> Result<UlistItems, Error> {
        let mut query = vec![("fields", "count")];
        if let Some(x) = user {
            query.push(("user", x))
        }
        self.get_request(GET_ULIST_URL, &query, user.is_none())
            .await
    }

    pub async fn get_request<'a, T>(
        &self,
        url: &str,
        query: &[(&str, &str)],
        token_required: bool,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let req = self.client.get(url).query(query);
        let req = match self.token() {
            Ok(x) => req.header("Authorization", format!("token {}", x)),
            Err(e) => {
                if token_required {
                    return Err(e);
                } else {
                    req
                }
            }
        };
        Ok(req.send().await?.json().await?)
    }

    pub async fn post_request<'a>(
        &'a self,
        url: &'a str,
        body: &'a QueryFormat<'a>,
        token_required: bool,
    ) -> Result<ResponseFormat, Error> {
        let req = self.client.post(url).json(&body);
        let req = match self.token() {
            Ok(token) => req.header("Authorization", format!("token {}", token)),
            Err(e) => {
                if token_required {
                    return Err(e);
                } else {
                    req
                }
            }
        };
        Ok(req.send().await?.json().await?)
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
