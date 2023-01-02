use libvndb::{client::Client, post_data::QueryFormat, urls};

#[tokio::test]
/// Defined at: https://api.vndb.org/kana#get-stats
async fn stats() {
    let client = Client::simple();
    let res = client.vndbstats().await;
    assert!(res.is_ok());
}

#[tokio::test]
/// Defined at: https://api.vndb.org/kana#get-user
async fn user() {
    let client = Client::simple();
    let res = client
        .users_stats(&["NoUserWithThisNameExists", "AYO", "u3"])
        .await;
    assert!(res.is_ok());

    let res = res.unwrap();

    assert_eq!(res.len(), 3);

    assert!(res.contains_key("NoUserWithThisNameExists"));
    assert!(res["NoUserWithThisNameExists"].is_none());

    assert!(res.contains_key("AYO"));
    assert!(res["AYO"].is_some());

    assert!(res.contains_key("u3"));
    assert!(res["u3"].is_some());
}

#[tokio::test]
/// Defined at: https://api.vndb.org/kana#get-authinfo
async fn authinfo() {
    let client = Client::with_token("cdhy-bqy1q-6zobu-8w9k-xobxh-wzz4o-84fn".to_string());
    let res = client.authinfo().await;
    assert!(res.is_err());
}

#[tokio::test]
/// Defined at: https://api.vndb.org/kana#get-ulist_labels
async fn ulist_labels() {
    let client = Client::simple();
    let res = client.ulist_labels(Some("u1")).await;
    assert!(res.is_ok());
}

#[tokio::test]
/// Defined at: https://api.vndb.org/kana#post-vn
async fn vn() {
    let client = Client::simple();
    let body = QueryFormat::builder()
        .fields("title, image.url")
        .filters(vec!["id".into(), "=".into(), "v17".into()])
        .build();
    let res = client.post_request(urls::POST_VN_URL, &body, false).await;
    assert!(res.is_ok());
}

#[tokio::test]
/// Defined at: https://api.vndb.org/kana#post-ulist
async fn ulist() {
    let client = Client::simple();
    let body = QueryFormat::builder()
        .user("u2")
        .fields("id, vote, vn.title")
        .filters(["label", "=", "7"].into_iter().map(Into::into).collect())
        .sort("vote")
        .reverse()
        .results(10)
        .build();
    let res = client
        .post_request(urls::POST_ULIST_URL, &body, false)
        .await;
    assert!(res.is_ok());
}
