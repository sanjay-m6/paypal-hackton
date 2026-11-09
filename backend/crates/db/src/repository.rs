/// Marker trait for repository implementations.
///
/// All concrete repositories must implement this trait to ensure they are
/// safe to share across async task boundaries.
pub trait Repository: Send + Sync {}
