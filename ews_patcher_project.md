# EWS IMMO Patcher MS43 - Project Documentation

## Project Overview
A Rust-based binary patcher with imgui-rs GUI for removing EWS (Electronic Immobilizer) from BMW MS43 ECU firmware files. The tool identifies ECU software versions and applies precise byte-level patches to disable immobilizer functionality.

## Technical Background

### MS43 ECU System
- **Target Platform**: BMW MS43 Engine Control Unit
- **File Format**: Binary firmware files (.DAT)
- **Supported Versions**: ca430037, ca430056 (5WK90015), ca430056 (5WK90017), ca430066, ca430069

### Version Detection
- **Location**: `0x70040` in binary file
- **Format**: Starts with "ca" followed by version string (e.g., "ca430037")
- **Implementation**: Read 16 bytes from offset, parse ASCII string

### EWS Delete Mechanism
The patch consists of three critical components that **MUST ALL** be applied:

1. **Jump Patch**: Redirects execution flow out of normal EWS handling subroutine
2. **Code Patch**: Implements EWS bypass program code
3. **DTC Patch**: Suppresses diagnostic trouble code `c_abc_inc_tout_imob` (immobilizer timeout)

⚠️ **Critical**: All three patches must be applied with exact original data validation. Partial patching will cause ECU malfunction.

## Patch Definitions

### ca430037.DAT
| Component | Offset | Original Bytes | Patched Bytes |
|-----------|--------|----------------|---------------|
| Jump | 0x54E8C | `DA 0B 5A 1C` | `DA 0D 0C 35` |
| Code | 0x5350C | `00 00 FF FF FF FF FF FF` | `DA 0B E6 39 6E 18 DB 00` |
| DTC | 0x7099B | `02` | `00` |

### ca430056.DAT (5WK90015)
| Component | Offset | Original Bytes | Patched Bytes |
|-----------|--------|----------------|---------------|
| Jump | 0x57D76 | `DA 0B 40 20` | `DA 0D B2 3B` |
| Code | 0x53BB2 | `00 00 FF FF FF FF FF FF` | `DA 0B B8 3F 9E 19 DB 00` |
| DTC | 0x70A14 | `02` | `00` |

### ca430056.DAT (5WK90017)
| Component | Offset | Original Bytes | Patched Bytes |
|-----------|--------|----------------|---------------|
| Jump | 0x57D76 | `DA 0B 40 20` | `DA 0D B2 3B` |
| Code | 0x53BB2 | `00 00 FF FF FF FF FF FF` | `DA 0B B8 3F 9E 19 DB 00` |
| DTC | 0x70A14 | `02` | `00` |

### ca430066.DAT
| Component | Offset | Original Bytes | Patched Bytes |
|-----------|--------|----------------|---------------|
| Jump | 0x600D8 | `DA 0A 64 DD` | `DA 0D F8 3B` |
| Code | 0x53BF8 | `00 00 FF FF FF FF FF FF` | `DA 0A DC FC 0E 1A DB 00` |
| DTC | 0x70A77 | `02` | `00` |

### ca430069.DAT
| Component | Offset | Original Bytes | Patched Bytes |
|-----------|--------|----------------|---------------|
| Jump | 0x600D8 | `DA 0A 6C DD` | `DA 0D F8 3B` |
| Code | 0x53BF8 | `00 00 FF FF FF FF FF FF` | `DA 0A E4 FC 0E 1A DB 00` |
| DTC | 0x70A6E | `02` | `00` |

## Architecture

### Core Components

#### 1. Version Detection Module
```rust
struct VersionInfo {
    version_string: String,  // e.g., "ca430037"
    hardware_variant: Option<String>, // e.g., "5WK90015", "5WK90017"
}
```
- Read bytes from `0x70040`
- Parse version identifier
- Match against known patch definitions

#### 2. Patch Definition System
```rust
struct Patch {
    name: String,           // "Jump", "Code", "DTC"
    offset: usize,          // Memory location
    original: Vec<u8>,      // Expected original bytes
    patched: Vec<u8>,       // Replacement bytes
}

struct PatchSet {
    version: String,
    patches: Vec<Patch>,    // Always 3 patches (Jump, Code, DTC)
}
```

#### 3. Binary Patcher Engine
- Load entire file into memory
- Validate file size (prevent corruption)
- Verify original bytes before patching
- Apply all three patches atomically
- Create backup with `.original` extension
- Write patched file

#### 4. Safety Validation
```rust
enum ValidationError {
    VersionNotFound,
    OriginalDataMismatch { offset: usize, expected: Vec<u8>, found: Vec<u8> },
    FileTooSmall,
    UnsupportedVersion(String),
}
```

### GUI Design (imgui-rs)

#### Main Window Layout
```
┌─────────────────────────────────────────┐
│   EWS IMMO Patcher MS43                 │
├─────────────────────────────────────────┤
│ File: [________________] [Browse...]    │
│                                         │
│ Detected Version: ca430037              │
│ Hardware: 5WK90015                      │
│                                         │
│ Patch Status:                           │
│   ✓ Jump Patch (0x54E8C)                │
│   ✓ Code Patch (0x5350C)                │
│   ✓ DTC Patch (0x7099B)                 │
│                                         │
│ [Apply Patches]  [Revert]               │
│                                         │
│ Log:                                    │
│ ┌─────────────────────────────────────┐ │
│ │ File loaded successfully...         │ │
│ │ Version detected: ca430037          │ │
│ │ Validating original bytes...        │ │
│ └─────────────────────────────────────┘ │
└─────────────────────────────────────────┘
```

#### Features
- Drag-and-drop file support
- Real-time validation feedback
- Hex viewer for patch locations
- Before/After comparison view
- Progress indication during patching
- Backup management

## Implementation Workflow

### 1. File Loading
```rust
fn load_file(path: &Path) -> Result<Vec<u8>, Error> {
    // Read entire binary
    // Validate minimum size
    // Return buffer
}
```

### 2. Version Detection
```rust
fn detect_version(data: &[u8]) -> Result<VersionInfo, Error> {
    let version_bytes = &data[0x70040..0x70050];
    // Parse "ca" prefix and version number
    // Optional: detect hardware variant (5WK90015 vs 5WK90017)
}
```

### 3. Patch Validation
```rust
fn validate_patches(data: &[u8], patches: &[Patch]) -> Result<(), ValidationError> {
    for patch in patches {
        let actual = &data[patch.offset..patch.offset + patch.original.len()];
        if actual != patch.original {
            return Err(ValidationError::OriginalDataMismatch {
                offset: patch.offset,
                expected: patch.original.clone(),
                found: actual.to_vec(),
            });
        }
    }
    Ok(())
}
```

### 4. Atomic Patching
```rust
fn apply_patches(data: &mut [u8], patches: &[Patch]) -> Result<(), Error> {
    // Validate ALL patches first
    validate_patches(data, patches)?;
    
    // Apply all patches
    for patch in patches {
        data[patch.offset..patch.offset + patch.patched.len()]
            .copy_from_slice(&patch.patched);
    }
    
    Ok(())
}
```

### 5. Backup Strategy
- Always create `.original` backup before patching
- Revert function restores from backup
- Detect already-patched files (check for patched bytes)

## Dependencies

```toml
[dependencies]
imgui = "0.11"
imgui-glium-renderer = "0.11"
glium = "0.34"
winit = "0.29"
rfd = "0.14"  # File dialog
```

## Error Handling Strategy

### User-Facing Errors
1. **File Not Found**: Clear message with path
2. **Unsupported Version**: Display detected version, list supported versions
3. **Original Data Mismatch**: Show hex dump of expected vs actual at specific offset
4. **Already Patched**: Detect and inform user, offer revert option

### Safety Checks
- File size validation (minimum expected size)
- All three patches must validate before any modification
- Atomic write operation (temp file → rename)
- Checksum validation (optional feature)

## Future Enhancements

### Phase 1 (MVP)
- [x] Core patching engine
- [x] Version detection
- [x] Basic GUI
- [x] Backup/Revert functionality

### Phase 2
- [ ] Hex editor view
- [ ] Patch verification (read-back check)
- [ ] Extended version support
- [ ] Checksum recalculation
- [ ] Advanced logging (export to file)

## Testing Strategy

### Unit Tests
- Version string parsing
- Byte validation logic
- Patch application at correct offsets

### Integration Tests
- Full patch workflow with test binaries
- Backup/restore functionality
- Error handling for corrupted files

### Test Files
Create synthetic test files with:
- Known version strings at 0x70040
- Test patterns at patch locations
- Various file sizes

## Security Considerations

⚠️ **IMPORTANT DISCLAIMERS**:
1. This tool modifies ECU firmware - incorrect patches can damage the ECU
2. Always keep original backup files
3. EWS deletion may have legal implications depending on jurisdiction
4. Tool should display warning on first launch
5. Recommend professional installation and testing

## Build Instructions

```bash
# Development
cargo build

# Release (optimized)
cargo build --release

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run
```

## Usage Guidelines for End Users

1. **Before Patching**:
   - Read entire ECU binary from vehicle
   - Verify file integrity
   - Keep original file in safe location

2. **Patching Process**:
   - Load file in patcher
   - Verify detected version matches your ECU
   - Review all three patch locations
   - Apply patches
   - Save patched file with new name

3. **After Patching**:
   - Verify patched file size matches original
   - Write patched binary back to ECU
   - Test ECU functionality on bench before installation
   - Keep both original and patched files

## Known Limitations

- Does not support encrypted firmware
- Requires exact version match
- Cannot auto-detect hardware variants (5WK90015 vs 5WK90017) - may need manual selection
- No checksum recalculation (if ECU validates checksums)

---

## Project Repository Structure

```
ews-immo-patcher/
├── src/
│   ├── main.rs           # Entry point + GUI loop
│   ├── patcher.rs        # Core patching engine
│   ├── version.rs        # Version detection
│   ├── patches.rs        # Patch definitions
│   └── gui/
│       ├── mod.rs
│       ├── main_window.rs
│       └── hex_viewer.rs
├── tests/
│   └── integration_tests.rs
├── Cargo.toml
├── README.md
└── PROJECT.md            # This file
```

## Detailed Module Descriptions

### `src/main.rs`
- Initialize imgui context
- Setup event loop with winit
- Handle file drag-and-drop events
- Coordinate between GUI and patcher engine
- Global error handling

### `src/patcher.rs`
- `load_file()` - Read binary from disk
- `save_file()` - Write patched binary
- `create_backup()` - Backup original file
- `apply_patches()` - Core patching logic
- `verify_patches()` - Post-patch validation
- `revert_patches()` - Restore from backup

### `src/version.rs`
- `detect_version()` - Parse version string from 0x70040
- `parse_hardware_variant()` - Detect 5WK90015 vs 5WK90017
- `get_patch_set()` - Return appropriate patches for version

### `src/patches.rs`
- Hardcoded patch definitions for all versions (no external config files)
- Struct definitions for Patch and PatchSet
```rust
pub fn get_all_patch_sets() -> Vec<PatchSet> {
    vec![
        // ca430037
        PatchSet {
            version: "ca430037".to_string(),
            hardware_variant: None,
            patches: vec![
                Patch {
                    name: "Jump".to_string(),
                    offset: 0x54E8C,
                    original: vec![0xDA, 0x0B, 0x5A, 0x1C],
                    patched: vec![0xDA, 0x0D, 0x0C, 0x35],
                },
                // ... etc
            ],
        },
        // ... other versions
    ]
}
```

### `src/gui/main_window.rs`
- File selection dialog
- Version display
- Patch status indicators
- Action buttons (Apply, Revert)
- Log window with scrollback

### `src/gui/hex_viewer.rs`
- Display binary data in hex format
- Highlight patch locations
- Before/After comparison mode
- Jump to offset functionality

## Hardcoded Patch Definitions

All patches are hardcoded in `src/patches.rs` - no external configuration files needed. This ensures reliability and prevents user error from malformed patch definitions.

## Error Messages Reference

| Code | Message | User Action |
|------|---------|-------------|
| E001 | File not found | Check file path |
| E002 | File too small (< 500KB) | Verify correct ECU dump file |
| E003 | Version not detected at 0x70040 | File may be corrupted or encrypted |
| E004 | Unsupported version: {version} | Check supported versions list |
| E005 | Original data mismatch at {offset} | File may already be patched or wrong version |
| E006 | Hardware variant ambiguous | Manually select 5WK90015 or 5WK90017 |
| E007 | Backup file already exists | Choose to overwrite or use different name |
| E008 | Write permission denied | Check file permissions |

## Development Roadmap

### v0.1.0 - MVP (Current)
- GUI with imgui-rs interface
- Single file patching
- Version detection
- Backup creation
- Real-time validation
- File browser integration

### v0.2.0 - Enhanced Features
- Hex viewer
- Before/after comparison
- Enhanced error reporting
- Log viewer with export

### v1.0.0 - Production Ready
- Checksum calculation/validation
- Comprehensive testing
- User documentation
- Installer/packaging

## Contributing Guidelines

### Code Style
- Follow Rust standard formatting (`cargo fmt`)
- Use `clippy` for linting (`cargo clippy`)
- Write doc comments for public APIs
- Maintain test coverage > 80%

### Pull Request Process
1. Create feature branch
2. Write tests for new functionality
3. Update documentation
4. Run full test suite
5. Submit PR with clear description

### Testing Requirements
- Unit tests for all core functions
- Integration tests for patch workflows
- Test with real ECU dump files (anonymized)
- Manual GUI testing on Windows/Linux/macOS

## License and Legal

**⚠️ LEGAL DISCLAIMER**:
- This tool is for educational and research purposes
- Modifying ECU firmware may void warranties
- EWS deletion may violate vehicle regulations in some jurisdictions
- Users assume all responsibility for use of this software
- Authors are not liable for any damages resulting from use

Suggested License: MIT or GPL-3.0 (to be determined)

## Support and Contact

- **Issues**: GitHub Issues
- **Discussions**: GitHub Discussions
- **Documentation**: Wiki (to be created)
- **Forum**: TBD (BMW tuning community)

---

**Document Version**: 1.0  
**Last Updated**: 2025-11-12  
**Author**: Project Team  
**Status**: Active Development