use std::ops;
use std::math;

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

    pub fn hue_shift(Color this, float radian) -> Color
    {
        let cosR : float = cos(radian);
        let sinR : float = sin(radian);

        let mut r_matrix : float[3];
        let mut g_matrix : float[3];
        let mut b_matrix : float[3];

        r_matrix[0] = cosR + (1.0 - cosR) / 3.0;
        r_matrix[1] = 1.0/3.0 * (1.0 - cosR) - sqrt(1.0/3.0) * sinR;
        r_matrix[2] = 1.0/3.0 * (1.0 - cosR) + sqrt(1.0/3.0) * sinR;

        g_matrix[0] = 1.0/3.0 * (1.0 - cosR) + sqrt(1.0/3.0) * sinR;
        g_matrix[1] = cosR + 1.0/3.0 * (1.0 - cosR);
        g_matrix[2] = 1.0/3.0 * (1.0 - cosR) - sqrt(1.0/3.0) * sinR;

        b_matrix[0] = 1.0/3.0 * (1.0 - cosR) - sqrt(1.0/3.0) * sinR;
        b_matrix[1] = 1.0/3.0 * (1.0 - cosR) + sqrt(1.0/3.0) * sinR;
        b_matrix[2] = cosR + (1.0 - cosR) / 3.0;
        
        Color {
            r: this.r * r_matrix[0] + this.g * r_matrix[1] + this.b * r_matrix[2],
            g: this.r * g_matrix[0] + this.g * g_matrix[1] + this.b * g_matrix[2],
            b: this.r * b_matrix[0] + this.g * b_matrix[1] + this.b * b_matrix[2],
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
