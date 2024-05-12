use toplogger_stats as lib;
use toplogger_stats::api;
use toplogger_stats::db;

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

    let db = db::establish_connection(&db_url).await?;

    let user = api::get_user(user_uid).await;
    db::insert_or_update_user_info(&db, user.unwrap()).await?;

    lib::insert_top_ten_climbs(&db, gym_id, user_uid).await?;
    lib::insert_non_top_ten_climbs(&db, gym_id, user_uid).await?;

    /*
    let climbs = lib::find_all_climbs_in_gym_topped_by_user(gym_id, user_uid).await?;
    log::info!("{climbs:?}");
    */
    Ok(())
}
