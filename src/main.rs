use toplogger_stats::api::{get_user, get_user_stats};
use toplogger_stats::db::{establish_connection, insert_or_update_user_info, select_users};
use toplogger_stats::find_all_climbs_in_gym_topped_by_user;

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();
    dotenv::dotenv()?;

    let user_uid: usize = env::var("USER_UID")?.parse::<usize>()?;
    let gym_id = env::var("GYM_ID")?.parse::<usize>()?;
    let db_url = env::var("DATABASE_URL")?;

    let user = get_user(user_uid).await;
    log::info!("{user:?}");

    let user_stats = get_user_stats(gym_id, user_uid).await;
    log::info!("{user_stats:?}");

    let db = establish_connection(&db_url).await?;

    let db_users = select_users(&db).await?;
    log::info!("{db_users:?}");

    insert_or_update_user_info(&db, user.unwrap()).await?;

    let db_users = select_users(&db).await?;
    log::info!("{db_users:?}");

    let climbs = find_all_climbs_in_gym_topped_by_user(gym_id, user_uid).await?;
    log::info!("{climbs:?}");
    Ok(())
}
