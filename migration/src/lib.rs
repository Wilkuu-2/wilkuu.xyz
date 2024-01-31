pub use sea_orm_migration::prelude::*;

mod m20240119_130354_create_posts;
mod m20240131_114949_create_user_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240119_130354_create_posts::Migration),
            Box::new(m20240131_114949_create_user_table::Migration),
        ]
    }
}
