use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    pub background_color: Color,
    texture: Option<Texture2D>,
}

fn vector3_to_color(v: Vector3) -> Color {
    Color::new(
        (v.x.clamp(0.0, 1.0) * 255.0).round() as u8,
        (v.y.clamp(0.0, 1.0) * 255.0).round() as u8,
        (v.z.clamp(0.0, 1.0) * 255.0).round() as u8,
        255,
    )
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let bg_color = Color::BLACK;
        let color_buffer = Image::gen_image_color(width as i32, height as i32, bg_color);
        Framebuffer {
            width,
            height,
            color_buffer,
            background_color: bg_color,
            texture: None,
        }
    }

    pub fn set_background_color(&mut self, color: Vector3) {
        self.background_color = vector3_to_color(color);
    }

    pub fn clear(&mut self) {
        self.color_buffer.clear_background(self.background_color);
    }

    pub fn point(&mut self, x: i32, y: i32, color: Vector3) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            self.color_buffer.draw_pixel(x, y, vector3_to_color(color));
        }
    }

    pub fn init_texture(&mut self, window: &mut RaylibHandle, thread: &RaylibThread) {
        if let Ok(texture) = window.load_texture_from_image(thread, &self.color_buffer) {
            self.texture = Some(texture);
        }
    }

    pub fn swap_buffers(&mut self, window: &mut RaylibHandle, thread: &RaylibThread) {
        if let Some(ref mut texture) = self.texture {
            let colors = self.color_buffer.get_image_data();
            let total_bytes = colors.len() * std::mem::size_of::<Color>();
            let pixels = unsafe { std::slice::from_raw_parts(colors.as_ptr().cast::<u8>(), total_bytes) };
            let _ = texture.update_texture(pixels);

            let mut d = window.begin_drawing(thread);
            d.clear_background(Color::BLACK);
            d.draw_texture(texture, 0, 0, Color::WHITE);
        } else if let Ok(texture) = window.load_texture_from_image(thread, &self.color_buffer) {
            let mut d = window.begin_drawing(thread);
            d.clear_background(Color::BLACK);
            d.draw_texture(&texture, 0, 0, Color::WHITE);
        }
    }
}
