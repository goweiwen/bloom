use std::fmt;

use crate::bannerfont::Banner;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_banner_code() {
        let code = "b10ss2bri10cbo2bo15";
        let banner = Banner::try_from_code(code).unwrap();
        assert_eq!(code, BannerCode(&banner).to_string());
    }
}
