use fractal_renderer_shared as shared;
use crate::app::AppData;
use crate::render::Render;
use crate::Target;
use crate::quad_cell::QuadPos;
use winit::dpi::PhysicalSize;
use glam::{dvec2, uvec2};


pub struct ThreadedCompute
{
    texture_size: PhysicalSize<u32>,
    aligned_width: u32
}

impl ThreadedCompute
{
    pub fn new(texture_size: PhysicalSize<u32>) -> Self
    {
        let aligned_width = wgpu::util::align_to(texture_size.width, wgpu::COPY_BYTES_PER_ROW_ALIGNMENT / std::mem::size_of::<u32>() as u32);

        Self
        {
            texture_size,
            aligned_width
        }
    }

    pub fn do_compute(&self, data: &mut [u32], params: shared::compute::Params64)
    {
        for y in 0..self.texture_size.height
        {
            for x in 0..self.texture_size.width
            {
                let pixel = &mut data[(x + y * self.aligned_width) as usize];
                *pixel = shared::compute::run(uvec2(x, y), uvec2(self.texture_size.width, self.texture_size.height), params.into());
            }
        }
    }

    fn compute_cell(&self, target: &Target, render: &Render, app: &mut AppData, pos: QuadPos)
    {
		let cell_size = pos.cell_size();
		let cell_pos = pos.cell_bottom_left();
        
        let mut data = vec![0_u32; (self.aligned_width * self.texture_size.height) as usize];

        self.do_compute(&mut data[..], shared::compute::Params64
		{
			min_pos: cell_pos + dvec2(0.0, cell_size),
			max_pos: cell_pos + dvec2(cell_size, 0.0),
			fractal: app.fractal_params,
		});

        let cell = app.make_cell(target, render, pos);
        let destination = cell.fractal_texture();
        target.queue.write_texture(
            wgpu::ImageCopyTexture
            {
                texture: destination,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            bytemuck::cast_slice(&data[..]),
            wgpu::ImageDataLayout
            {
                offset: 0,
                bytes_per_row: Some(self.aligned_width * std::mem::size_of::<u32>() as u32),
                rows_per_image: None
            },
            destination.size()
        );
    }
}

impl super::Compute for ThreadedCompute
{
    fn reset(&mut self)
    { }

    fn update_before_render(&mut self, target: &Target, render: &Render, app: &mut AppData, _commands: &mut wgpu::CommandEncoder)
    {
        // Find new cell to load
		for pos in app.visible_cells()
		{
            if !app.is_cell_loaded(pos)
            {
                self.compute_cell(target, render, app, pos);
                return;
            }
		}
    }
}
