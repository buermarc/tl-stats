//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Deserialize)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: i64,
    pub uid: i64,
    pub anonymous: bool,
    pub avatar: String,
    pub first_name: String,
    pub last_name: String,
    pub gender: String,
    pub score_count_gym: i64,
    pub score_count_gym_boulders: i64,
    pub score_grade_boulders: String,
    pub score_grade_boulders_count: i64,
    pub score_grade_routes: String,
    pub score_grade_routes_count: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
