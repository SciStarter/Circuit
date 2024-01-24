use crate::model::involvement::Involvement;
use crate::model::{involvement, Error};
use crate::Database;
use async_std::prelude::*;
use sqlx::{prelude::*, Postgres};
use uuid::Uuid;

use super::{Opportunity, Review, Reviews};

pub async fn reviews_for_slug(db: &Database, slug: &str) -> Result<Reviews, Error> {
    let mut ret = Reviews {
        average: 0.0,
        reviews: vec![],
    };

    let mut stream = sqlx::query_file_as!(Review, "db/opportunity/all_reviews.sql", slug).fetch(db);

    while let Some(result) = stream.next().await {
        let review = result?;
        ret.average += review.rating as f32;
        ret.reviews.push(review);
    }

    // Divide by zero produces a NaN value, which is serialized into
    // JSON as null
    ret.average /= ret.reviews.len() as f32;

    Ok(ret)
}

pub async fn add_review_for_slug(
    db: &Database,
    slug: &str,
    person: &Uuid,
    rating: i16,
    comment: &str,
) -> Result<i32, Error> {
    let opp_id = Opportunity::id_by_slug(db, slug)
        .await?
        .ok_or_else(|| Error::NoSuch("opportunity"))?;

    let result = sqlx::query(
        r"
            insert into c_opportunity_review (opportunity_id, person, rating, comment)
            values ($1, $2, $3, $4)
            on conflict (opportunity_id, person) do update set rating = $3, comment = $4
            returning id
        ",
    )
    .bind(opp_id)
    .bind(person)
    .bind(rating)
    .bind(comment)
    .map(|row: <Postgres as sqlx::Database>::Row| row.get::<i32, _>(0))
    .fetch_one(db)
    .await?;

    Ok(result)
}

pub async fn report_review(db: &Database, id: i32) -> Result<(), Error> {
    sqlx::query("update c_opportunity_review set flags = flags + 1 where id = $1")
        .bind(id)
        .execute(db)
        .await?;
    Ok(())
}

pub async fn add_like_for_slug(
    db: &Database,
    slug: &str,
    person: &Option<Uuid>,
) -> Result<(), Error> {
    let opp_id = Opportunity::id_by_slug(db, slug)
        .await?
        .ok_or_else(|| Error::NoSuch("opportunity"))?;

    sqlx::query_file!("db/opportunity/add_like.sql", opp_id, person.clone())
        .execute(db)
        .await?;

    Ok(())
}

pub async fn remove_like_for_slug(db: &Database, slug: &str, person: &Uuid) -> Result<(), Error> {
    let opp_id = Opportunity::id_by_slug(db, slug)
        .await?
        .ok_or_else(|| Error::NoSuch("opportunity"))?;

    sqlx::query!(
        r#"delete from c_opportunity_like where opportunity_id = $1 and person = $2"#,
        opp_id,
        person
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn likes_for_slug(db: &Database, slug: &str) -> Result<u32, Error> {
    Ok(sqlx::query_file!("db/opportunity/count_likes.sql", slug)
        .map(|row| row.likes)
        .fetch_one(db)
        .await?
        .unwrap_or(0) as u32)
}

pub async fn likes_for_slug_and_person(
    db: &Database,
    slug: &str,
    person: &Uuid,
) -> Result<u32, Error> {
    Ok(
        sqlx::query_file!("db/opportunity/count_person_likes.sql", slug, person)
            .map(|row| row.likes)
            .fetch_one(db)
            .await?
            .unwrap_or(0) as u32,
    )
}

pub async fn saves_for_slug(db: &Database, slug: &str) -> Result<u32, Error> {
    Ok(
        sqlx::query_file!("db/opportunity/count_saves_by_slug.sql", slug)
            .map(|row| row.saves)
            .fetch_one(db)
            .await?
            .unwrap_or(0) as u32,
    )
}

pub async fn add_save_for_slug(db: &Database, slug: &str, person: &Uuid) -> Result<(), Error> {
    let uid = Opportunity::uid_by_slug(db, slug)
        .await?
        .ok_or_else(|| Error::NoSuch("opportunity"))?;

    Involvement::upgrade(db, person, &uid, involvement::Mode::Saved, &None).await?;

    Ok(())
}

pub async fn remove_save_for_slug(db: &Database, slug: &str, person: &Uuid) -> Result<(), Error> {
    let uid = Opportunity::uid_by_slug(db, slug)
        .await?
        .ok_or_else(|| Error::NoSuch("opportunity"))?;

    if let Some(mut inv) =
        Involvement::load_by_participant_and_opportunity(db, person, &uid).await?
    {
        if inv.exterior.mode == involvement::Mode::Saved {
            inv.exterior.mode = involvement::Mode::Interest;
            inv.store(db).await?;
        }
    }

    Ok(())
}

pub async fn add_interest_for_slug(db: &Database, slug: &str, person: &Uuid) -> Result<(), Error> {
    let uid = Opportunity::uid_by_slug(db, slug)
        .await?
        .ok_or_else(|| Error::NoSuch("opportunity"))?;

    Involvement::upgrade(db, person, &uid, involvement::Mode::Interest, &None).await?;

    Ok(())
}

pub async fn didits_for_slug(db: &Database, slug: &str) -> Result<u32, Error> {
    Ok(
        sqlx::query_file!("db/opportunity/count_didit_by_slug.sql", slug)
            .map(|row| row.didit)
            .fetch_one(db)
            .await?
            .unwrap_or(0) as u32,
    )
}

pub async fn add_didit_for_slug(db: &Database, slug: &str, person: &Uuid) -> Result<(), Error> {
    let uid = Opportunity::uid_by_slug(db, slug)
        .await?
        .ok_or_else(|| Error::NoSuch("opportunity"))?;

    Involvement::upgrade(db, person, &uid, involvement::Mode::Logged, &None).await?;

    Ok(())
}

pub async fn remove_didit_for_slug(db: &Database, slug: &str, person: &Uuid) -> Result<(), Error> {
    let uid = Opportunity::uid_by_slug(db, slug)
        .await?
        .ok_or_else(|| Error::NoSuch("opportunity"))?;

    if let Some(mut inv) =
        Involvement::load_by_participant_and_opportunity(db, person, &uid).await?
    {
        if inv.exterior.mode == involvement::Mode::Logged {
            inv.exterior.mode = involvement::Mode::Saved;
            inv.store(db).await?;
        }
    }

    Ok(())
}
