//! Windows resources for Decode CAD. The icon is rendered from original Decode CAD geometry,
//! rather than loading inherited product artwork.

use std::path::Path;

fn set_pixel(pixels: &mut [u8], size: usize, x: i32, y: i32, color: [u8; 4]) {
    if x < 0 || y < 0 || x >= size as i32 || y >= size as i32 {
        return;
    }
    let at = (y as usize * size + x as usize) * 4;
    pixels[at..at + 4].copy_from_slice(&color);
}

fn disk(pixels: &mut [u8], size: usize, cx: i32, cy: i32, radius: i32, color: [u8; 4]) {
    for dy in -radius..=radius {
        for dx in -radius..=radius {
            if dx * dx + dy * dy <= radius * radius {
                set_pixel(pixels, size, cx + dx, cy + dy, color);
            }
        }
    }
}

fn line(pixels: &mut [u8], size: usize, a: (i32, i32), b: (i32, i32), color: [u8; 4], width: i32) {
    let steps = (b.0 - a.0).abs().max((b.1 - a.1).abs()).max(1);
    for n in 0..=steps {
        let x = a.0 + (b.0 - a.0) * n / steps;
        let y = a.1 + (b.1 - a.1) * n / steps;
        disk(pixels, size, x, y, width, color);
    }
}

fn point(size: usize, x: f32, y: f32) -> (i32, i32) {
    ((size as f32 / 2.0 + x * size as f32).round() as i32, (size as f32 / 2.0 + y * size as f32).round() as i32)
}

fn decode_icon_pixels(size: usize) -> Vec<u8> {
    let mut pixels = vec![0u8; size * size * 4];
    let navy = [14, 24, 42, 255];
    let border = [43, 65, 94, 255];
    let blue = [56, 137, 255, 255];
    let cyan = [43, 220, 220, 255];
    let white = [244, 248, 255, 255];
    let orange = [255, 166, 74, 255];
    let inset = (size as i32 * 6 / 256).max(1);
    let radius = (size as i32 * 40 / 256).max(2);
    let edge = size as i32 - inset - 1;
    for y in 0..size as i32 {
        for x in 0..size as i32 {
            let nearest_x = x.clamp(inset + radius, edge - radius);
            let nearest_y = y.clamp(inset + radius, edge - radius);
            let dx = x - nearest_x;
            let dy = y - nearest_y;
            if dx * dx + dy * dy <= radius * radius {
                set_pixel(&mut pixels, size, x, y, navy);
            }
        }
    }
    // Subtle inset edge; the mark itself has an alpha channel outside the rounded square.
    for x in inset + radius..=edge - radius {
        set_pixel(&mut pixels, size, x, inset, border);
        set_pixel(&mut pixels, size, x, edge, border);
    }
    for y in inset + radius..=edge - radius {
        set_pixel(&mut pixels, size, inset, y, border);
        set_pixel(&mut pixels, size, edge, y, border);
    }
    let cube = [(-0.30, -0.14), (0.0, -0.31), (0.30, -0.14), (0.30, 0.17), (0.0, 0.34), (-0.30, 0.17), (-0.30, -0.14)];
    for segment in cube.windows(2) {
        line(&mut pixels, size, point(size, segment[0].0, segment[0].1), point(size, segment[1].0, segment[1].1), blue, (size as i32 / 128).max(1));
    }
    for (a, b) in [
        (point(size, 0.0, -0.31), point(size, 0.0, 0.0)),
        (point(size, -0.30, 0.17), point(size, 0.0, 0.0)),
        (point(size, 0.30, 0.17), point(size, 0.0, 0.0)),
    ] {
        line(&mut pixels, size, a, b, cyan, (size as i32 / 170).max(1));
    }
    let d = [
        (-0.105, -0.155), (-0.105, 0.155), (-0.01, 0.155), (0.075, 0.105),
        (0.105, 0.0), (0.075, -0.105), (-0.01, -0.155), (-0.105, -0.155),
    ];
    for segment in d.windows(2) {
        line(&mut pixels, size, point(size, segment[0].0, segment[0].1), point(size, segment[1].0, segment[1].1), white, (size as i32 / 190).max(1));
    }
    let accent_a = point(size, 0.15, -0.22);
    let accent_b = point(size, 0.23, -0.265);
    line(&mut pixels, size, accent_a, accent_b, orange, (size as i32 / 180).max(1));
    disk(&mut pixels, size, accent_b.0, accent_b.1, (size as i32 / 86).max(1), orange);
    // Retain a visible border on all four straight sections, including its antialias-free inner edge.
    for x in inset + radius..=edge - radius {
        set_pixel(&mut pixels, size, x, inset + 1, border);
        set_pixel(&mut pixels, size, x, edge - 1, border);
    }
    for y in inset + radius..=edge - radius {
        set_pixel(&mut pixels, size, inset + 1, y, border);
        set_pixel(&mut pixels, size, edge - 1, y, border);
    }
    pixels
}

fn write_decode_ico(path: &Path) -> std::io::Result<()> {
    let size = 256usize;
    let pixels = decode_icon_pixels(size);
    let xor_len = pixels.len();
    let mask_row = size.div_ceil(32) * 4;
    let mask_len = mask_row * size;
    let bytes_in_resource = 40usize + xor_len + mask_len;
    let mut ico = Vec::with_capacity(22 + bytes_in_resource);
    ico.extend_from_slice(&0u16.to_le_bytes());
    ico.extend_from_slice(&1u16.to_le_bytes());
    ico.extend_from_slice(&1u16.to_le_bytes());
    ico.extend_from_slice(&[0, 0, 0, 0]); // Width/height zero denote 256x256.
    ico.extend_from_slice(&[0, 0]); // Palette and reserved.
    ico.extend_from_slice(&1u16.to_le_bytes());
    ico.extend_from_slice(&32u16.to_le_bytes());
    ico.extend_from_slice(&(bytes_in_resource as u32).to_le_bytes());
    ico.extend_from_slice(&22u32.to_le_bytes());

    // BITMAPINFOHEADER for a 32-bit BGRA DIB, bottom-up, with a 1-bit AND mask.
    ico.extend_from_slice(&40u32.to_le_bytes());
    ico.extend_from_slice(&(size as i32).to_le_bytes());
    ico.extend_from_slice(&((size * 2) as i32).to_le_bytes());
    ico.extend_from_slice(&1u16.to_le_bytes());
    ico.extend_from_slice(&32u16.to_le_bytes());
    ico.extend_from_slice(&0u32.to_le_bytes());
    ico.extend_from_slice(&(xor_len as u32).to_le_bytes());
    ico.extend_from_slice(&0i32.to_le_bytes());
    ico.extend_from_slice(&0i32.to_le_bytes());
    ico.extend_from_slice(&0u32.to_le_bytes());
    ico.extend_from_slice(&0u32.to_le_bytes());
    for y in (0..size).rev() {
        for x in 0..size {
            let i = (y * size + x) * 4;
            ico.extend_from_slice(&[pixels[i + 2], pixels[i + 1], pixels[i], pixels[i + 3]]);
        }
    }
    ico.resize(ico.len() + mask_len, 0);
    std::fs::write(path, ico)
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CADCRAFT_REQUIRE_WINRES");
    println!("cargo:rerun-if-env-changed=CADCRAFT_BUILD_SHA");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let Some(out_dir) = std::env::var_os("OUT_DIR").map(std::path::PathBuf::from) else {
        println!("cargo:warning=Decode CAD icon was not generated: OUT_DIR is unset");
        return;
    };
    let icon_path = out_dir.join("decode-cad.ico");
    if let Err(error) = write_decode_ico(&icon_path) {
        println!("cargo:warning=Decode CAD icon generation failed: {error}");
    }
    let mut res = winresource::WindowsResource::new();
    if icon_path.exists() {
        if let Some(icon) = icon_path.to_str() {
            res.set_icon(icon);
        }
    }
    res.set("ProductName", "Decode CAD")
        .set("FileDescription", "Decode CAD — computer-aided design")
        .set("CompanyName", "Decode Analytics")
        .set("LegalCopyright", "Copyright (c) 2026 Decode Analytics and contributors; see bundled license notices. MIT OR Apache-2.0.")
        .set("OriginalFilename", "cadcraft.exe")
        .set("InternalName", "cadcraft");
    if let Err(error) = res.compile() {
        if std::env::var_os("CADCRAFT_REQUIRE_WINRES").is_some() {
            eprintln!("embedding Windows resources failed: {error}");
            std::process::exit(1);
        }
        println!("cargo:warning=Decode CAD built without Windows version/icon resources: {error}");
    }
}
