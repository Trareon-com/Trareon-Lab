//! Foundation result alias — callers use `LabResult<T>`, not bare strings.

use crate::error::LabError;

/// Standard result for authoritative lab-core APIs.
pub type LabResult<T> = Result<T, LabError>;
