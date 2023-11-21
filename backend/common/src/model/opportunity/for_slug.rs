use crate::model::involvement::Involvement;
use crate::model::{involvement, Error};
use crate::Database;
use async_std::prelude::*;
use uuid::Uuid;

use super::{Opportunity, Review, Reviews};

pub async fn reviews_for_slug(db: &Database, slug: &str) -> Result<Reviews, Error> {
    let mut ret = Reviews {
        average: 0.0,
        reviews: vec![],
    };

    let mut stream = sqlx::query_as!(
        Review,
        r#"
        SELECT R."id", R."person", R."rating", R."comment", R."when",
          CASE WHEN P."exterior" IS NOT null THEN (P."exterior" ->> 'username') ELSE '' END AS "username",
          CASE WHEN P."exterior" IS NOT null THEN (P."exterior" ->> 'image_url') ELSE '' END AS "image_url"
        FROM
          c_opportunity_review R
          INNER JOIN c_opportunity O on R.opportunity_id = O.id
          LEFT OUTER JOIN c_person P on R.person = (P.exterior ->> 'uid')::uuid
        WHERE
          O."slug" = lower($1)
        ORDER BY
          R."when" DESC
        "#,
        slug
    ).fetch(db);

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

    let result = sqlx::query!(
        r#"
        INSERT INTO c_opportunity_review (opportunity_id, person, rating, comment)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (opportunity_id, person) DO UPDATE SET rating = $3, comment = $4
        RETURNING id
        "#,
        opp_id,
        person,
        rating,
        comment
    )
    .map(|row| row.id)
    .fetch_one(db)
    .await?;

    Ok(result)
}

pub async fn report_review(db: &Database, id: i32) -> Result<(), Error> {
    sqlx::query!(
        "UPDATE c_opportunity_review SET flags = flags + 1 WHERE id = $1",
        id
    )
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

    sqlx::query!(
        r#"INSERT INTO c_opportunity_like ("opportunity_id", "person") VALUES ($1, $2) ON CONFLICT DO NOTHING"#,
        opp_id,
        person.clone()
    )
        .execute(db)
        .await?;

    Ok(())
}

pub async fn remove_like_for_slug(db: &Database, slug: &str, person: &Uuid) -> Result<(), Error> {
    let opp_id = Opportunity::id_by_slug(db, slug)
        .await?
        .ok_or_else(|| Error::NoSuch("opportunity"))?;

    sqlx::query!(
        r#"DELETE FROM c_opportunity_like WHERE opportunity_id = $1 AND person = $2"#,
        opp_id,
        person
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn likes_for_slug(db: &Database, slug: &str) -> Result<u32, Error> {
    Ok(sqlx::query!(r#"SELECT count(*) AS "likes" FROM c_opportunity_like AS "l" INNER JOIN c_opportunity AS "o" ON "l"."opportunity_id" = "o"."id" WHERE "o"."slug" = lower($1)"#, slug)
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
        sqlx::query!(
            r#"SELECT count(*) AS "likes" FROM c_opportunity_like AS "l" INNER JOIN c_opportunity AS "o" ON "l"."opportunity_id" = "o"."id" WHERE "o"."slug" = lower($1) AND "l"."person" = $2"#,
            slug,
            person)
            .map(|row| row.likes)
            .fetch_one(db)
            .await?
            .unwrap_or(0) as u32,
    )
}

pub async fn saves_for_slug(db: &Database, slug: &str) -> Result<u32, Error> {
    Ok(sqlx::query!(
        r#"
        SELECT count(*) AS "saves"
        FROM
          c_involvement AS "i"
          INNER JOIN c_opportunity AS "o" ON ("i"."exterior"->>'opportunity')::uuid = "o"."uid"
        WHERE
          ("i"."exterior"->'mode')::integer = 20
          AND "o"."slug" = lower($1)
        "#,
        slug
    )
    .map(|row| row.saves)
    .fetch_one(db)
    .await?
    .unwrap_or(0) as u32)
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
    Ok(sqlx::query!(
        r#"
        SELECT count(*) AS "didit"
        FROM
          c_involvement AS "i"
          INNER JOIN c_opportunity AS "o" ON ("i"."exterior"->>'opportunity')::uuid = "o"."uid"
        WHERE
          ("i"."exterior"->'mode')::integer >= 30
          AND "o"."slug" = lower($1)
        "#,
        slug
    )
    .map(|row| row.didit)
    .fetch_one(db)
    .await?
    .unwrap_or(0) as u32)
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
