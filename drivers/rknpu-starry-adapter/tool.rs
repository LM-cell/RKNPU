use rdrive::probe::OnProbeError;
use crate::NonNull;
use alloc::format;



/// Map one MMIO range described by the platform and return it as a non-null
/// byte pointer.
///
/// The platform `iomap` helper returns an address wrapper; the NPU driver keeps
/// raw non-null pointers because lower layers index into register blocks
/// directly. Any platform mapping failure is converted into [`OnProbeError`] so
/// probe can abort cleanly.
pub fn iomap(base: u64, size: usize) -> Result<NonNull<u8>, OnProbeError> {
    axklib::mem::iomap((base as usize).into(), size)
        .map(|ptr| unsafe { NonNull::new_unchecked(ptr.as_mut_ptr()) })
        .map_err(|e| OnProbeError::Other(format!("{e:?}").into()))
}
