use std::mem;
pub mod functions;
use skia_safe::{
    Color, Data, EncodedImageFormat, Font, Image, ImageGenerator, Paint, PaintStyle, Path, Point,
    Surface,
};

pub struct Canvas {
    surface: Surface,
    path: Path,
    paint: Paint,
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Canvas {
        let surface = Surface::new_raster_n32_premul((width, height)).expect("no surface!");
        let path = Path::new();
        let paint = Paint::default();
        Canvas {
            surface,
            path,
            paint,
        }
    }

    #[inline]
    pub fn save(&mut self) {
        self.surface.canvas().save();
    }

    #[inline]
    pub fn translate(&mut self, dx: f32, dy: f32) {
        self.surface.canvas().translate((dx, dy));
    }

    #[inline]
    pub fn scale(&mut self, sx: f32, sy: f32) {
        self.surface.canvas().scale((sx, sy));
    }

    #[inline]
    pub fn move_to(&mut self, x: f32, y: f32) {
        self.begin_path();
        self.path.move_to((x, y));
    }

    #[inline]
    pub fn line_to(&mut self, x: f32, y: f32) {
        self.path.line_to((x, y));
    }

    #[inline]
    pub fn quad_to(&mut self, cpx: f32, cpy: f32, x: f32, y: f32) {
        self.path.quad_to((cpx, cpy), (x, y));
    }

    #[allow(dead_code)]
    #[inline]
    pub fn bezier_curve_to(&mut self, cp1x: f32, cp1y: f32, cp2x: f32, cp2y: f32, x: f32, y: f32) {
        self.path.cubic_to((cp1x, cp1y), (cp2x, cp2y), (x, y));
    }

    #[allow(dead_code)]
    #[inline]
    pub fn close_path(&mut self) {
        self.path.close();
    }

    #[inline]
    pub fn begin_path(&mut self) {
        let new_path = Path::new();
        self.surface.canvas().draw_path(&self.path, &self.paint);
        let _ = mem::replace(&mut self.path, new_path);
    }

    #[inline]
    pub fn stroke(&mut self) {
        self.paint.set_style(PaintStyle::Stroke);
        self.surface.canvas().draw_path(&self.path, &self.paint);
    }

    #[inline]
    pub fn fill(&mut self) {
        self.paint.set_style(PaintStyle::Fill);
        self.surface.canvas().draw_path(&self.path, &self.paint);
    }

    #[inline]
    pub fn set_line_width(&mut self, width: f32) {
        self.paint.set_stroke_width(width);
    }

    #[inline]
    pub fn data(&mut self) -> Data {
        let image = self.surface.image_snapshot();
        image.encode_to_data(EncodedImageFormat::PNG).unwrap()
    }

    #[inline]
    fn canvas(&mut self) -> &mut skia_safe::Canvas {
        self.surface.canvas()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.surface.canvas().clear(Color::WHITE);
    }

    #[inline]
    pub fn set_color(&mut self, color: Color) {
        self.paint.set_color(color);
    }

    #[inline]
    pub fn draw_image(&mut self, data: &[u8], left_top: impl Into<Point>) {
        let img_g = ImageGenerator::from_encoded(Data::new_copy(data)).unwrap();
        let img = Image::from_generator(img_g).unwrap();
        self.surface.canvas().draw_image(img, left_top, None);
    }

    pub fn fill_text(&mut self, text: &str, origin: impl Into<Point>, font: &Font) {
        let paint = &self.paint;
        self.surface.canvas().draw_str(text, origin, font, paint);
    }
}
