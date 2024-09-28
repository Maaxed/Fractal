use crate::Target;

use egui::{Context, Visuals, epaint::Shadow};
use egui_wgpu::{Renderer, ScreenDescriptor};

use egui_winit::{EventResponse, State};
use wgpu::{CommandEncoder, TextureView};
use winit::event::WindowEvent;
use winit::window::Theme;


pub struct EguiRenderer
{
    state: State,
    renderer: Renderer,
}

impl EguiRenderer
{
    pub fn new(target: &Target) -> EguiRenderer
    {
        let context = Context::default();
        let id = context.viewport_id();

        context.set_visuals(
            Visuals
            {
                window_shadow: Shadow::NONE,
                faint_bg_color: egui::Color32::from_additive_luminance(15),
                ..Visuals::dark()
            }
        );

        let state = State::new(context, id, &target.window, Some(target.window.scale_factor() as f32), Some(Theme::Dark), None);

        let renderer = Renderer::new(&target.device, target.config.format, None, 1, false);

        EguiRenderer
        {
            state,
            renderer,
        }
    }

    pub fn handle_input(&mut self, target: &Target, event: &WindowEvent) -> EventResponse
    {
        self.state.on_window_event(&target.window, event)
    }

    pub fn draw(
        &mut self,
        target: &Target,
        commands: &mut CommandEncoder,
        window_surface_view: &TextureView,
        run_ui: impl FnMut(&Context),
    )
    {
        let pixels_per_point = target.window.scale_factor() as f32;
        self.state.egui_ctx().set_pixels_per_point(pixels_per_point);

        let raw_input = self.state.take_egui_input(target.window.as_ref());
        let full_output = self.state.egui_ctx().run(raw_input, run_ui);

        self.state.handle_platform_output(target.window.as_ref(), full_output.platform_output);

        let triangles = self.state.egui_ctx().tessellate(full_output.shapes, full_output.pixels_per_point);
        for (id, image_delta) in &full_output.textures_delta.set
        {
            self.renderer.update_texture(&target.device, &target.queue, *id, &image_delta);
        }
        let screen_descriptor = ScreenDescriptor
        {
            size_in_pixels: [target.config.width, target.config.height],
            pixels_per_point,
        };

        self.renderer.update_buffers(&target.device, &target.queue, commands, &triangles, &screen_descriptor);

        {
            let mut render_pass = commands.begin_render_pass(
                &wgpu::RenderPassDescriptor
                {
                    label: Some("egui_render_pass"),
                    color_attachments: &[
                        Some(wgpu::RenderPassColorAttachment
                        {
                            view: window_surface_view,
                            resolve_target: None,
                            ops: wgpu::Operations
                            {
                                load: wgpu::LoadOp::Load,
                                store: wgpu::StoreOp::Store,
                            },
                        }),
                    ],
                    ..Default::default()
                }
            ).forget_lifetime();

            self.renderer.render(&mut render_pass, &triangles, &screen_descriptor);
        }

        for texture_id in &full_output.textures_delta.free
        {
            self.renderer.free_texture(texture_id)
        }
    }
}