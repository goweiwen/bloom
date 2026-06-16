use std::fmt;
use std::str::FromStr;

use crate::bannerfont::{Banner, Color, Error, Layer, Pattern};

/// The compact text code for a banner: each layer as its pattern code followed
/// by its color code (e.g. `b0cre0`). Encodes via [`Display`](fmt::Display) and
/// parses back via [`Banner`]'s [`FromStr`]; used as the IndexedDB key.
pub struct BannerCode<'a>(pub &'a Banner);

impl fmt::Display for BannerCode<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for layer in &self.0.layers {
            f.write_str(layer.pattern.code())?;
            f.write_str(layer.color.code())?;
        }
        Ok(())
    }
}

impl FromStr for Banner {
    type Err = Error;

    fn from_str(mut code: &str) -> Result<Self, Error> {
        let mut layers = Vec::new();

        while !code.is_empty() {
            let pattern_end = code
                .find(|c: char| !c.is_ascii_lowercase())
                .unwrap_or(code.len());
            let pattern = &code[..pattern_end];
            if pattern.is_empty() {
                return Err(Error::UnexpectedEndOfCode);
            }
            let pattern = Pattern::try_from_code(pattern)?;
            code = &code[pattern_end..];

            let color_end = code
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(code.len());
            let color = &code[..color_end];
            if color.is_empty() {
                return Err(Error::UnexpectedEndOfCode);
            }
            let color = Color::try_from_code(color)?;
            code = &code[color_end..];

            layers.push(Layer::new(pattern, color));
        }

        Ok(Banner::new(layers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrips() {
        let code = "b10ss2bri10cbo2bo15";
        let banner: Banner = code.parse().unwrap();
        assert_eq!(code, BannerCode(&banner).to_string());
    }
}
