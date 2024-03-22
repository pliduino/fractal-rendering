use std::ops;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    #[allow(dead_code)]
    pub fn new() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub(crate) fn hue_shift(&self, radian: f64) -> Color {
        let cos_r: f64 = radian.cos();
        let sin_r: f64 = radian.sin();

        let mut r_matrix: [f64; 3] = [0.0; 3];
        let mut g_matrix: [f64; 3] = [0.0; 3];
        let mut b_matrix: [f64; 3] = [0.0; 3];

        r_matrix[0] = cos_r + (1.0 - cos_r) / 3.0;
        r_matrix[1] = 1.0 / 3.0 * (1.0 - cos_r) - f64::sqrt(1.0 / 3.0) * sin_r;
        r_matrix[2] = 1.0 / 3.0 * (1.0 - cos_r) + f64::sqrt(1.0 / 3.0) * sin_r;

        g_matrix[0] = 1.0 / 3.0 * (1.0 - cos_r) + f64::sqrt(1.0 / 3.0) * sin_r;
        g_matrix[1] = cos_r + 1.0 / 3.0 * (1.0 - cos_r);
        g_matrix[2] = 1.0 / 3.0 * (1.0 - cos_r) - f64::sqrt(1.0 / 3.0) * sin_r;

        b_matrix[0] = 1.0 / 3.0 * (1.0 - cos_r) - f64::sqrt(1.0 / 3.0) * sin_r;
        b_matrix[1] = 1.0 / 3.0 * (1.0 - cos_r) + f64::sqrt(1.0 / 3.0) * sin_r;
        b_matrix[2] = cos_r + (1.0 - cos_r) / 3.0;

        Color {
            r: self.r * r_matrix[0] + self.g * r_matrix[1] + self.b * r_matrix[2],
            g: self.r * g_matrix[0] + self.g * g_matrix[1] + self.b * g_matrix[2],
            b: self.r * b_matrix[0] + self.g * b_matrix[1] + self.b * b_matrix[2],
        }
    }
}

impl ops::Add for Color {
    type Output = Color;

    fn add(self, other: Self) -> Self::Output {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl ops::Sub for Color {
    type Output = Color;

    fn sub(self, other: Self) -> Self::Output {
        Color {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl<T: Into<f64>> ops::Mul<T> for Color {
    type Output = Color;

    fn mul(self, other: T) -> Self::Output {
        let other = other.into();
        Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_add_test() {
        let color_1 = Color {
            r: 5.0,
            g: 2.0,
            b: 24.0,
        };
        let color_2 = Color {
            r: 10.0,
            g: 5.0,
            b: 14.0,
        };

        let result = color_1 + color_2;
        assert_eq!(
            result,
            Color {
                r: 15.0,
                g: 7.0,
                b: 38.0
            }
        );
    }
}
