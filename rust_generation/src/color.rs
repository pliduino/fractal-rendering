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

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Self::Output {
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
