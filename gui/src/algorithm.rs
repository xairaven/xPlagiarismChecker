use engine::Database;

#[derive(Debug, Default)]
pub struct AlgorithmContext {
    pub database: Option<Database>,
}
