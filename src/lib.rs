pub mod api;
pub mod db;
pub mod dto;

use anyhow::Result;
use kdam::tqdm;

pub async fn find_all_climbs_in_gym_topped_by_user(
    gym_id: usize,
    user_uid: usize,
) -> Result<Vec<dto::ClimbPreview>> {
    let mut topped_climbs = Vec::new();
    let climbs = api::get_climbs(gym_id).await?;
    for climb in tqdm!(climbs.into_iter()) {
        if climb.nr_of_ascends == 0 || climb.live == false || climb.deleted == true {
            continue;
        }
        let stats = api::get_climb_stats(climb.id, gym_id).await?;
        let uids: Vec<usize> = stats.toppers.iter().map(|item| item.uid).collect();
        if uids.contains(&user_uid) {
            topped_climbs.push(climb);
        }
    }
    Ok(topped_climbs)
}
