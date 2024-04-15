use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;



#[test]
fn test_get_rustaceans(){
    // setup
    let client = Client::new();
    let new_rustacean1 = common::create_test_rustance(&client);
    let new_rustacean2 = common::create_test_rustance(&client);

    // Test
    let response = client.get(format!("{}/rustaceans",common::APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&new_rustacean1));
    assert!(json.as_array().unwrap().contains(&new_rustacean2));

    // cleanup
    common::delete_test_rustance(&client,new_rustacean1);
    common::delete_test_rustance(&client,new_rustacean2);

}

#[test]
fn test_creat_rustacean(){
    // setup
    let client = Client::new();
    let rustaceans_response: Value = common::create_test_rustance(&client);
    assert_eq!(rustaceans_response, json!({
        "id": rustaceans_response["id"],
        "name":"muzman01",
        "email":"deneme@deneme.com",
        "created_at":rustaceans_response["created_at"]
    }));

    common::delete_test_rustance(&client,rustaceans_response);

}

#[test]
fn test_get_one_rustacean(){
    let client = Client::new();
    let rustaceans_response: Value = common::create_test_rustance(&client);
    let response = client.get(format!("{}/rustaceans/{}",common::APP_HOST,rustaceans_response["id"])).send().unwrap();
    let rustaceans_response: Value = response.json().unwrap();
    assert_eq!(rustaceans_response, json!({
        "id": rustaceans_response["id"],
        "name":"muzman01",
        "email":"deneme@deneme.com",
        "created_at":rustaceans_response["created_at"]
    }));

    

}
#[test]
fn test_update_rustacean(){
    let client = Client::new();
    let rustaceans_response: Value = common::create_test_rustance(&client);
    let response = client.put(format!("{}/rustaceans/{}",common::APP_HOST,rustaceans_response["id"])).json(&json!({
        "name":"muzman2",
        "email":"naber@naber.com"
    })).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let rustaceans_response: Value = response.json().unwrap();
    assert_eq!(rustaceans_response, json!({
        "id": rustaceans_response["id"],
        "name":"muzman2",
        "email":"naber@naber.com",
        "created_at":rustaceans_response["created_at"]
    }));
   common::delete_test_rustance(&client,rustaceans_response);

}

#[test]
fn test_delete_rustacean(){
    let client = Client::new();
    let rustaceans_response: Value = common::create_test_rustance(&client);
    common::delete_test_rustance(&client,rustaceans_response);

   
}

// #[test]
// fn test_update_rustacean_multiple_times() {
//     let client = Client::new();

//     for _ in 0..20000000 {
//         let response = client.post("http://localhost:8000/rustaceans").json(&json!({
//             "name": "muzman01",
//             "email": "deneme@deneme.com"
//         })).send().unwrap();
//         assert_eq!(response.status(), StatusCode::CREATED);
//         let rustaceans_response: Value = response.json().unwrap();

//         let response = client.put(format!("http://localhost:8000/rustaceans/{}", rustaceans_response["id"])).json(&json!({
//             "name": "muzman2",
//             "email": "naber@naber.com"
//         })).send().unwrap();
//         assert_eq!(response.status(), StatusCode::OK);
//         let updated_rustaceans_response: Value = response.json().unwrap();

//         assert_eq!(updated_rustaceans_response, json!({
//             "id": rustaceans_response["id"],
//             "name": "muzman2",
//             "email": "naber@naber.com",
//             "created_at": rustaceans_response["created_at"]
//         }));
//     }
// }
