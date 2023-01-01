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

    pub async fn vndbstats(&self) -> Result<VndbStats, reqwest::Error> {
        self.client.get(GET_STATS_URL).send().await?.json().await
    }

    pub async fn users_stats(&self, id_or_name: &[&str]) -> Result<UserStats, reqwest::Error> {
        self.client
            .get(GET_USER_URL)
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

    pub async fn authinfo(&self) -> Result<AuthInfo, Error> {
        let token = self.token()?;
        Ok(self
            .client
            .get(GET_AUTHINFO_URL)
            .header("Authorization", format!("token {}", token))
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn ulist_labels(&self, user: Option<&str>) -> Result<UlistItems, Error> {
        let req = self.client.get(GET_ULIST_URL).query(&[("fields", "count")]);
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
