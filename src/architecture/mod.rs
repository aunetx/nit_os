//! Architecture-specific parts of the kernel, such as `init`.
//!
//! Enables testing-specific features.
//!
//! The architecture should be provided thanks to features. Default one is `amd64`.
//!

// ! ------------- testing -------------
// FIXME prevent loading module `testing` when not in test
pub mod testing;

// ! ------------- QEMU -------------
#[cfg(feature = "qemu")]
pub mod qemu;

// ! ------------- amd64 -------------
#[cfg(feature = "amd64")]
pub mod amd64;
#[cfg(feature = "amd64")]
pub use amd64::*;

// ! ------------- ERROR -------------
#[cfg(not(any(feature = "amd64")))]
/// Initialize architecture-specific parts of the kernel.
///
/// ## Panic
///
/// This function will be called if the architecture is not recognized yet.
///
/// Therefore, it will unconditionally panic.
pub fn init() {
    compile_error!("architecture not supported yet")
}
