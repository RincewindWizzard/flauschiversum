//! Dekodieren, Drehen, Skalieren, Badge aufbringen und Kodieren eines Bildes.

use super::meta;
use crate::config::{FULL_MAX_BYTES, FULL_WIDTH, PREVIEW_WIDTH, THUMB_WIDTH};
use anyhow::{Context, Result, bail};
use fast_image_resize::images::Image as FirImage;
use fast_image_resize::{FilterType, PixelType, ResizeAlg, ResizeOptions, Resizer};
use image::{DynamicImage, ImageDecoder, ImageEncoder, ImageReader};
use std::io::Cursor;

/// Startqualität für verlustbehaftete Formate.
pub const QUALITY: u8 = 82;
const QUALITY_STEP: u8 = 6;
const QUALITY_MIN: u8 = 40;
/// Unterhalb dieser Breite bekommt eine Variante kein Badge.
const BADGE_MIN_IMAGE_WIDTH: u32 = 240;
const BADGE_MIN_WIDTH: u32 = 60;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Jpeg,
    Png,
    WebP,
    Gif,
}

impl Format {
    pub fn from_file_name(name: &str) -> Option<Format> {
        let ext = name.rsplit('.').next()?.to_ascii_lowercase();
        Some(match ext.as_str() {
            "jpg" | "jpeg" => Format::Jpeg,
            "png" => Format::Png,
            "webp" => Format::WebP,
            "gif" => Format::Gif,
            _ => return None,
        })
    }
}

/// Eine erzeugte Variante.
#[derive(Debug, Clone)]
pub struct Variant {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

/// Rohbild im Speicher (RGB oder RGBA, 8 Bit).
struct Raster {
    width: u32,
    height: u32,
    alpha: bool,
    pixels: Vec<u8>,
}

impl Raster {
    fn channels(&self) -> usize {
        if self.alpha { 4 } else { 3 }
    }

    fn pixel_type(&self) -> PixelType {
        if self.alpha { PixelType::U8x4 } else { PixelType::U8x3 }
    }

    /// Skaliert auf höchstens `width` Pixel Breite (nie vergrößern).
    fn scaled_to(&self, width: u32) -> Result<Raster> {
        if self.width <= width {
            return Ok(Raster { pixels: self.pixels.clone(), ..*self });
        }
        let height = ((self.height as u64 * width as u64 + self.width as u64 / 2) / self.width as u64).max(1) as u32;
        resize(self, width, height)
    }
}

impl Clone for Raster {
    fn clone(&self) -> Self {
        Raster { pixels: self.pixels.clone(), ..*self }
    }
}

fn resize(src: &Raster, width: u32, height: u32) -> Result<Raster> {
    let src_img = FirImage::from_vec_u8(src.width, src.height, src.pixels.clone(), src.pixel_type())?;
    let mut dst = FirImage::new(width, height, src.pixel_type());
    let options = ResizeOptions::new()
        .resize_alg(ResizeAlg::Convolution(FilterType::Lanczos3))
        .use_alpha(src.alpha);
    Resizer::new().resize(&src_img, &mut dst, &options)?;
    Ok(Raster { width, height, alpha: src.alpha, pixels: dst.into_vec() })
}

/// Die Badge-Vorlage (RGBA, nicht vormultipliziert).
pub struct Badge {
    raster: Raster,
}

impl Badge {
    pub fn from_png(bytes: &[u8]) -> Result<Badge> {
        let img = image::load_from_memory(bytes).context("Badge-Vorlage kann nicht gelesen werden")?.to_rgba8();
        Ok(Badge { raster: Raster { width: img.width(), height: img.height(), alpha: true, pixels: img.into_raw() } })
    }

    /// Breite des Badges für eine Variante der Breite `image_width`, oder `None` ohne Badge.
    pub fn width_for(image_width: u32) -> Option<u32> {
        (image_width >= BADGE_MIN_IMAGE_WIDTH).then(|| (image_width / 5).max(BADGE_MIN_WIDTH))
    }

    /// Setzt das Badge unten rechts mit 2,5 % Randabstand auf das Bild.
    fn apply(&self, target: &mut Raster) -> Result<()> {
        let Some(bw) = Badge::width_for(target.width) else { return Ok(()) };
        let bh = ((self.raster.height as u64 * bw as u64) / self.raster.width as u64).max(1) as u32;
        let badge = resize(&self.raster, bw, bh)?;
        let margin = ((target.width as f32) * 0.025).round() as u32;
        if bw + margin > target.width || bh + margin > target.height {
            return Ok(());
        }
        let x0 = target.width - bw - margin;
        let y0 = target.height - bh - margin;
        let tc = target.channels();
        for y in 0..bh {
            for x in 0..bw {
                let s = ((y * bw + x) * 4) as usize;
                let a = badge.pixels[s + 3] as u32;
                if a == 0 {
                    continue;
                }
                let d = (((y0 + y) * target.width + (x0 + x)) as usize) * tc;
                for c in 0..3 {
                    let src = badge.pixels[s + c] as u32;
                    let dst = target.pixels[d + c] as u32;
                    target.pixels[d + c] = ((src * a + dst * (255 - a) + 127) / 255) as u8;
                }
                if target.alpha {
                    let da = target.pixels[d + 3] as u32;
                    target.pixels[d + 3] = (a + (da * (255 - a) + 127) / 255).min(255) as u8;
                }
            }
        }
        Ok(())
    }
}

fn decode(bytes: &[u8]) -> Result<DynamicImage> {
    let reader = ImageReader::new(Cursor::new(bytes)).with_guessed_format()?;
    let mut decoder = reader.into_decoder()?;
    let orientation = decoder.orientation()?;
    let mut img = DynamicImage::from_decoder(decoder)?;
    img.apply_orientation(orientation);
    Ok(img)
}

fn encode_jpeg(r: &Raster, quality: u8) -> Result<Vec<u8>> {
    let rgb;
    let pixels = if r.alpha {
        rgb = r.pixels.chunks_exact(4).flat_map(|p| [p[0], p[1], p[2]]).collect::<Vec<_>>();
        &rgb
    } else {
        &r.pixels
    };
    let mut out = Vec::new();
    let mut enc = jpeg_encoder::Encoder::new(&mut out, quality);
    enc.set_optimized_huffman_tables(true);
    enc.set_progressive(true);
    enc.add_exif_metadata(&meta::exif_tiff())?;
    enc.add_app_segment(1, meta::jpeg_xmp_segment())?;
    enc.add_app_segment(13, meta::jpeg_iptc_segment())?;
    enc.encode(pixels, r.width as u16, r.height as u16, jpeg_encoder::ColorType::Rgb)?;
    Ok(out)
}

fn encode_png(r: &Raster) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    let enc = image::codecs::png::PngEncoder::new_with_quality(
        &mut out,
        image::codecs::png::CompressionType::Best,
        image::codecs::png::FilterType::Adaptive,
    );
    let color = if r.alpha { image::ExtendedColorType::Rgba8 } else { image::ExtendedColorType::Rgb8 };
    enc.write_image(&r.pixels, r.width, r.height, color)?;
    Ok(meta::png_with_rights(&out))
}

fn encode_webp(r: &Raster, quality: u8) -> Result<Vec<u8>> {
    let enc = if r.alpha {
        webp::Encoder::from_rgba(&r.pixels, r.width, r.height)
    } else {
        webp::Encoder::from_rgb(&r.pixels, r.width, r.height)
    };
    let mem = enc.encode(quality as f32);
    Ok(meta::webp_with_rights(&mem, r.width, r.height))
}

fn encode_gif(r: &Raster) -> Result<Vec<u8>> {
    let rgba;
    let pixels = if r.alpha {
        &r.pixels
    } else {
        rgba = r.pixels.chunks_exact(3).flat_map(|p| [p[0], p[1], p[2], 255]).collect::<Vec<_>>();
        &rgba
    };
    let mut out = Vec::new();
    {
        let mut enc = image::codecs::gif::GifEncoder::new_with_speed(&mut out, 10);
        enc.encode(pixels, r.width, r.height, image::ExtendedColorType::Rgba8)?;
    }
    Ok(meta::gif_with_rights(&out))
}

fn encode(r: &Raster, format: Format, quality: u8) -> Result<Vec<u8>> {
    match format {
        Format::Jpeg => encode_jpeg(r, quality),
        Format::Png => encode_png(r),
        Format::WebP => encode_webp(r, quality),
        Format::Gif => encode_gif(r),
    }
}

/// Kodiert die Full-Variante; verlustbehaftete Formate senken die Qualität, bis
/// die Größengrenze eingehalten ist. Liefert außerdem, ob die Grenze überschritten bleibt.
fn encode_limited(r: &Raster, format: Format) -> Result<(Vec<u8>, bool)> {
    let lossy = matches!(format, Format::Jpeg | Format::WebP);
    let mut quality = QUALITY;
    loop {
        let data = encode(r, format, quality)?;
        if data.len() <= FULL_MAX_BYTES {
            return Ok((data, false));
        }
        if !lossy || quality <= QUALITY_MIN {
            return Ok((data, true));
        }
        quality = quality.saturating_sub(QUALITY_STEP).max(QUALITY_MIN);
    }
}

/// Ergebnis der Verarbeitung eines Originals: Full (1000), Preview (700), Thumbnail (200).
/// Nur angeforderte Varianten werden kodiert.
pub struct Processed {
    pub variants: [Option<Variant>; 3],
    /// Full-Variante überschreitet die Größengrenze (nur bei verlustfreien Formaten möglich).
    pub oversized: bool,
}

/// Verarbeitet ein Original. `need` wählt Full, Preview und Thumbnail aus.
pub fn process(bytes: &[u8], format: Format, badge: &Badge, need: [bool; 3]) -> Result<Processed> {
    let img = decode(bytes)?;
    let alpha = img.color().has_alpha() && format != Format::Jpeg;
    let original = if alpha {
        let i = img.to_rgba8();
        Raster { width: i.width(), height: i.height(), alpha, pixels: i.into_raw() }
    } else {
        let i = img.to_rgb8();
        Raster { width: i.width(), height: i.height(), alpha, pixels: i.into_raw() }
    };
    drop(img);
    if original.width == 0 || original.height == 0 {
        bail!("Bild hat keine Pixel");
    }
    let full = original.scaled_to(FULL_WIDTH)?;
    drop(original);
    let preview = full.scaled_to(PREVIEW_WIDTH)?;

    let mut variants: [Option<Variant>; 3] = [None, None, None];
    let mut oversized = false;
    if need[2] {
        let thumb = preview.scaled_to(THUMB_WIDTH)?;
        let data = encode(&thumb, format, QUALITY)?;
        variants[2] = Some(Variant { width: thumb.width, height: thumb.height, data });
    }
    if need[1] {
        let mut r = preview;
        badge.apply(&mut r)?;
        let data = encode(&r, format, QUALITY)?;
        variants[1] = Some(Variant { width: r.width, height: r.height, data });
    }
    if need[0] {
        let mut r = full;
        badge.apply(&mut r)?;
        let (data, over) = encode_limited(&r, format)?;
        oversized = over;
        variants[0] = Some(Variant { width: r.width, height: r.height, data });
    }
    Ok(Processed { variants, oversized })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn badge() -> Badge {
        Badge::from_png(include_bytes!("../../assets/badge.png")).unwrap()
    }

    fn all(bytes: &[u8], format: Format) -> [Variant; 3] {
        process(bytes, format, &badge(), [true; 3]).unwrap().variants.map(Option::unwrap)
    }

    #[test]
    fn only_requested_variants_are_encoded() {
        let p = process(&jpeg(800, 600, None), Format::Jpeg, &badge(), [false, false, true]).unwrap();
        assert!(p.variants[0].is_none() && p.variants[1].is_none() && p.variants[2].is_some());
    }

    fn jpeg(width: u32, height: u32, orientation: Option<u16>) -> Vec<u8> {
        let mut pixels = Vec::with_capacity((width * height * 3) as usize);
        for y in 0..height {
            for x in 0..width {
                pixels.extend_from_slice(&[(x % 256) as u8, (y % 256) as u8, ((x * y) % 256) as u8]);
            }
        }
        let mut out = Vec::new();
        let mut enc = jpeg_encoder::Encoder::new(&mut out, 90);
        if let Some(o) = orientation {
            // Minimales EXIF mit Orientation-Tag
            let mut tiff = b"II\x2A\x00\x08\x00\x00\x00\x01\x00\x12\x01\x03\x00\x01\x00\x00\x00".to_vec();
            tiff.extend_from_slice(&o.to_le_bytes());
            tiff.extend_from_slice(&[0, 0, 0, 0, 0, 0]);
            enc.add_exif_metadata(&tiff).unwrap();
        }
        enc.encode(&pixels, width as u16, height as u16, jpeg_encoder::ColorType::Rgb).unwrap();
        out
    }

    #[test]
    fn portrait_original_is_not_upscaled() {
        let [full, preview, thumb] = all(&jpeg(562, 750, None), Format::Jpeg);
        assert_eq!((thumb.width, thumb.height), (200, 267));
        assert_eq!((preview.width, preview.height), (562, 750));
        assert_eq!((full.width, full.height), (562, 750));
    }

    #[test]
    fn large_original_is_scaled_and_limited() {
        let p = process(&jpeg(4592, 3448, None), Format::Jpeg, &badge(), [true; 3]).unwrap();
        assert!(!p.oversized);
        let [full, preview, _] = p.variants.map(Option::unwrap);
        assert_eq!((full.width, full.height), (1000, 751));
        assert_eq!(preview.width, 700);
        assert!(full.data.len() <= FULL_MAX_BYTES);
    }

    #[test]
    fn exif_orientation_is_applied() {
        let [full, ..] = all(&jpeg(800, 600, Some(6)), Format::Jpeg);
        assert_eq!((full.width, full.height), (600, 800));
    }

    #[test]
    fn badge_is_a_fifth_of_the_width() {
        assert_eq!(Badge::width_for(562), Some(112));
        assert_eq!(Badge::width_for(700), Some(140));
        assert_eq!(Badge::width_for(1000), Some(200));
        assert_eq!(Badge::width_for(300), Some(60));
        assert_eq!(Badge::width_for(200), None);
    }

    #[test]
    fn thumbnail_has_no_badge_but_preview_has() {
        // Einfarbiges Schwarz: das Badge hellt die Ecke auf
        let mut out = Vec::new();
        jpeg_encoder::Encoder::new(&mut out, 95)
            .encode(&vec![0u8; 1000 * 800 * 3], 1000, 800, jpeg_encoder::ColorType::Rgb)
            .unwrap();
        let [full, preview, thumb] = all(&out, Format::Jpeg);
        let corner_brightness = |v: &Variant| {
            let img = image::load_from_memory(&v.data).unwrap().to_luma8();
            let (w, h) = img.dimensions();
            let region = (w * 3 / 4..w).flat_map(|x| (h * 4 / 5..h).map(move |y| (x, y)));
            region.map(|(x, y)| img.get_pixel(x, y)[0] as u64).max().unwrap()
        };
        assert!(corner_brightness(&preview) > 60, "Badge fehlt auf Preview");
        assert!(corner_brightness(&full) > 60, "Badge fehlt auf Full");
        assert!(corner_brightness(&thumb) < 20, "Thumbnail darf kein Badge haben");
    }

    #[test]
    fn output_format_matches_extension() {
        let img = DynamicImage::new_rgba8(300, 200);
        let mut png = Vec::new();
        img.write_to(&mut Cursor::new(&mut png), image::ImageFormat::Png).unwrap();
        let [p, ..] = all(&png, Format::Png);
        assert!(p.data.starts_with(b"\x89PNG"));
        image::load_from_memory(&p.data).unwrap();
        let [w, ..] = all(&png, Format::WebP);
        assert!(w.data.starts_with(b"RIFF") && &w.data[8..12] == b"WEBP");
        image::load_from_memory(&w.data).unwrap();
        let [g, ..] = all(&png, Format::Gif);
        assert!(g.data.starts_with(b"GIF8"));
        image::load_from_memory(&g.data).unwrap();
        let [j, ..] = all(&jpeg(300, 200, None), Format::Jpeg);
        assert!(j.data.starts_with(b"\xFF\xD8"));
    }

    #[test]
    fn rights_metadata_is_embedded_and_gps_dropped() {
        // Original mit GPS-IFD-Zeiger
        let mut pixels = vec![128u8; 400 * 300 * 3];
        pixels[0] = 1;
        let mut orig = Vec::new();
        let mut enc = jpeg_encoder::Encoder::new(&mut orig, 90);
        let gps_tiff = b"II\x2A\x00\x08\x00\x00\x00\x01\x00\x25\x88\x04\x00\x01\x00\x00\x00\x1A\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x02\x00\x02\x00\x00\x00\x4E\x00\x00\x00\x00\x00\x00\x00";
        enc.add_exif_metadata(gps_tiff).unwrap();
        enc.encode(&pixels, 400, 300, jpeg_encoder::ColorType::Rgb).unwrap();
        assert!(exif::Reader::new().read_from_container(&mut Cursor::new(&orig)).unwrap()
            .fields().any(|f| f.ifd_num == exif::In::PRIMARY && f.tag.context() == exif::Context::Gps));

        for v in &all(&orig, Format::Jpeg) {
            let e = exif::Reader::new().read_from_container(&mut Cursor::new(&v.data)).unwrap();
            assert!(e.fields().all(|f| f.tag.context() != exif::Context::Gps));
            let artist = e.get_field(exif::Tag::Artist, exif::In::PRIMARY).unwrap();
            assert!(artist.display_value().to_string().contains("Flauschiversum / Ermeline Wollknoll"));
            let text = String::from_utf8_lossy(&v.data);
            assert!(text.contains("xmpRights:WebStatement"));
            assert!(v.data.windows(4).any(|w| w == b"8BIM"));
        }
    }
}
