//! Independent CPU reference renderer. Input/output are premultiplied RGBA.
//! No host calls, network access, image assets or original binary code.
use std::collections::VecDeque;

pub type Pixel = [f32; 4];
pub type Color = [f32; 4];
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Box2 {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
}
impl Box2 {
    fn center(self) -> [f32; 2] {
        [(self.x0 + self.x1) * 0.5, (self.y0 + self.y1) * 0.5]
    }
}
#[derive(Clone, Debug)]
pub struct Settings {
    pub key: Color,
    pub threshold: f32,
    pub show_mask: bool,
    pub blur: f32,
    pub alpha_layer: bool,
    pub quality: usize,
    pub boxes: bool,
    pub border: Color,
    pub border_width: f32,
    pub fill: Color,
    pub fill_opacity: f32,
    pub gaps: bool,
    pub gap_size: f32,
    pub text: bool,
    pub text_color: Color,
    pub text_scale: f32,
    pub text_offset: f32,
    pub markers: bool,
    pub marker_type: usize,
    pub marker_color: Color,
    pub marker_size: f32,
    pub marker_width: f32,
    pub connections: bool,
    pub line_color: Color,
    pub line_opacity: f32,
    pub line_width: f32,
    pub curvature: f32,
    pub dashed: bool,
    pub dash: f32,
    pub gap: f32,
    pub arrows: bool,
    pub arrow_size: f32,
    pub arrow_angle: f32,
    pub arrow_position: f32,
    pub arrow_filled: bool,
    /// Render pixels per full-resolution pixel. Independent X/Y preview scales.
    pub scale: [f32; 2],
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            key: [0., 1., 0., 1.],
            threshold: 10.,
            show_mask: false,
            blur: 0.,
            alpha_layer: false,
            quality: 2,
            boxes: true,
            border: [1.; 4],
            border_width: 1.,
            fill: [0., 0., 0., 1.],
            fill_opacity: 0.,
            gaps: false,
            gap_size: 0.2,
            text: true,
            text_color: [1.; 4],
            text_scale: 1.,
            text_offset: 4.,
            markers: true,
            marker_type: 1,
            marker_color: [1.; 4],
            marker_size: 8.,
            marker_width: 1.,
            connections: true,
            line_color: [1.; 4],
            line_opacity: 1.,
            line_width: 1.,
            curvature: 0.5,
            dashed: false,
            dash: 10.,
            gap: 5.,
            arrows: false,
            arrow_size: 8.,
            arrow_angle: 30.,
            arrow_position: 0.5,
            arrow_filled: true,
            scale: [1.; 2],
        }
    }
}
impl Settings {
    pub fn validate(&self) -> Result<(), RenderError> {
        let numbers = [
            self.threshold,
            self.blur,
            self.border_width,
            self.fill_opacity,
            self.gap_size,
            self.text_scale,
            self.text_offset,
            self.marker_size,
            self.marker_width,
            self.line_opacity,
            self.line_width,
            self.curvature,
            self.dash,
            self.gap,
            self.arrow_size,
            self.arrow_angle,
            self.arrow_position,
            self.scale[0],
            self.scale[1],
        ];
        if numbers
            .iter()
            .chain(self.key.iter())
            .chain(self.border.iter())
            .chain(self.fill.iter())
            .chain(self.text_color.iter())
            .chain(self.marker_color.iter())
            .chain(self.line_color.iter())
            .any(|v| !v.is_finite())
        {
            return Err(RenderError::InvalidSettings);
        }
        if !(0. ..=100.).contains(&self.threshold)
            || !(0. ..=100.).contains(&self.blur)
            || !(0. ..=1.).contains(&self.fill_opacity)
            || !(0. ..=1.).contains(&self.gap_size)
            || !(0. ..=1.).contains(&self.line_opacity)
            || !(-1. ..=1.).contains(&self.curvature)
            || !(0. ..=1.).contains(&self.arrow_position)
            || !(1. ..=89.).contains(&self.arrow_angle)
            || !(0.1..=20.).contains(&self.text_scale)
            || self.quality > 5
            || self.marker_type > 2
            || self.scale.iter().any(|x| *x <= 0. || *x > 1.)
            || [
                self.border_width,
                self.marker_size,
                self.marker_width,
                self.line_width,
                self.dash,
                self.gap,
                self.arrow_size,
            ]
            .iter()
            .any(|x| *x < 0. || *x > 1000.)
        {
            return Err(RenderError::InvalidSettings);
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderError {
    Dimensions,
    InvalidSettings,
    Cancelled,
    Allocation,
}
impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for RenderError {}
fn zeros<T: Default + Clone>(n: usize) -> Result<Vec<T>, RenderError> {
    let mut v = Vec::new();
    v.try_reserve_exact(n)
        .map_err(|_| RenderError::Allocation)?;
    v.resize(n, T::default());
    Ok(v)
}
fn checkpoint(cb: &mut impl FnMut() -> bool) -> Result<(), RenderError> {
    if cb() {
        Ok(())
    } else {
        Err(RenderError::Cancelled)
    }
}
/// Gaussian blur of RGB distance, followed by the binary midpoint of smoothstep.
/// The threshold in the UI is a percentage; the binary cutoff is half that distance.
pub fn mask(
    input: &[Pixel],
    w: usize,
    h: usize,
    s: &Settings,
    cb: &mut impl FnMut() -> bool,
) -> Result<Vec<u8>, RenderError> {
    s.validate()?;
    if w.checked_mul(h) != Some(input.len()) || w == 0 || h == 0 {
        return Err(RenderError::Dimensions);
    }
    let mut d = zeros::<f32>(input.len())?;
    for y in 0..h {
        if y % 32 == 0 {
            checkpoint(cb)?;
        }
        for x in 0..w {
            let i = y * w + x;
            let p = input[i];
            d[i] =
                ((p[0] - s.key[0]).powi(2) + (p[1] - s.key[1]).powi(2) + (p[2] - s.key[2]).powi(2))
                    .sqrt();
        }
    }
    // Separable Gaussian is equivalent to the square 2D kernel, with clamped edges.
    // Radius remains in rendered pixels to match a shader texture-space radius.
    let radius = s.blur.ceil() as i32;
    if radius > 0 {
        let sigma = s.blur.max(0.001);
        let mut kernel: Vec<f32> = (-radius..=radius)
            .map(|x| (-(x * x) as f32 / (2. * sigma * sigma)).exp())
            .collect();
        let sum: f32 = kernel.iter().sum();
        for k in &mut kernel {
            *k /= sum;
        }
        let mut tmp = zeros::<f32>(d.len())?;
        for y in 0..h {
            if y % 8 == 0 {
                checkpoint(cb)?;
            }
            for x in 0..w {
                tmp[y * w + x] = kernel
                    .iter()
                    .enumerate()
                    .map(|(k, a)| {
                        let xx = (x as i32 + k as i32 - radius).clamp(0, w as i32 - 1) as usize;
                        d[y * w + xx] * a
                    })
                    .sum();
            }
        }
        for y in 0..h {
            if y % 8 == 0 {
                checkpoint(cb)?;
            }
            for x in 0..w {
                d[y * w + x] = kernel
                    .iter()
                    .enumerate()
                    .map(|(k, a)| {
                        let yy = (y as i32 + k as i32 - radius).clamp(0, h as i32 - 1) as usize;
                        tmp[yy * w + x] * a
                    })
                    .sum();
            }
        }
    }
    let cutoff = s.threshold * 0.005;
    Ok(d.iter()
        .zip(input)
        .map(|(v, p)| u8::from(*v <= cutoff && p[3] > 0.))
        .collect())
}
/// Eight-connected components in scan order. At most 512 visible boxes.
/// Quality sampling is an independent policy; the original CPU policy is unknown.
pub fn detect(
    m: &[u8],
    w: usize,
    h: usize,
    quality: usize,
    cb: &mut impl FnMut() -> bool,
) -> Result<Vec<Box2>, RenderError> {
    if w.checked_mul(h) != Some(m.len()) || w == 0 || h == 0 || quality > 5 {
        return Err(RenderError::Dimensions);
    }
    let step = [1, 2, 3, 4, 6, 8][quality];
    let gw = w.div_ceil(step);
    let gh = h.div_ceil(step);
    let mut visited = zeros::<u8>(gw * gh)?;
    let mut queue = VecDeque::new();
    let mut boxes = Vec::new();
    for gy in 0..gh {
        checkpoint(cb)?;
        for gx in 0..gw {
            let idx = gy * gw + gx;
            if visited[idx] != 0 || m[(gy * step) * w + gx * step] == 0 {
                continue;
            }
            visited[idx] = 1;
            queue.push_back((gx, gy));
            let (mut xmin, mut xmax, mut ymin, mut ymax) = (gx, gx, gy, gy);
            let mut count = 0usize;
            while let Some((x, y)) = queue.pop_front() {
                count += 1;
                if count.is_multiple_of(4096) {
                    checkpoint(cb)?;
                }
                xmin = xmin.min(x);
                xmax = xmax.max(x);
                ymin = ymin.min(y);
                ymax = ymax.max(y);
                for yy in y.saturating_sub(1)..=(y + 1).min(gh - 1) {
                    for xx in x.saturating_sub(1)..=(x + 1).min(gw - 1) {
                        let i = yy * gw + xx;
                        if visited[i] == 0 && m[(yy * step) * w + xx * step] != 0 {
                            visited[i] = 1;
                            queue.push_back((xx, yy));
                        }
                    }
                }
            }
            boxes.push(Box2 {
                x0: (xmin * step) as f32,
                y0: (ymin * step) as f32,
                x1: ((xmax + 1) * step).min(w) as f32,
                y1: ((ymax + 1) * step).min(h) as f32,
            });
            if boxes.len() == 512 {
                return Ok(boxes);
            }
        }
    }
    Ok(boxes)
}
struct Coverage {
    w: usize,
    h: usize,
    a: Vec<f32>,
}
impl Coverage {
    fn new(w: usize, h: usize) -> Result<Self, RenderError> {
        Ok(Self {
            w,
            h,
            a: zeros(w * h)?,
        })
    }
    fn clear(&mut self) {
        self.a.fill(0.);
    }
    fn plot(&mut self, x: i32, y: i32, a: f32) {
        if x >= 0 && y >= 0 && (x as usize) < self.w && (y as usize) < self.h {
            let p = &mut self.a[y as usize * self.w + x as usize];
            *p = p.max(a.clamp(0., 1.));
        }
    }
    fn region(&mut self, b: Box2, mut f: impl FnMut(f32, f32) -> f32) {
        let x0 = (b.x0.floor() as i32).max(0);
        let x1 = (b.x1.ceil() as i32).min(self.w as i32);
        let y0 = (b.y0.floor() as i32).max(0);
        let y1 = (b.y1.ceil() as i32).min(self.h as i32);
        for y in y0..y1 {
            for x in x0..x1 {
                self.plot(x, y, f(x as f32 + 0.5, y as f32 + 0.5));
            }
        }
    }
    fn rect(&mut self, b: Box2) {
        self.region(b, |x, y| {
            ((x - b.x0).min(b.x1 - x).min(y - b.y0).min(b.y1 - y) + 0.5).clamp(0., 1.)
        });
    }
    fn segment(&mut self, a: [f32; 2], b: [f32; 2], r: f32, dash: Option<(f32, f32, f32)>) {
        if r <= 0. {
            return;
        }
        let vx = b[0] - a[0];
        let vy = b[1] - a[1];
        let ll = vx * vx + vy * vy;
        let len = ll.sqrt();
        self.region(
            Box2 {
                x0: a[0].min(b[0]) - r - 1.,
                y0: a[1].min(b[1]) - r - 1.,
                x1: a[0].max(b[0]) + r + 1.,
                y1: a[1].max(b[1]) + r + 1.,
            },
            |x, y| {
                let t = if ll > 0. {
                    ((x - a[0]) * vx + (y - a[1]) * vy) / ll
                } else {
                    0.
                };
                let t = t.clamp(0., 1.);
                if let Some((on, off, start)) = dash
                    && (on <= 0. || (on + off > 0. && (start + t * len) % (on + off) > on))
                {
                    return 0.;
                }
                let dist = ((x - a[0] - t * vx).powi(2) + (y - a[1] - t * vy).powi(2)).sqrt();
                (r + 0.5 - dist).clamp(0., 1.)
            },
        );
    }
    fn triangle(&mut self, a: [f32; 2], b: [f32; 2], c: [f32; 2]) {
        fn edge(a: [f32; 2], b: [f32; 2], p: [f32; 2]) -> f32 {
            (p[0] - a[0]) * (b[1] - a[1]) - (p[1] - a[1]) * (b[0] - a[0])
        }
        let area = edge(a, b, c);
        if area.abs() < 0.001 {
            return;
        }
        self.region(
            Box2 {
                x0: a[0].min(b[0]).min(c[0]) - 1.,
                y0: a[1].min(b[1]).min(c[1]) - 1.,
                x1: a[0].max(b[0]).max(c[0]) + 1.,
                y1: a[1].max(b[1]).max(c[1]) + 1.,
            },
            |x, y| {
                let mut n = 0.;
                for ox in [-0.25, 0.25] {
                    for oy in [-0.25, 0.25] {
                        let p = [x + ox, y + oy];
                        let u = edge(a, b, p) / area;
                        let v = edge(b, c, p) / area;
                        let w = edge(c, a, p) / area;
                        if u >= 0. && v >= 0. && w >= 0. {
                            n += 0.25;
                        }
                    }
                }
                n
            },
        );
    }
    fn composite(&self, out: &mut [Pixel], color: Color, opacity: f32) {
        for (p, a) in out.iter_mut().zip(&self.a) {
            let a = (a * color[3] * opacity).clamp(0., 1.);
            for c in 0..3 {
                p[c] = color[c] * a + p[c] * (1. - a);
            }
            p[3] = a + p[3] * (1. - a);
        }
    }
}
fn point(a: [f32; 2], b: [f32; 2], c: [f32; 2], t: f32) -> [f32; 2] {
    let u = 1. - t;
    [
        u * u * a[0] + 2. * u * t * b[0] + t * t * c[0],
        u * u * a[1] + 2. * u * t * b[1] + t * t * c[1],
    ]
}
fn connection(c: &mut Coverage, a: [f32; 2], b: [f32; 2], s: &Settings) {
    let dx = b[0] - a[0];
    let dy = b[1] - a[1];
    let len = dx.hypot(dy);
    if len < 0.001 {
        return;
    }
    let control = [
        (a[0] + b[0]) * 0.5 - dy * s.curvature,
        (a[1] + b[1]) * 0.5 + dx * s.curvature,
    ];
    let scale = (s.scale[0] + s.scale[1]) * 0.5;
    let samples = ((len * (1. + s.curvature.abs()) / 2.).ceil() as usize).clamp(16, 2048);
    let mut previous = a;
    let mut traveled = 0.;
    for i in 1..=samples {
        let next = point(a, control, b, i as f32 / samples as f32);
        let dash = s
            .dashed
            .then_some((s.dash * scale, s.gap * scale, traveled));
        c.segment(previous, next, s.line_width * scale, dash);
        traveled += (next[0] - previous[0]).hypot(next[1] - previous[1]);
        previous = next;
    }
    if s.arrows && s.arrow_size > 0. {
        let t = s.arrow_position;
        let p = point(a, control, b, t);
        let vx = 2. * ((1. - t) * (control[0] - a[0]) + t * (b[0] - control[0]));
        let vy = 2. * ((1. - t) * (control[1] - a[1]) + t * (b[1] - control[1]));
        let angle = vy.atan2(vx);
        let spread = s.arrow_angle.to_radians();
        let size = s.arrow_size * scale;
        let left = [
            p[0] - size * (angle - spread).cos(),
            p[1] - size * (angle - spread).sin(),
        ];
        let right = [
            p[0] - size * (angle + spread).cos(),
            p[1] - size * (angle + spread).sin(),
        ];
        if s.arrow_filled {
            c.triangle(p, left, right)
        } else {
            c.segment(left, p, s.line_width * scale, None);
            c.segment(p, right, s.line_width * scale, None);
        }
    }
}
// Small original 5x7 numeric font. No external or extracted font atlas.
fn glyph(ch: char) -> [u8; 7] {
    match ch {
        '0' => [14, 17, 19, 21, 25, 17, 14],
        '1' => [4, 12, 4, 4, 4, 4, 14],
        '2' => [14, 17, 1, 2, 4, 8, 31],
        '3' => [30, 1, 1, 14, 1, 1, 30],
        '4' => [2, 6, 10, 18, 31, 2, 2],
        '5' => [31, 16, 16, 30, 1, 1, 30],
        '6' => [14, 16, 16, 30, 17, 17, 14],
        '7' => [31, 1, 2, 4, 8, 8, 8],
        '8' => [14, 17, 17, 14, 17, 17, 14],
        '9' => [14, 17, 17, 15, 1, 1, 14],
        'x' => [0, 0, 17, 10, 4, 10, 17],
        'y' => [0, 0, 17, 17, 15, 1, 14],
        ':' => [0, 4, 4, 0, 4, 4, 0],
        '-' => [0, 0, 0, 14, 0, 0, 0],
        _ => [0; 7],
    }
}
fn label(c: &mut Coverage, b: Box2, s: &Settings) {
    let text = format!(
        "x:{} y:{}",
        (b.x0 / s.scale[0]).round() as i32,
        (b.y0 / s.scale[1]).round() as i32
    );
    let sx = s.text_scale * s.scale[0];
    let sy = s.text_scale * s.scale[1];
    let x = b.x0;
    let y = b.y0 - s.text_offset * s.scale[1] - 7. * sy;
    for (i, ch) in text.chars().enumerate() {
        for (row, bits) in glyph(ch).iter().enumerate() {
            for col in 0..5 {
                if bits & (1 << (4 - col)) != 0 {
                    let x0 = x + (i * 6 + col) as f32 * sx;
                    let y0 = y + row as f32 * sy;
                    c.rect(Box2 {
                        x0,
                        y0,
                        x1: x0 + sx,
                        y1: y0 + sy,
                    });
                }
            }
        }
    }
}
#[derive(Debug)]
pub struct Output {
    pub pixels: Vec<Pixel>,
    pub boxes: Vec<Box2>,
}
pub fn render(
    input: &[Pixel],
    w: usize,
    h: usize,
    s: &Settings,
    mut cb: impl FnMut() -> bool,
) -> Result<Output, RenderError> {
    let m = mask(input, w, h, s, &mut cb)?;
    let boxes = detect(&m, w, h, s.quality, &mut cb)?;
    let mut out = zeros::<Pixel>(input.len())?;
    for (i, p) in out.iter_mut().enumerate() {
        *p = if s.alpha_layer {
            [0.; 4]
        } else if s.show_mask {
            let v = m[i] as f32;
            [v, v, v, 1.]
        } else {
            input[i]
        };
    }
    let mut c = Coverage::new(w, h)?;
    let scale = (s.scale[0] + s.scale[1]) * 0.5;
    if s.boxes {
        for b in &boxes {
            checkpoint(&mut cb)?;
            c.rect(*b);
        }
        c.composite(&mut out, s.fill, s.fill_opacity);
        c.clear();
        for b in &boxes {
            checkpoint(&mut cb)?;
            let r = s.border_width * scale;
            if r <= 0. {
                continue;
            }
            c.region(*b, |x, y| {
                let vertical = (x - b.x0).min(b.x1 - x) < r;
                let horizontal = (y - b.y0).min(b.y1 - y) < r;
                let gapx =
                    s.gaps && (x - (b.x0 + b.x1) * 0.5).abs() < (b.x1 - b.x0) * s.gap_size * 0.5;
                let gapy =
                    s.gaps && (y - (b.y0 + b.y1) * 0.5).abs() < (b.y1 - b.y0) * s.gap_size * 0.5;
                if (horizontal && !gapx) || (vertical && !gapy) {
                    1.
                } else {
                    0.
                }
            });
        }
        c.composite(&mut out, s.border, 1.);
        c.clear();
    }
    if s.connections && boxes.len() > 1 {
        for pair in boxes.windows(2) {
            checkpoint(&mut cb)?;
            connection(&mut c, pair[0].center(), pair[1].center(), s);
        }
        if boxes.len() > 2 {
            connection(
                &mut c,
                boxes[boxes.len() - 1].center(),
                boxes[0].center(),
                s,
            );
        }
        c.composite(&mut out, s.line_color, s.line_opacity);
        c.clear();
    }
    if s.markers {
        for b in &boxes {
            checkpoint(&mut cb)?;
            let p = b.center();
            let size = s.marker_size * scale;
            if size <= 0. {
                continue;
            }
            let r = s.marker_width * scale;
            let half = size * 0.5;
            match s.marker_type {
                0 => c.region(
                    Box2 {
                        x0: p[0] - size - 1.,
                        y0: p[1] - size - 1.,
                        x1: p[0] + size + 1.,
                        y1: p[1] + size + 1.,
                    },
                    |x, y| (size + 0.5 - (x - p[0]).hypot(y - p[1])).clamp(0., 1.),
                ),
                1 => {
                    c.segment([p[0] - half, p[1]], [p[0] + half, p[1]], r, None);
                    c.segment([p[0], p[1] - half], [p[0], p[1] + half], r, None);
                }
                _ => {
                    c.segment(
                        [p[0] - half, p[1] - half],
                        [p[0] + half, p[1] + half],
                        r,
                        None,
                    );
                    c.segment(
                        [p[0] - half, p[1] + half],
                        [p[0] + half, p[1] - half],
                        r,
                        None,
                    );
                }
            }
        }
        c.composite(&mut out, s.marker_color, 1.);
        c.clear();
    }
    if s.text {
        for b in &boxes {
            checkpoint(&mut cb)?;
            label(&mut c, *b, s);
        }
        c.composite(&mut out, s.text_color, 1.);
    }
    checkpoint(&mut cb)?;
    Ok(Output { pixels: out, boxes })
}
