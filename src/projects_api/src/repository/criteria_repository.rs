use crate::dto::prelude::{CreatedCriteria, NewHistory};
use crate::enums::history_event::HistoryEvent;
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
                event: HistoryEvent::CriteriaSetCreated.to_string(),
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
        .bind(criterion.point)
        .bind(criteria_set_id)
        .execute(&self.pool)
        .await?;

        self.history_repository
            .create_history(&NewHistory {
                event: HistoryEvent::CriterionAddedToSet.to_string(),
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

    pub async fn get_all_criteria(&self) -> Result<Vec<CriteriaSet>> {
        let rows = sqlx::query(
            r#"
        SELECT
            S.id AS criteria_set_id,
            S.name AS criteria_set_name,
            C.id AS criterion_id,
            C.name AS criterion_name,
            C.point AS criterion_point
        FROM
            criteria_sets S
        JOIN
            criteria C ON S.id = C.criteria_set_id
        ORDER BY
            C.name DESC
        "#,
        )
        .fetch_all(&self.pool)
        .await?;

        use std::collections::HashMap;

        let mut criteria_map: HashMap<u32, CriteriaSet> = HashMap::new();

        for row in rows {
            let criteria_set_id = row.get::<u32, _>("criteria_set_id");
            let criteria_set_name = row.get::<String, _>("criteria_set_name");
            let criterion_name = row.get::<String, _>("criterion_name");
            let criterion_point = row.get::<i32, _>("criterion_point");

            // Mevcut CriteriaSet varsa güncelle, yoksa oluştur
            criteria_map
                .entry(criteria_set_id)
                .and_modify(|set| {
                    set.set.push(Criterion {
                        name: criterion_name.clone(),
                        point: criterion_point,
                    });
                })
                .or_insert_with(|| CriteriaSet {
                    id: criteria_set_id,
                    name: criteria_set_name,
                    set: vec![Criterion {
                        name: criterion_name,
                        point: criterion_point,
                    }],
                });
        }

        let result = criteria_map.into_values().collect::<Vec<CriteriaSet>>();
        Ok(result)
    }

    pub async fn delete_criterion(&self, criteria_id: u32, name: String) -> Result<u64> {
        let deleted_criterion = sqlx::query(
            r#"
        DELETE FROM criteria WHERE criteria_set_id = ? AND name = ?
        "#,
        )
        .bind(criteria_id)
        .bind(name.clone())
        .execute(&self.pool)
        .await?;

        if deleted_criterion.rows_affected() > 0 {
            self.history_repository
                .create_history(&NewHistory {
                    event: HistoryEvent::CriterionDeleted.to_string(),
                    description: format!("Criterion '{}' deleted.", name),
                })
                .await?;
        }

        Ok(deleted_criterion.rows_affected())
    }
}
