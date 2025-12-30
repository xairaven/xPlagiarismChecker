use engine::Database;

#[derive(Debug, Default)]
pub struct EngineContext {
    pub database: Option<Database>,
}
