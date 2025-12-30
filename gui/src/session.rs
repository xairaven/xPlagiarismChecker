use engine::Database;

#[derive(Debug, Default)]
pub struct Session {
    pub database: Option<Database>,
}
