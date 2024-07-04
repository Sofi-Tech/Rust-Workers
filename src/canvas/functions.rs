use std::{fs::File, io::Read};

use skia_safe::{
    font_style::{Slant, Weight, Width},
    Font, FontStyle, Typeface,
};

use super::Canvas; // 0.2.21, features = ["macros"]

pub struct Card<'a> {
    pub image: Vec<u8>,
    pub element: String,
    pub gen: i32,
    pub name: &'a str,
    pub series: &'a str,
}

pub fn draw_card(mut canvas: Canvas, card: Card, x: f32, y: f32, dw: f32, dh: f32) -> Canvas {
    let mut frame = File::open(format!("./frames/{}-drop.png", card.element)).unwrap();
    let mut frame_bytes = Vec::new();
    let gen_text = format!("G{}", card.gen);
    frame.read_to_end(&mut frame_bytes).unwrap();

    canvas.draw_image_with_size(&card.image, 7. + x, 4. + y, dw, dh);
    canvas.draw_image(&frame_bytes, (x as i32, 0));
    canvas.fill_text(
        &gen_text,
        (16. + x, 450.),
        &Font::from_typeface(
            Typeface::new(
                "Roboto",
                FontStyle::new(Weight::from(400), Width::NORMAL, Slant::Upright),
            )
            .unwrap(),
            29.0,
        ),
    );
    canvas.fill_text(
        card.name,
        (14. + x, 480.),
        &Font::from_typeface(
            Typeface::new(
                "Roboto",
                FontStyle::new(Weight::from(700), Width::NORMAL, Slant::Upright),
            )
            .unwrap(),
            28.0,
        ),
    );
    canvas.fill_text(
        card.series,
        (14. + x, 507.),
        &Font::from_typeface(
            Typeface::new(
                "Roboto",
                FontStyle::new(Weight::from(500), Width::NORMAL, Slant::Upright),
            )
            .unwrap(),
            22.0,
        ),
    );
    canvas
}

pub fn draw_card_ref(mut canvas: Canvas, card: &Card, x: f32, y: f32, dw: f32, dh: f32) -> Canvas {
    let mut frame = File::open(format!("./frames/{}-drop.png", card.element)).unwrap();
    let mut frame_bytes = Vec::new();
    let gen_text = format!("G{}", card.gen);
    frame.read_to_end(&mut frame_bytes).unwrap();

    canvas.draw_image_with_size(&card.image, 6. + x, 4.0 + y, dw, dh);
    canvas.draw_image(&frame_bytes, (x as i32, 0));
    canvas.fill_text(
        &gen_text,
        (16. + x, 450.),
        &Font::from_typeface(
            Typeface::new(
                "Roboto",
                FontStyle::new(Weight::from(400), Width::NORMAL, Slant::Upright),
            )
            .unwrap(),
            29.0,
        ),
    );
    canvas.fill_text(
        card.name,
        (14. + x, 480.),
        &Font::from_typeface(
            Typeface::new(
                "Roboto",
                FontStyle::new(Weight::from(700), Width::NORMAL, Slant::Upright),
            )
            .unwrap(),
            28.0,
        ),
    );
    canvas.fill_text(
        card.series,
        (14. + x, 507.),
        &Font::from_typeface(
            Typeface::new(
                "Roboto",
                FontStyle::new(Weight::from(500), Width::NORMAL, Slant::Upright),
            )
            .unwrap(),
            22.0,
        ),
    );
    canvas
}
