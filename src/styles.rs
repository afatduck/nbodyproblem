use macroquad::{color::{Color, WHITE}, file::load_file, math::RectOffset, texture::Image, ui::{Skin, Style, root_ui}};

pub trait ColorToImage {
    fn to_image(self) -> Image;
}

impl ColorToImage for Color {
    fn to_image(self) -> Image {
        let mut img = Image::empty();
        img.bytes = vec![0; 16*16*4];
        img.width = 16;
        img.height = 16;
        for i in 0..16 {
            for j in 0..16 {
                img.set_pixel(i, j, self);
            }
        }
        img
    }
}

pub fn create_button_style(font: &[u8]) -> Style {
    let normal = Color::from_rgba(40, 40, 40, 255);      // dark gray
    let hovered = Color::from_rgba(60, 60, 60, 255);     // lighter gray
    let clicked = Color::from_rgba(25, 25, 25, 255);     // almost black
    let text_color = WHITE;

    root_ui().style_builder()
        .background(normal.to_image())
        .background_hovered(hovered.to_image())
        .background_clicked(clicked.to_image())
        .text_color(text_color)
        .font_size(16)
        .font(font).unwrap()
        .margin(RectOffset::new(10.0, 10.0, 5.0, 5.0))
        .background_margin(RectOffset::new(8.0, 8.0, 8.0, 8.0))
        .build()
}

pub async fn set_app_style() {
    let font = load_file("assets/JetBrainsMono.ttf").await.unwrap();
    let button_style = create_button_style(&font);
    let skin = Skin {
        button_style,
        ..root_ui().default_skin()
    };
    root_ui().push_skin(&skin);
}