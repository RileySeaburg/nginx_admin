use rocket::serde::{json::Json, Deserialize, Serialize};

use crate::controllers::nginx::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct NginxResponse {
    pub message: String,
    pub error: Option<String>,
    pub status: Option<i32>,
}

// reload nginx - sends a JSON response to the dashboard from the controller
#[get("/reload_nginx")]
pub fn reload_nginx_route() -> Json<NginxResponse> {
    // parse the output into three variables
    let output = reload_nginx().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e);
    });
    println!("status: {:?}", output);
    let response = String::from_utf8_lossy(&output.stdout).to_string();
    let error = String::from_utf8_lossy(&output.stderr).to_string();
    let status = output.status.code().unwrap_or_else(|| {
        panic!("failed to execute process: {}", error);
    });
    // return the JSON response
    Json(NginxResponse {
        message: response,
        error: Some(error),
        status: Some(status),
    })
}

// stop nginx - sends a JSON response to the dashboard from the controller
#[get("/stop_nginx")]
pub fn stop_nginx_route() -> Json<NginxResponse> {
    // parse the output into three variables
    let output = stop_nginx().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e);
    });
    println!("status: {:?}", output);
    let response = String::from_utf8_lossy(&output.stdout).to_string();
    let error = String::from_utf8_lossy(&output.stderr).to_string();
    let status = output.status.code().unwrap_or_else(|| {
        panic!("failed to execute process: {}", error);
    });
    // return the JSON response
    Json(NginxResponse {
        message: response,
        error: Some(error),
        status: Some(status),
    })
}

// start nginx - sends a JSON response to the dashboard from the controller


#[get("/start_nginx")]
#[no_mangle]
pub extern fn start_nginx_route() -> Json<NginxResponse> {
    // parse the output into three variables
    let output = start_nginx().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e);
    });
    println!("status: {:?}", output);
    let response = String::from_utf8_lossy(&output.stdout).to_string();
    let error = String::from_utf8_lossy(&output.stderr).to_string();
    let status = output.status.code().unwrap_or_else(|| {
        panic!("failed to execute process: {}", error);
    });
    // return the JSON response
    Json(NginxResponse {
        message: response,
        error: Some(error),
        status: Some(status),
    })
}


// create vhost - sends a JSON response to the dashboard from the controller
#[get("/create_vhost/<domain>")]
pub fn create_vhost_route(domain: &str) -> Json<VhostResponse> {
    VhostHandler::create_vhost(domain.to_string()).into_iter().for_each(|vhost| {
        println!("vhost: {:?}", vhost);
    });
    Json(VhostResponse {
        message: format!("Created vhost for {}", domain),
        error: None,
        status: None,
    })


    
}