use anyhow::{bail, Context, Result};

/// Represents a color with the RGB color model.
///
/// See [Wikipedia](https://en.wikipedia.org/wiki/RGB_color_model).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    /// Returns a new color.
    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    /// Returns a new gray color.
    ///
    /// ```
    /// use tilemap::math::color::Color;
    /// assert_eq!(Color::gray(42), Color::from_rgb(42, 42, 42));
    /// ```
    pub const fn gray(value: u8) -> Color {
        Color {
            r: value,
            g: value,
            b: value,
        }
    }

    /// Converts a string to a color, if possible:
    ///
    /// ```
    /// use tilemap::math::color::{Color, ORANGE};
    /// assert_eq!(Color::from_hex("#FFA500").unwrap(), ORANGE);
    /// assert_eq!(Color::from_hex("#ffa500").unwrap(), ORANGE);
    /// ```
    pub fn from_hex(hex_code: &str) -> Result<Color> {
        if !hex_code.starts_with('#') {
            bail!("'{}' needs to start with # to be a color", hex_code);
        } else if hex_code.len() != 7 {
            bail!("'{}' needs to be 7 characters long to be a color", hex_code);
        }

        let r: u8 = u8::from_str_radix(&hex_code[1..3], 16).context(format!(
            "Failed to parse the value of red from '{}'",
            hex_code
        ))?;
        let g: u8 = u8::from_str_radix(&hex_code[3..5], 16).context(format!(
            "Failed to parse the value of green from '{}'",
            hex_code
        ))?;
        let b: u8 = u8::from_str_radix(&hex_code[5..7], 16).context(format!(
            "Failed to parse the value of blue from '{}'",
            hex_code
        ))?;

        Ok(Color::from_rgb(r, g, b))
    }

    /// Returns the red component.
    ///
    /// ```
    /// use tilemap::math::color::ORANGE;
    /// assert_eq!(ORANGE.r(), 255);
    /// ```
    pub fn r(&self) -> u8 {
        self.r
    }

    /// Returns the green component
    ///
    /// ```
    ///# use tilemap::math::color::ORANGE;
    /// assert_eq!(ORANGE.g(), 165);
    /// ```
    pub fn g(&self) -> u8 {
        self.g
    }

    /// Returns the blue component.
    ///
    /// ```
    ///# use tilemap::math::color::ORANGE;
    /// assert_eq!(ORANGE.b(), 0);
    /// ```
    pub fn b(&self) -> u8 {
        self.b
    }
}

impl Default for Color {
    fn default() -> Self {
        PINK
    }
}

pub const BLACK: Color = Color::from_rgb(0, 0, 0);
pub const BLUE: Color = Color::from_rgb(0, 0, 255);
pub const CYAN: Color = Color::from_rgb(0, 255, 255);
pub const GREEN: Color = Color::from_rgb(0, 255, 0);
pub const MAGENTA: Color = Color::from_rgb(255, 0, 255);
pub const ORANGE: Color = Color::from_rgb(255, 165, 0);
pub const RED: Color = Color::from_rgb(255, 0, 0);
pub const PINK: Color = Color::from_rgb(255, 0, 128);
pub const WHITE: Color = Color::from_rgb(255, 255, 255);
pub const YELLOW: Color = Color::from_rgb(255, 255, 0);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_hex_empty_string() {
        assert!(Color::from_hex("").is_err());
    }

    #[test]
    fn test_from_hex_invalid_start() {
        assert!(Color::from_hex("FFA500").is_err());
    }

    #[test]
    fn test_from_hex_part() {
        assert!(Color::from_hex("#").is_err());
        assert!(Color::from_hex("#FF").is_err());
        assert!(Color::from_hex("#FFA5").is_err());
        assert!(Color::from_hex("#FFA50").is_err());
    }

    #[test]
    fn test_from_hex_ignore_case() {
        assert_eq!(Color::from_hex("#FFA500").unwrap(), ORANGE);
        assert_eq!(Color::from_hex("#ffa500").unwrap(), ORANGE);
    }
}
