/// Represents styling options for text display in the terminal, primarily intended to be used in tandem with the `dekor!()` macro.
///
/// This enum includes variants for foreground (text) and background (highlight) color, as well as text styles like bold, italic and underline.
/// Each variant corresponds to an ANSI escape code.
///
/// # Variants
///
/// - Colors: Black, Red, Green, Yellow, Blue, Purple, Cyan, and White
/// - Foreground (FG) and Background (BG) implementations of these colors (e.g. FGBlue or BGWhite)
/// - Styling: Bold, Italic, and Underline styles
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use dekor::style::Style;
///
/// // Example of using a text color variant
/// let text_color = Style::FGRed;
/// println!("\x1b[{}mRed Text\x1b[0m", text_color);
///
/// // Example of using a style variant
/// let text_style = Style::Bold;
/// println!("\x1b[{}mBold Text\x1b[0m", text_style);
/// ```
///
#[derive(Clone, Copy)]
pub enum Style {
    Reset = 0,
    Bold = 1,
    Italic = 3,
    Underline = 4,
    FGBlack = 30,
    FGRed = 31,
    FGGreen = 32,
    FGYellow = 33,
    FGBlue = 34,
    FGPurple = 35,
    FGCyan = 36,
    FGWhite = 37,
    BGBlack = 40,
    BGRed = 41,
    BGGreen = 42,
    BGYellow = 43,
    BGBlue = 44,
    BGPurple = 45,
    BGCyan = 46,
    BGWhite = 47,
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            _ => write!(f, "{}", *self as u32),
        }
    }
}

pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    if !hex.starts_with("#") {
        return None;
    }

    let r = u8::from_str_radix(&hex[1..3], 16);
    let g = u8::from_str_radix(&hex[3..5], 16);
    let b = u8::from_str_radix(&hex[5..7], 16);

    if let (Ok(red), Ok(green), Ok(blue)) = (r, g, b) {
        return Some((red, green, blue));
    } else {
        return None;
    }
}

/// Applies ANSI styling to a string using the `Style` enum.
///
/// This macro simplifies the process of adding text styling, like colors and text attributes (e.g., bold, underline),
/// to a string for output in terminal applications. It constructs an ANSI escape sequence using the provided `Style` variants.
///
/// # Usage
///
/// Pass one or more `Style` enum variants, followed by `=>` and the string to format.
///
/// # Examples
///
/// ```
/// use dekor::*;
///
/// let styled_text = style!(Bold, FGBlue, BGWhite => "Styled Text");
/// println!("{}", styled_text);
/// ```
///
/// This example applies bold blue text on a white background to "Styled Text".
///
/// # Notes
///
/// - The macro appends an ANSI reset sequence at the end of the formatted string to ensure that the styling does not affect subsequent text.
///
#[macro_export]
macro_rules! style {
    ($($v:ident),+ => $input:expr$(,)?) => {
        {
            let mut codes = Vec::new();
            $(
                codes.push(
                    format!("{}", $crate::style::Style::$v),
                );
            )+
            let fmt = codes.join(";");
            format!("\x1b[{}m{}\x1b[0m", fmt, $input)
        }
    };
}
