pub use sea_orm_migration::prelude::*;

mod m20230722_201310_create_user;
mod m20230724_015050_create_restaurant;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230722_201310_create_user::Migration),
            Box::new(m20230724_015050_create_restaurant::Migration),
        ]
    }
}
