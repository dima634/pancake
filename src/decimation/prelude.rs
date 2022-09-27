use super::edge_decimation::{IncrementalDecimator, QuadricError};

/// Mesh decimation through edge collapsing. For details see [IncrementalDecimator].
pub type EdgeDecimator<TMesh> = IncrementalDecimator<TMesh, QuadricError<TMesh>>;
