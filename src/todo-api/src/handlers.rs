use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    error::{AppError, AppResult},
    models::{CreateTodoRequest, Todo, TodoDifficulty, UpdateTodoRequest},
};

const SELECT_COLS: &str =
    "id, title, status, difficulty, deadline, created_at, updated_at";

/// GET /api/todos — Tüm görevleri listele
pub async fn get_all_todos(State(pool): State<PgPool>) -> AppResult<Json<Vec<Todo>>> {
    let todos = sqlx::query_as::<_, Todo>(&format!(
        "SELECT {SELECT_COLS} FROM todos ORDER BY created_at DESC"
    ))
    .fetch_all(&pool)
    .await?;

    Ok(Json(todos))
}

/// GET /api/todos/completed — Tamamlanmış görevleri listele
pub async fn get_completed_todos(State(pool): State<PgPool>) -> AppResult<Json<Vec<Todo>>> {
    let todos = sqlx::query_as::<_, Todo>(&format!(
        "SELECT {SELECT_COLS} FROM todos WHERE status = 'done' ORDER BY updated_at DESC"
    ))
    .fetch_all(&pool)
    .await?;

    Ok(Json(todos))
}

/// GET /api/todos/incomplete — Tamamlanmamış görevleri listele
pub async fn get_incomplete_todos(State(pool): State<PgPool>) -> AppResult<Json<Vec<Todo>>> {
    let todos = sqlx::query_as::<_, Todo>(&format!(
        "SELECT {SELECT_COLS} FROM todos WHERE status = 'undone' ORDER BY created_at DESC"
    ))
    .fetch_all(&pool)
    .await?;

    Ok(Json(todos))
}

/// GET /api/todos/:id — Tek bir görevi getir
pub async fn get_todo_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<Todo>> {
    let todo = sqlx::query_as::<_, Todo>(&format!(
        "SELECT {SELECT_COLS} FROM todos WHERE id = $1"
    ))
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(todo))
}

/// POST /api/todos — Yeni görev oluştur
pub async fn create_todo(
    State(pool): State<PgPool>,
    Json(req): Json<CreateTodoRequest>,
) -> AppResult<(StatusCode, Json<Todo>)> {
    let title = req.title.trim().to_string();
    if title.is_empty() {
        return Err(AppError::BadRequest("Invalid title. Title can't be empty".to_string()));
    }

    let difficulty = req.difficulty.unwrap_or(TodoDifficulty::Medium);

    let todo = sqlx::query_as::<_, Todo>(&format!(
        r#"
        INSERT INTO todos (title, status, difficulty, deadline)
        VALUES ($1, 'undone', $2, $3)
        RETURNING {SELECT_COLS}
        "#
    ))
    .bind(&title)
    .bind(difficulty)
    .bind(req.deadline)
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(todo)))
}

/// PUT /api/todos/:id — Görevi güncelle (kısmi güncelleme desteklenir)
pub async fn update_todo(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTodoRequest>,
) -> AppResult<Json<Todo>> {
    let existing = sqlx::query_as::<_, Todo>(&format!(
        "SELECT {SELECT_COLS} FROM todos WHERE id = $1"
    ))
    .bind(id)
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NotFound)?;

    let title = req
        .title
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .unwrap_or(existing.title);
    let status = req.status.unwrap_or(existing.status);
    let difficulty = req.difficulty.unwrap_or(existing.difficulty);
    let deadline = req.deadline.or(existing.deadline);

    let todo = sqlx::query_as::<_, Todo>(&format!(
        r#"
        UPDATE todos
        SET title = $1,
            status = $2,
            difficulty = $3,
            deadline = $4,
            updated_at = NOW()
        WHERE id = $5
        RETURNING {SELECT_COLS}
        "#
    ))
    .bind(title)
    .bind(status)
    .bind(difficulty)
    .bind(deadline)
    .bind(id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(todo))
}

/// DELETE /api/todos/:id — Görevi sil
pub async fn delete_todo(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> AppResult<StatusCode> {
    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}

/// PATCH /api/todos/overdue — Süresi geçmiş görevleri 'undone' olarak işaretle
/// Tamamlanmamış (done olmayan) ve deadline'ı geçmiş görevleri etkiler
pub async fn update_overdue_todos(State(pool): State<PgPool>) -> AppResult<Json<Vec<Todo>>> {
    let todos = sqlx::query_as::<_, Todo>(&format!(
        r#"
        UPDATE todos
        SET status = 'undone',
            updated_at = NOW()
        WHERE deadline IS NOT NULL
          AND deadline < NOW()
          AND status <> 'done'
        RETURNING {SELECT_COLS}
        "#
    ))
    .fetch_all(&pool)
    .await?;

    Ok(Json(todos))
}
