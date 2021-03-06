mod utils;

#[cfg(target_os="windows")]
#[cfg(feature = "openvr")]
mod openvr;
#[cfg(target_os="windows")]
#[cfg(feature = "openvr")]
pub use self::openvr::OpenVRServiceCreator;


#[cfg(feature = "mock")]
mod mock;
#[cfg(feature = "mock")]
pub use self::mock::MockServiceCreator;

#[cfg(feature = "googlevr")]
mod googlevr;
#[cfg(feature = "googlevr")]
pub use self::googlevr::GoogleVRServiceCreator;
#[cfg(all(feature = "googlevr", target_os= "android"))]
pub use self::googlevr::jni::*;
