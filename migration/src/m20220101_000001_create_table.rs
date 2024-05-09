use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Uid).big_integer().not_null())
                    .col(ColumnDef::new(User::Anonymous).boolean().not_null())
                    .col(ColumnDef::new(User::Avatar).string().not_null())
                    .col(ColumnDef::new(User::FirstName).string().not_null())
                    .col(ColumnDef::new(User::LastName).string().not_null())
                    .col(ColumnDef::new(User::Gender).string().not_null())
                    .col(ColumnDef::new(User::ScoreCountGym).big_integer().not_null())
                    .col(
                        ColumnDef::new(User::ScoreCountGymBoulders)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::ScoreGradeBoulders).string().not_null())
                    .col(
                        ColumnDef::new(User::ScoreGradeBouldersCount)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(User::ScoreGradeRoutes).string().not_null())
                    .col(
                        ColumnDef::new(User::ScoreGradeRoutesCount)
                            .big_integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Climb::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Climb::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Climb::ClimbId).big_integer().not_null())
                    .col(ColumnDef::new(Climb::DateLogged).string().not_null())
                    .col(ColumnDef::new(Climb::Checks).big_integer().not_null())
                    .col(ColumnDef::new(Climb::GradeAdj).string().not_null())
                    .col(ColumnDef::new(Climb::Grade).string().not_null())
                    .col(ColumnDef::new(Climb::Number).string())
                    .col(ColumnDef::new(Climb::Color).string().not_null())
                    .col(ColumnDef::new(Climb::ColorSecondary).string())
                    .col(ColumnDef::new(Climb::WallName).string().not_null())
                    .col(ColumnDef::new(Climb::Name).string())
                    .col(ColumnDef::new(Climb::SetterName).string())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(UserToClimb::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserToClimb::Id)
                            .big_integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(UserToClimb::UserId).big_integer().not_null())
                    .col(
                        ColumnDef::new(UserToClimb::ClimbId)
                            .big_integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Climb::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserToClimb::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Anonymous,
    Avatar,
    FirstName,
    Gender,
    Id,
    LastName,
    ScoreCountGym,
    ScoreCountGymBoulders,
    ScoreGradeBoulders,
    ScoreGradeBouldersCount,
    ScoreGradeRoutes,
    ScoreGradeRoutesCount,
    Uid,
}

#[derive(DeriveIden)]
enum Climb {
    Table,
    Id,
    ClimbId,
    DateLogged,
    Checks,
    GradeAdj,
    Grade,
    Number,
    Color,
    ColorSecondary,
    WallName,
    Name,
    SetterName,
}

#[derive(DeriveIden)]
enum UserToClimb {
    Table,
    Id,
    UserId,
    ClimbId,
}
