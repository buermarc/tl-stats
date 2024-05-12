use anyhow::Result;

use migration::{Migrator, MigratorTrait};
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, Database, DbConn, EntityTrait, QueryFilter};

use entity::climb::{self, Model as Climb};
use entity::user::{self, Model as User};
use entity::user_to_climb;

// TODO: write advanced queries e.g. sqlite> select avg(grade) from climb inner join user_to_climb on climb.climb_id = user_to_climb.climb_id where user_id = (select uid from user where first_name like '%username%');

pub async fn establish_connection(database_url: &str) -> Result<DbConn> {
    let db = Database::connect(database_url)
        .await
        .expect("Failed to setup the database");
    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations for tests");

    Ok(db)
}

pub async fn insert_or_update_user_info(db: &DbConn, user: User) -> Result<()> {
    match user::Entity::find_by_id(user.id).one(db).await? {
        Some(_) => {
            user::Entity::update::<user::ActiveModel>(user.into())
                .exec(db)
                .await?;
        }
        None => {
            user::Entity::insert::<user::ActiveModel>(user.into())
                .exec(db)
                .await?;
        }
    }
    Ok(())
}

pub async fn set_climb_topped(db: &DbConn, climb: &Climb, user_id: usize) -> Result<()> {
    let entry = user_to_climb::ActiveModel {
        user_id: Set(user_id as i64),
        climb_id: Set(climb.climb_id),
        ..Default::default()
    };
    match user_to_climb::Entity::find()
        .filter(user_to_climb::Column::ClimbId.eq(climb.climb_id))
        .one(db)
        .await?
    {
        Some(_) => {
            user_to_climb::Entity::update::<user_to_climb::ActiveModel>(entry)
                .exec(db)
                .await?;
        }
        None => {
            user_to_climb::Entity::insert::<user_to_climb::ActiveModel>(entry)
                .exec(db)
                .await?;
        }
    }
    Ok(())
}

pub async fn insert_or_update_climb(db: &DbConn, climb: Climb) -> Result<()> {
    match climb::Entity::find()
        .filter(climb::Column::ClimbId.eq(climb.climb_id))
        .one(db)
        .await?
    {
        Some(_) => {
            climb::Entity::update::<climb::ActiveModel>(climb.into())
                .exec(db)
                .await?;
        }
        None => {
            climb::Entity::insert::<climb::ActiveModel>(climb.into())
                .exec(db)
                .await?;
        }
    }
    Ok(())
}

pub async fn select_users(db: &DbConn) -> Result<Vec<User>> {
    let users = user::Entity::find().all(db).await?;
    return Ok(users);
}
