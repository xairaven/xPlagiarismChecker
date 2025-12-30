// Re-exports
/// Library error type. Critical
pub use errors::LibError;
/// Library error type for bad files, can be shown for users
pub use io::FileError;
/// Loading submissions from files
pub use io::FileLoader;
/// Database model
pub use models::database::Database;

// Modules
pub mod errors;
pub mod io;
pub mod models;
