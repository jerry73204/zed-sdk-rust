#[cfg(target_env = "msvc")]
pub const MAX_CAMERA_PLUGIN: usize = 20;

#[cfg(not(target_env = "msvc"))]
pub const MAX_CAMERA_PLUGIN: usize = 4;
