use sea_orm_migration::prelude::*;

//mod m20220120_000001_create_post_table;

struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
	    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20220120_000001_create_post_table::Migration)]
    }
}

#[async_std::main]
async fn main() {
	cli::run_cli(Migrator).await;
}