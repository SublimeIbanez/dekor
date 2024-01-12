//! # Dekor
//! 
//! Simple to use general character and styling library for Rust.
//! 
//! ## Features
//! - **Ease of Use**: Apply multiple text styles with a single macro call
//! - **Safety**: Compile-time checks prevent the use of invalid style names
//! - `style!()` macro which can be used to:
//!   - Generate a styled string for console output using `ansi` codes
//!   - Colors the text forground and background depending on passed parameters
//!   - Can bold, underline, and italicize the text as desired
//! - `Utf8` enum contains various UTF-8 characters
//!   - Intention is to complete the list of characters - this will be accomplished over time
//!   - Character list source: <https://www.fileformat.info/info/charset/UTF-8/list.htm>
//! 
//! ## Getting Started
//! To start using Dekor, add the following to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! dekor = "0.1.0"
//! ```
//! - Minimum supported Rust version: `1.56.1`
//! 
//! ## Usage
//! ```rust
//! use dekor::*;
//! 
//! fn main() {
//!   let decorated_text = style!(Bold, Underline, FGBlue => "This is decorated text");
//!   let pipes = format!("{}\n{}{}\n{}{}",
//!     Utf8::VPipeSlim, 
//!     Utf8::JointPipeSlim, Utf8::HPipeSlim, 
//!     Utf8::NodePipeSlim, Utf8::HPipeSlim,
//!   );
//! 
//!   // Output:
//!   // This is decorated text   <-- Will be blue text that is underlined and bolded
//!   // │
//!   // ├—
//!   // └—
//!   println!("{}\n{}", decorated_text, pipes);
//! }
//! ```
pub mod style;
pub mod characters;

pub use style::Style;
pub use characters::Utf8;


#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
