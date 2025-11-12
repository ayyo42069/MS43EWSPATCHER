//! This module contains the core logic for applying and reverting patches to the firmware binary.

use crate::patches::{Patch, PatchSet};

#[derive(Debug, thiserror::Error)]
pub enum PatcherError {
    #[error("Validation failed: Original data mismatch at offset {offset:#X}. Expected {expected:02X?}, found {found:02X?}. The file may be of the wrong version or already modified.")]
    ValidationMismatch {
        offset: usize,
        expected: Vec<u8>,
        found: Vec<u8>,
    },
    #[error("File is too small to apply patch '{patch_name}' at offset {offset:#X}.")]
    FileTooSmall {
        patch_name: &'static str,
        offset: usize,
    },
}

/// Validates that the original bytes in the data slice match the expected original bytes for all patches in the set.
///
/// # Arguments
///
/// * `data` - The binary data of the firmware file.
/// * `patch_set` - The set of patches to validate against.
///
/// # Returns
///
/// * `Ok(())` if all original bytes match.
/// * `Err(PatcherError)` if there is a mismatch or the file is too small.
pub fn validate_pre_patch(data: &[u8], patch_set: &PatchSet) -> Result<(), PatcherError> {
    for patch in &patch_set.patches {
        let end_offset = patch.offset + patch.original.len();
        if data.len() < end_offset {
            return Err(PatcherError::FileTooSmall { patch_name: patch.name, offset: patch.offset });
        }

        let actual_bytes = &data[patch.offset..end_offset];
        if actual_bytes != patch.original.as_slice() {
            return Err(PatcherError::ValidationMismatch {
                offset: patch.offset,
                expected: patch.original.clone(),
                found: actual_bytes.to_vec(),
            });
        }
    }
    Ok(())
}

/// Applies the patches to the firmware data after validation.
///
/// This function first validates the data and then applies all patches.
///
/// # Arguments
///
/// * `data` - A mutable slice of the firmware binary data.
/// * `patch_set` - The `PatchSet` to apply.
///
/// # Returns
///
/// * `Ok(())` on success.
/// * `Err(PatcherError)` if validation fails.
pub fn apply_patches(data: &mut [u8], patch_set: &PatchSet) -> Result<Vec<String>, PatcherError> {
    // First, ensure the file is in the expected state before modifying anything.
    validate_pre_patch(data, patch_set)?;

    let mut logs = Vec::new();

    // If validation passes, apply all patches.
    for patch in &patch_set.patches {
        let end_offset = patch.offset + patch.patched.len();
        if data.len() < end_offset {
            // This check is somewhat redundant due to validate_pre_patch, but good for safety.
             return Err(PatcherError::FileTooSmall { patch_name: patch.name, offset: patch.offset });
        }
        data[patch.offset..end_offset].copy_from_slice(&patch.patched);
        logs.push(format!("  Applied {} patch at offset {:#X}", patch.name, patch.offset));
    }

    Ok(logs)
}


/// Reverts the patches from the firmware data.
///
/// This function validates that the data is currently patched, then restores the original bytes.
///
/// # Arguments
///
/// * `data` - A mutable slice of the firmware binary data.
/// * `patch_set` - The `PatchSet` that was originally applied.
///
/// # Returns
///
/// * `Ok(Vec<String>)` on success with a vector of log messages.
/// * `Err(PatcherError)` if the data does not appear to be patched as expected.
pub fn revert_patches(data: &mut [u8], patch_set: &PatchSet) -> Result<Vec<String>, PatcherError> {
    // Validate that the file is currently in a patched state before reverting.
    for patch in &patch_set.patches {
        let end_offset = patch.offset + patch.patched.len();
        if data.len() < end_offset {
            return Err(PatcherError::FileTooSmall { patch_name: patch.name, offset: patch.offset });
        }

        let actual_bytes = &data[patch.offset..end_offset];
        if actual_bytes != patch.patched.as_slice() {
            return Err(PatcherError::ValidationMismatch {
                offset: patch.offset,
                expected: patch.patched.clone(),
                found: actual_bytes.to_vec(),
            });
        }
    }

    let mut logs = Vec::new();

    // If validation passes, revert all patches.
    for patch in &patch_set.patches {
        let end_offset = patch.offset + patch.original.len();
         if data.len() < end_offset {
             return Err(PatcherError::FileTooSmall { patch_name: patch.name, offset: patch.offset });
        }
        data[patch.offset..end_offset].copy_from_slice(&patch.original);
        logs.push(format!("  Reverted {} patch at offset {:#X}", patch.name, patch.offset));
    }

    Ok(logs)
}


/// Represents the state of a single patch location in the file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatchStatus {
    /// The file bytes match the `patched` state.
    Patched,
    /// The file bytes match the `original` state.
    Unpatched,
    /// The file bytes match neither the original nor the patched state.
    Unknown,
}

/// Checks the status of each patch in the set against the provided data.
///
/// # Returns
///
/// A tuple `(PatchStatus, PatchStatus, PatchStatus)` corresponding to the status of (Jump, Code, DTC).
pub fn check_patch_status(data: &[u8], patch_set: &PatchSet) -> (PatchStatus, PatchStatus, PatchStatus) {
    let mut status = (PatchStatus::Unknown, PatchStatus::Unknown, PatchStatus::Unknown);

    if let Some(jump_patch) = patch_set.patches.iter().find(|p| p.name == "Jump") {
        status.0 = get_patch_status(data, jump_patch);
    }

    if let Some(code_patch) = patch_set.patches.iter().find(|p| p.name == "Code") {
        status.1 = get_patch_status(data, code_patch);
    }

    if let Some(dtc_patch) = patch_set.patches.iter().find(|p| p.name == "DTC") {
        status.2 = get_patch_status(data, dtc_patch);
    }

    status
}

/// Helper function to determine the status of a single patch.
fn get_patch_status(data: &[u8], patch: &Patch) -> PatchStatus {
    // Check against patched bytes first. Note that lengths can differ.
    let patched_end = patch.offset + patch.patched.len();
    if data.len() >= patched_end {
        if &data[patch.offset..patched_end] == patch.patched.as_slice() {
            return PatchStatus::Patched;
        }
    }

    // Check against original bytes.
    let original_end = patch.offset + patch.original.len();
    if data.len() >= original_end {
        if &data[patch.offset..original_end] == patch.original.as_slice() {
            return PatchStatus::Unpatched;
        }
    }

    PatchStatus::Unknown
}

