use core::fmt;

use crate::RknpuError;

/// High-level service errors returned by the scheduler/ioctl integration layer.
///
/// The low-level driver still reports [`RknpuError`]. Service code wraps those
/// driver failures together with userspace-copy, lookup, and blocking-wait
/// failures so OS adapters only need to translate one error enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RknpuServiceError {
    /// Userspace passed an invalid ioctl argument or malformed payload.
    InvalidInput,
    /// Userspace memory could not be copied or validated.
    InvalidData,
    /// The requested submit or object handle does not exist.
    NotFound,
    /// The underlying device or OS resource is temporarily busy.
    Busy,
    /// A blocking wait was interrupted before terminal completion.
    Interrupted,
    /// The ioctl number is not recognized by the RKNPU service.
    BadIoctl,
    /// One low-level driver operation failed.
    Driver(RknpuError),
    /// Internal service invariant was violated.
    Internal,
}

impl RknpuServiceError {
    /// Convert one service-layer failure into the closest driver-facing error.
    ///
    /// Scheduler bookkeeping stores terminal errors as [`RknpuError`], so this
    /// helper is used when an OS/service failure must be reflected back into
    /// submit state.
    pub fn to_driver_error(self) -> RknpuError {
        match self {
            Self::InvalidInput | Self::InvalidData | Self::BadIoctl => RknpuError::InvalidParameter,
            Self::NotFound => RknpuError::InvalidHandle,
            Self::Busy => RknpuError::DeviceBusy,
            Self::Interrupted => RknpuError::Interrupted,
            Self::Driver(err) => err,
            Self::Internal => RknpuError::InternalError,
        }
    }
}

impl From<RknpuError> for RknpuServiceError {
    /// Wrap one low-level driver error as a service-layer error.
    fn from(value: RknpuError) -> Self {
        Self::Driver(value)
    }
}

impl fmt::Display for RknpuServiceError {
    /// Render a short userspace-facing description for one service error.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput => write!(f, "invalid input"),
            Self::InvalidData => write!(f, "invalid data"),
            Self::NotFound => write!(f, "not found"),
            Self::Busy => write!(f, "busy"),
            Self::Interrupted => write!(f, "interrupted"),
            Self::BadIoctl => write!(f, "bad ioctl"),
            Self::Driver(err) => write!(f, "driver error: {err}"),
            Self::Internal => write!(f, "internal error"),
        }
    }
}

impl core::error::Error for RknpuServiceError {}
