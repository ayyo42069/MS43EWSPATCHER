//! This module handles the detection of the firmware version from the binary data.

use crate::patches::{PatchSet, PATCH_SETS_MAP};

const VERSION_STRING_OFFSET: usize = 0x70040;
const VERSION_STRING_LENGTH: usize = 16;

/// Custom error types for version detection.
#[derive(Debug, thiserror::Error)]
pub enum VersionError {
    #[error("File is too small to contain a version string.")]
    FileTooSmall,
    #[error("Version string at offset {0:#X} is not valid UTF-8.")]
    InvalidUtf8(usize),
    #[error("Unsupported or unrecognized version. Found: '{0}'")]
    UnsupportedVersion(String),
    #[error("Could not identify firmware version string at offset 0x70040.")]
    UnknownVersion,
}

/// Detects the firmware version from the provided binary data.
///
/// It reads a string from a fixed offset, cleans it, and attempts to match it against a known list of firmware versions.
pub fn detect_version(data: &[u8]) -> Result<&'static PatchSet, VersionError> {
    // 1. Ensure the file is large enough.
    if data.len() < VERSION_STRING_OFFSET + VERSION_STRING_LENGTH {
        return Err(VersionError::FileTooSmall);
    }

    // 2. Read the raw bytes.
    let version_bytes = &data[VERSION_STRING_OFFSET..(VERSION_STRING_OFFSET + VERSION_STRING_LENGTH)];

    // 3. Parse the bytes by taking printable ASCII characters until a null byte is found.
    // This is much more robust than assuming valid UTF-8.
    let version_str_cleaned: String = version_bytes
        .iter()
        .take_while(|&&b| b != 0) // Stop at the first null terminator
        .filter(|&&b| b >= 0x20 && b <= 0x7e) // Filter for printable ASCII range
        .map(|&b| b as char)
        .collect();


    // 4. Check if the cleaned string looks like a version we handle.
    if !version_str_cleaned.starts_with("ca") {
        return Err(VersionError::UnknownVersion);
    }

    // 5. Find the corresponding PatchSet in our map using a more robust check.
    // We check if the cleaned string from the file *starts with* a known version string.
    // This handles cases where the file might have extra garbage after the version number.
    PATCH_SETS_MAP
        .iter()
        .find(|((version_key, _), _)| version_str_cleaned.starts_with(*version_key))
        .map(|(_, patch_set)| patch_set)
        .ok_or_else(|| VersionError::UnsupportedVersion(version_str_cleaned.to_string()))
}

