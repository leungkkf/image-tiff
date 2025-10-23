/// CIE XYZ to linear RGB matrix
/// http://www.brucelindbloom.com/index.html?ColorCalcHelp.html
const CIEXYZ_TO_LINEAR_RGB: [f32; 9] = [
    3.2404542, -1.5371385, -0.4985314, //
    -0.969266, 1.8760108, 0.0415560, //
    0.0556434, -0.2040259, 1.0572252,
];

#[derive(Debug)]
pub(crate) struct CieLab {
    l: f32,
    a: f32,
    b: f32,
}

impl CieLab {
    pub(crate) fn new(l: f32, a: f32, b: f32) -> Self {
        Self { l, a, b }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct CieXyz {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct ReferenceWhite {
    x0: f32,
    y0: f32,
    z0: f32,
}

impl ReferenceWhite {
    pub(crate) fn new(x0: f32, y0: f32, z0: f32) -> Self {
        Self { x0, y0, z0 }
    }
}

impl CieXyz {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug)]
pub(crate) struct ColorConverter {
    ref_white: ReferenceWhite,
}

impl ColorConverter {
    /// Create a colour converter.
    pub(crate) fn build(ref_white: &ReferenceWhite) -> Self {
        Self {
            ref_white: *ref_white,
        }
    }

    pub(crate) fn cielab_to_rgb(&self, cielab: CieLab) -> (u8, u8, u8) {
        self.ciexyz_to_rgb(self.cielab_to_xyz(cielab))
    }

    fn inverse_f(t: f32) -> f32 {
        let delta = 6.0 / 29.0;

        if t > delta {
            t * t * t
        } else {
            3.0 * delta * delta * (t - 4.0 / 29.0)
        }
    }

    /// Convert CIE L*a*b* to CIE XYZ.
    /// https://en.wikipedia.org/wiki/CIELAB_color_space
    fn cielab_to_xyz(&self, cielab: CieLab) -> CieXyz {
        let t = (cielab.l + 16.0) / 116.0 + cielab.a / 500.0;
        let x = self.ref_white.x0 * ColorConverter::inverse_f(t);

        let t = (cielab.l + 16.0) / 116.0;
        let y = self.ref_white.y0 * ColorConverter::inverse_f(t);

        let t = (cielab.l + 16.0) / 116.0 - cielab.b / 200.0;
        let z = self.ref_white.z0 * ColorConverter::inverse_f(t);

        CieXyz::new(x / 100.0, y / 100.0, z / 100.0)
    }

    fn gamma_correction(u: f32) -> f32 {
        255.0
            * if u < 0.0031308 {
                12.92 * u
            } else {
                1.055 * u.powf(1.0 / 2.4) - 0.055
            }
    }

    /// Convert CIE XYZ to sRGB.
    /// http://www.brucelindbloom.com/index.html?ColorCalcHelp.html
    fn ciexyz_to_rgb(&self, ciexyz: CieXyz) -> (u8, u8, u8) {
        // CIE XYZ to  linear RGB.
        let yr = CIEXYZ_TO_LINEAR_RGB[0] * ciexyz.x
            + CIEXYZ_TO_LINEAR_RGB[1] * ciexyz.y
            + CIEXYZ_TO_LINEAR_RGB[2] * ciexyz.z;
        let yg = CIEXYZ_TO_LINEAR_RGB[3] * ciexyz.x
            + CIEXYZ_TO_LINEAR_RGB[4] * ciexyz.y
            + CIEXYZ_TO_LINEAR_RGB[5] * ciexyz.z;
        let yb = CIEXYZ_TO_LINEAR_RGB[6] * ciexyz.x
            + CIEXYZ_TO_LINEAR_RGB[7] * ciexyz.y
            + CIEXYZ_TO_LINEAR_RGB[8] * ciexyz.z;

        // sRGB Companding - apply gamma correction.
        let r = ColorConverter::gamma_correction(yr)
            .round()
            .clamp(0.0, 255.0) as u8;
        let g = ColorConverter::gamma_correction(yg)
            .round()
            .clamp(0.0, 255.0) as u8;
        let b = ColorConverter::gamma_correction(yb)
            .round()
            .clamp(0.0, 255.0) as u8;

        (r, g, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_cielab_to_xyz() {
        let ref_white = ReferenceWhite::new(95.0489, 100.0, 108.8840);

        println!("{:?}", ref_white);

        let converter = ColorConverter::build(&ref_white);

        // Numbers from the calculator in http://www.brucelindbloom.com/
        let xyz = converter.cielab_to_xyz(CieLab {
            l: 50.0,
            a: 10.0,
            b: 5.0,
        });
        println!("{:?}", xyz);
        assert_relative_eq!(xyz.x, 0.194182, epsilon = 1.0e-5);
        assert_relative_eq!(xyz.y, 0.184187, epsilon = 1.0e-5);
        assert_relative_eq!(xyz.z, 0.175257, epsilon = 1.0e-5);

        let rgb = converter.ciexyz_to_rgb(xyz);
        println!("{:?}", rgb);
        assert_eq!(rgb, (139, 113, 111));
    }
}
