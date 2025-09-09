use gloo_net::http::Request;
use gloo_storage::{LocalStorage, Storage};
use crate::types::{CreateTodoRequest, Todo, TodoUpdate};

const API_BASE_URL: &str = "http://127.0.0.1:3001/api";
const TOKEN_KEY: &str = "auth_token";

pub struct ApiService;

impl ApiService {
    fn get_auth_header() -> Result<String, String> {
        let token: String = LocalStorage::get(TOKEN_KEY)
            .map_err(|_| "No auth token found")?;
        Ok(format!("Bearer {}", token))
    }

    pub async fn get_todos() -> Result<Vec<Todo>, String> {
        let auth_header = Self::get_auth_header()?;

        let response = Request::get(&format!("{}/todos", API_BASE_URL))
            .header("Authorization", &auth_header)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            let todos: Vec<Todo> = response
                .json()
                .await
                .map_err(|e| format!("Parse error: {}", e))?;
            Ok(todos)
        } else {
            Err(format!("Failed to fetch todos: {}", response.status()))
        }
    }

    pub async fn create_todo(title: String, description: Option<String>) -> Result<Todo, String> {
        let auth_header = Self::get_auth_header()?;
        let request = CreateTodoRequest { title, description };

        let response = Request::post(&format!("{}/todos", API_BASE_URL))
            .header("Authorization", &auth_header)
            .header("Content-Type", "application/json")
            .json(&request)
            .map_err(|e| format!("Request error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            let todo: Todo = response
                .json()
                .await
                .map_err(|e| format!("Parse error: {}", e))?;
            Ok(todo)
        } else {
            Err(format!("Failed to create todo: {}", response.status()))
        }
    }

    pub async fn update_todo(id: &str, update: TodoUpdate) -> Result<Todo, String> {
        let auth_header = Self::get_auth_header()?;

        let response = Request::patch(&format!("{}/todos/{}", API_BASE_URL, id))
            .header("Authorization", &auth_header)
            .header("Content-Type", "application/json")
            .json(&update)
            .map_err(|e| format!("Request error: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            let todo: Todo = response
                .json()
                .await
                .map_err(|e| format!("Parse error: {}", e))?;
            Ok(todo)
        } else {
            Err(format!("Failed to update todo: {}", response.status()))
        }
    }

    pub async fn delete_todo(id: &str) -> Result<(), String> {
        let auth_header = Self::get_auth_header()?;

        let response = Request::delete(&format!("{}/todos/{}", API_BASE_URL, id))
            .header("Authorization", &auth_header)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            Ok(())
        } else {
            Err(format!("Failed to delete todo: {}", response.status()))
        }
    }

    pub async fn toggle_todo_completion(id: &str, completed: bool) -> Result<Todo, String> {
        let update = TodoUpdate {
            title: None,
            description: None,
            completed: Some(!completed),
        };
        Self::update_todo(id, update).await
    }

    pub async fn update_todo_content(id: &str, title: Option<String>, description: Option<String>) -> Result<Todo, String> {
        let update = TodoUpdate {
            title,
            description,
            completed: None,
        };
        Self::update_todo(id, update).await
    }
}