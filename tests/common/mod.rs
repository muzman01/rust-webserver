use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub static APP_HOST: &'static str = "http://127.0.0.1:8000";


pub fn create_test_rustance(client: &Client) -> Value {
    let response = client.post(format!("{}/rustaceans",APP_HOST)).json(&json!({
        "name":"muzman01",
        "email":"deneme@deneme.com"
    })).send().unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    response.json().unwrap()
}

pub fn delete_test_rustance(client: &Client, rustaceans_response: Value){
    let response = client.delete(format!("{}/rustaceans/{}",APP_HOST,rustaceans_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

pub fn create_test_crate(client: &Client, rustaceans_response: &Value) -> Value {
    let response = client.post(format!("{}/creates",APP_HOST)).json(&json!({
        "rustacean_id":rustaceans_response["id"],
        "code":"deneme",
        "name": rustaceans_response["name"],
        "version":"0.1.0",
        "description":"deneme deneme deneme"
    })).send().unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    response.json().unwrap()
}

pub fn delete_test_crate(client: &Client, crates_response: Value){
    let response = client.delete(format!("{}/creates/{}",APP_HOST,crates_response["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}