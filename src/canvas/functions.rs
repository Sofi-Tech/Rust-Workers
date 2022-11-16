use skia_safe::{
    font_style::{Slant, Weight, Width},
    Font, FontStyle, Typeface,
};

use super::Canvas;

pub async fn fetch_buffer(url: &str) -> Vec<u8> {
    let response = reqwest::get(url);
    let buffer = response.await.unwrap().bytes().await.unwrap().to_vec();
    buffer
}

pub fn draw_card(mut canvas: Canvas, image: &[u8], frame: &[u8], dx: i32) -> Canvas {
    // let mut canvas = Canvas::new(314, 524);
    canvas.draw_image(image, (6 + dx, 4));
    canvas.draw_image(frame, (0 + dx, 0));
    canvas.fill_text(
        "G1",
        (16 + dx, 450),
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
        "Gojo Satoru",
        (14 + dx, 480),
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
        "Jujutsu Kaisen",
        (14 + dx, 507),
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
