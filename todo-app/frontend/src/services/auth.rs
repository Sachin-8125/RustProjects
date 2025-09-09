use gloo_storage::{LocalStorage, Storage};
use gloo_net::http::Request;
use crate::types::{AuthResponse,LoginRequest,RegisterRequest};

const API_BASE_URL: &str = "http://127.0.0.1:3001/api";
const TOKEN_KEY: &str = "auth_token";

#[derive(Clone)]
pub struct AuthService;

impl AuthService{
    pub fn new() -> Self{
        Self
    }

    pub fn is_logged_in(&self) -> bool {
        LocalStorage::get::<String>(TOKEN_KEY).is_ok()
    }

    pub fn get_token(&self) -> Option<String> {
        LocalStorage::get(TOKEN_KEY).ok()
    }

    pub fn logout(&self){
        let _ = LocalStorage::delete(TOKEN_KEY);
        web_sys::window()
        .unwrap()
        .location()
        .reload()
        .unwrap()
    }

    pub async fn login(&self, email: String, password: String) -> Result<(), String>{
        let request = LoginRequest{email, password};
        
        let response = Request::post(&format!("{}/login",API_BASE_URL))
            .header("Content-Type", "application/json")
            .json(&request)
            .map_err(|e| format!("Request Error: {}",e))?
            .send()
            .await
            .map_err(|e| format!("Network Error: {}",e))?;
        
        if response.ok() {
            let auth_response: AuthResponse = response
                .json()
                .await
                .map_err(|e| format!("Parse Error: {}",e))?;

            LocalStorage::set(TOKEN_KEY,auth_response.token)
            .map_err(|e| format!("Storage Error: {}",e))?;
            Ok(())
        }else{
            let status = response.status();
            Err(format!("Login failed with status: {}",status))
        }
    }

    pub async fn register(&self, username: String, email: String, password: String) -> Result<(),String>{
        let request = RegisterRequest{username, email, password};

        let response = Request::post(&format!("{}/register",API_BASE_URL))
            .header("Content-Type", "application/json")
            .json(&request)
            .map_err(|e| format!("Request Error: {}",e))?
            .send()
            .await
            .map_err(|e| format!("Network Error: {}",e))?;
        
            if response.ok() {
                let auth_response: AuthResponse = response
                    .json()
                    .await
                    .map_err(|e| format!("Parse Error: {}",e))?;
    
                LocalStorage::set(TOKEN_KEY,auth_response.token)
                .map_err(|e| format!("Storage Error: {}",e))?;
                Ok(())
            }else{
                let status = response.status();
                match status {
                    409 => Err("User already exists".to_string()),
                    _ => Err(format!("Registration failed with status: {}",status)),
                }
            }
    }
}
