//! Deterministic preview of the actual core renderer, without After Effects.
use glitchtrace::core::*;
use std::io::Write;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let w = 640;
    let h = 360;
    let mut image = vec![[0.025, 0.025, 0.025, 1.]; w * h];
    for (x0, y0, x1, y1) in [
        (75, 85, 165, 165),
        (280, 155, 360, 225),
        (450, 65, 530, 115),
    ] {
        for y in y0..y1 {
            for x in x0..x1 {
                image[y * w + x] = [0., 1., 0., 1.];
            }
        }
    }
    let settings = Settings {
        quality: 0,
        text_scale: 1.5,
        text_offset: 10.,
        border_width: 2.,
        fill: [0.1, 0.1, 0.1, 1.],
        fill_opacity: 0.25,
        dashed: true,
        dash: 10.,
        gap: 5.,
        arrows: true,
        marker_size: 12.,
        ..Settings::default()
    };
    let out = render(&image, w, h, &settings, || true)?;
    assert_eq!(out.boxes.len(), 3);
    let path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "fixture.ppm".into());
    let mut f = std::fs::File::create(path)?;
    write!(f, "P6\n{w} {h}\n255\n")?;
    for p in out.pixels {
        f.write_all(&[
            (p[0] * 255.) as u8,
            (p[1] * 255.) as u8,
            (p[2] * 255.) as u8,
        ])?;
    }
    Ok(())
}
