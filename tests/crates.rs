use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_cerate_crate(){
    // setup
    let client = Client::new();
    let new_rustance = common::create_test_rustance(&client);

    // Test
    let response = common::create_test_crate(&client,&new_rustance);
    let crate_response: Value = response;
    assert_eq!(crate_response, json!({
        "id": crate_response["id"],
        "rustacean_id":new_rustance["id"],
        "code":"deneme",
        "name": new_rustance["name"],
        "version":"0.1.0",
        "description":"deneme deneme deneme",
        "created_at":crate_response["created_at"]
    }));

    // cleanup
    common::delete_test_crate(&client,crate_response);
    common::delete_test_rustance(&client,new_rustance);

}

#[test]
fn test_get_crates(){
    // setup
    let client = Client::new();
    let new_rustance = common::create_test_rustance(&client);
    let new_crates = common::create_test_crate(&client,&new_rustance);
    let test_crates1 = common::create_test_crate(&client,&new_rustance);
    let test_crates2 = common::create_test_crate(&client,&new_rustance);

    // Test
    let response = client.get(format!("{}/creates",common::APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&test_crates1));
    assert!(json.as_array().unwrap().contains(&test_crates2));
    // cleanup
    common::delete_test_crate(&client,new_crates);
    common::delete_test_crate(&client,test_crates1);
    common::delete_test_crate(&client,test_crates2);
    common::delete_test_rustance(&client,new_rustance);

}

#[test]
fn test_get_one_crates(){
    // setup
    let client = Client::new();
    let new_rustance = common::create_test_rustance(&client);
    let new_crates = common::create_test_crate(&client,&new_rustance);

    // Test
    let response = client.get(format!("{}/creates/{}",common::APP_HOST,new_crates["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    // cleanup
    common::delete_test_crate(&client,new_crates);
    common::delete_test_rustance(&client,new_rustance);

}

#[test]
fn test_update_crates(){
    // setup
    let client = Client::new();
    let new_rustance = common::create_test_rustance(&client);
    let new_crates = common::create_test_crate(&client,&new_rustance);

    // Test
    let response = client.put(format!("{}/creates/{}",common::APP_HOST,new_crates["id"])).json(&json!({
        "rustacean_id":new_rustance["id"],
        "code":"deneme",
        "name": new_rustance["name"],
        "version":"0.1.0",
        "description":"deneme deneme deneme"
    })).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let update_crates_response:Value = response.json().unwrap();
    assert_eq!(update_crates_response, json!({
        "id": update_crates_response["id"],
        "rustacean_id":new_rustance["id"],
        "code":"deneme",
        "name": new_rustance["name"],
        "version":"0.1.0",
        "description":"deneme deneme deneme",
        "created_at":update_crates_response["created_at"]
    }));
    
    // cleanup
    common::delete_test_crate(&client,new_crates);
    common::delete_test_rustance(&client,new_rustance);

}


#[test]
fn test_delete_crate(){
    // setup
    let client = Client::new();
    let new_rustance = common::create_test_rustance(&client);
    let new_crates = common::create_test_crate(&client,&new_rustance);

    // Test
    let response = client.delete(format!("{}/creates/{}",common::APP_HOST,new_crates["id"])).send().unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);    
    // cleanup
    common::delete_test_rustance(&client,new_rustance);

}