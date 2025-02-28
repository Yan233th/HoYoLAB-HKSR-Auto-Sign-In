use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue, COOKIE},
};
use serde_json::{json, Value};
use std::env;

pub fn get_tokens() -> Vec<String> {
    let tokens: Vec<String> = match env::var("USER_TOKENS") {
        Ok(val) => val.split('|').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect(),
        Err(_) => panic!("Unable to find USER_TOKENS environment variable!"),
    };
    if tokens.is_empty() {
        panic!("No user tokens found!");
    } else {
        println!("Got {} user tokens successfully!", tokens.len());
    }
    return tokens;
}

pub fn generate_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("Accept", HeaderValue::from_static("application/json, text/plain, */*"));
    headers.insert("Accept-Encoding", HeaderValue::from_static("gzip, deflate, br"));
    headers.insert("Connection", HeaderValue::from_static("keep-alive"));
    headers.insert("x-rpc-client_type", HeaderValue::from_static("4"));
    headers.insert("x-rpc-app_version", HeaderValue::from_static("2.34.1"));
    headers.insert(
        "User-Agent",
        HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36 Edg/131.0.0.0"),
    );
    headers.insert("Referer", HeaderValue::from_static("https://act.hoyolab.com/"));
    headers.insert("Origin", HeaderValue::from_static("https://act.hoyolab.com"));
    return headers;
}

pub fn do_sign(client: &Client, mut headers: HeaderMap, token: String) {
    headers.insert(COOKIE, HeaderValue::from_str(&token).unwrap());
    let response: Value = client
        .post("https://sg-public-api.hoyolab.com/event/luna/hkrpg/os/sign")
        .headers(headers)
        .json(&json!({"act_id": "e202303301540311", "lang": "zh-cn"}))
        .send()
        .unwrap()
        .json()
        .unwrap();
    let is_risk = match response.get("data") {
        Some(data) => {
            let gt_risk = data.get("gt_result").and_then(|gt| gt.get("is_risk")).and_then(|v| v.as_bool()).unwrap_or(false);
            let direct_risk = data.get("is_risk").and_then(|v| v.as_bool()).unwrap_or(false);
            gt_risk || direct_risk
        }
        None => false,
    };
    if is_risk {
        eprintln!("The sign-in failed due to triggering risk control.");
    } else if response["message"] != "OK" {
        eprintln!("Sign-in failed for some reason.");
        eprintln!("{:?}", response);
    } else {
        println!("Sign-in completed successfully.");
    }
}
