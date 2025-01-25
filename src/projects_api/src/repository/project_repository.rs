use crate::dto::prelude::*;
use crate::model::prelude::*;
use sqlx::{sqlite::SqlitePool, Result, Row};

pub struct ProjectRepository {
    pool: SqlitePool,
}

impl ProjectRepository {
    pub fn new(pool: SqlitePool) -> Self {
        ProjectRepository { pool }
    }

    pub async fn create_project(&self, project: &Project) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO projects (id, name, language, summary, criteria_set_id)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(project.id)
        .bind(&project.name)
        .bind(&project.language)
        .bind(&project.summary)
        .bind(&project.criteria_set_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_project(&self, project_id: u32) -> Result<ProjectWithCriteria> {
        let rows = sqlx::query(
            r#"
        SELECT
            p.id AS project_id, p.name AS project_name, p.language, p.summary, p.criteria_set_id,
            cs.id AS criteria_set_id, cs.name AS criteria_set_name,
            c.name AS criterion_name, c.point
        FROM projects p
        LEFT JOIN criteria_sets cs ON p.criteria_set_id = cs.id
        LEFT JOIN criteria c ON c.criteria_set_id = cs.id
        WHERE p.id = ?;
        "#,
        )
        .bind(project_id)
        .fetch_all(&self.pool)
        .await?;

        let first_row = rows.first().ok_or(sqlx::Error::RowNotFound)?;

        let project = Project {
            id: first_row.get::<u32, _>("project_id"),
            name: first_row.get::<String, _>("project_name"),
            language: first_row.get::<String, _>("language"),
            summary: first_row.get::<String, _>("summary"),
            criteria_set_id: first_row.get::<u32, _>("criteria_set_id"),
        };

        let criteria_set = CriteriaSet {
            id: first_row.get::<u32, _>("criteria_set_id"),
            name: first_row.get::<String, _>("criteria_set_name"),
            set: rows
                .iter()
                .filter_map(|row| {
                    let criterion_name: Option<String> = row.try_get("criterion_name").ok();
                    let point: Option<i32> = row.try_get("point").ok();
                    if let (Some(name), Some(point)) = (criterion_name, point) {
                        Some(Criterion { name, point })
                    } else {
                        None
                    }
                })
                .collect(),
        };

        Ok(ProjectWithCriteria {
            project,
            criteria_set,
        })
    }
}
