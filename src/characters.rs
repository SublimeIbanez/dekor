/// The `Utf8` enum represents a collection of UTF-8 characters.
///
/// Each variant of this enum corresponds to a specific character.
/// Source: <https://www.fileformat.info/info/charset/UTF-8/list.htm>
/// NOTE: The list is currently incomplete and will be expanded on slowly
// TODO: Complete the list
pub enum Utf8 {
    /// Modifier letter left arrowhead (˂)
    ModLetterLeftArrowhead,
    /// Modifier letter right arrowhead (˃)
    ModLetterRightArrowhead,
    /// Modifier letter up arrowhead (˄)
    ModLetterUpArrowhead,
    /// Modifier letter down arrowhead (˅)
    ModLetterDownArrowhead,
    /// End of guard area (—)
    EndGuardArea,
    /// Horizontal pipe, slim variant (—)
    HPipeSlim, //EndGuardArea 
    /// Vertical pipe, slim variant (│)
    VPipeSlim, // ??? - uknown link
    /// Joint for slim horizontal and vertical pipes (├)
    JointPipeSlim, // ??? - unknown link
    /// Node for slim pipes (└)
    NodePipeSlim, // ??? - unknown link
    /// Curved node for slim pipes (╰)
    NodePipeCurved, // ??? - unknown link
    /// Horizontal pipe, bold variant (━)
    HPipeBold, // ??? - unknown link
    /// Vertical pipe, bold variant (┃)
    VPipeBold, // ??? - unknown link
    /// Joint for bold horizontal and vertical pipes (┣)
    JointPipeBold, // ??? - unknown link
    /// Node for bold pipes (┗)
    NodePipeBold, // ??? - unknown link
    /// Horizontal double pipe (═)
    HPipeDouble, // ??? - unknown link
    /// Vertical double pipe (║)
    VPipeDouble, // ??? - unknown link
    /// Joint for double horizontal and vertical pipes (╠)
    JointPipeDouble, // ??? - unknown link
    /// Node for double pipes (╚)
    NodePipeDouble, // ??? - unknown link
    /// Canadian Aboriginal Syllabics letter E (ᐁ)
    CanadianSyllabicsE,
    /// Canadian Aboriginal Syllabics letter I (ᐃ)
    CanadianSyllabicsI,
    /// Canadian Aboriginal Syllabics letter O (ᐅ)
    CanadianSyllabicsO,
    /// Canadian Aboriginal Syllabics letter A (ᐊ)
    CanadianSyllabicsA,
    /// Triangle pointing downwards, represented by Canadian Syllabics E (ᐁ)
    TriangleDown,
    /// Triangle pointing upwards, represented by Canadian Syllabics I (ᐃ)
    TriangleUp,
    /// Triangle pointing to the right, represented by Canadian Syllabics O (ᐅ)
    TriangleRight,
    /// Triangle pointing to the left, represented by Canadian Syllabics A (ᐊ)
    TriangleLeft,
}

impl std::fmt::Display for Utf8 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Utf8::ModLetterLeftArrowhead => write!(f, "˂"),
            Utf8::ModLetterRightArrowhead => write!(f, "˃"),
            Utf8::ModLetterUpArrowhead => write!(f, "˄"),
            Utf8::ModLetterDownArrowhead => write!(f, "˅"),
            Utf8::EndGuardArea => write!(f, "—"),
            Utf8::CanadianSyllabicsE => write!(f, "ᐁ"),
            Utf8::CanadianSyllabicsI => write!(f, "ᐃ"),
            Utf8::CanadianSyllabicsO => write!(f, "ᐅ"),
            Utf8::CanadianSyllabicsA => write!(f, "ᐊ"),
            Utf8::TriangleDown => write!(f, "ᐁ"),
            Utf8::TriangleUp => write!(f, "ᐃ"),
            Utf8::TriangleRight => write!(f, "ᐅ"),
            Utf8::TriangleLeft => write!(f, "ᐊ"),
            Utf8::HPipeSlim => write!(f, "—"),
            Utf8::VPipeSlim => write!(f, "│"),
            Utf8::JointPipeSlim => write!(f, "├"),
            Utf8::NodePipeSlim => write!(f, "└"),
            Utf8::NodePipeCurved => write!(f, "╰"),
            Utf8::HPipeBold => write!(f, "━"),
            Utf8::VPipeBold => write!(f, "┃"),
            Utf8::JointPipeBold => write!(f, "┣"),
            Utf8::NodePipeBold => write!(f, "┗"),
            Utf8::HPipeDouble => write!(f, "═"),
            Utf8::VPipeDouble => write!(f, "║"),
            Utf8::JointPipeDouble => write!(f, "╠"),
            Utf8::NodePipeDouble => write!(f, "╚"),
        }
    }
}

impl Utf8 {
    /// Repeats the character representation of the enum variant `n` times.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of times to repeat the character.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use dekor::characters::Utf8;
    ///
    /// let repeated_arrow = Utf8::ModLetterLeftArrowhead.repeat(3);
    /// assert_eq!(repeated_arrow, "˂˂˂");
    ///
    /// let repeated_pipe = Utf8::VPipeSlim.repeat(4);
    /// assert_eq!(repeated_pipe, "││││");
    /// ```
    ///
    pub fn repeat(&self, n: usize) -> String {
        std::iter::repeat(self.to_string()).take(n).collect()
    }
}
