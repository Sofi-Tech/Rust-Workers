use std::{fs::File, io::Read};

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

// TODO: take card struct as input and get image url, frame url and other info
// required for draw_card from it
pub async fn generate_drop() -> Canvas {
    let buf = fetch_buffer(
        "https://cdn.w1st.xyz/cards/characters/1e364732-dfee-4672-bc0e-75796d3f9f78.jpg",
    )
    .await;

    let mut frame = File::open("./frames/blue-drop.png").unwrap();
    let mut frame_bytes = Vec::new();
    frame.read_to_end(&mut frame_bytes).unwrap();

    let canvas = Canvas::new(1_008, 524);

    // Here we are passing canvas to the draw_card fn so it's ownership will be
    // lost. We can't use it in the next line. So instead we return it from the
    // function and pass it again in 2nd function. This way we don't need to clone
    // or add any lifetime and we can use the canvas in the next line.
    // Not sure if adding lifetime will have any issue or something so before I do
    // research on it, will do it this way.
    let image_one = draw_card(canvas, &buf, &frame_bytes, 1);
    let image_two = draw_card(image_one, &buf, &frame_bytes, 347);
    let image_three = draw_card(image_two, &buf, &frame_bytes, 692);
    image_three
}

// TODO: take card struct as input and get gen, name, series from it
pub fn draw_card(mut canvas: Canvas, image: &[u8], frame: &[u8], dx: i32) -> Canvas {
    canvas.draw_image(image, (6 + dx, 4));
    canvas.draw_image(frame, (dx, 0));
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
