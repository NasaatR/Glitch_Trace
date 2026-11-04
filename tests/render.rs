use glitchtrace::core::*;
fn plain() -> Settings {
    Settings {
        quality: 0,
        boxes: false,
        text: false,
        markers: false,
        connections: false,
        ..Settings::default()
    }
}
fn fixture() -> (Vec<Pixel>, usize, usize) {
    let (w, h) = (80, 60);
    let mut p = vec![[0., 0., 0., 1.]; w * h];
    for y in 20..30 {
        for x in 10..20 {
            p[y * w + x] = [0., 1., 0., 1.];
        }
    }
    for y in 35..45 {
        for x in 50..65 {
            p[y * w + x] = [0., 1., 0., 1.];
        }
    }
    (p, w, h)
}
#[test]
fn finds_two_regions_and_preserves_source() {
    let (p, w, h) = fixture();
    let o = render(&p, w, h, &plain(), || true).unwrap();
    assert_eq!(o.pixels, p);
    assert_eq!(
        o.boxes,
        vec![
            Box2 {
                x0: 10.,
                y0: 20.,
                x1: 20.,
                y1: 30.
            },
            Box2 {
                x0: 50.,
                y0: 35.,
                x1: 65.,
                y1: 45.
            }
        ]
    );
}
#[test]
fn mask_cutoff_blur_and_transparency() {
    let s = Settings {
        threshold: 20.,
        ..plain()
    };
    let p = vec![
        [0., 1., 0., 1.],
        [0., 0.95, 0., 1.],
        [0., 0.8, 0., 1.],
        [0., 1., 0., 0.],
    ];
    assert_eq!(mask(&p, 4, 1, &s, &mut || true).unwrap(), vec![1, 1, 0, 0]);
    let p = vec![[0., 1., 0., 1.]; 25];
    assert!(
        mask(&p, 5, 5, &Settings { blur: 2., ..s }, &mut || true)
            .unwrap()
            .iter()
            .all(|x| *x == 1)
    );
}
#[test]
fn alpha_mode_and_union_opacity() {
    let (p, w, h) = fixture();
    let mut s = plain();
    s.alpha_layer = true;
    s.boxes = true;
    s.border_width = 0.;
    s.fill = [1., 0., 0., 1.];
    s.fill_opacity = 0.5;
    let o = render(&p, w, h, &s, || true).unwrap();
    assert_eq!(o.pixels[0], [0.; 4]);
    assert_eq!(o.pixels[25 * w + 15], [0.5, 0., 0., 0.5]);
}
#[test]
fn zero_settings_are_not_replaced() {
    let (p, w, h) = fixture();
    let s = Settings {
        alpha_layer: true,
        boxes: true,
        fill_opacity: 0.,
        border_width: 0.,
        markers: true,
        marker_size: 0.,
        marker_width: 0.,
        ..plain()
    };
    assert!(
        render(&p, w, h, &s, || true)
            .unwrap()
            .pixels
            .iter()
            .all(|p| *p == [0.; 4])
    );
}
#[test]
fn markers_lines_arrows_text_and_gaps_draw() {
    let (p, w, h) = fixture();
    let base = Settings {
        alpha_layer: true,
        ..plain()
    };
    for s in [
        Settings {
            markers: true,
            marker_type: 0,
            ..base.clone()
        },
        Settings {
            markers: true,
            marker_type: 1,
            ..base.clone()
        },
        Settings {
            markers: true,
            marker_type: 2,
            ..base.clone()
        },
        Settings {
            connections: true,
            ..base.clone()
        },
        Settings {
            connections: true,
            dashed: true,
            arrows: true,
            ..base.clone()
        },
        Settings {
            text: true,
            ..base.clone()
        },
        Settings {
            boxes: true,
            gaps: true,
            ..base.clone()
        },
    ] {
        let o = render(&p, w, h, &s, || true).unwrap();
        assert!(o.pixels.iter().any(|p| p[3] > 0.));
        assert!(
            o.pixels
                .iter()
                .all(|p| p.iter().all(|x| x.is_finite() && *x >= 0. && *x <= 1.))
        );
    }
}
#[test]
fn deterministic_empty_and_black_transparent() {
    let p = vec![[0.; 4]; 100];
    let s = Settings {
        key: [0., 0., 0., 1.],
        ..plain()
    };
    let a = render(&p, 10, 10, &s, || true).unwrap();
    assert!(a.boxes.is_empty());
    assert_eq!(a.pixels, p);
    let (p, w, h) = fixture();
    assert_eq!(
        render(&p, w, h, &s, || true).unwrap().pixels,
        render(&p, w, h, &s, || true).unwrap().pixels
    );
}
#[test]
fn bounds_invalid_values_and_cancel() {
    assert_eq!(
        render(&[], 0, 0, &plain(), || true).unwrap_err(),
        RenderError::Dimensions
    );
    assert_eq!(
        render(&[[0.; 4]], 2, 2, &plain(), || true).unwrap_err(),
        RenderError::Dimensions
    );
    let (p, w, h) = fixture();
    assert_eq!(
        render(
            &p,
            w,
            h,
            &Settings {
                blur: f32::NAN,
                ..plain()
            },
            || true
        )
        .unwrap_err(),
        RenderError::InvalidSettings
    );
    assert_eq!(
        render(&p, w, h, &plain(), || false).unwrap_err(),
        RenderError::Cancelled
    );
}
#[test]
fn connected_diagonal_and_region_cap() {
    let m = vec![1, 0, 0, 1];
    assert_eq!(detect(&m, 2, 2, 0, &mut || true).unwrap().len(), 1);
    let w = 100;
    let h = 100;
    let mut m = vec![0; w * h];
    for y in (0..h).step_by(3) {
        for x in (0..w).step_by(3) {
            m[y * w + x] = 1;
        }
    }
    assert_eq!(detect(&m, w, h, 0, &mut || true).unwrap().len(), 512);
}
#[test]
fn quality_modes_and_narrow_frames() {
    for q in 0..6 {
        let s = Settings {
            quality: q,
            ..plain()
        };
        let p = vec![[0., 1., 0., 1.]; 63];
        assert_eq!(render(&p, 1, 63, &s, || true).unwrap().boxes.len(), 1);
        assert_eq!(render(&p, 63, 1, &s, || true).unwrap().boxes.len(), 1);
    }
}
