use mysql::*;

pub fn get_conn_builder() -> std::result::Result<OptsBuilder, Box<dyn std::error::Error>> {
    let mut builder = OptsBuilder::new();
    builder = builder.ip_or_hostname(Some(std::env::var("DB_HOST")?));
    builder = builder.db_name(Some(std::env::var("DB_NAME")?));
    builder = builder.user(Some(std::env::var("DB_USERNAME")?));
    builder = builder.pass(Some(std::env::var("DB_PASSWORD")?));
    Ok(builder)
}