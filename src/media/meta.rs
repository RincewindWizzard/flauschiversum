//! Rechteangaben als EXIF, XMP und IPTC sowie deren Einbettung in PNG, WebP und GIF.
//! (Für JPEG übernimmt der Encoder das Einbetten der Segmente.)

use crate::config::{RIGHTS_CREATOR, RIGHTS_WEB_STATEMENT};

fn copyright_ascii() -> String {
    format!("Copyright {RIGHTS_CREATOR}")
}

fn copyright_text() -> String {
    format!("© {RIGHTS_CREATOR}")
}

/// TIFF-Struktur (little endian) mit IFD0: Artist und Copyright.
pub fn exif_tiff() -> Vec<u8> {
    let artist = format!("{RIGHTS_CREATOR}\0");
    let copyright = format!("{}\0", copyright_ascii());
    let entries = 2u16;
    let data_start = 8 + 2 + 12 * entries as u32 + 4;
    let mut out = Vec::new();
    out.extend_from_slice(b"II\x2A\x00");
    out.extend_from_slice(&8u32.to_le_bytes());
    out.extend_from_slice(&entries.to_le_bytes());
    let mut offset = data_start;
    for (tag, text) in [(0x013Bu16, &artist), (0x8298u16, &copyright)] {
        out.extend_from_slice(&tag.to_le_bytes());
        out.extend_from_slice(&2u16.to_le_bytes()); // ASCII
        out.extend_from_slice(&(text.len() as u32).to_le_bytes());
        out.extend_from_slice(&offset.to_le_bytes());
        offset += text.len() as u32;
    }
    out.extend_from_slice(&0u32.to_le_bytes()); // kein weiteres IFD
    out.extend_from_slice(artist.as_bytes());
    out.extend_from_slice(copyright.as_bytes());
    out
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// XMP-Paket mit Urheber, Rechten, Lizenzhinweis und Credit.
pub fn xmp_packet() -> String {
    let creator = xml_escape(RIGHTS_CREATOR);
    let rights = xml_escape(&copyright_text());
    let statement = xml_escape(RIGHTS_WEB_STATEMENT);
    format!(
        concat!(
            "<?xpacket begin=\"\u{feff}\" id=\"W5M0MpCehiHzreSzNTczkc9d\"?>",
            "<x:xmpmeta xmlns:x=\"adobe:ns:meta/\">",
            "<rdf:RDF xmlns:rdf=\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\">",
            "<rdf:Description rdf:about=\"\"",
            " xmlns:dc=\"http://purl.org/dc/elements/1.1/\"",
            " xmlns:xmpRights=\"http://ns.adobe.com/xap/1.0/rights/\"",
            " xmlns:photoshop=\"http://ns.adobe.com/photoshop/1.0/\">",
            "<dc:creator><rdf:Seq><rdf:li>{creator}</rdf:li></rdf:Seq></dc:creator>",
            "<dc:rights><rdf:Alt><rdf:li xml:lang=\"x-default\">{rights}</rdf:li></rdf:Alt></dc:rights>",
            "<xmpRights:Marked>True</xmpRights:Marked>",
            "<xmpRights:WebStatement>{statement}</xmpRights:WebStatement>",
            "<photoshop:Credit>{creator}</photoshop:Credit>",
            "</rdf:Description></rdf:RDF></x:xmpmeta>",
            "<?xpacket end=\"r\"?>"
        ),
        creator = creator,
        rights = rights,
        statement = statement
    )
}

/// Inhalt eines JPEG-APP1-Segments mit XMP.
pub fn jpeg_xmp_segment() -> Vec<u8> {
    let mut out = b"http://ns.adobe.com/xap/1.0/\0".to_vec();
    out.extend_from_slice(xmp_packet().as_bytes());
    out
}

/// Inhalt eines JPEG-APP13-Segments mit IPTC-IIM (By-line, Copyright Notice, Credit).
pub fn jpeg_iptc_segment() -> Vec<u8> {
    let mut iim = Vec::new();
    let mut record = |rec: u8, ds: u8, data: &[u8]| {
        iim.extend_from_slice(&[0x1C, rec, ds]);
        iim.extend_from_slice(&(data.len() as u16).to_be_bytes());
        iim.extend_from_slice(data);
    };
    record(1, 90, b"\x1B%G"); // Zeichensatz UTF-8
    record(2, 80, RIGHTS_CREATOR.as_bytes()); // By-line
    record(2, 116, copyright_text().as_bytes()); // Copyright Notice
    record(2, 110, RIGHTS_CREATOR.as_bytes()); // Credit
    let mut out = b"Photoshop 3.0\0".to_vec();
    out.extend_from_slice(b"8BIM");
    out.extend_from_slice(&0x0404u16.to_be_bytes());
    out.extend_from_slice(&[0, 0]); // leerer Pascal-Name (gepolstert)
    out.extend_from_slice(&(iim.len() as u32).to_be_bytes());
    out.extend_from_slice(&iim);
    if iim.len() % 2 == 1 {
        out.push(0);
    }
    out
}

fn png_chunk(kind: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(12 + data.len());
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    out.extend_from_slice(kind);
    out.extend_from_slice(data);
    let mut crc = crc32fast::Hasher::new();
    crc.update(kind);
    crc.update(data);
    out.extend_from_slice(&crc.finalize().to_be_bytes());
    out
}

/// Fügt eXIf- und XMP-Chunk (iTXt) direkt nach IHDR in ein PNG ein.
pub fn png_with_rights(png: &[u8]) -> Vec<u8> {
    // Signatur (8) + IHDR-Chunk (4 + 4 + 13 + 4)
    let ihdr_end = 8 + 25;
    let mut itxt = b"XML:com.adobe.xmp\0\0\0\0\0".to_vec();
    itxt.extend_from_slice(xmp_packet().as_bytes());
    let mut out = Vec::with_capacity(png.len() + 2048);
    out.extend_from_slice(&png[..ihdr_end]);
    out.extend_from_slice(&png_chunk(b"eXIf", &exif_tiff()));
    out.extend_from_slice(&png_chunk(b"iTXt", &itxt));
    out.extend_from_slice(&png[ihdr_end..]);
    out
}

fn riff_chunk(id: &[u8; 4], data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(8 + data.len() + 1);
    out.extend_from_slice(id);
    out.extend_from_slice(&(data.len() as u32).to_le_bytes());
    out.extend_from_slice(data);
    if data.len() % 2 == 1 {
        out.push(0);
    }
    out
}

/// Baut ein WebP im erweiterten Format (VP8X) mit EXIF- und XMP-Chunk.
pub fn webp_with_rights(webp: &[u8], width: u32, height: u32) -> Vec<u8> {
    let mut image_chunks = Vec::new();
    let mut has_alpha = false;
    let mut i = 12;
    while i + 8 <= webp.len() {
        let id: [u8; 4] = webp[i..i + 4].try_into().unwrap();
        let len = u32::from_le_bytes(webp[i + 4..i + 8].try_into().unwrap()) as usize;
        let end = (i + 8 + len).min(webp.len());
        match &id {
            b"ALPH" => {
                has_alpha = true;
                image_chunks.extend_from_slice(&riff_chunk(&id, &webp[i + 8..end]));
            }
            b"VP8 " => image_chunks.extend_from_slice(&riff_chunk(&id, &webp[i + 8..end])),
            b"VP8L" => {
                // Alpha-Hinweis im VP8L-Kopf (Bit 28 nach Signatur-Byte)
                if end - (i + 8) >= 5 {
                    let bits = u32::from_le_bytes(webp[i + 9..i + 13].try_into().unwrap());
                    has_alpha |= bits & (1 << 28) != 0;
                }
                image_chunks.extend_from_slice(&riff_chunk(&id, &webp[i + 8..end]));
            }
            b"VP8X" => {
                has_alpha |= webp.get(i + 8).is_some_and(|f| f & 0x10 != 0);
            }
            _ => {}
        }
        i = end + (len % 2);
    }
    let mut flags = 0x08 | 0x04; // EXIF + XMP
    if has_alpha {
        flags |= 0x10;
    }
    let mut vp8x = vec![flags, 0, 0, 0];
    vp8x.extend_from_slice(&(width - 1).to_le_bytes()[..3]);
    vp8x.extend_from_slice(&(height - 1).to_le_bytes()[..3]);
    let mut body = b"WEBP".to_vec();
    body.extend_from_slice(&riff_chunk(b"VP8X", &vp8x));
    body.extend_from_slice(&image_chunks);
    body.extend_from_slice(&riff_chunk(b"EXIF", &exif_tiff()));
    body.extend_from_slice(&riff_chunk(b"XMP ", xmp_packet().as_bytes()));
    let mut out = b"RIFF".to_vec();
    out.extend_from_slice(&(body.len() as u32).to_le_bytes());
    out.extend_from_slice(&body);
    out
}

/// Fügt XMP als Application Extension vor dem ersten Einzelbild in ein GIF ein.
pub fn gif_with_rights(gif: &[u8]) -> Vec<u8> {
    let mut pos = 13;
    let flags = gif[10];
    if flags & 0x80 != 0 {
        pos += 3 * (1 << ((flags & 0x07) + 1));
    }
    let mut ext = vec![0x21, 0xFF, 0x0B];
    ext.extend_from_slice(b"XMP DataXMP");
    ext.extend_from_slice(xmp_packet().as_bytes());
    // „Magic Trailer“: 0x01, 0xFF … 0x00, Terminator 0x00
    ext.push(0x01);
    ext.extend((0..=255u8).rev());
    ext.push(0x00);
    let mut out = Vec::with_capacity(gif.len() + ext.len());
    out.extend_from_slice(&gif[..pos]);
    out.extend_from_slice(&ext);
    out.extend_from_slice(&gif[pos..]);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exif_is_readable() {
        let exif = exif::Reader::new().read_raw(exif_tiff()).unwrap();
        let artist = exif.get_field(exif::Tag::Artist, exif::In::PRIMARY).unwrap();
        assert!(artist.display_value().to_string().contains("Flauschiversum / Ermeline Wollknoll"));
        let copyright = exif.get_field(exif::Tag::Copyright, exif::In::PRIMARY).unwrap();
        assert!(copyright.display_value().to_string().contains("Copyright Flauschiversum"));
    }

    #[test]
    fn xmp_contains_rights() {
        let x = xmp_packet();
        assert!(x.contains("<dc:creator><rdf:Seq><rdf:li>Flauschiversum / Ermeline Wollknoll</rdf:li>"));
        assert!(x.contains("<xmpRights:WebStatement>https://flauschiversum.de/impressum/</xmpRights:WebStatement>"));
    }

    #[test]
    fn iptc_segment_is_well_formed() {
        let seg = jpeg_iptc_segment();
        assert!(seg.starts_with(b"Photoshop 3.0\08BIM\x04\x04"));
        let len = u32::from_be_bytes(seg[22..26].try_into().unwrap()) as usize;
        assert_eq!(seg.len(), 26 + len + len % 2);
    }
}
