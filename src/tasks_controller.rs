use axum::{
    extract::{Path, State},
    http::StatusCode, Json
};

use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;

#[derive(Serialize)]
struct TaskRow {
    task_id: i32,
    name: String,
    priority: Option<i32>
}

#[derive(Deserialize)]
pub struct CreateTaskReq {
    name: String,
    priority: Option<i32>
}

#[derive(Serialize)]
pub struct CreateTaskRow {
    task_id: i32
}

#[derive(Deserialize)]
pub struct UpdateTaskReq {
    name: Option<String>,
    priority: Option<i32>
}

pub async fn get_tasks(State(pg_pool): State<PgPool>) -> Result<(StatusCode, String), (StatusCode, String)>{
    let rows = sqlx::query_as!(TaskRow, "SELECT * FROM tasks ORDER BY task_id")
    .fetch_all(&pg_pool)
    .await
    .map_err(|err| (
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({"success": false, "message": err.to_string()}).to_string()
    ))?;

    Ok((
        StatusCode::OK,
        json!({"success": true, "data": rows}).to_string()
    ))
}

pub async fn create_task(
    State(pg_pool): State<PgPool>,
    Json(task): Json<CreateTaskReq>
) -> Result<(StatusCode, String), (StatusCode, String)>{
    let row = sqlx::query_as!(
        CreateTaskRow,
        "INSERT INTO tasks (name, priority) VALUES ($1, $2) RETURNING task_id",
        task.name,
        task.priority
    ).fetch_one(&pg_pool)
    .await
    .map_err(|err| (
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({"success": false, "messagge": err.to_string()}).to_string()
    ))?;

    Ok((
        StatusCode::CREATED,
        json!({"success": true, "data": row}).to_string()
    ))
}

pub async fn update_task(
    State(pg_pool): State<PgPool>,
    Path(task_id): Path<i32>,
    Json(task): Json<UpdateTaskReq>
) -> Result<(StatusCode, String), (StatusCode, String)>{
    sqlx::query!("
    UPDATE tasks SET
        name = $2,
        priority = $3
    WHERE task_id = $1
    ",
    task_id, 
    task.name,
    task.priority
    ).execute(&pg_pool)
    .await
    .map_err(|err| (
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({"success": false, "message": err.to_string()}).to_string()
    ))?;

    Ok((StatusCode::OK, json!({"success": true}).to_string()))
}

pub async fn delete_task(
    State(pg_pool): State<PgPool>, 
    Path(task_id): Path<i32>
) -> Result<(StatusCode, String), (StatusCode, String)>{
    sqlx::query!("
    DELETE FROM tasks WHERE task_id = $1"
    ,task_id)
    .execute(&pg_pool)
    .await
    .map_err(|err| (
        StatusCode::INTERNAL_SERVER_ERROR,
        json!({"success": false, "message": err.to_string()}).to_string()
    ))?;

    Ok((StatusCode::OK, json!({"success": true}).to_string()))
}