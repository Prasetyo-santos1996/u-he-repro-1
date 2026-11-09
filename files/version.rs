//! Static build info for U He Repro 1.

pub const BUILD_VERSION: &str  = "v2.6.29";
pub const BUILD_CHANNEL: &str  = "beta";
pub const BUILD_COMMIT: &str   = "c51a95da6ad6";
pub const BUILD_TIMESTAMP: u64 = 1780252923;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "U He Repro 1", BUILD_VERSION, BUILD_CHANNEL)
}
