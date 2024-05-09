use anyhow::Result;

use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DbConn, EntityTrait};

use entity::user::{self, Model as User};

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

pub async fn select_users(db: &DbConn) -> Result<Vec<User>> {
    let users = user::Entity::find().all(db).await?;
    return Ok(users);
}
