// src/config/mod.rs
use std::env;

pub fn init_config() {
    dotenv::dotenv().ok();  // Load environment variables from a .env file
    env::set_var("RUST_LOG", "actix_web=info");
}


pub fn get_server_port() -> u16 {
    // Try to read the `PORT` variable from the environment
    // Default to 8080 if `PORT` is not set
    env::var("PORT")
        .unwrap_or_else(|_| "3001".to_string())
        .parse()
        .expect("PORT must be a valid u16")
}

 pub fn get_api_access_token() -> String {    
    return env::var("API_ACCESS_TOKEN")        
         .expect("API_ACCESS_TOKEN must be set")
 }

 pub fn get_aws_infra_base_url() -> String {    
    return env::var("AWS_INFRA_BASE_URL")        
         .expect("AWS_INFRA_BASE_URL must be set")
 }