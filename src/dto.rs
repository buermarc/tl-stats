use entity::climb::Model as Climb;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserStats {
    pub grade: Option<String>,
    pub grade_count: usize,
    pub top_ten: Vec<Climb>,
    pub user: ShortUserInfo,
}

#[derive(Deserialize, Debug)]
pub struct ShortUserInfo {
    pub avatar: String,
    pub full_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ClimbPreview {
    pub id: usize,
    pub grade: String,
    // pub wall_id: usize,
    pub position_x: String,
    pub position_y: String,
    pub climb_type: String,
    pub suitable_for_kids: bool,
    pub gym_id: usize,
    // pub setter_id: usize,
    pub hold_id: usize,
    pub live: bool,
    pub lived: bool,
    pub deleted: bool,
    // pub date_live_start: String,
    pub date_set: String,
    // pub date_removed: String,
    pub created_at: String,
    pub nr_of_ascends: usize,
    pub average_opinion: String,
    pub auto_grade: bool,
    pub grade_stability: String,
    pub grade_stability_admin: String,
    pub zones: usize,
    pub renew: bool,
}

#[derive(Deserialize, Debug)]
pub struct CommunityGrade {
    pub grade: String,
    pub count: usize,
}

#[derive(Deserialize, Debug)]
pub struct CommunityOpinion {
    pub stars: usize,
    pub votes: usize,
}

#[derive(Deserialize, Debug)]
pub struct ClimbStats {
    pub community_grades: Vec<CommunityGrade>,
    pub community_opinions: Vec<CommunityOpinion>,
    pub toppers: Vec<ClimbTopper>,
}

#[derive(Deserialize, Debug)]
pub struct ClimbTopper {
    pub uid: usize,
    pub full_name: String,
    // avatar: String,
    // top_type: String,
    pub date_logged: String,
    pub checks: usize,
}
