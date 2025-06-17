use image::{GrayImage, Luma};

pub fn atkinson(img: &GrayImage) -> GrayImage {
    let (w, h) = (img.width(), img.height());
    let mut out = img.clone();

    for y in 0..h {
        for x in 0..w {
            let old = out[(x, y)][0];
            let new = if old > 127 { 255 } else { 0 };
            out[(x, y)] = Luma([new]);

            let err = (old as i16 - new as i16) / 8;

            // Atkinson dithering
            for &(dx, dy) in &[(1, 0), (2, 0), (-1, 1), (0, 1), (1, 1), (0, 2)] {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);

                if nx >= 0 && nx < w as i32 && ny >= 0 && ny < h as i32 {
                    let (nx, ny) = (nx as u32, ny as u32);
                    let val = (out[(nx, ny)][0] as i16 + err).clamp(0, 255) as u8;
                    out[(nx, ny)] = Luma([val]);
                }
            }
        }
    }

    out
}
