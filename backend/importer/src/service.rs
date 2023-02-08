use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::time::{Duration, Instant};

use sqlx::postgres::PgPoolOptions;

mod config;

const RETRY_DELAY: Duration = Duration::new(300, 0);

#[async_std::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let importers = config::setup();
    let mut schedule: BinaryHeap<Reverse<(Instant, usize)>> = BinaryHeap::new();

    for idx in 0..importers.len() {
        schedule.push(Reverse((Instant::now(), idx)));
    }

    let pool = PgPoolOptions::new()
        .min_connections(1)
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;

    common::migrate(&pool).await?;

    while !schedule.is_empty() {
        let now = Instant::now();

        if let Some(Reverse((when, idx))) = schedule.pop() {
            if when <= now {
                println!("Importing from {:?}", &importers[idx]);
                let delay = match (&importers[idx]).import(pool.clone()).await {
                    Ok(optional) => optional,
                    Err(error) => {
                        println!("Error {:?} while importing {:?}", error, &importers[idx]);
                        Some(RETRY_DELAY)
                    }
                };

                if let Some(duration) = delay {
                    schedule.push(Reverse((Instant::now() + duration, idx)));
                }
            } else {
                schedule.push(Reverse((when, idx)));
                async_std::task::sleep(when - now).await;
            }
        }
    }

    Ok(())
}
