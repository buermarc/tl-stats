pub const SERVER_URL: &'static str = "https://api.toplogger.nu/v1";

use crate::dto;

use anyhow::Result;
use url::Url;

use entity::user::Model as User;

/// Get the status of a user, uid can be found in the url of the toplogger site when logged in
pub async fn get_user(user_uid: usize) -> Result<User> {
    let resp = reqwest::get(format!("{SERVER_URL}/users/{user_uid}")).await?;
    resp.error_for_status_ref()?;
    let json = resp.json::<User>().await?;
    return Ok(json);
}

pub async fn get_user_stats(gym_id: usize, user_uid: usize) -> Result<dto::UserStats> {
    let url = Url::parse_with_params(
        format!("{SERVER_URL}/users/{user_uid}/stats").as_str(),
        &[
            ("gym_id", gym_id.to_string()),
            ("climb_types", "boulders".to_string()),
        ],
    )?;
    log::debug!("URL: {url}");

    let resp = reqwest::get(url).await?;
    resp.error_for_status_ref()?;
    let json = resp.json::<dto::UserStats>().await?;
    return Ok(json);
}

pub async fn get_climbs(gym_id: usize) -> Result<Vec<dto::ClimbPreview>> {
    let url = Url::parse_with_params(
        format!("{SERVER_URL}/gyms/{gym_id}/climbs").as_str(),
        &[("deleted", "false"), ("live", "true")],
    )?;
    log::debug!("URL: {url}");

    let resp = reqwest::get(url).await?;
    resp.error_for_status_ref()?;
    let json = resp.json::<Vec<dto::ClimbPreview>>().await?;
    return Ok(json);
}

pub async fn get_climb_stats(climb_id: usize, gym_id: usize) -> Result<dto::ClimbStats> {
    let url = format!("{SERVER_URL}/gyms/{gym_id}/climbs/{climb_id}/stats");
    log::debug!("URL: {url}");

    let resp = reqwest::get(url).await?;
    resp.error_for_status_ref()?;
    let json = resp.json::<dto::ClimbStats>().await?;
    return Ok(json);
}
