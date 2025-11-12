mod gui;
mod patches;
mod patcher;
mod version;

use crate::gui::main_window::{render_main_window, AppState};
use glium::backend::glutin::SimpleWindowBuilder;
use glium::Surface;
use imgui::{Context, FontSource, StyleColor};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::WinitPlatform;
use std::time::Instant;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{EventLoop};

fn apply_custom_style(ctx: &mut Context) {
    let style = ctx.style_mut();
    style.window_padding = [15.0, 15.0];
    style.frame_padding = [8.0, 4.0];
    style.item_spacing = [10.0, 8.0];
    style.item_inner_spacing = [6.0, 6.0];
    style.window_rounding = 8.0;
    style.frame_rounding = 4.0;
    style.child_rounding = 4.0;
    style.grab_rounding = 4.0;
    style.popup_rounding = 4.0;
    style.scrollbar_rounding = 6.0;
    style.tab_rounding = 4.0;

    style.colors[StyleColor::Text as usize] = [0.90, 0.90, 0.90, 1.00];
    style.colors[StyleColor::TextDisabled as usize] = [0.50, 0.50, 0.50, 1.00];
    style.colors[StyleColor::WindowBg as usize] = [0.13, 0.14, 0.15, 1.00];
    style.colors[StyleColor::ChildBg as usize] = [0.13, 0.14, 0.15, 1.00];
    style.colors[StyleColor::PopupBg as usize] = [0.08, 0.08, 0.08, 0.94];
    style.colors[StyleColor::Border as usize] = [0.43, 0.43, 0.50, 0.50];
    style.colors[StyleColor::BorderShadow as usize] = [0.00, 0.00, 0.00, 0.00];
    style.colors[StyleColor::FrameBg as usize] = [0.25, 0.25, 0.25, 0.54];
    style.colors[StyleColor::FrameBgHovered as usize] = [0.38, 0.38, 0.38, 0.40];
    style.colors[StyleColor::FrameBgActive as usize] = [0.42, 0.42, 0.42, 0.67];
    style.colors[StyleColor::TitleBg as usize] = [0.04, 0.04, 0.04, 1.00];
    style.colors[StyleColor::TitleBgActive as usize] = [0.16, 0.29, 0.48, 1.00];
    style.colors[StyleColor::TitleBgCollapsed as usize] = [0.00, 0.00, 0.00, 0.51];
    style.colors[StyleColor::MenuBarBg as usize] = [0.14, 0.14, 0.14, 1.00];
    style.colors[StyleColor::ScrollbarBg as usize] = [0.02, 0.02, 0.02, 0.53];
    style.colors[StyleColor::ScrollbarGrab as usize] = [0.31, 0.31, 0.31, 1.00];
    style.colors[StyleColor::ScrollbarGrabHovered as usize] = [0.41, 0.41, 0.41, 1.00];
    style.colors[StyleColor::ScrollbarGrabActive as usize] = [0.51, 0.51, 0.51, 1.00];
    style.colors[StyleColor::CheckMark as usize] = [0.26, 0.59, 0.98, 1.00];
    style.colors[StyleColor::SliderGrab as usize] = [0.24, 0.52, 0.88, 1.00];
    style.colors[StyleColor::SliderGrabActive as usize] = [0.26, 0.59, 0.98, 1.00];
    style.colors[StyleColor::Button as usize] = [0.26, 0.59, 0.98, 0.40];
    style.colors[StyleColor::ButtonHovered as usize] = [0.26, 0.59, 0.98, 1.00];
    style.colors[StyleColor::ButtonActive as usize] = [0.06, 0.53, 0.98, 1.00];
    style.colors[StyleColor::Header as usize] = [0.26, 0.59, 0.98, 0.31];
    style.colors[StyleColor::HeaderHovered as usize] = [0.26, 0.59, 0.98, 0.80];
    style.colors[StyleColor::HeaderActive as usize] = [0.26, 0.59, 0.98, 1.00];
    style.colors[StyleColor::Separator as usize] = style.colors[StyleColor::Border as usize];
    style.colors[StyleColor::SeparatorHovered as usize] = [0.10, 0.40, 0.75, 0.78];
    style.colors[StyleColor::SeparatorActive as usize] = [0.10, 0.40, 0.75, 1.00];
    style.colors[StyleColor::ResizeGrip as usize] = [0.26, 0.59, 0.98, 0.25];
    style.colors[StyleColor::ResizeGripHovered as usize] = [0.26, 0.59, 0.98, 0.67];
    style.colors[StyleColor::ResizeGripActive as usize] = [0.26, 0.59, 0.98, 0.95];
    style.colors[StyleColor::Tab as usize] = style.colors[StyleColor::Header as usize];
    style.colors[StyleColor::TabHovered as usize] = style.colors[StyleColor::HeaderHovered as usize];
    style.colors[StyleColor::TabActive as usize] = style.colors[StyleColor::HeaderActive as usize];
    style.colors[StyleColor::TabUnfocused as usize] = style.colors[StyleColor::Tab as usize];
    style.colors[StyleColor::TabUnfocusedActive as usize] = style.colors[StyleColor::TabActive as usize];
    style.colors[StyleColor::PlotLines as usize] = [0.61, 0.61, 0.61, 1.00];
    style.colors[StyleColor::PlotLinesHovered as usize] = [1.00, 0.43, 0.35, 1.00];
    style.colors[StyleColor::PlotHistogram as usize] = [0.90, 0.70, 0.00, 1.00];
    style.colors[StyleColor::PlotHistogramHovered as usize] = [1.00, 0.60, 0.00, 1.00];
    style.colors[StyleColor::TextSelectedBg as usize] = [0.26, 0.59, 0.98, 0.35];
    style.colors[StyleColor::DragDropTarget as usize] = [1.00, 1.00, 0.00, 0.90];
    style.colors[StyleColor::NavHighlight as usize] = [0.26, 0.59, 0.98, 1.00];
    style.colors[StyleColor::NavWindowingHighlight as usize] = [1.00, 1.00, 1.00, 0.70];
    style.colors[StyleColor::NavWindowingDimBg as usize] = [0.80, 0.80, 0.80, 0.20];
    style.colors[StyleColor::ModalWindowDimBg as usize] = [0.80, 0.80, 0.80, 0.35];
}

fn main() {
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let (window, display) = SimpleWindowBuilder::new()
        .with_title("EWS IMMO Patcher MS43")
        .with_inner_size(1024, 768)
        .build(&event_loop);

    // window.set_resizable(false); // Allow window to be resizable

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);
    apply_custom_style(&mut imgui);

    let mut platform = WinitPlatform::new(&mut imgui);
    platform.attach_window(
        imgui.io_mut(),
        &window,
        imgui_winit_support::HiDpiMode::Default,
    );

    let hidpi_factor = platform.hidpi_factor();
    let font_size = (14.0 * hidpi_factor) as f32;
    imgui.fonts().add_font(&[FontSource::TtfData {
        data: include_bytes!("../../../../../../../Windows/Fonts/segoeui.ttf"),
        size_pixels: font_size,
        config: Some(imgui::FontConfig {
            rasterizer_multiply: 1.5,
            ..Default::default()
        }),
    }]);

    let mut renderer = Renderer::new(&mut imgui, &display).expect("Failed to initialize renderer");

    let mut last_frame = Instant::now();
    let mut app_state = AppState::default();

    event_loop
        .run(move |event, window_target| {
            match event {
                Event::NewEvents(_) => {
                    let now = Instant::now();
                    imgui.io_mut().update_delta_time(now - last_frame);
                    last_frame = now;
                }
                Event::AboutToWait => {
                    platform
                        .prepare_frame(imgui.io_mut(), &window)
                        .expect("Failed to prepare frame");
                    window.request_redraw();
                }
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    let ui = imgui.new_frame();

                    render_main_window(ui, &mut app_state);

                    let mut target = display.draw();
                    // Use the same background color as the custom style
                    target.clear_color_srgb(0.13, 0.14, 0.15, 1.0);
                    platform.prepare_render(ui, &window);
                    let draw_data = imgui.render();
                    renderer
                        .render(&mut target, draw_data)
                        .expect("Rendering failed");
                    target.finish().expect("Failed to swap buffers");
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => window_target.exit(),
                event => {
                    platform.handle_event(imgui.io_mut(), &window, &event);
                }
            }
        })
        .expect("Event loop error");
}