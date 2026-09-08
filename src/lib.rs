//! Mandelbrot-set variants rendered into a fixed RGBA buffer for the browser.
//!
//! Exports (plain C ABI, no wasm-bindgen):
//! - `buf_ptr() -> *const u32`  — start of the W*H RGBA pixel buffer
//! - `render(variant, cx, cy, height, max_iter, scale)` — fill the buffer for a view
//!
//! scale > 1 renders a W/scale × H/scale preview into the first buffer entries
//! (used for low-latency interaction frames).

const W: usize = 960;
const H: usize = 640;
const JULIA_C: (f64, f64) = (-0.7, 0.27015);

/// Variants: 0 Mandelbrot z²+c · 1 Julia z²+c, c fixed · 2 Multibrot-3 z³+c
///           3 Burning Ship (|Re|,|Im|)²+c · 4 Tricorn conj(z)²+c

static mut BUF: [u32; W * H] = [0; W * H];
/// Per-pixel escape step counts of the last render (same layout as BUF).
static mut ITERS: [u32; W * H] = [0; W * H];

/// Solid-fractal DE field (f32, negative inside), layout z·g² + y·g + x.
const FIELD_MAX: usize = 96;
static mut FIELD: [f32; FIELD_MAX * FIELD_MAX * FIELD_MAX] = [0.0; FIELD_MAX * FIELD_MAX * FIELD_MAX];

#[derive(Clone, Copy)]
struct Cx {
    re: f64,
    im: f64,
}

impl Cx {
    fn abs2(self) -> f64 {
        self.re * self.re + self.im * self.im
    }
}

impl std::ops::Mul for Cx {
    type Output = Self;
    fn mul(self, o: Self) -> Self {
        Cx { re: self.re * o.re - self.im * o.im, im: self.re * o.im + self.im * o.re }
    }
}

impl std::ops::Add for Cx {
    type Output = Self;
    fn add(self, o: Self) -> Self {
        Cx { re: self.re + o.re, im: self.im + o.im }
    }
}

/// Cosine palette driven by the smoothed iteration count mu = n + 1 − log2(ln|z|²/2).
fn color(mu: f64) -> u32 {
    let t = 0.05 * mu;
    let pal = |o: f64| ((0.5 + 0.5 * (std::f64::consts::TAU * (t + o)).cos()) * 255.0) as u32;
    let (r, g, b) = (pal(0.0), pal(0.33), pal(0.67));
    0xFF00_0000 | (b << 16) | (g << 8) | r
}

// Per-variant seed (z0, c) and recurrence, monomorphized into the loop below.
fn seed_zc(x: f64, y: f64) -> (Cx, Cx) {
    (Cx { re: 0.0, im: 0.0 }, Cx { re: x, im: y })
}
fn seed_julia(x: f64, y: f64) -> (Cx, Cx) {
    (Cx { re: x, im: y }, Cx { re: JULIA_C.0, im: JULIA_C.1 })
}
fn st_mandelbrot(z: Cx, c: Cx) -> Cx {
    z * z + c
}
fn st_multibrot3(z: Cx, c: Cx) -> Cx {
    z * z * z + c
}
fn st_burning_ship(z: Cx, c: Cx) -> Cx {
    let a = Cx { re: z.re.abs(), im: z.im.abs() };
    a * a + c
}
fn st_tricorn(z: Cx, c: Cx) -> Cx {
    let b = Cx { re: z.re, im: -z.im };
    b * b + c
}

#[no_mangle]
pub extern "C" fn render(variant: u32, cx: f64, cy: f64, height: f64, max_iter: u32, scale: u32) {
    let scale = (scale.max(1)) as usize;
    let w = W / scale;
    let h = H / scale;
    let s = height / H as f64; // complex units per full-res pixel
    let sx = scale as f64;
    let buf = unsafe { &mut *std::ptr::addr_of_mut!(BUF) };
    let iters = unsafe { &mut *std::ptr::addr_of_mut!(ITERS) };

    // Variant dispatch hoisted here: each arm monomorphizes the pixel loop
    // with its seed and recurrence inlined.
    macro_rules! loop_pixels {
        ($seed:expr, $step:expr) => {
            for py in 0..h {
                let y = cy + (py as f64 * sx - H as f64 / 2.0) * s;
                for px in 0..w {
                    let x = cx + (px as f64 * sx - W as f64 / 2.0) * s;
                    let (mut z, c) = $seed(x, y);
                    let mut n = 0u32;
                    while n < max_iter && z.abs2() <= 4.0 {
                        z = $step(z, c);
                        n += 1;
                    }
                    buf[py * w + px] = if n >= max_iter {
                        0xFF00_0000
                    } else {
                        color(n as f64 + 1.0 - (z.abs2() / 2.0).ln().ln() / std::f64::consts::LN_2)
                    };
                    iters[py * w + px] = n;
                }
            }
        };
    }
    match variant {
        1 => loop_pixels!(seed_julia, st_mandelbrot),
        2 => loop_pixels!(seed_zc, st_multibrot3),
        3 => loop_pixels!(seed_zc, st_burning_ship),
        4 => loop_pixels!(seed_zc, st_tricorn),
        _ => loop_pixels!(seed_zc, st_mandelbrot),
    }
}

#[no_mangle]
pub extern "C" fn buf_ptr() -> *const u32 {
    std::ptr::addr_of!(BUF) as *const u32
}

#[no_mangle]
pub extern "C" fn iters_ptr() -> *const u32 {
    std::ptr::addr_of!(ITERS) as *const u32
}

// ---- Solid 3D fractals: signed-distance fields ----

/// Variants: 0 bulb p8 · 1 bulb p2 · 2 bulb p3 · 3 bulb p4 · 4 bulb p6 · 5 Mandelbox s3
fn bulb_de(x: f64, y: f64, z: f64, power: u32, iters: u32) -> f32 {
    let n = power as f64;
    let (mut zx, mut zy, mut zz): (f64, f64, f64) = (0.0, 0.0, 0.0);
    let mut dr = 1.0;
    let mut r = 0.0;
    let mut escaped = false;
    for _ in 0..iters {
        r = (zx * zx + zy * zy + zz * zz).sqrt();
        if r > 2.0 {
            escaped = true;
            break;
        }
        if r < 1e-12 {
            // zⁿ = 0 → z ← c; dr recursion degenerates to 1
            dr = 1.0;
            zx = x;
            zy = y;
            zz = z;
            continue;
        }
        let theta = (zz / r).acos() * n;
        // φⁿ via complex recurrence instead of sin/cos of the scaled angle
        let (cf, sf) = (zy.atan2(zx).cos(), zy.atan2(zx).sin());
        let (mut c, mut s) = (1.0, 0.0);
        for _ in 0..power {
            let t = c * cf - s * sf;
            s = s * cf + c * sf;
            c = t;
        }
        let r2 = r * r;
        let zr = match power {
            2 => r2,
            3 => r2 * r,
            4 => r2 * r2,
            6 => r2 * r2 * r2,
            _ => r2 * r2 * r2 * r2,
        };
        let (st, ct) = (theta.sin(), theta.cos());
        zx = zr * st * c + x;
        zy = zr * st * s + y;
        zz = zr * ct + z;
        dr = zr / r * n * dr + 1.0;
    }
    if escaped {
        (0.5 * r.ln() * r / dr) as f32
    } else {
        -0.05
    }
}

fn box_de(x: f64, y: f64, z: f64, scale: f64, iters: u32) -> f32 {
    let (mut zx, mut zy, mut zz) = (x, y, z);
    let mut dr = 1.0;
    let mut escaped = false;
    for _ in 0..iters {
        // box fold
        zx = zx.clamp(-1.0, 1.0) * 2.0 - zx;
        zy = zy.clamp(-1.0, 1.0) * 2.0 - zy;
        zz = zz.clamp(-1.0, 1.0) * 2.0 - zz;
        // sphere fold (fixedR² = 1, minR² = 0.25)
        let r2 = zx * zx + zy * zy + zz * zz;
        let m = if r2 < 0.25 {
            4.0
        } else if r2 < 1.0 {
            1.0 / r2
        } else {
            1.0
        };
        zx *= m;
        zy *= m;
        zz *= m;
        dr *= m;
        zx = zx * scale + x;
        zy = zy * scale + y;
        zz = zz * scale + z;
        dr *= scale;
        if zx * zx + zy * zy + zz * zz > 100.0 {
            escaped = true;
            break;
        }
    }
    if escaped {
        ((zx * zx + zy * zy + zz * zz).sqrt() / dr) as f32
    } else {
        -0.05
    }
}

/// Fill FIELD[z·g² + y·g + x] for z-slabs [z0, z1); call repeatedly with progress.
#[no_mangle]
pub extern "C" fn bulb_field(variant: u32, grid: u32, iters: u32, z0: u32, z1: u32) {
    let g = grid.clamp(8, FIELD_MAX as u32) as usize;
    let g2 = g * g;
    let (span, power) = match variant {
        5 => (1.8, 0),
        1 => (1.2, 2),
        2 => (1.2, 3),
        3 => (1.2, 4),
        4 => (1.2, 6),
        _ => (1.2, 8),
    };
    let field = unsafe { &mut *std::ptr::addr_of_mut!(FIELD) };
    let step = (2.0 * span) / (g as f64 - 1.0);
    let z1 = (z1 as usize).min(g);
    for z in (z0 as usize)..z1 {
        let wz = z as f64 * step - span;
        for y in 0..g {
            let wy = y as f64 * step - span;
            for x in 0..g {
                let wx = x as f64 * step - span;
                let de = if variant == 5 {
                    box_de(wx, wy, wz, 3.0, iters)
                } else {
                    bulb_de(wx, wy, wz, power, iters)
                };
                field[z * g2 + y * g + x] = de;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn field_ptr() -> *const f32 {
    std::ptr::addr_of!(FIELD) as *const f32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    /// Tests share the single static buffer — render+assert must be serialized.
    static BUF_LOCK: Mutex<()> = Mutex::new(());
    static FIELD_LOCK: Mutex<()> = Mutex::new(());

    fn buf() -> &'static [u32] {
        unsafe { &*std::ptr::addr_of!(BUF) }
    }

    fn default_view(variant: u32) -> (f64, f64, f64) {
        match variant {
            1 | 2 => (0.0, 0.0, 3.4),
            3 => (-1.75, -0.03, 0.15),
            _ => (-0.6, 0.0, 2.7),
        }
    }

    #[test]
    fn interior_view_is_all_black() {
        let _g = BUF_LOCK.lock().unwrap();
        // A 0.02-unit view around the origin sits fully inside the main cardoid.
        render(0, 0.0, 0.0, 0.02, 64, 1);
        assert!(buf().iter().all(|&p| p == 0xFF00_0000));
    }

    #[test]
    fn fully_exterior_view_is_colored_and_varied() {
        let _g = BUF_LOCK.lock().unwrap();
        // The quadrant around (0.9, 0.9) is entirely outside the set.
        render(0, 0.9, 0.9, 0.5, 256, 1);
        let mut colors = std::collections::HashSet::new();
        for &p in buf() {
            assert_ne!(p, 0xFF00_0000, "unexpected interior pixel");
            assert_eq!(p >> 24, 0xFF);
            colors.insert(p & 0x00FF_FFFF);
        }
        assert!(colors.len() > 10, "colors should vary with escape count");
    }

    #[test]
    fn higher_iteration_limit_never_grows_interior() {
        let _g = BUF_LOCK.lock().unwrap();
        render(0, -0.6, 0.0, 2.7, 32, 1);
        let black32 = buf().iter().filter(|&&p| p == 0xFF00_0000).count();
        render(0, -0.6, 0.0, 2.7, 256, 1);
        let black256 = buf().iter().filter(|&&p| p == 0xFF00_0000).count();
        assert!(black32 >= black256);
    }

    #[test]
    fn every_variant_shows_set_and_exterior() {
        let _g = BUF_LOCK.lock().unwrap();
        for v in 0..5u32 {
            let (cx, cy, h) = default_view(v);
            render(v, cx, cy, h, 256, 1);
            let mut black = 0;
            let mut colors = std::collections::HashSet::new();
            for &p in buf() {
                assert_eq!(p >> 24, 0xFF, "variant {v}: not opaque");
                if p == 0xFF00_0000 {
                    black += 1;
                } else {
                    colors.insert(p & 0x00FF_FFFF);
                }
            }
            assert!(black > 100, "variant {v}: no interior found");
            assert!(colors.len() > 50, "variant {v}: exterior not colored");
        }
    }

    #[test]
    fn scale2_is_compact_opaque_and_matches_full_res() {
        let _g = BUF_LOCK.lock().unwrap();
        render(0, -0.6, 0.0, 2.7, 256, 2);
        let (w, h) = (W / 2, H / 2);
        assert!(buf()[..w * h].iter().all(|&p| p >> 24 == 0xFF));
        // Same complex-plane point ⇒ identical pixel value.
        let samples: Vec<(usize, u32)> = (0..h).step_by(37)
            .flat_map(|j| (0..w).step_by(41).map(move |i| (j * w + i, buf()[j * w + i])))
            .collect();
        render(0, -0.6, 0.0, 2.7, 256, 1);
        for &(idx, expected) in &samples {
            let j = idx / w;
            let i = idx % w;
            assert_eq!(buf()[j * 2 * W + i * 2], expected);
        }
    }

    #[test]
    fn bulb_field_separates_inside_outside_and_slabs_compose() {
        let _g = FIELD_LOCK.lock().unwrap();
        let field = || unsafe { &*std::ptr::addr_of!(FIELD) };
        // Two half-slabs
        bulb_field(0, 32, 12, 0, 16);
        bulb_field(0, 32, 12, 16, 32);
        let two_part: Vec<f32> = field()[..32 * 32 * 32].to_vec();
        // Whole grid in one call must equal the slab composition
        bulb_field(0, 32, 12, 0, 32);
        assert_eq!(&field()[..32 * 32 * 32], &two_part[..]);
        // Origin is interior (c = 0 never escapes), corner is far outside
        let center = field()[16 * 32 * 32 + 16 * 32 + 16];
        let corner = field()[0];
        assert!(center < 0.0, "origin voxel should be inside: {center}");
        assert!(corner > 0.0, "corner voxel should be outside: {corner}");
    }

    #[test]
    fn mandelbox_field_sane() {
        let _g = FIELD_LOCK.lock().unwrap();
        bulb_field(5, 32, 12, 0, 32);
        let field = unsafe { &*std::ptr::addr_of!(FIELD) };
        let inside = field[..32 * 32 * 32].iter().filter(|&&v| v < 0.0).count();
        let outside = field[..32 * 32 * 32].iter().filter(|&&v| v > 0.0).count();
        assert!(inside > 100, "mandelbox: too few interior voxels: {inside}");
        assert!(outside > 1000, "mandelbox: too few exterior voxels");
        assert!(field[..32 * 32 * 32].iter().all(|v| v.is_finite()));
    }

    #[test]
    fn iteration_buffer_matches_classification() {
        let _g = BUF_LOCK.lock().unwrap();
        let iters = || unsafe { &*std::ptr::addr_of!(ITERS) };
        render(0, -0.6, 0.0, 2.7, 256, 2);
        let (w, h) = (W / 2, H / 2);
        // All counts within limit; count agrees with color classification.
        for k in 0..w * h {
            let n = iters()[k];
            assert!(n <= 256);
            assert_eq!(n == 256, buf()[k] == 0xFF00_0000);
        }
        // Scale-1 counts agree with scale-2 counts at shared pixels.
        let samples: Vec<(usize, u32)> = (0..h).step_by(53)
            .flat_map(|j| (0..w).step_by(59).map(move |i| (j * w + i, iters()[j * w + i])))
            .collect();
        render(0, -0.6, 0.0, 2.7, 256, 1);
        for &(idx, expected) in &samples {
            let j = idx / w;
            let i = idx % w;
            assert_eq!(iters()[j * 2 * W + i * 2], expected);
        }
    }
}
