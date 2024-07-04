pub mod functions;
pub mod models;
pub mod types;

mod consts;
pub use consts::*;

use anyhow::Result;
use log::info;

use sea_orm::{ConnectOptions, ConnectionTrait, Database, Schema};

macro_rules! create_table {
    ($db:path, $builder:path, $table:ident) => {
        $db.execute(
            $builder.build(
                Schema::new($builder)
                    .create_table_from_entity(models::$table::Entity)
                    .if_not_exists(),
            ),
        )
        .await?;
    };
}

pub async fn init() -> Result<()> {
    let mut options = ConnectOptions::new(DATABASE_URL.clone());
    options.sqlx_logging_level(log::LevelFilter::Debug);

    let db = Database::connect(options).await.unwrap();
    let builder = db.get_database_backend();

    create_table!(db, builder, global_config);
    create_table!(db, builder, user);
    create_table!(db, builder, image_file);
    create_table!(db, builder, document_file);
    create_table!(db, builder, thread);

    info!("Database is ready");
    DB_CONN.get_or_init(|| db.to_owned());

    Ok(())
}
