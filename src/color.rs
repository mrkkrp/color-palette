use image::Primitive;
use image::Rgb;
use std::primitive::f64;

/// Color represented using the red-yellow-blue subtractive color model.
pub struct Ryb<T: Primitive>(pub [T; 3]);

pub const BLACK: Ryb<f64> = Ryb([1.0, 1.0, 1.0]);
pub const BLUE: Ryb<f64> = Ryb([0.0, 0.0, 1.0]);
pub const CYAN: Ryb<f64> = Ryb([0.0, 0.5, 1.0]);
pub const GREEN: Ryb<f64> = Ryb([0.0, 1.0, 1.0]);
pub const PURPLE: Ryb<f64> = Ryb([1.0, 0.0, 0.5]);
pub const RED: Ryb<f64> = Ryb([1.0, 0.0, 0.0]);
pub const WHITE: Ryb<f64> = Ryb([0.0, 0.0, 0.0]);
pub const YELLOW: Ryb<f64> = Ryb([0.0, 1.0, 0.0]);

/// Trait for components of colors. We demand that any `Component` can be
/// converted to and from `f64`.
pub trait Component: Primitive {
    /// Convert a value to `f64` in the range from 0 to 1 inclusive.
    fn to_f64(self) -> f64;
    /// Convert a `f64` value in the range from 0 to 1 to the given type.
    fn from_f64(_: f64) -> Self;
}

impl Component for f32 {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn from_f64(x: f64) -> f32 {
        x as f32
    }
}

macro_rules! derive_scaling_component {
    ($type: ty) => {
        impl Component for $type {
            fn to_f64(self) -> f64 {
                (self as f64) / (<$type>::MAX as f64)
            }
            fn from_f64(x: f64) -> $type {
                (x * (<$type>::MAX as f64)) as $type
            }
        }
    };
}

derive_scaling_component!(usize);
derive_scaling_component!(u8);
derive_scaling_component!(u16);
derive_scaling_component!(u32);
derive_scaling_component!(u64);

impl<T: Component> Ryb<T> {
    /// Create a new `Ryb` from an array of red, yellow, and blue components.
    pub fn new(v: [T; 3]) -> Ryb<T> {
        Ryb(v)
    }

    // /// A version of `new` for the cases when the source array is known to
    // /// contain `f64`.
    // pub fn new_f64([r, y, b]: [f64; 3]) -> Ryb<T> {
    //     Ryb([Component::from_f64(r), Component::from_f64(y), Component::from_f64(b)])
    // }

    // /// Convert the components to `f64`.
    // pub fn ryb_f64(&self) -> Ryb<f64> {
    //     let Ryb([r, y, b]) = *self;
    //     Ryb([r.to_f64(), y.to_f64(), b.to_f64()])
    // }

    /// Create a new `Ryb` from an `Rgb` value.
    pub fn new_rgb(Rgb([r1_rgb, g1_rgb, b1_rgb]): Rgb<T>) -> Ryb<T> {
        let r0_rgb = r1_rgb.to_f64();
        let g0_rgb = g1_rgb.to_f64();
        let b0_rgb = b1_rgb.to_f64();

        let i_w = f64::min(f64::min(r0_rgb, g0_rgb), b0_rgb);

        let r_rgb = r0_rgb - i_w;
        let g_rgb = g0_rgb - i_w;
        let b_rgb = b0_rgb - i_w;

        let r_ryb = r_rgb - f64::min(r_rgb, g_rgb);
        let y_ryb = (g_rgb + f64::min(r_rgb, g_rgb)) / 2.0;
        let b_ryb = (b_rgb + g_rgb - f64::min(r_rgb, g_rgb)) / 2.0;

        let n = f64::max(f64::max(r_ryb, y_ryb), b_ryb) / f64::max(f64::max(r_rgb, g_rgb), b_rgb);

        let rp_ryb = r_ryb / n;
        let yp_ryb = y_ryb / n;
        let bp_ryb = b_ryb / n;

        let i_b = f64::min(f64::min(1.0 - r0_rgb, 1.0 - g0_rgb), 1.0 - b0_rgb);

        let r0_ryb = rp_ryb + i_b;
        let y0_ryb = yp_ryb + i_b;
        let b0_ryb = bp_ryb + i_b;

        Ryb([
            Component::from_f64(r0_ryb),
            Component::from_f64(y0_ryb),
            Component::from_f64(b0_ryb),
        ])
    }

    /// Convert to the RGB representation.
    pub fn rgb(&self) -> Rgb<T> {
        let Ryb([r1_ryb, y1_ryb, b1_ryb]) = *self;

        let r0_ryb = r1_ryb.to_f64();
        let y0_ryb = y1_ryb.to_f64();
        let b0_ryb = b1_ryb.to_f64();

        let i_w = f64::min(f64::min(r0_ryb, y0_ryb), b0_ryb);

        let r_ryb = r0_ryb - i_w;
        let y_ryb = y0_ryb - i_w;
        let b_ryb = b0_ryb - i_w;

        let r_rgb = r_ryb + y_ryb - f64::min(y_ryb, b_ryb);
        let g_rgb = y_ryb + 2.0 * f64::min(y_ryb, b_ryb);
        let b_rgb = 2.0 * (b_ryb - f64::min(y_ryb, b_ryb));

        let n = f64::max(f64::max(r_ryb, y_ryb), b_ryb) / f64::max(f64::max(r_rgb, g_rgb), b_rgb);

        let rp_rgb = r_rgb / n;
        let gp_rgb = g_rgb / n;
        let bp_rgb = b_rgb / n;

        let i_b = f64::min(f64::min(1.0 - r0_ryb, 1.0 - y0_ryb), 1.0 - b_ryb);

        let r0_ryb = rp_rgb + i_b;
        let g0_ryb = gp_rgb + i_b;
        let b0_ryb = bp_rgb + i_b;

        Rgb([
            Component::from_f64(r0_ryb),
            Component::from_f64(g0_ryb),
            Component::from_f64(b0_ryb),
        ])
    }
}

/// Mix a collection of weighted colors.
pub fn mix<T: Component>(_colors: Vec<(T, Ryb<T>)>) -> Ryb<T> {
    todo!()
}
