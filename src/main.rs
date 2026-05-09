pub mod post;
pub mod profile;
pub mod user;

use std::error::Error;

use sea_orm::{ActiveValue::Set, Database, DatabaseConnection, HasOneModel};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .init();

    info!("Connecting to db...");
    let db: DatabaseConnection = Database::connect("sqlite::memory:").await?;

    db.get_schema_registry("delete-one-to-one::*")
        .sync(&db)
        .await?;

    let user = user::ActiveModelEx {
        name: Set("User A".to_string()),
        profile: HasOneModel::set(profile::ActiveModelEx {
            picture: Set("picture.png".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    }
    .save(&db)
    .await?;

    info!("Inserted user:\n{:#?}", user);

    // Update the inserted user with a different profile
    let updated_user = user::ActiveModelEx {
        id: user.id,
        name: Set("User A renamed".to_string()),
        profile: HasOneModel::set(profile::ActiveModelEx {
            picture: Set("picture changed.png".to_string()),
            ..Default::default()
        }),
        ..Default::default()
    }
    .save(&db)
    .await?;

    // We expect that the old profile will be deleted, and a new
    // one inserted for the same user - instead we get:
    //
    // Error: Query(SqlxError(Database(SqliteError { code: 2067, message: "UNIQUE constraint failed: profile.user_id" })))
    //
    // Note that if we comment out the `profile` fields for either `user`
    // or `updated_user` (or both) in the `save`s above, this does not occur.
    //
    // So there's only a problem when trying to change from one profile to
    // another.
    info!("Updated user:\n{:#?}", updated_user);

    // New feature: It would be great to be able to delete the profile with
    // something like the commented code below:

    // Delete the user's profile
    // let updated_user = user::ActiveModelEx {
    //     id: user.id,
    //     name: Set("User A profile deleted".to_string()),
    //     profile: HasOneModel::Delete, // Note `Delete` doesn't exist
    //     ..Default::default()
    // }
    // .save(&db)
    // .await?;

    Ok(())
}
