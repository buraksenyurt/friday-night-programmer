use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;
use uuid::Uuid;

/// Görev durumu
#[derive(Debug, Serialize, Deserialize, Type, Clone, PartialEq)]
#[sqlx(type_name = "todo_status")]
pub enum TodoStatus {
    #[sqlx(rename = "done")]
    #[serde(rename = "done")]
    Done,

    #[sqlx(rename = "undone")]
    #[serde(rename = "undone")]
    Undone,

    #[sqlx(rename = "inprogress")]
    #[serde(rename = "inprogress")]
    InProgress,
}

/// Görev zorluk derecesi
#[derive(Debug, Serialize, Deserialize, Type, Clone, PartialEq)]
#[sqlx(type_name = "todo_difficulty")]
pub enum TodoDifficulty {
    #[sqlx(rename = "easy")]
    #[serde(rename = "easy")]
    Easy,

    #[sqlx(rename = "medium")]
    #[serde(rename = "medium")]
    Medium,

    #[sqlx(rename = "hard")]
    #[serde(rename = "hard")]
    Hard,
}

/// Veritabanı modeli
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub status: TodoStatus,
    pub difficulty: TodoDifficulty,
    pub deadline: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Yeni görev oluşturma isteği
#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub difficulty: Option<TodoDifficulty>,
    pub deadline: Option<DateTime<Utc>>,
}

/// Görev güncelleme isteği (kısmi güncelleme)
#[derive(Debug, Deserialize)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub status: Option<TodoStatus>,
    pub difficulty: Option<TodoDifficulty>,
    pub deadline: Option<DateTime<Utc>>,
}
