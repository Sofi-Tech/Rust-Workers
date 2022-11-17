use std::{fs::File, io::Read};

use skia_safe::{
    font_style::{Slant, Weight, Width},
    Font, FontStyle, Typeface,
};

use super::Canvas;

pub struct Card {
    pub image: Vec<u8>,
    pub frame_url: String,
    pub gen: i32,
    pub name: String,
    pub series: String,
}

pub async fn fetch_buffer(url: &str) -> Vec<u8> {
    let response = reqwest::get(url);
    let buffer = response.await.unwrap().bytes().await.unwrap().to_vec();
    buffer
}

pub async fn fetch_utf(url: &str) -> String {
    let response = reqwest::get(url);
    let buffer = response.await.unwrap().text().await.unwrap();
    buffer
}

pub fn draw_card(mut canvas: Canvas, card: Card, dx: i32) -> Canvas {
    let mut frame = File::open(card.frame_url).unwrap();
    let mut frame_bytes = Vec::new();
    let gen_text = format!("G{}", card.gen);
    frame.read_to_end(&mut frame_bytes).unwrap();

    canvas.draw_image(&card.image, (6 + dx, 4));
    canvas.draw_image(&frame_bytes, (dx, 0));
    canvas.fill_text(
        &gen_text,
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
        &card.name,
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
        &card.series,
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

#[cfg(test)]
mod tests {
    use tokio::join;

    use super::*;

    #[tokio::test]
    async fn generate_and_save_the_drop_image() {
        let (image_one, image_two, image_three) = join!(
            fetch_buffer(
                "https://cdn.w1st.xyz/cards/characters/42739898-0dc5-43ec-b918-889fd1a993b0.jpg"
            ),
            fetch_buffer(
                "https://cdn.w1st.xyz/cards/characters/1e364732-dfee-4672-bc0e-75796d3f9f78.jpg"
            ),
            fetch_buffer(
                "https://cdn.w1st.xyz/cards/characters/358445c8-0bd8-43ff-943b-4bdfa1264275.jpg"
            )
        );

        let canvas = Canvas::new(1_008, 524);

        let canvas = draw_card(
            canvas,
            Card {
                image: image_one,
                frame_url: "./frames/cyan-drop.png".to_string(),
                gen: 1,
                name: "Rose".to_string(),
                series: "Blackpink".to_string(),
            },
            1,
        );
        let canvas = draw_card(
            canvas,
            Card {
                image: image_two,
                frame_url: "./frames/purple-drop.png".to_string(),
                gen: 1,
                name: "Gojo Satoru".to_string(),
                series: "Jujutsu Kaisen".to_string(),
            },
            347,
        );
        let _canvas = draw_card(
            canvas,
            Card {
                image: image_three,
                frame_url: "./frames/yellow-drop.png".to_string(),
                gen: 1,
                name: "Demon Slayer".to_string(),
                series: "Nezuko Kamado".to_string(),
            },
            692,
        );
    }
}
