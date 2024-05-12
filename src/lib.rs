pub mod api;
pub mod db;
pub mod dto;

use entity::climb::Model as Climb;
use sea_orm::DbConn;

use anyhow::Result;
use kdam::tqdm;

pub async fn find_all_climbs_in_gym_topped_by_user(
    gym_id: usize,
    user_uid: usize,
) -> Result<Vec<(dto::ClimbPreview, dto::ClimbTopper)>> {
    let mut topped_climbs = Vec::new();
    let climbs = api::get_climbs(gym_id).await?;
    for climb in tqdm!(climbs.into_iter()) {
        if climb.nr_of_ascends == 0 || climb.live == false || climb.deleted == true {
            continue;
        }
        let stats = api::get_climb_stats(climb.id, gym_id).await?;
        for topper in stats.toppers {
            if topper.uid == user_uid {
                topped_climbs.push((climb.clone(), topper));
            }
        }
    }
    Ok(topped_climbs)
}

pub fn convert_climb_preview_to_climb(
    climb_preview: dto::ClimbPreview,
    topper: dto::ClimbTopper,
) -> Climb {
    return Climb {
        id: climb_preview.id as i64,
        climb_id: climb_preview.id as i64,
        date_logged: topper.date_logged,
        checks: topper.checks as i64,
        grade_adj: climb_preview.grade.clone(),
        grade: climb_preview.grade,
        number: None,
        color: "#000000".to_string(),
        color_secondary: None,
        wall_name: "Unknown".to_string(),
        name: None,
        setter_name: None,
    };
}

pub async fn insert_non_top_ten_climbs(db: &DbConn, gym_id: usize, user_uid: usize) -> Result<()> {
    let stats = api::get_user_stats(gym_id, user_uid).await?;
    let top_ten_ids: Vec<usize> = stats
        .top_ten
        .into_iter()
        .map(|climb| climb.climb_id as usize)
        .collect();

    let climbs = find_all_climbs_in_gym_topped_by_user(gym_id, user_uid).await?;
    for (climb_preview, topper) in climbs {
        if !top_ten_ids.contains(&climb_preview.id) {
            let climb = convert_climb_preview_to_climb(climb_preview, topper);
            db::set_climb_topped(db, &climb, user_uid).await?;
            db::insert_or_update_climb(db, climb).await?;
        }
    }
    Ok(())
}

pub async fn insert_top_ten_climbs(db: &DbConn, gym_id: usize, user_uid: usize) -> Result<()> {
    let stats = api::get_user_stats(gym_id, user_uid).await?;
    for climb in stats.top_ten {
        db::set_climb_topped(db, &climb, user_uid).await?;
        db::insert_or_update_climb(db, climb).await?;
    }
    Ok(())
}
