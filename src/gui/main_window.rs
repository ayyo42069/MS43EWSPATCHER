use crate::patcher::{self, check_patch_status, PatchStatus};
use crate::patches::PatchSet;
use crate::version::detect_version;
use imgui::{Condition, StyleVar, Ui};
use std::fs;

pub struct AppState {
    pub file_path: String,
    pub file_data: Option<Vec<u8>>,
    pub patch_set: Option<&'static PatchSet>,
    pub selected_patch_index: Option<usize>,
    pub detected_version: String,
    pub hardware_variant: String,
    pub patch_status: (PatchStatus, PatchStatus, PatchStatus), // Jump, Code, DTC
    pub log: Vec<String>,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            file_path: String::new(),
            file_data: None,
            patch_set: None,
            selected_patch_index: None,
            detected_version: "N/A".to_string(),
            hardware_variant: "N/A".to_string(),
            patch_status: (PatchStatus::Unknown, PatchStatus::Unknown, PatchStatus::Unknown),
            log: vec!["Welcome to EWS IMMO Patcher MS43!".to_string()],
        }
    }
}

fn reset_state(app_state: &mut AppState) {
    app_state.file_data = None;
    app_state.patch_set = None;
    app_state.selected_patch_index = None;
    app_state.detected_version = "N/A".to_string();
    app_state.hardware_variant = "N/A".to_string();
    app_state.patch_status = (PatchStatus::Unknown, PatchStatus::Unknown, PatchStatus::Unknown);
}

fn log_color(message: &str) -> [f32; 4] {
    if message.starts_with("Success") {
        [0.2, 0.8, 0.2, 1.0]
    } else if message.starts_with("Error") || message.starts_with("Failed") {
        [1.0, 0.2, 0.2, 1.0]
    } else {
        [0.7, 0.7, 0.7, 1.0]
    }
}

/// Converts a byte slice to a formatted, spaced-out hex string.
fn bytes_to_hex_string(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02X}", b))
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn render_main_window(ui: &mut Ui, app_state: &mut AppState) {
    let display_size = ui.io().display_size;
    ui.window("EWS Patcher")
        .size(display_size, Condition::Always)
        .position([0.0, 0.0], Condition::Always)
        .flags(
            imgui::WindowFlags::NO_TITLE_BAR
                | imgui::WindowFlags::NO_RESIZE
                | imgui::WindowFlags::NO_MOVE
                | imgui::WindowFlags::NO_COLLAPSE
                | imgui::WindowFlags::NO_SAVED_SETTINGS,
        )
        .build(|| {
            let main_content_width = display_size[0] * 0.6;
            ui.child_window("MainContent")
                .size([main_content_width, 0.0])
                .build(|| {
                    // Top section for file selection
                    ui.child_window("FileSelection")
                        .size([0.0, 80.0])
                        .build(|| {
                            ui.text("Firmware File");
                            let _style = ui.push_style_var(StyleVar::FrameRounding(4.0));
                            ui.input_text("##file_path", &mut app_state.file_path)
                                .read_only(true)
                                .build();
                            ui.same_line();
                            if ui.button("Browse...") {
                                if let Some(path) = rfd::FileDialog::new().add_filter("Binary firmware files", &["bin", "dat"]).pick_file() {
                                    let file_path_str = path.display().to_string();
                                    app_state.log.push(format!("Loading file: {}", file_path_str));
                                    reset_state(app_state); // Reset state before loading new file
                                    app_state.file_path = file_path_str; // Keep file path after reset

                                    match fs::read(&path) {
                                        Ok(data) => {
                                            app_state.log.push(format!("Successfully read {} bytes.", data.len()));
                                            match detect_version(&data) {
                                                Ok(patch_set) => {
                                                    app_state.log.push(format!("Success: Detected version '{}'", patch_set.version_string));
                                                    app_state.detected_version = patch_set.version_string.to_string();
                                                    app_state.hardware_variant = patch_set.hardware_variant.unwrap_or("N/A").to_string();
                                                    app_state.patch_status = check_patch_status(&data, patch_set);
                                                    app_state.patch_set = Some(patch_set);
                                                    app_state.file_data = Some(data);
                                                }
                                                Err(e) => app_state.log.push(format!("Error: Version detection failed: {}", e)),
                                            }
                                        }
                                        Err(e) => app_state.log.push(format!("Error: Failed to read file: {}", e)),
                                    }
                                }
                            }
                        });

                    // Middle section for status and actions
                    ui.child_window("StatusAndActions")
                        .size([0.0, 150.0])
                        .build(|| {
                            ui.text(format!("Detected Version: {}", app_state.detected_version));
                            ui.text("Patch Status (click to view diff):");

                            let (jump, code, dtc) = app_state.patch_status;
                            let green = [0.1, 0.9, 0.1, 1.0];
                            let red = [0.9, 0.1, 0.1, 1.0];
                            let grey = [0.5, 0.5, 0.5, 1.0];

                            let (jump_char, jump_color) = match jump {
                                PatchStatus::Patched => ('✓', green),
                                PatchStatus::Unpatched => ('✗', grey),
                                PatchStatus::Unknown => ('?', red),
                            };
                            let (code_char, code_color) = match code {
                                PatchStatus::Patched => ('✓', green),
                                PatchStatus::Unpatched => ('✗', grey),
                                PatchStatus::Unknown => ('?', red),
                            };
                            let (dtc_char, dtc_color) = match dtc {
                                PatchStatus::Patched => ('✓', green),
                                PatchStatus::Unpatched => ('✗', grey),
                                PatchStatus::Unknown => ('?', red),
                            };

                            let _jump_color = ui.push_style_color(imgui::StyleColor::Text, jump_color);
                            if ui.selectable_config(format!("  {} Jump Patch", jump_char))
                                .selected(app_state.selected_patch_index == Some(0))
                                .build() {
                                app_state.selected_patch_index = Some(0);
                            }
                            
                            let _code_color = ui.push_style_color(imgui::StyleColor::Text, code_color);
                            if ui.selectable_config(format!("  {} Code Patch", code_char))
                                .selected(app_state.selected_patch_index == Some(1))
                                .build() {
                                app_state.selected_patch_index = Some(1);
                            }

                            let _dtc_color = ui.push_style_color(imgui::StyleColor::Text, dtc_color);
                            if ui.selectable_config(format!("  {} DTC Patch", dtc_char))
                                .selected(app_state.selected_patch_index == Some(2))
                                .build() {
                                app_state.selected_patch_index = Some(2);
                            }

                            ui.spacing();
                            ui.separator();
                            ui.spacing();

                            let can_apply = matches!(app_state.patch_status, (PatchStatus::Unpatched, PatchStatus::Unpatched, PatchStatus::Unpatched));
                            let can_revert = matches!(app_state.patch_status, (PatchStatus::Patched, PatchStatus::Patched, PatchStatus::Patched));

                            let button_size = [120.0, 30.0];
                            let content_width = ui.content_region_avail()[0];
                            let buttons_total_width = button_size[0] * 2.0 + unsafe { ui.style() }.item_spacing[0];
                            let cursor_x = (content_width - buttons_total_width) * 0.5;
                            if cursor_x > 0.0 {
                                ui.set_cursor_pos([cursor_x, ui.cursor_pos()[1]]);
                            }

                            ui.disabled(!can_apply, || {
                                if ui.button_with_size("Apply Patches", button_size) {
                                    if let (Some(data), Some(patch_set)) = (app_state.file_data.as_mut(), app_state.patch_set) {
                                        match patcher::apply_patches(data, patch_set) {
                                            Ok(logs) => {
                                                app_state.log.push("Success: Patches applied.".to_string());
                                                app_state.log.extend(logs);
                                                if let Some(save_path) = rfd::FileDialog::new().set_file_name("patched_firmware.bin").save_file() {
                                                    match fs::write(&save_path, &*data) {
                                                        Ok(()) => {
                                                            app_state.log.push(format!("Success: Patched file saved to {}", save_path.display()));
                                                            app_state.patch_status = (PatchStatus::Patched, PatchStatus::Patched, PatchStatus::Patched);
                                                        }
                                                        Err(e) => app_state.log.push(format!("Error: Failed to save file: {}", e)),
                                                    }
                                                } else {
                                                    app_state.log.push("Save operation cancelled.".to_string());
                                                }
                                            }
                                            Err(e) => app_state.log.push(format!("Error applying patches: {}", e)),
                                        }
                                    }
                                }
                            });
                            ui.same_line();
                            ui.disabled(!can_revert, || {
                                if ui.button_with_size("Revert", button_size) {
                                    if let (Some(data), Some(patch_set)) = (app_state.file_data.as_mut(), app_state.patch_set) {
                                        match patcher::revert_patches(data, patch_set) {
                                            Ok(logs) => {
                                                app_state.log.push("Success: Patches reverted.".to_string());
                                                app_state.log.extend(logs);
                                                if let Some(save_path) = rfd::FileDialog::new().set_file_name("reverted_firmware.bin").save_file() {
                                                    match fs::write(&save_path, &*data) {
                                                        Ok(()) => {
                                                            app_state.log.push(format!("Success: Reverted file saved to {}", save_path.display()));
                                                            app_state.patch_status = (PatchStatus::Unpatched, PatchStatus::Unpatched, PatchStatus::Unpatched);
                                                        }
                                                        Err(e) => app_state.log.push(format!("Error: Failed to save file: {}", e)),
                                                    }
                                                } else {
                                                    app_state.log.push("Save operation cancelled.".to_string());
                                                }
                                            }
                                            Err(e) => app_state.log.push(format!("Error reverting patches: {}", e)),
                                        }
                                    }
                                }
                            });
                        });

                    // Bottom section for logs
                    ui.child_window("Log")
                        .size([0.0, 0.0])
                        .border(true)
                        .build(|| {
                            ui.text("Log");
                            ui.separator();
                            let _log_rounding = ui.push_style_var(StyleVar::FrameRounding(4.0));
                            ui.child_window("LogContent")
                                .build(|| {
                                    for message in &app_state.log {
                                        let color = log_color(message);
                                        ui.text_colored(color, message);
                                    }
                                    if ui.cursor_pos()[1] > ui.window_content_region_max()[1] {
                                        ui.set_scroll_here_y_with_ratio(1.0);
                                    }
                                });
                        });
                });

            ui.same_line();

            // Right column for Hex Viewer
            let _style = ui.push_style_var(StyleVar::WindowPadding([10.0, 10.0]));
            ui.child_window("HexViewer")
                .size([0.0, 0.0])
                .border(true)
                .build(|| {
                    if let (Some(patch_set), Some(index)) = (app_state.patch_set, app_state.selected_patch_index) {
                        if let Some(patch) = patch_set.patches.get(index) {
                            ui.text(format!("Diff for '{}' at offset {:#X}", patch.name, patch.offset));
                            ui.separator();

                            let original_hex = bytes_to_hex_string(&patch.original);
                            let patched_hex = bytes_to_hex_string(&patch.patched);

                            ui.text("Original:");
                            let _green = ui.push_style_color(imgui::StyleColor::Text, [0.9, 0.1, 0.1, 1.0]);
                            ui.text_wrapped(&original_hex);

                            ui.spacing();

                            ui.text("Patched:");
                            let _red = ui.push_style_color(imgui::StyleColor::Text, [0.1, 0.9, 0.1, 1.0]);
                            ui.text_wrapped(&patched_hex);

                        } else {
                            ui.text("No patch selected.");
                        }
                    } else {
                        ui.text("Load a file and select a patch to view differences.");
                    }
                });
        });
}

