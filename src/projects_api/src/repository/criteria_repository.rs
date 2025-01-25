use crate::model::prelude::*;
use sqlx::{sqlite::SqlitePool, Result, Row};

pub struct CriteriaRepository {
    pool: SqlitePool,
}

impl CriteriaRepository {
    pub fn new(pool: SqlitePool) -> Self {
        CriteriaRepository { pool }
    }

    pub async fn create_criteria_set(&self, criteria_set: &CriteriaSet) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO criteria_sets (id, name)
            VALUES (?, ?)
            "#,
        )
        .bind(criteria_set.id)
        .bind(&criteria_set.name)
        .execute(&self.pool)
        .await?;

        for criterion in &criteria_set.set {
            self.add_criterion_to_criteria_set(criteria_set.id, criterion)
                .await?;
        }

        Ok(())
    }

    pub async fn add_criterion_to_criteria_set(
        &self,
        criteria_set_id: u32,
        criterion: &Criterion,
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO criteria (name, point, criteria_set_id)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(&criterion.name)
        .bind(&criterion.point)
        .bind(criteria_set_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_criteria_set(&self, criteria_set_id: u32) -> Result<CriteriaSet> {
        let criteria_set_row = sqlx::query(
            r#"
            SELECT id, name
            FROM criteria_sets
            WHERE id = ?
            "#,
        )
        .bind(criteria_set_id)
        .fetch_one(&self.pool)
        .await?;

        let criteria_set = CriteriaSet {
            id: criteria_set_row.get::<u32, _>(0),
            name: criteria_set_row.get::<String, _>(1),
            set: vec![],
        };

        let criterion_rows = sqlx::query(
            r#"
            SELECT name,point
            FROM criteria
            WHERE criteria_set_id = ?
            "#,
        )
        .bind(criteria_set_id)
        .fetch_all(&self.pool)
        .await?;

        let criterion_list = criterion_rows
            .iter()
            .map(|row| Criterion {
                name: row.get::<String, _>(0),
                point: row.get::<i32, _>(1),
            })
            .collect();

        Ok(CriteriaSet {
            id: criteria_set_id,
            name: criteria_set.name,
            set: criterion_list,
        })
    }
}
