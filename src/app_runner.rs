use super::app_config::AppConfig;
use anyhow::Result;
use std::rc::Rc;
use thiserror::Error;

pub trait App: Sized {
    fn new(gl: Rc<glow::Context>) -> Self;
    fn draw(&mut self, gl: &glow::Context);
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Failed to create OpenGL context")]
    ContextCreationError,
    #[error("Failed to create shader program")]
    ShaderCreationError,
    #[error("Other error: {0}")]
    Other(String),
}

pub struct AppRunner {}
impl AppRunner {
    pub fn run<A: App>(app_config: AppConfig) -> Result<(), AppError> {
        unsafe {
            // #[cfg(target_arch = "wasm32")]
            // let (gl, shader_version) = {
            //     use wasm_bindgen::JsCast;
            //     let canvas = web_sys::window()
            //         .unwrap()
            //         .document()
            //         .unwrap()
            //         .get_element_by_id("canvas")
            //         .unwrap()
            //         .dyn_into::<web_sys::HtmlCanvasElement>()
            //         .unwrap();
            //     let webgl2_context = canvas
            //         .get_context("webgl2")
            //         .unwrap()
            //         .unwrap()
            //         .dyn_into::<web_sys::WebGl2RenderingContext>()
            //         .unwrap();
            //     let gl = glow::Context::from_webgl2_context(webgl2_context);
            //     (gl, "#version 300 es")
            // };

            #[cfg(feature = "glutin_winit")]
            let (gl, gl_surface, gl_context, _window, event_loop) = {
                use glutin::{
                    config::{ConfigTemplateBuilder, GlConfig},
                    context::{ContextApi, ContextAttributesBuilder, NotCurrentGlContext},
                    display::{GetGlDisplay, GlDisplay},
                    surface::{GlSurface, SwapInterval},
                };

                use glutin_winit::{DisplayBuilder, GlWindow};
                use raw_window_handle::HasRawWindowHandle;
                use std::num::NonZeroU32;

                let event_loop = winit::event_loop::EventLoopBuilder::new().build().unwrap();
                let window_builder = winit::window::WindowBuilder::new()
                    .with_title(&app_config.window_title)
                    .with_inner_size(winit::dpi::LogicalSize::new(
                        app_config.window_width as f64,
                        app_config.window_height as f64,
                    ));

                let template = ConfigTemplateBuilder::new();

                let display_builder =
                    DisplayBuilder::new().with_window_builder(Some(window_builder));

                let (window, gl_config) = display_builder
                    .build(&event_loop, template, |configs| {
                        configs
                            .reduce(|accum, config| {
                                if config.num_samples() > accum.num_samples() {
                                    config
                                } else {
                                    accum
                                }
                            })
                            .unwrap()
                    })
                    .unwrap();

                let raw_window_handle = window.as_ref().map(|window| window.raw_window_handle());

                let gl_display = gl_config.display();
                let context_attributes = ContextAttributesBuilder::new()
                    .with_context_api(ContextApi::OpenGl(Some(glutin::context::Version {
                        major: app_config.gl_version_major,
                        minor: app_config.gl_version_minor,
                    })))
                    .build(raw_window_handle);

                let not_current_gl_context = gl_display
                    .create_context(&gl_config, &context_attributes)
                    .unwrap();

                let window = window.unwrap();

                let attrs = window.build_surface_attributes(Default::default());
                let gl_surface = gl_display
                    .create_window_surface(&gl_config, &attrs)
                    .unwrap();

                let gl_context = not_current_gl_context.make_current(&gl_surface).unwrap();

                let gl = Rc::new(glow::Context::from_loader_function_cstr(|s| {
                    gl_display.get_proc_address(s)
                }));

                gl_surface
                    .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
                    .unwrap();

                (gl, gl_surface, gl_context, window, event_loop)
            };

            let mut app = A::new(gl.clone());

            #[cfg(feature = "glutin_winit")]
            {
                use glutin::prelude::GlSurface;
                use winit::event::{Event, WindowEvent};
                let _ = event_loop.run(move |event, elwt| {
                    if let Event::WindowEvent { event, .. } = event {
                        match event {
                            WindowEvent::CloseRequested => {
                                elwt.exit();
                            }
                            WindowEvent::RedrawRequested => {
                                app.draw(&gl);
                                gl_surface.swap_buffers(&gl_context).unwrap();
                            }
                            _ => (),
                        }
                    }
                });
            }

            Ok(())
        }
    }
}
