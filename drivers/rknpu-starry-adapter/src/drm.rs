use core::ffi::{c_char, c_int, c_ulong};

/// IOCTL number bits
const IOC_NRBITS: u32 = 8;
/// IOCTL number mask
const IOC_NRMASK: u32 = (1 << IOC_NRBITS) - 1;
/// IOCTL type bits
const IOC_TYPEBITS: u32 = 8;
/// IOCTL size bits
const IOC_SIZEBITS: u32 = 14;
/// IOCTL number shift
const IOC_NRSHIFT: u32 = 0;
/// IOCTL type shift
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
/// IOCTL size shift
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
/// IOCTL size mask
const IOC_SIZEMASK: u32 = (1 << IOC_SIZEBITS) - 1;
/// Linux DRM ioctl type (`'d'`).
const DRM_IOCTL_TYPE: u32 = b'd' as u32;
/// Legacy RKNPU ioctl type (`'r'`).
const RKNPU_IOCTL_TYPE: u32 = b'r' as u32;
/// Base value for DRM ioctl commands
const DRM_COMMAND_BASE: u32 = 0x40;
/// One past the last RKNPU command currently handled by the driver.
const RKNPU_COMMAND_END: u32 = 0x09;

/// DRM version information structure, corresponds to Linux's `struct
/// drm_version` Used for ioctl: DRM_IOCTL_VERSION
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub struct DrmVersion {
    /// Major version
    pub version_major: c_int,
    /// Minor version
    pub version_minor: c_int,
    /// Patch level
    pub version_patchlevel: c_int,
    /// Length of name buffer
    pub name_len: c_ulong,
    /// Pointer to user-space buffer holding driver name
    pub name: *mut c_char,
    /// Length of date buffer
    pub date_len: c_ulong,
    /// Pointer to user-space buffer holding build date
    pub date: *mut c_char,
    /// Length of description buffer
    pub desc_len: c_ulong,
    /// Pointer to user-space buffer holding description
    pub desc: *mut c_char,
}

/// Extracts the ioctl command number from a DRM ioctl command
pub fn ioctl_nr(cmd: u32) -> u32 {
    cmd & IOC_NRMASK
}

/// Extracts the ioctl type (also called the ioctl magic) from a command.
pub fn ioctl_type(cmd: u32) -> u32 {
    (cmd >> IOC_TYPESHIFT) & ((1 << IOC_TYPEBITS) - 1)
}

/// Checks whether an ioctl command uses one of the supported RKNPU encodings.
///
/// RKNPU userspace exists in two forms: Linux DRM clients use type `'d'` and
/// command numbers starting at `DRM_COMMAND_BASE`, while the RKNN runtime uses
/// the legacy type `'r'` with zero-based command numbers.  The number alone is
/// ambiguous because raw RKNPU ACTION (`'r'`, 0) and DRM VERSION (`'d'`, 0)
/// share the same value.
pub fn is_driver_ioctl(cmd: u32) -> bool {
    let nr = ioctl_nr(cmd);
    match ioctl_type(cmd) {
        RKNPU_IOCTL_TYPE => nr < RKNPU_COMMAND_END,
        DRM_IOCTL_TYPE => (DRM_COMMAND_BASE..DRM_COMMAND_BASE + RKNPU_COMMAND_END).contains(&nr),
        _ => false,
    }
}

/// Extracts the size of the data structure from a DRM ioctl command
pub fn io_size(cmd: u32) -> u32 {
    (cmd >> IOC_SIZESHIFT) & IOC_SIZEMASK
}

#[cfg(test)]
mod tests {
    use super::is_driver_ioctl;

    #[test]
    fn raw_rknpu_action_is_a_driver_ioctl() {
        assert!(is_driver_ioctl(0xc008_7200));
    }

    #[test]
    fn drm_encoded_rknpu_action_is_a_driver_ioctl() {
        assert!(is_driver_ioctl(0xc008_6440));
    }

    #[test]
    fn drm_version_is_not_a_driver_ioctl() {
        assert!(!is_driver_ioctl(0xc040_6400));
    }

    #[test]
    fn unrelated_ioctl_type_is_not_a_driver_ioctl() {
        assert!(!is_driver_ioctl(0xc008_7300));
    }
}
