use super::Error;
use crate::Database;
use serde::{Deserialize, Serialize};
use sqlx;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Block {
    pub id: Option<i32>,
    pub language: String,
    pub group: String,
    pub item: String,
    pub tags: String,
    pub label: String,
    pub content: String,
}

impl Block {
    pub async fn list_languages(db: &Database) -> Result<Vec<String>, Error> {
        Ok(sqlx::query_file!("db/block/list_languages.sql")
            .fetch_all(db)
            .await?
            .iter()
            .map(|r| r.language.to_string())
            .collect())
    }

    pub async fn list_groups(db: &Database, language: &str) -> Result<Vec<String>, Error> {
        Ok(sqlx::query_file!("db/block/list_groups.sql", language)
            .fetch_all(db)
            .await?
            .iter()
            .map(|r| r.group.to_string())
            .collect())
    }

    pub async fn list_items(
        db: &Database,
        language: &str,
        group: &str,
    ) -> Result<Vec<String>, Error> {
        Ok(
            sqlx::query_file!("db/block/list_items.sql", language, group)
                .fetch_all(db)
                .await?
                .iter()
                .map(|r| r.item.to_string())
                .collect(),
        )
    }

    pub async fn all_items(
        db: &Database,
        language: &str,
        group: &str,
    ) -> Result<Vec<Block>, Error> {
        Ok(
            sqlx::query_file!("db/block/get_group_items.sql", language, group)
                .fetch_all(db)
                .await?
                .iter()
                .map(|r| Block {
                    id: Some(r.id),
                    language: r.language.to_string(),
                    group: r.group.to_string(),
                    item: r.item.to_string(),
                    tags: r.item.to_string(),
                    label: r.item.to_string(),
                    content: r.content.to_string(),
                })
                .collect(),
        )
    }

    pub fn validate(&mut self) -> Result<(), Error> {
        Ok(())
    }

    pub async fn load_by_id(db: &Database, id: i32) -> Result<Block, Error> {
        let rec = sqlx::query_file!("db/block/get_by_id.sql", id)
            .fetch_one(db)
            .await?;

        Ok(Block {
            id: Some(rec.id),
            language: rec.language,
            group: rec.group,
            item: rec.item,
            tags: rec.tags,
            label: rec.label,
            content: rec.content,
        })
    }

    pub async fn load(
        db: &Database,
        language: &str,
        group: &str,
        item: &str,
    ) -> Result<Block, Error> {
        let rec = sqlx::query_file!("db/block/get_by_logical_id.sql", language, group, item)
            .fetch_one(db)
            .await?;

        Ok(Block {
            id: Some(rec.id),
            language: rec.language,
            group: rec.group,
            item: rec.item,
            tags: rec.tags,
            label: rec.label,
            content: rec.content,
        })
    }

    pub async fn store(&mut self, db: &Database) -> Result<(), Error> {
        self.validate()?;

        if let Some(id) = self.id {
            sqlx::query_file!(
                "db/block/update.sql",
                id,
                self.language,
                self.group,
                self.item,
                self.tags,
                self.label,
                self.content,
            )
            .execute(db)
            .await?;
        } else {
            let rec = sqlx::query_file!(
                "db/block/insert.sql",
                self.language,
                self.group,
                self.item,
                self.tags,
                self.label,
                self.content,
            )
            .fetch_one(db)
            .await?;

            self.id = Some(rec.id);
        };

        Ok(())
    }
}
