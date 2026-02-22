#[cfg(target_os = "macos")]
pub mod appkit;

#[cfg(target_os = "macos")]
pub use appkit::implementation::AppkitEventPump as EventPumpImpl;
#[cfg(target_os = "macos")]
pub use appkit::implementation::AppkitWindow as WindowImpl;
#[cfg(target_os = "macos")]
pub use appkit::implementation::AppkitWindowManager as WindowManagerImpl;
