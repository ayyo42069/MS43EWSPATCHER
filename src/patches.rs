//! This module defines the data structures for patches and contains the hardcoded patch data for each supported firmware version.

use std::collections::HashMap;
use lazy_static::lazy_static;

/// Represents a single modification in the binary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Patch {
    pub name: &'static str,
    pub offset: usize,
    pub original: Vec<u8>,
    pub patched: Vec<u8>,
}

/// Represents a complete set of patches for a specific firmware version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatchSet {
    pub version_string: &'static str,
    pub hardware_variant: Option<&'static str>,
    pub patches: Vec<Patch>,
}

/// Returns a list of all supported patch sets.
pub fn get_all_patch_sets() -> Vec<PatchSet> {
    vec![
        PatchSet {
            version_string: "ca430037",
            hardware_variant: None,
            patches: vec![
                Patch { name: "Jump", offset: 0x54E8C, original: vec![0xDA, 0x0B, 0x5A, 0x1C], patched: vec![0xDA, 0x0D, 0x0C, 0x35] },
                Patch { name: "Code", offset: 0x5350C, original: vec![0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], patched: vec![0xDA, 0x0B, 0xE6, 0x39, 0x6E, 0x18, 0xDB, 0x00] },
                Patch { name: "DTC", offset: 0x7099B, original: vec![0x02], patched: vec![0x00] },
            ],
        },
        PatchSet {
            version_string: "ca430056",
            hardware_variant: Some("5WK90015"),
            patches: vec![
                Patch { name: "Jump", offset: 0x57D76, original: vec![0xDA, 0x0B, 0x40, 0x20], patched: vec![0xDA, 0x0D, 0xB2, 0x3B] },
                Patch { name: "Code", offset: 0x53BB2, original: vec![0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], patched: vec![0xDA, 0x0B, 0xB8, 0x3F, 0x9E, 0x19, 0xDB, 0x00] },
                Patch { name: "DTC", offset: 0x70A14, original: vec![0x02], patched: vec![0x00] },
            ],
        },
        PatchSet {
            version_string: "ca430056",
            hardware_variant: Some("5WK90017"),
            patches: vec![
                Patch { name: "Jump", offset: 0x57D76, original: vec![0xDA, 0x0B, 0x40, 0x20], patched: vec![0xDA, 0x0D, 0xB2, 0x3B] },
                Patch { name: "Code", offset: 0x53BB2, original: vec![0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], patched: vec![0xDA, 0x0B, 0xB8, 0x3F, 0x9E, 0x19, 0xDB, 0x00] },
                Patch { name: "DTC", offset: 0x70A14, original: vec![0x02], patched: vec![0x00] },
            ],
        },
        PatchSet {
            version_string: "ca430066",
            hardware_variant: None,
            patches: vec![
                Patch { name: "Jump", offset: 0x600D8, original: vec![0xDA, 0x0A, 0x64, 0xDD], patched: vec![0xDA, 0x0D, 0xF8, 0x3B] },
                Patch { name: "Code", offset: 0x53BF8, original: vec![0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], patched: vec![0xDA, 0x0A, 0xDC, 0xFC, 0x0E, 0x1A, 0xDB, 0x00] },
                Patch { name: "DTC", offset: 0x70A77, original: vec![0x02], patched: vec![0x00] },
            ],
        },
        PatchSet {
            version_string: "ca430069",
            hardware_variant: None,
            patches: vec![
                Patch { name: "Jump", offset: 0x600D8, original: vec![0xDA, 0x0A, 0x6C, 0xDD], patched: vec![0xDA, 0x0D, 0xF8, 0x3B] },
                Patch { name: "Code", offset: 0x53BF8, original: vec![0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], patched: vec![0xDA, 0x0A, 0xE4, 0xFC, 0x0E, 0x1A, 0xDB, 0x00] },
                Patch { name: "DTC", offset: 0x70A6E, original: vec![0x02], patched: vec![0x00] },
            ],
        },
    ]
}

// A lazily-initialized HashMap for quick lookups of patch sets by version string.
lazy_static! {
    pub static ref PATCH_SETS_MAP: HashMap<(&'static str, Option<&'static str>), PatchSet> = {
        let mut m = HashMap::new();
        for patch_set in get_all_patch_sets() {
            m.insert((patch_set.version_string, patch_set.hardware_variant), patch_set);
        }
        m
    };
}
