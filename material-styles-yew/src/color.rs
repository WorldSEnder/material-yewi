use std::convert::TryFrom;

/// CSS-3 <color> spec, only including rgb(a) and hsl(a)
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CssColor {
    #[non_exhaustive] // internally assumed to be normalized
    RGB { r: f32, g: f32, b: f32, a: f32 },
    #[non_exhaustive] // internally assumed to be normalized
    HSL { h: f32, s: f32, l: f32, a: f32 },
}

impl CssColor {
    /// Convert an rgb color spec, given in components in the range [0-255]
    ///
    /// ```
    /// # use material_styles_yew::CssColor;
    /// let c = CssColor::rgb(192, 192, 192);
    /// assert_eq!(c.to_css_value(), "rgb(75.294%, 75.294%, 75.294%)")
    /// ```
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba_f(
            (r as f32) / 255f32,
            (g as f32) / 255f32,
            (b as f32) / 255f32,
            1f32,
        )
    }

    /// Convert an rgb color spec, given in components in the range [0-255] plus an additional alpha value
    /// in the usual range [0, 1]
    ///
    /// ```
    /// # use material_styles_yew::CssColor;
    /// let c = CssColor::rgba(192, 192, 192, 0.5);
    /// assert_eq!(c.to_css_value(), "rgba(75.294%, 75.294%, 75.294%, 0.50000)")
    /// ```
    pub fn rgba(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self::rgba_f(
            (r as f32) / 255f32,
            (g as f32) / 255f32,
            (b as f32) / 255f32,
            a,
        )
    }

    /// Convert an rgb color spec, given in components in the range [0, 1] plus an additional alpha value
    /// in the usual range [0, 1]
    ///
    /// ```
    /// # use material_styles_yew::CssColor;
    /// let c = CssColor::rgba_f(0.3, 0.2, 0.1, 0.5);
    /// assert_eq!(c.to_css_value(), "rgba(30.000%, 20.000%, 10.000%, 0.50000)")
    /// ```
    pub fn rgba_f(r: f32, g: f32, b: f32, a: f32) -> Self {
        let normalize_color = |c: f32| {
            if c.is_nan() {
                0f32
            } else {
                c.clamp(0f32, 1f32)
            }
        };
        let normalize_alpha = |a: f32| {
            if a.is_nan() {
                1f32
            } else {
                a.clamp(0f32, 1f32)
            }
        };
        Self::RGB {
            r: normalize_color(r),
            g: normalize_color(g),
            b: normalize_color(b),
            a: normalize_alpha(a),
        }
    }

    /// Convert a hsl color spec. The hue is given in degrees and normalized between [0, 360].
    /// The saturation and lightness are clamped to the range [0, 1].
    ///
    /// ```
    /// # use material_styles_yew::CssColor;
    /// let c = CssColor::hsl(120.0, 1.0, 0.8);
    /// assert_eq!(c.to_css_value(), "hsl(120.00, 100.000%, 80.000%)")
    /// ```
    pub fn hsl(hue: f32, saturation: f32, lightness: f32) -> Self {
        Self::hsla(hue, saturation, lightness, 1f32)
    }

    /// Convert a hsl color spec. The hue is given in degrees and normalized between [0, 360].
    /// The given saturation, lightness and alpha values are clamped to the range [0, 1].
    ///
    /// ```
    /// # use material_styles_yew::CssColor;
    /// let c = CssColor::hsla(120.0, 1.0, 0.8, 0.5);
    /// assert_eq!(c.to_css_value(), "hsla(120.00, 100.000%, 80.000%, 0.50000)")
    /// ```
    pub fn hsla(hue: f32, saturation: f32, lightness: f32, alpha: f32) -> Self {
        let normalize_hue = |h: f32| {
            if h.is_nan() {
                0f32
            } else {
                // hue is given in angles
                h.rem_euclid(360f32)
            }
        };
        let normalize_sla = |a: f32| {
            if a.is_nan() {
                1f32
            } else {
                a.clamp(0f32, 1f32)
            }
        };
        Self::HSL {
            h: normalize_hue(hue),
            s: normalize_sla(saturation),
            l: normalize_sla(lightness),
            a: normalize_sla(alpha),
        }
    }

    pub fn alpha_multiply(self, alpha: f32) -> Self {
        let coeff = if alpha.is_nan() { 1f32 } else { alpha };
        match self {
            CssColor::RGB { r, g, b, a } => CssColor::RGB {
                r,
                g,
                b,
                a: (a * coeff).clamp(0f32, 1f32),
            },
            CssColor::HSL { h, s, l, a } => CssColor::HSL {
                h,
                s,
                l,
                a: (a * coeff).clamp(0f32, 1f32),
            },
        }
    }

    pub fn to_css_value(&self) -> String {
        let omit_alpha = |a: f32| a >= 0.99999f32; // alpha clamped to [0, 1] anyway
        match self {
            CssColor::RGB { r, g, b, a } => {
                if omit_alpha(*a) {
                    format!(
                        "rgb({:.3}%, {:.3}%, {:.3}%)",
                        r * 100f32,
                        g * 100f32,
                        b * 100f32
                    )
                } else {
                    format!(
                        "rgba({:.3}%, {:.3}%, {:.3}%, {:.5})",
                        r * 100f32,
                        g * 100f32,
                        b * 100f32,
                        a
                    )
                }
            }
            CssColor::HSL { h, s, l, a } => {
                if omit_alpha(*a) {
                    format!("hsl({:.2}, {:.3}%, {:.3}%)", h, s * 100f32, l * 100f32)
                } else {
                    format!(
                        "hsla({:.2}, {:.3}%, {:.3}%, {:.5})",
                        h,
                        s * 100f32,
                        l * 100f32,
                        a
                    )
                }
            }
        }
    }
}

impl std::fmt::Display for CssColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.to_css_value())
    }
}

/// Try to convert from a hash-bang spec to css color.
///
/// ```
/// # use std::convert::TryFrom;
/// # use material_styles_yew::CssColor;
/// assert_eq!(CssColor::try_from("#fff")?, CssColor::rgb(0xff, 0xff, 0xff));
/// # Ok::<(), String>(())
/// ```
/// ""
impl<'a> TryFrom<&'a str> for CssColor {
    type Error = String;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        if s.len() == 4 {
            if &s[0..1] != "#" {
                return Err("expected '#' to come first".to_string());
            }
            let r = u8::from_str_radix(&s[1..2].repeat(2), 16)
                .map_err(|_| "error parsing red component".to_string())?;
            let g = u8::from_str_radix(&s[2..3].repeat(2), 16)
                .map_err(|_| "error parsing green component".to_string())?;
            let b = u8::from_str_radix(&s[3..4].repeat(2), 16)
                .map_err(|_| "error parsing blue component".to_string())?;
            Ok(Self::rgb(r, g, b))
        } else if s.len() == 7 {
            if &s[0..1] != "#" {
                return Err("expected '#' to come first".to_string());
            }
            let r = u8::from_str_radix(&s[1..3], 16)
                .map_err(|_| "error parsing red component".to_string())?;
            let g = u8::from_str_radix(&s[3..5], 16)
                .map_err(|_| "error parsing green component".to_string())?;
            let b = u8::from_str_radix(&s[5..7], 16)
                .map_err(|_| "error parsing blue component".to_string())?;
            Ok(Self::rgb(r, g, b))
        } else {
            Err("wrong length for parsing, only accepts '#xxx' and '#xxxxxx' formats".to_string())
        }
    }
}
