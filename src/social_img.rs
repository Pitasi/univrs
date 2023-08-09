use axum::{extract::Path, response::IntoResponse, Extension};
use image::{imageops::overlay, ImageBuffer, ImageFormat, RgbaImage};
use imageproc::{
    drawing::{draw_filled_rect_mut, draw_text_mut, text_size},
    rect::Rect,
};
use lru::LruCache;
use once_cell::sync::Lazy;
use reqwest::header::{CACHE_CONTROL, CONTENT_TYPE};
use rusttype::{Font, Scale};
use std::{
    io::{BufWriter, Cursor},
    num::NonZeroUsize,
    sync::Mutex,
};

use crate::articles::ArticlesRepo;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 600;

const COLOR_BG: image::Rgba<u8> = image::Rgba([255, 250, 240, 255]);
const COLOR_ACCENT: image::Rgba<u8> = image::Rgba([226, 0, 147, 255]);
const COLOR_GRAY: image::Rgba<u8> = image::Rgba([80, 80, 80, 255]);
const COLOR_BLACK: image::Rgba<u8> = image::Rgba([0, 0, 0, 255]);
const COLOR_YELLOW: image::Rgba<u8> = image::Rgba([246, 255, 95, 255]);

static mut LRU_CACHE: Lazy<Mutex<LruCache<String, Vec<u8>>>> =
    Lazy::new(|| Mutex::new(LruCache::new(NonZeroUsize::new(10).unwrap())));

pub async fn social_image_article(
    articles_repo: Extension<ArticlesRepo>,
    Path(slug): Path<String>,
) -> impl IntoResponse {
    let mut cache = unsafe { LRU_CACHE.lock() }.unwrap();

    let bytes = cache.get_or_insert(slug.clone(), || _social_image_article(articles_repo, &slug));

    (
        axum::response::AppendHeaders([
            (CONTENT_TYPE, "image/png"),
            (CACHE_CONTROL, "public, max-age=21600, immutable"),
        ]),
        bytes.clone(),
    )
}

fn _social_image_article(articles_repo: Extension<ArticlesRepo>, slug: &str) -> Vec<u8> {
    let a = articles_repo
        .get_article_by_slug(slug)
        .unwrap_or_else(|| panic!("Article not found"));

    let mut image: RgbaImage = ImageBuffer::new(WIDTH, HEIGHT);
    image.pixels_mut().for_each(|pixel| {
        *pixel = COLOR_BG;
    });

    let mut bulb = image::load_from_memory(include_bytes!("../static/bulb.png"))
        .unwrap()
        .resize(600, 600, image::imageops::FilterType::Triangle)
        .to_rgba8();
    bulb.pixels_mut().for_each(|pixel| {
        *pixel = image::Rgba([pixel[0], pixel[1], pixel[2], pixel[3].saturating_sub(175)]);
    });

    overlay(&mut image, &bulb, (WIDTH - bulb.width() - 10).into(), 10);

    let title_font = Vec::from(include_bytes!(
        "../static/Clash Display/Fonts/OTF/ClashDisplay-Bold.otf"
    ) as &[u8]);
    let title_font = Font::try_from_vec(title_font).unwrap();

    let body_font = Vec::from(include_bytes!(
        "../static/Clash Display/Fonts/OTF/ClashDisplay-Regular.otf"
    ) as &[u8]);
    let body_font = Font::try_from_vec(body_font).unwrap();

    let font_size = 80.0;
    let mut text = a.title.as_str();
    let (lines, line_height) = split_lines(font_size, &title_font, &mut text, (WIDTH as i32) - 200);

    lines.iter().enumerate().for_each(|(i, line)| {
        draw_text_mut(
            &mut image,
            COLOR_ACCENT,
            100,
            100 + (i as i32 * line_height),
            Scale {
                x: font_size,
                y: font_size,
            },
            &title_font,
            &line,
        );
    });

    let offset = 150 + (line_height * lines.len() as i32);
    let font_size = 30.0;
    draw_text_mut(
        &mut image,
        COLOR_GRAY,
        100,
        offset,
        Scale {
            x: font_size,
            y: font_size,
        },
        &body_font,
        &format!("Written on {}", (a.datetime.format("%B %d, %Y"))),
    );

    // FOOTER
    let footer_height = 60;
    draw_filled_rect_mut(
        &mut image,
        Rect::at(0, (HEIGHT - footer_height).try_into().unwrap()).of_size(WIDTH, footer_height),
        COLOR_YELLOW,
    );

    let font_size = 26.0;
    let footer_text = format!("Antonio Pitasi - https://anto.pt/articles/{}", a.slug);
    let (_, text_height) = text_size(
        Scale {
            x: font_size,
            y: font_size,
        },
        &body_font,
        &footer_text,
    );
    draw_text_mut(
        &mut image,
        COLOR_BLACK,
        100,
        (HEIGHT - (footer_height / 2) - (text_height / 2) as u32)
            .try_into()
            .unwrap(),
        Scale {
            x: font_size,
            y: font_size,
        },
        &body_font,
        &footer_text,
    );

    let mut buffer = BufWriter::new(Cursor::new(Vec::new()));
    image.write_to(&mut buffer, ImageFormat::Png).unwrap();

    let bytes: Vec<u8> = buffer.into_inner().unwrap().into_inner();

    bytes
}

fn split_lines(font_size: f32, font: &Font, txt: &str, max_width: i32) -> (Vec<String>, i32) {
    let (w, line_height) = text_size(
        Scale {
            x: font_size,
            y: font_size,
        },
        font,
        txt,
    );
    if w <= max_width {
        return (vec![txt.to_string()], line_height);
    }

    let mut lines = Vec::new();
    let mut line = String::new();
    for word in txt.split_whitespace() {
        let (w, _) = text_size(
            Scale {
                x: font_size,
                y: font_size,
            },
            font,
            &format!("{}{}", line, word),
        );
        if w <= max_width {
            line.push_str(&format!("{} ", word));
        } else {
            lines.push(line);
            line = format!("{} ", word);
        }
    }

    if !line.is_empty() {
        lines.push(line);
    }

    (lines, line_height)
}
