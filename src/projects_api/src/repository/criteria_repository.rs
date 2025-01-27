use crate::dto::prelude::{CreatedCriteria, NewHistory};
use crate::model::prelude::*;
use crate::repository::history_repository::HistoryRepository;
use sqlx::{sqlite::SqlitePool, Result, Row};

pub struct CriteriaRepository {
    pool: SqlitePool,
    history_repository: HistoryRepository,
}

impl CriteriaRepository {
    pub fn new(pool: SqlitePool) -> Self {
        let history_repository = HistoryRepository::new(pool.clone());
        CriteriaRepository {
            pool,
            history_repository,
        }
    }

    pub async fn create_criteria_set(&self, criteria_set: &CriteriaSet) -> Result<CreatedCriteria> {
        let inserted: (i64,) = sqlx::query_as(
            r#"
        INSERT INTO criteria_sets (name)
        VALUES (?)
        RETURNING id
        "#,
        )
        .bind(&criteria_set.name)
        .fetch_one(&self.pool)
        .await?;

        for criterion in &criteria_set.set {
            self.add_criterion_to_criteria_set(inserted.0 as u32, criterion)
                .await?;
        }

        self.history_repository
            .create_history(&NewHistory {
                event: "CriteriaSetCreated".to_string(),
                description: format!(
                    "'{}' created with id '{}'",
                    criteria_set.name, criteria_set.id
                ),
            })
            .await?;

        Ok(CreatedCriteria {
            id: inserted.0 as u32,
            name: criteria_set.name.clone(),
            criteria_count: criteria_set.set.len(),
        })
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

        self.history_repository
            .create_history(&NewHistory {
                event: "CriterionAddedToSet".to_string(),
                description: format!("'{}' added to set '{}'", criterion.name, criteria_set_id),
            })
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
