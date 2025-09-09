use axum::{
    extract::{Path, State},
    http::StatusCode,
    middleware,
    response::Json,
    routing::{get, patch, post},
    Router,
};
use sqlx::{Row, PgPool};
use std::env;
use dotenvy::dotenv;
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

mod auth;
mod models;

use auth::{auth_middleware, Claims, create_token, hash_password, verify_password};
use models::{CreateTodoRequest, LoginRequest, RegisterRequest, Todo, TodoUpdate};

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // Database setup
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_|
        "postgres://avnadmin:AVNS_uhhrxMwIhJa5-dVkpoc@pg-7ee904a-sachin2317080-81fe.d.aivencloud.com:25431/defaultdb?sslmode=require".to_string()
    );
    let pool = PgPool::connect(&database_url).await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    let state = AppState { db: pool };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/register", post(register))
        .route("/api/login", post(login))
        .route("/api/todos", get(get_todos).post(create_todo))
        .route("/api/todos/:id", patch(update_todo).delete(delete_todo))
        .layer(middleware::from_fn(auth_middleware))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await?;
    println!("Server running on http://127.0.0.1:3001");
    
    axum::serve(listener, app).await?;

    Ok(())
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let hashed_password = hash_password(&payload.password)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_id = Uuid::new_v4();
    
    let result = sqlx::query(
        "INSERT INTO users (id, username, email, password_hash) VALUES ($1, $2, $3, $4)"
    )
    .bind(user_id.to_string())
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(hashed_password)
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => {
            let token = create_token(&user_id.to_string())
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            Ok(Json(serde_json::json!({
                "message": "User created successfully",
                "token": token
            })))
        }
        Err(_) => Err(StatusCode::CONFLICT),
    }
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let row = sqlx::query("SELECT id, password_hash FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(row) = row {
        let user_id: String = row.get("id");
        let password_hash: String = row.get("password_hash");

        if verify_password(&payload.password, &password_hash)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        {
            let token = create_token(&user_id)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            
            return Ok(Json(serde_json::json!({
                "message": "Login successful",
                "token": token
            })));
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

async fn get_todos(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Vec<Todo>>, StatusCode> {
    let rows = sqlx::query("SELECT * FROM todos WHERE user_id = $1 ORDER BY created_at DESC")
        .bind(&claims.sub)
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let todos: Vec<Todo> = rows
        .iter()
        .map(|row| Todo {
            id: row.get("id"),
            user_id: row.get("user_id"),
            title: row.get("title"),
            description: row.get("description"),
            completed: row.get("completed"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
        .collect();

    Ok(Json(todos))
}

async fn create_todo(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<Json<Todo>, StatusCode> {
    let todo_id = Uuid::new_v4();
    let now = chrono::Utc::now();

    sqlx::query(
        "INSERT INTO todos (id, user_id, title, description, completed, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7)"
    )
    .bind(todo_id.to_string())
    .bind(&claims.sub)
    .bind(&payload.title)
    .bind(payload.description.as_deref().unwrap_or(""))
    .bind(false)
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let todo = Todo {
        id: todo_id.to_string(),
        user_id: claims.sub,
        title: payload.title,
        description: payload.description.clone(),
        completed: false,
        created_at: now,
        updated_at: now,
    };

    Ok(Json(todo))
}

async fn update_todo(
    Path(id): Path<String>,
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<TodoUpdate>,
) -> Result<Json<Todo>, StatusCode> {
    let now = chrono::Utc::now();

    // First, get the current todo to verify ownership and get current values
    let row = sqlx::query("SELECT * FROM todos WHERE id = $1 AND user_id = $2")
        .bind(&id)
        .bind(&claims.sub)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let current_todo = match row {
        Some(row) => Todo {
            id: row.get("id"),
            user_id: row.get("user_id"),
            title: row.get("title"),
            description: row.get("description"),
            completed: row.get("completed"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        },
        None => return Err(StatusCode::NOT_FOUND),
    };

    // Update with new values or keep existing ones
    let new_title = payload.title.unwrap_or(current_todo.title);
    let new_description = payload.description.or(current_todo.description);
    let new_completed = payload.completed.unwrap_or(current_todo.completed);

    sqlx::query(
        "UPDATE todos SET title = $1, description = $2, completed = $3, updated_at = $4 WHERE id = $5 AND user_id = $6"
    )
    .bind(&new_title)
    .bind(new_description.as_deref())
    .bind(new_completed)
    .bind(now)
    .bind(&id)
    .bind(&claims.sub)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let updated_todo = Todo {
        id,
        user_id: claims.sub,
        title: new_title,
        description: new_description,
        completed: new_completed,
        created_at: current_todo.created_at,
        updated_at: now,
    };

    Ok(Json(updated_todo))
}

async fn delete_todo(
    Path(id): Path<String>,
    State(state): State<AppState>,
    claims: Claims,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM todos WHERE id = $1 AND user_id = $2")
        .bind(&id)
        .bind(&claims.sub)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}