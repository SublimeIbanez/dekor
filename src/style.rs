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
    Reset       = 0,
    Bold        = 1,
    Italic      = 3,
    Underline   = 4,
    FGBlack     = 30,
    FGRed       = 31,
    FGGreen     = 32,
    FGYellow    = 33,
    FGBlue      = 34,
    FGPurple    = 35,
    FGCyan      = 36,
    FGWhite     = 37,
    FGRGB       = 38,
    BGBlack     = 40,
    BGRed       = 41,
    BGGreen     = 42,
    BGYellow    = 43,
    BGBlue      = 44,
    BGPurple    = 45,
    BGCyan      = 46,
    BGWhite     = 47,
    BGRGB       = 48,
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            _ => write!(f, "{}", *self as u64)
        }
    }
}

/// Converts a hexadecimal color string to its RGB components.
///
/// # Arguments
///
/// * `value` - A value implementing `Display` that represents the hexadecimal color string in the form of "#RRGGBB" or "RRGGBB".
///
/// # Returns
///
/// A tuple `(u8, u8, u8)` representing the RGB components of the color.
/// - If the input is not in a valid hexadecimal format, returns `(0, 0, 0)`.
///
/// # Example
///
/// ```
/// use dekor::*;
/// 
/// let rgb = as_rgb("#FF5733");
/// assert_eq!(rgb, (255, 87, 51));
/// 
/// let not_rgb = as_rgb("this is not valid");
/// assert_eq!(not_rgb, (0, 0, 0));
/// ```
///
pub fn as_rgb<D: std::fmt::Display>(value: D) -> (u8, u8, u8) {
    let v = &value.to_string();
    let hex = if v.starts_with('#') { &v[1..] } else { v };

    match hex.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            return (r, g, b);
        },
        _ => return (0, 0, 0),
    };
}

/// An error encountered while converting a hexadecimal string to an RGB color.
///
/// This enum covers the range of errors that can occur when parsing a hexadecimal
/// string representation of a color into its RGB components.
///
/// Variants:
/// - `ParseError`: Wraps a `std::num::ParseIntError` encountered during the parsing
///   of the hexadecimal string into integers.
/// - `InvalidLength`: Indicates that the provided hexadecimal string does not have
///   a valid length for RGB color representation.
/// 
#[derive(Debug)]
pub enum HexError {
    ParseError(std::num::ParseIntError),
    InvalidLength,
}

impl From<std::num::ParseIntError> for HexError {
    fn from(err: std::num::ParseIntError) -> Self {
        HexError::ParseError(err)
    }
}

impl std::fmt::Display for HexError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HexError::ParseError(e) => write!(f, "{}", e),
            HexError::InvalidLength => write!(f, "Invalid hex length."),
        }
    }
}

/// Converts a hexadecimal color string to its RGB components.
///
/// Accepts a hexadecimal string in the format of "RRGGBB" or "#RRGGBB" and converts
/// it into a tuple of three `u8` values representing the red, green, and blue components
/// of the color, respectively.
///
/// # Arguments
///
/// * `value` - A value implementing `Display` that represents the hexadecimal color string.
///
/// # Returns
///
/// A `Result` containing either the RGB components as a tuple `(u8, u8, u8)` upon successful
/// conversion, or an `HexError` indicating the nature of the failure (either a parsing error
/// or an invalid string length).
///
/// # Examples
///
/// ```
/// # use dekor::*;
/// 
/// let rgb = to_rgb("#FF5733").unwrap();
/// assert_eq!(rgb, (255, 87, 51));
///
/// assert!(to_rgb("123456").is_ok());
///
/// assert!(matches!(to_rgb("GGGGGG"), Err(HexError::ParseError(_))));
/// assert!(matches!(to_rgb("123"), Err(HexError::InvalidLength)));
/// ```
pub fn to_rgb<D: std::fmt::Display>(value: D) -> Result<(u8, u8, u8), HexError> {
    let v = &value.to_string();
    let hex = if v.starts_with('#') { &v[1..] } else { v };

    match hex.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16)?;
            let g = u8::from_str_radix(&hex[2..4], 16)?;
            let b = u8::from_str_radix(&hex[4..6], 16)?;
            return Ok((r, g, b));
        },
        _ => return Err(HexError::InvalidLength),
    };
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
    ($($v:ident),+ => $input:expr$(,)?) => {{
        let mut codes = Vec::new();
        $(
            codes.push(
                format!("{}", $crate::style::Style::$v),
            );
        )+
        format!("\x1b[{}m{}\x1b[0m", codes.join(";"), $input)
    }};

    ($($v1:ident,)? $(($rgb:ident, $r:expr, $g:expr, $b:expr)),+ $(,$v2:ident)? => $input:expr$(,)?) => {{
        let mut codes = Vec::new();
        $(codes.push(
            format!("{}", $crate::style::Style::$v1)
        );)?
        $(codes.push(
            format!("{};2;{};{};{}", $crate::style::Style::$rgb, $r, $g, $b),
        );)+
        $(codes.push(
            format!("{}", $crate::style::Style::$v2)
        );)?
        format!("\x1b[{}m{}\x1b[0m", codes.join(";"), $input)
    }}
}

/// Applies ANSI styling to a string based on a list of `Style` variants and a displayable input.
///
/// This function takes an iterator over `Style` enum variants and an input that implements
/// the `Display` trait. It constructs a styled string by applying the ANSI escape codes
/// corresponding to the provided styles. The function is useful for dynamically applying
/// multiple styles to text at runtime.
/// - Does not currently support the `BGRGB` or `FGRGB` styles
///
/// # Arguments
///
/// * `styles`: An iterator over `Style` enum variants (e.g., `Vec<Style>`, `&[Style]`, etc.).
/// * `input`: The input text to which the styles will be applied. It must implement the `Display`
///   trait.
///
/// # Returns
///
/// Returns a `String` that contains the input text wrapped in ANSI escape codes for the specified
/// styles. The returned string, when printed to a terminal that supports ANSI escape codes, will
/// display the input text with the desired styling applied.
///
/// # Example Usage
///
/// ```rust
/// use dekor::*;
///
/// fn main() {
///     let styles_vec = vec![Style::BGBlue, Style::FGRed, Style::Underline];
///     let styled_text = style(styles_vec, "Hello, world!");
///     println!("{}", styled_text);
///
///     let styles_array = [Style::Bold, Style::FGGreen];
///     let styled_text_array = style(styles_array, "Bold Green Text");
///     println!("{}", styled_text_array);
/// }
/// ```
///
pub fn style<I, D>(styles: I, input: D) -> String 
where I: IntoIterator<Item = Style>, D: std::fmt::Display
{
    let styles: Vec<String> = styles.into_iter().map(|s| s.to_string()).collect();
    
    return format!("\x1b[{}m{}\x1b[0m", styles.join(";"), input);
}

/// Applies ANSI styling to a string based on a mix of `Style` variants and RGB color specifications.
///
/// This function accepts an iterator over tuples where each tuple consists of a `Style` variant 
/// and three `u8` values representing RGB colors. It constructs a styled string by applying the 
/// ANSI escape codes corresponding to the provided styles and colors. The function supports 
/// dynamic application of multiple styles and colors to text at runtime, including RGB colors 
/// for foreground (`FGRGB`) and background (`BGRGB`) alongside other standard styles.
///
/// # Arguments
///
/// * `styles`: An iterator over tuples, where each tuple contains a `Style` variant followed by 
///   three `u8` values for RGB colors. For non-RGB styles, RGB values are ignored.
/// * `input`: The input text to which the styles and colors will be applied. It must implement the
///   `Display` trait, allowing for flexible text input types.
///
/// # Returns
///
/// Returns a `String` that contains the input text wrapped in ANSI escape codes for the specified
/// styles and colors. The returned string, when printed to a terminal that supports ANSI escape 
/// codes, will display the input text with the desired styling and coloring applied.
///
/// # Example Usage
///
/// ```rust
/// use dekor::*;
///
/// fn main() {
///     let styles = vec![
///         (Style::Bold, 0, 0, 0), // RGB values are ignored for non-RGB styles
///         (Style::FGRGB, 255, 100, 50), // Apply RGB color for foreground
///         (Style::Underline, 0, 0, 0), // Ignored RGB for non-RGB style
///         (Style::BGRGB, 20, 40, 60), // Apply RGB color for background
///     ];
///     let styled_text = styler(styles, "Hello, styled and colored world!");
///     println!("{}", styled_text);
/// }
/// ```
///
/// Note: When specifying `Style::FGRGB` or `Style::BGRGB`, the corresponding RGB values are used 
/// to set the foreground or background color respectively. For other `Style` variants, the RGB 
/// values are ignored.
///
pub fn styler<I, D>(styles: I, input: D) -> String 
where I: IntoIterator<Item = (Style, u8, u8, u8)>, D: std::fmt::Display
{
    let styles: Vec<String> = styles.into_iter().map(
        |s| match s.0 {
            Style::FGRGB | Style::BGRGB => format!("{};2;{};{};{}", s.0, s.1, s.2, s.3),
            _ => s.0.to_string(),
        }).collect();
    
    return format!("\x1b[{}m{}\x1b[0m", styles.join(";"), input);
}

// ################################################# Tests #################################################

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_rgb_valid_hex() {
        assert_eq!(to_rgb("#FF5733").unwrap(), (255, 87, 51));
        assert_eq!(to_rgb("FF5733").unwrap(), (255, 87, 51));
        assert_eq!(to_rgb("#ff5733").unwrap(), (255, 87, 51)); // Test case sensitivity
    }

    #[test]
    fn test_to_rgb_invalid_hex() {
        match to_rgb("GGGGGG") {
            Err(HexError::ParseError(_)) => (),
            _ => panic!("Expected ParseError"),
        }
        assert!(matches!(to_rgb("123"), Err(HexError::InvalidLength)));
    }

    #[test]
    fn test_style_output() {
        let styles = vec![Style::Bold, Style::FGRed];
        let input = "Hello, world!";
        let expected = "\x1b[1;31mHello, world!\x1b[0m"; // Expected ANSI escape code sequence
        assert_eq!(style(styles.iter().copied(), input), expected);
    }

    #[test]
    fn test_styler_with_rgb() {
        let styles = vec![
            (Style::FGRGB, 255, 0, 0), // Red foreground
            (Style::BGRGB, 0, 0, 255), // Blue background
        ];
        let input = "Hello, RGB!";
        // Note: Adjust the expected string based on the actual ANSI codes your implementation generates
        let expected = "\x1b[38;2;255;0;0;48;2;0;0;255mHello, RGB!\x1b[0m";
        assert_eq!(styler(styles, input), expected);
    }

}