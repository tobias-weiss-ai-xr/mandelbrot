//! Mandelbrot-set variants rendered into a fixed RGBA buffer for the browser.
//!
//! Exports (plain C ABI, no wasm-bindgen):
//! - `buf_ptr() -> *const u32`  — start of the W*H RGBA pixel buffer
//! - `render(variant, cx, cy, height, max_iter)` — fill the buffer for a view

const W: usize = 960;
const H: usize = 640;
const JULIA_C: (f64, f64) = (-0.7, 0.27015);

/// Variants: 0 Mandelbrot z²+c · 1 Julia z²+c, c fixed · 2 Multibrot-3 z³+c
///           3 Burning Ship (|Re|,|Im|)²+c · 4 Tricorn conj(z)²+c

static mut BUF: [u32; W * H] = [0; W * H];

#[derive(Clone, Copy, PartialEq)]
struct Cx {
    re: f64,
    im: f64,
}

impl Cx {
    fn new(re: f64, im: f64) -> Self {
        Cx { re, im }
    }
    fn abs2(self) -> f64 {
        self.re * self.re + self.im * self.im
    }
    fn sqr(self) -> Self {
        self * self
    }
    fn conj(self) -> Self {
        Cx::new(self.re, -self.im)
    }
}

impl std::ops::Mul for Cx {
    type Output = Self;
    fn mul(self, o: Self) -> Self {
        Cx::new(self.re * o.re - self.im * o.im, self.re * o.im + self.im * o.re)
    }
}

impl std::ops::Add for Cx {
    type Output = Self;
    fn add(self, o: Self) -> Self {
        Cx::new(self.re + o.re, self.im + o.im)
    }
}

fn step(variant: u32, z: Cx, c: Cx) -> Cx {
    match variant {
        2 => z.sqr() * z + c,
        3 => Cx::new(z.re.abs(), z.im.abs()).sqr() + c,
        4 => z.conj().sqr() + c,
        _ => z.sqr() + c, // Mandelbrot and Julia share the recurrence
    }
}

/// Iterations until escape plus the escaped z (for smooth coloring),
/// or `(max_iter, z)` if the point is in the set.
fn escape(variant: u32, x: f64, y: f64, max_iter: u32) -> (u32, Cx) {
    let (mut z, c) = match variant {
        1 => (Cx::new(x, y), Cx::new(JULIA_C.0, JULIA_C.1)),
        _ => (Cx::new(0.0, 0.0), Cx::new(x, y)),
    };
    let mut n = 0;
    while n < max_iter && z.abs2() <= 4.0 {
        z = step(variant, z, c);
        n += 1;
    }
    (n, z)
}

/// Cosine palette driven by the smoothed iteration count mu = n + 1 − log2(ln|z|²/2);
/// interior is black.
fn pixel_color(variant: u32, x: f64, y: f64, max_iter: u32) -> u32 {
    let (n, z) = escape(variant, x, y, max_iter);
    if n >= max_iter {
        return 0xFF00_0000;
    }
    let mu = n as f64 + 1.0 - (z.abs2().ln() / 2.0).ln() / std::f64::consts::LN_2;
    color(mu)
}

fn color(mu: f64) -> u32 {
    let t = 0.05 * mu;
    let pal = |o: f64| ((0.5 + 0.5 * (std::f64::consts::TAU * (t + o)).cos()) * 255.0) as u32;
    let (r, g, b) = (pal(0.0), pal(0.33), pal(0.67));
    0xFF00_0000 | (b << 16) | (g << 8) | r
}

/// Fill the static buffer: canvas height maps to `height` complex units,
/// x/y use the same units per pixel (square aspect).
#[no_mangle]
pub extern "C" fn render(variant: u32, cx: f64, cy: f64, height: f64, max_iter: u32) {
    let s = height / H as f64;
    let buf = unsafe { &mut *std::ptr::addr_of_mut!(BUF) };
    for py in 0..H {
        let y = cy + (py as f64 - H as f64 / 2.0) * s;
        for px in 0..W {
            let x = cx + (px as f64 - W as f64 / 2.0) * s;
            buf[py * W + px] = pixel_color(variant, x, y, max_iter);
        }
    }
}

#[no_mangle]
pub extern "C" fn buf_ptr() -> *const u32 {
    std::ptr::addr_of!(BUF) as *const u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mandelbrot_interior_is_black() {
        assert_eq!(pixel_color(0, 0.0, 0.0, 256), 0xFF00_0000);
    }

    #[test]
    fn exterior_color_tracks_escape_count() {
        // Real-axis points beyond the c > 1/4 boundary: escapes at n = 2 and n = 3.
        let a = (0, 1.5, 0.0);
        let b = (0, 1.0, 0.0);
        let (na, nb) = (escape(a.0, a.1, a.2, 256).0, escape(b.0, b.1, b.2, 256).0);
        assert_ne!(na, nb);
        assert_ne!(na, 256);
        assert_ne!(pixel_color(a.0, a.1, a.2, 256), 0xFF00_0000);
        assert_ne!(pixel_color(b.0, b.1, b.2, 256), 0xFF00_0000);
        assert_ne!(pixel_color(a.0, a.1, a.2, 256), pixel_color(b.0, b.1, b.2, 256));
    }

    #[test]
    fn higher_iteration_limit_never_shrinks_interior() {
        let count_black = |iters| {
            (0..40)
                .flat_map(|i| (0..30).map(move |j| (i, j)))
                .filter(|(i, j)| {
                    let x = -2.2 + *i as f64 * 3.0 / 39.0;
                    let y = -1.2 + *j as f64 * 2.4 / 29.0;
                    pixel_color(0, x, y, iters) == 0xFF00_0000
                })
                .count()
        };
        // Points that escape between the two limits are interior at the low
        // limit but exterior at the high one → interior only ever shrinks.
        assert!(count_black(32) >= count_black(256));
    }

    /// Per-variant default viewports (mirrored in web/index.html).
    fn default_view(variant: u32) -> (f64, f64, f64) {
        match variant {
            1 | 2 => (0.0, 0.0, 3.4),
            3 => (-1.75, -0.03, 0.15),
            _ => (-0.6, 0.0, 2.7),
        }
    }

    #[test]
    fn every_variant_shows_set_and_exterior() {
        for v in 0..5u32 {
            let (cx, cy, h) = default_view(v);
            render(v, cx, cy, h, 256);
            let buf = unsafe { &*std::ptr::addr_of!(BUF) };
            let mut black = 0;
            let mut colors = std::collections::HashSet::new();
            for (i, p) in buf.iter().enumerate() {
                assert_eq!(p >> 24, 0xFF, "variant {v}: pixel {i} not opaque");
                if *p == 0xFF00_0000 {
                    black += 1;
                } else {
                    colors.insert(*p & 0x00FF_FFFF);
                }
            }
            assert!(black > 100, "variant {v}: no interior found");
            assert!(colors.len() > 100, "variant {v}: exterior not colored");
        }
    }
}
