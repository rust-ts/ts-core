//! Low-level Rust lexer.
//!
//! The idea with `librustc_lexer` is to make a reusable library,
//! by separating out pure lexing and rustc-specific concerns, like spans,
//! error reporting, and interning.  So, rustc_lexer operates directly on `&str`,
//! produces simple tokens which are a pair of type-tag and a bit of original text,
//! and does not report errors, instead storing them as flags on the token.
//!
//! Tokens produced by this lexer are not yet ready for parsing the Rust syntax.
//! For that see [`librustc_parse::lexer`], which converts this basic token stream
//! into wide tokens used by actual parser.
//!
//! The purpose of this crate is to convert raw sources into a labeled sequence
//! of well-known token types, so building an actual Rust token stream will
//! be easier.
//!
//! The main entity of this crate is the [`TokenKind`] enum which represents common
//! lexeme types.
//!
//! [`librustc_parse::lexer`]: ../rustc_parse/lexer/index.html
// We want to be able to build this crate with a stable compiler, so no
// `#![feature]` attributes should be added.

mod cursor;
pub mod unescape;

#[cfg(test)]
mod tests;

use self::LiteralKind::*;
use self::TokenKind::*;
use crate::cursor::Cursor;

/// Parsed token.
/// It doesn't contain information about data that has been parsed,
/// only the type of the token and its size.
#[derive(Debug)]
pub struct Token {
  pub kind: TokenKind,
  pub len: usize,
}

impl Token {
  fn new(kind: TokenKind, len: usize) -> Token {
    Token { kind, len }
  }
}

/// Enum representing common lexeme types.
// perf note: Changing all `usize` to `u32` doesn't change performance. See #77629
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
  // Multi-char tokens:
  /// "// comment"
  LineComment,
  /// `/* block comment */`
  ///
  /// Block comments can be recursive, so the sequence like `/* /* */`
  /// will not be considered terminated and will result in a parsing error.
  BlockComment { terminated: bool },
  /// Any whitespace characters sequence.
  Whitespace,
  /// Any line break characters sequence.
  LineBreak,
  /// "ident" or "continue"
  /// At this step keywords are also considered identifiers.
  Ident,
  /// "12_u8", "1.0e-40". See `LiteralKind` for more details.
  Literal { kind: LiteralKind, suffix_start: usize },
  // One-char tokens:
  /// ";"
  Semi,
  /// ","
  Comma,
  /// "."
  Dot,
  /// "("
  OpenParen,
  /// ")"
  CloseParen,
  /// "{"
  OpenBrace,
  /// "}"
  CloseBrace,
  /// "["
  OpenBracket,
  /// "]"
  CloseBracket,
  /// "@"
  At,
  /// "#"
  Pound,
  /// "~"
  Tilde,
  /// "?"
  Question,
  /// ":"
  Colon,
  /// "$"
  Dollar,
  /// "="
  Eq,
  /// "!"
  Bang,
  /// "<"
  Lt,
  /// ">"
  Gt,
  /// "-"
  Minus,
  /// "&"
  And,
  /// "|"
  Or,
  /// "+"
  Plus,
  /// "*"
  Star,
  /// "/"
  Slash,
  /// "^"
  Caret,
  /// "%"
  Percent,

  /// Unknown token, not expected by the lexer, e.g. "№"
  Unknown,
}

/// see more https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Lexical_grammar
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
  /// "12_u8", "0o100", "0b120i99", "123n", "0x123n", ""
  Numeric { base: Base, empty_int: bool, empty_exponent: bool },
  /// ""abc"", ""abc", "'abc'", "`abc${d}`"
  Str { terminated: bool },
}

/// Base of numeric literal encoding according to its prefix.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
  /// Literal starts with "0b".
  Binary,
  /// Literal starts with "0o".
  Octal,
  /// Literal starts with "0x".
  Hexadecimal,
  /// Literal doesn't contain a prefix.
  Decimal,
}

/// `rustc` allows files to have a shebang, e.g. "#!/usr/bin/rustrun",
/// but shebang isn't a part of rust syntax.
pub fn strip_shebang(input: &str) -> Option<usize> {
  // Shebang must start with `#!` literally, without any preceding whitespace.
  // For simplicity we consider any line starting with `#!` a shebang,
  // regardless of restrictions put on shebangs by specific platforms.
  if let Some(input_tail) = input.strip_prefix("#!") {
    // Ok, this is a shebang but if the next non-whitespace token is `[`,
    // then it may be valid Rust code, so consider it Rust code.
    let next_non_whitespace_token = tokenize(input_tail).map(|tok| tok.kind).find(|tok| {
      !matches!(
        tok,
        TokenKind::Whitespace
          | TokenKind::LineBreak
          | TokenKind::LineComment
          | TokenKind::BlockComment { .. }
      )
    });
    if next_non_whitespace_token != Some(TokenKind::OpenBracket) {
      // No other choice than to consider this a shebang.
      return Some(2 + input_tail.lines().next().unwrap_or_default().len());
    }
  }
  None
}

/// Parses the first token from the provided input string.
pub fn first_token(input: &str) -> Token {
  debug_assert!(!input.is_empty());
  Cursor::new(input).advance_token()
}

/// Creates an iterator that produces tokens from the input string.
pub fn tokenize(mut input: &str) -> impl Iterator<Item = Token> + '_ {
  std::iter::from_fn(move || {
    if input.is_empty() {
      return None;
    }
    let token = first_token(input);
    input = &input[token.len..];
    Some(token)
  })
}

/// True if `c` is considered a whitespace according to ECMAScript language definition.
/// See [ECMA262](https://tc39.es/ecma262/#sec-white-space)
/// for definitions of these classes.
pub fn is_whitespace(c: char) -> bool {
  // This is Pattern_White_Space.
  //
  // Note that this set is stable (ie, it doesn't change with different
  // Unicode versions), so it's ok to just hard-code the values.

  matches!(
    c,
    // Usual ASCII suspects
    '\u{0009}'       // <TAB>    \t
        | '\u{000B}' // <VT>     Vertical Tab or Line Tabulation
        | '\u{000C}' // <FF>     Form Feed
        | '\u{0020}' // <SP>     " "
        | '\u{00A0}' // <NBSP>   No-Break space
        | '\u{FEFF}' // <ZWNBSP> Zero Width No-Nreak Space

        // other Unicode Space Separators
        // see [Space]https://en.wikipedia.org/wiki/Space_%28punctuation%29#Spaces_in_Unicode
        | '\u{1680}' // Ogham space mark
        | '\u{180E}' // Mongolian Vowel Separator
        | '\u{2002}' // En Space
        | '\u{2003}' // Em space
        | '\u{2004}' // Three-Per-Em Space
        | '\u{2005}' // Four-Per-Em Space
        | '\u{2006}' // Six-Per-Em Space
        | '\u{2007}' // Figure Space
        | '\u{2008}' // Punctuation Space
        | '\u{2009}' // Thin Space
        | '\u{200A}' // Hair space
        | '\u{200B}' // Zero Width Space
        | '\u{200C}' // Zero Width Non Joiner
        | '\u{200D}' // Zero Width Joiner
        | '\u{202F}' // Narrow No-Break Space
        | '\u{205F}' // Medium Mathematical Space
        | '\u{2060}' // Word Joiner
        | '\u{3000}' // Ideographic Space
  )
}

/// True if `c` is considered a line terminator according to ECMAScript language definition.
/// See [ECMA262](https://tc39.es/ecma262/#sec-line-terminators)
/// for definitions of these classes.
pub fn is_line_break(c: char) -> bool {
  matches!(
    c,
    // Usual ASCII suspects
    '\u{000A}'     // <LF> Line Feed
      | '\u{000D}' // <CR> Carriage Return
      | '\u{2028}' // <LS> Line Separator
      | '\u{2029}' // <PS> Paragraph Separator
  )
}

///
/// Generated by scripts/regenerate-unicode-identifier-parts.js on node v12.4.0 with unicode 12.1
/// based on http://www.unicode.org/reports/tr31/ and https://www.ecma-international.org/ecma-262/6.0/#sec-names-and-keywords
/// unicodeESNextIdentifierStart corresponds to the ID_Start and Other_ID_Start property, and
/// unicodeESNextIdentifierPart corresponds to ID_Continue, Other_ID_Continue, plus ID_Start and Other_ID_Start
///
#[rustfmt::skip]
pub static UNICODE_ESNEXT_IDENTIFIER_START: [u32; 1218]  = [65, 90, 97, 122, 170, 170, 181, 181, 186, 186, 192, 214, 216, 246, 248, 705, 710, 721, 736, 740, 748, 748, 750, 750, 880, 884, 886, 887, 890, 893, 895, 895, 902, 902, 904, 906, 908, 908, 910, 929, 931, 1013, 1015, 1153, 1162, 1327, 1329, 1366, 1369, 1369, 1376, 1416, 1488, 1514, 1519, 1522, 1568, 1610, 1646, 1647, 1649, 1747, 1749, 1749, 1765, 1766, 1774, 1775, 1786, 1788, 1791, 1791, 1808, 1808, 1810, 1839, 1869, 1957, 1969, 1969, 1994, 2026, 2036, 2037, 2042, 2042, 2048, 2069, 2074, 2074, 2084, 2084, 2088, 2088, 2112, 2136, 2144, 2154, 2208, 2228, 2230, 2237, 2308, 2361, 2365, 2365, 2384, 2384, 2392, 2401, 2417, 2432, 2437, 2444, 2447, 2448, 2451, 2472, 2474, 2480, 2482, 2482, 2486, 2489, 2493, 2493, 2510, 2510, 2524, 2525, 2527, 2529, 2544, 2545, 2556, 2556, 2565, 2570, 2575, 2576, 2579, 2600, 2602, 2608, 2610, 2611, 2613, 2614, 2616, 2617, 2649, 2652, 2654, 2654, 2674, 2676, 2693, 2701, 2703, 2705, 2707, 2728, 2730, 2736, 2738, 2739, 2741, 2745, 2749, 2749, 2768, 2768, 2784, 2785, 2809, 2809, 2821, 2828, 2831, 2832, 2835, 2856, 2858, 2864, 2866, 2867, 2869, 2873, 2877, 2877, 2908, 2909, 2911, 2913, 2929, 2929, 2947, 2947, 2949, 2954, 2958, 2960, 2962, 2965, 2969, 2970, 2972, 2972, 2974, 2975, 2979, 2980, 2984, 2986, 2990, 3001, 3024, 3024, 3077, 3084, 3086, 3088, 3090, 3112, 3114, 3129, 3133, 3133, 3160, 3162, 3168, 3169, 3200, 3200, 3205, 3212, 3214, 3216, 3218, 3240, 3242, 3251, 3253, 3257, 3261, 3261, 3294, 3294, 3296, 3297, 3313, 3314, 3333, 3340, 3342, 3344, 3346, 3386, 3389, 3389, 3406, 3406, 3412, 3414, 3423, 3425, 3450, 3455, 3461, 3478, 3482, 3505, 3507, 3515, 3517, 3517, 3520, 3526, 3585, 3632, 3634, 3635, 3648, 3654, 3713, 3714, 3716, 3716, 3718, 3722, 3724, 3747, 3749, 3749, 3751, 3760, 3762, 3763, 3773, 3773, 3776, 3780, 3782, 3782, 3804, 3807, 3840, 3840, 3904, 3911, 3913, 3948, 3976, 3980, 4096, 4138, 4159, 4159, 4176, 4181, 4186, 4189, 4193, 4193, 4197, 4198, 4206, 4208, 4213, 4225, 4238, 4238, 4256, 4293, 4295, 4295, 4301, 4301, 4304, 4346, 4348, 4680, 4682, 4685, 4688, 4694, 4696, 4696, 4698, 4701, 4704, 4744, 4746, 4749, 4752, 4784, 4786, 4789, 4792, 4798, 4800, 4800, 4802, 4805, 4808, 4822, 4824, 4880, 4882, 4885, 4888, 4954, 4992, 5007, 5024, 5109, 5112, 5117, 5121, 5740, 5743, 5759, 5761, 5786, 5792, 5866, 5870, 5880, 5888, 5900, 5902, 5905, 5920, 5937, 5952, 5969, 5984, 5996, 5998, 6000, 6016, 6067, 6103, 6103, 6108, 6108, 6176, 6264, 6272, 6312, 6314, 6314, 6320, 6389, 6400, 6430, 6480, 6509, 6512, 6516, 6528, 6571, 6576, 6601, 6656, 6678, 6688, 6740, 6823, 6823, 6917, 6963, 6981, 6987, 7043, 7072, 7086, 7087, 7098, 7141, 7168, 7203, 7245, 7247, 7258, 7293, 7296, 7304, 7312, 7354, 7357, 7359, 7401, 7404, 7406, 7411, 7413, 7414, 7418, 7418, 7424, 7615, 7680, 7957, 7960, 7965, 7968, 8005, 8008, 8013, 8016, 8023, 8025, 8025, 8027, 8027, 8029, 8029, 8031, 8061, 8064, 8116, 8118, 8124, 8126, 8126, 8130, 8132, 8134, 8140, 8144, 8147, 8150, 8155, 8160, 8172, 8178, 8180, 8182, 8188, 8305, 8305, 8319, 8319, 8336, 8348, 8450, 8450, 8455, 8455, 8458, 8467, 8469, 8469, 8472, 8477, 8484, 8484, 8486, 8486, 8488, 8488, 8490, 8505, 8508, 8511, 8517, 8521, 8526, 8526, 8544, 8584, 11264, 11310, 11312, 11358, 11360, 11492, 11499, 11502, 11506, 11507, 11520, 11557, 11559, 11559, 11565, 11565, 11568, 11623, 11631, 11631, 11648, 11670, 11680, 11686, 11688, 11694, 11696, 11702, 11704, 11710, 11712, 11718, 11720, 11726, 11728, 11734, 11736, 11742, 12293, 12295, 12321, 12329, 12337, 12341, 12344, 12348, 12353, 12438, 12443, 12447, 12449, 12538, 12540, 12543, 12549, 12591, 12593, 12686, 12704, 12730, 12784, 12799, 13312, 19893, 19968, 40943, 40960, 42124, 42192, 42237, 42240, 42508, 42512, 42527, 42538, 42539, 42560, 42606, 42623, 42653, 42656, 42735, 42775, 42783, 42786, 42888, 42891, 42943, 42946, 42950, 42999, 43009, 43011, 43013, 43015, 43018, 43020, 43042, 43072, 43123, 43138, 43187, 43250, 43255, 43259, 43259, 43261, 43262, 43274, 43301, 43312, 43334, 43360, 43388, 43396, 43442, 43471, 43471, 43488, 43492, 43494, 43503, 43514, 43518, 43520, 43560, 43584, 43586, 43588, 43595, 43616, 43638, 43642, 43642, 43646, 43695, 43697, 43697, 43701, 43702, 43705, 43709, 43712, 43712, 43714, 43714, 43739, 43741, 43744, 43754, 43762, 43764, 43777, 43782, 43785, 43790, 43793, 43798, 43808, 43814, 43816, 43822, 43824, 43866, 43868, 43879, 43888, 44002, 44032, 55203, 55216, 55238, 55243, 55291, 63744, 64109, 64112, 64217, 64256, 64262, 64275, 64279, 64285, 64285, 64287, 64296, 64298, 64310, 64312, 64316, 64318, 64318, 64320, 64321, 64323, 64324, 64326, 64433, 64467, 64829, 64848, 64911, 64914, 64967, 65008, 65019, 65136, 65140, 65142, 65276, 65313, 65338, 65345, 65370, 65382, 65470, 65474, 65479, 65482, 65487, 65490, 65495, 65498, 65500, 65536, 65547, 65549, 65574, 65576, 65594, 65596, 65597, 65599, 65613, 65616, 65629, 65664, 65786, 65856, 65908, 66176, 66204, 66208, 66256, 66304, 66335, 66349, 66378, 66384, 66421, 66432, 66461, 66464, 66499, 66504, 66511, 66513, 66517, 66560, 66717, 66736, 66771, 66776, 66811, 66816, 66855, 66864, 66915, 67072, 67382, 67392, 67413, 67424, 67431, 67584, 67589, 67592, 67592, 67594, 67637, 67639, 67640, 67644, 67644, 67647, 67669, 67680, 67702, 67712, 67742, 67808, 67826, 67828, 67829, 67840, 67861, 67872, 67897, 67968, 68023, 68030, 68031, 68096, 68096, 68112, 68115, 68117, 68119, 68121, 68149, 68192, 68220, 68224, 68252, 68288, 68295, 68297, 68324, 68352, 68405, 68416, 68437, 68448, 68466, 68480, 68497, 68608, 68680, 68736, 68786, 68800, 68850, 68864, 68899, 69376, 69404, 69415, 69415, 69424, 69445, 69600, 69622, 69635, 69687, 69763, 69807, 69840, 69864, 69891, 69926, 69956, 69956, 69968, 70002, 70006, 70006, 70019, 70066, 70081, 70084, 70106, 70106, 70108, 70108, 70144, 70161, 70163, 70187, 70272, 70278, 70280, 70280, 70282, 70285, 70287, 70301, 70303, 70312, 70320, 70366, 70405, 70412, 70415, 70416, 70419, 70440, 70442, 70448, 70450, 70451, 70453, 70457, 70461, 70461, 70480, 70480, 70493, 70497, 70656, 70708, 70727, 70730, 70751, 70751, 70784, 70831, 70852, 70853, 70855, 70855, 71040, 71086, 71128, 71131, 71168, 71215, 71236, 71236, 71296, 71338, 71352, 71352, 71424, 71450, 71680, 71723, 71840, 71903, 71935, 71935, 72096, 72103, 72106, 72144, 72161, 72161, 72163, 72163, 72192, 72192, 72203, 72242, 72250, 72250, 72272, 72272, 72284, 72329, 72349, 72349, 72384, 72440, 72704, 72712, 72714, 72750, 72768, 72768, 72818, 72847, 72960, 72966, 72968, 72969, 72971, 73008, 73030, 73030, 73056, 73061, 73063, 73064, 73066, 73097, 73112, 73112, 73440, 73458, 73728, 74649, 74752, 74862, 74880, 75075, 77824, 78894, 82944, 83526, 92160, 92728, 92736, 92766, 92880, 92909, 92928, 92975, 92992, 92995, 93027, 93047, 93053, 93071, 93760, 93823, 93952, 94026, 94032, 94032, 94099, 94111, 94176, 94177, 94179, 94179, 94208, 100343, 100352, 101106, 110592, 110878, 110928, 110930, 110948, 110951, 110960, 111355, 113664, 113770, 113776, 113788, 113792, 113800, 113808, 113817, 119808, 119892, 119894, 119964, 119966, 119967, 119970, 119970, 119973, 119974, 119977, 119980, 119982, 119993, 119995, 119995, 119997, 120003, 120005, 120069, 120071, 120074, 120077, 120084, 120086, 120092, 120094, 120121, 120123, 120126, 120128, 120132, 120134, 120134, 120138, 120144, 120146, 120485, 120488, 120512, 120514, 120538, 120540, 120570, 120572, 120596, 120598, 120628, 120630, 120654, 120656, 120686, 120688, 120712, 120714, 120744, 120746, 120770, 120772, 120779, 123136, 123180, 123191, 123197, 123214, 123214, 123584, 123627, 124928, 125124, 125184, 125251, 125259, 125259, 126464, 126467, 126469, 126495, 126497, 126498, 126500, 126500, 126503, 126503, 126505, 126514, 126516, 126519, 126521, 126521, 126523, 126523, 126530, 126530, 126535, 126535, 126537, 126537, 126539, 126539, 126541, 126543, 126545, 126546, 126548, 126548, 126551, 126551, 126553, 126553, 126555, 126555, 126557, 126557, 126559, 126559, 126561, 126562, 126564, 126564, 126567, 126570, 126572, 126578, 126580, 126583, 126585, 126588, 126590, 126590, 126592, 126601, 126603, 126619, 126625, 126627, 126629, 126633, 126635, 126651, 131072, 173782, 173824, 177972, 177984, 178205, 178208, 183969, 183984, 191456, 194560, 195101];
#[rustfmt::skip]
pub static UNICODE_ESNEXT_IDENTIFIER_PART: [u32; 1426] = [48, 57, 65, 90, 95, 95, 97, 122, 170, 170, 181, 181, 183, 183, 186, 186, 192, 214, 216, 246, 248, 705, 710, 721, 736, 740, 748, 748, 750, 750, 768, 884, 886, 887, 890, 893, 895, 895, 902, 906, 908, 908, 910, 929, 931, 1013, 1015, 1153, 1155, 1159, 1162, 1327, 1329, 1366, 1369, 1369, 1376, 1416, 1425, 1469, 1471, 1471, 1473, 1474, 1476, 1477, 1479, 1479, 1488, 1514, 1519, 1522, 1552, 1562, 1568, 1641, 1646, 1747, 1749, 1756, 1759, 1768, 1770, 1788, 1791, 1791, 1808, 1866, 1869, 1969, 1984, 2037, 2042, 2042, 2045, 2045, 2048, 2093, 2112, 2139, 2144, 2154, 2208, 2228, 2230, 2237, 2259, 2273, 2275, 2403, 2406, 2415, 2417, 2435, 2437, 2444, 2447, 2448, 2451, 2472, 2474, 2480, 2482, 2482, 2486, 2489, 2492, 2500, 2503, 2504, 2507, 2510, 2519, 2519, 2524, 2525, 2527, 2531, 2534, 2545, 2556, 2556, 2558, 2558, 2561, 2563, 2565, 2570, 2575, 2576, 2579, 2600, 2602, 2608, 2610, 2611, 2613, 2614, 2616, 2617, 2620, 2620, 2622, 2626, 2631, 2632, 2635, 2637, 2641, 2641, 2649, 2652, 2654, 2654, 2662, 2677, 2689, 2691, 2693, 2701, 2703, 2705, 2707, 2728, 2730, 2736, 2738, 2739, 2741, 2745, 2748, 2757, 2759, 2761, 2763, 2765, 2768, 2768, 2784, 2787, 2790, 2799, 2809, 2815, 2817, 2819, 2821, 2828, 2831, 2832, 2835, 2856, 2858, 2864, 2866, 2867, 2869, 2873, 2876, 2884, 2887, 2888, 2891, 2893, 2902, 2903, 2908, 2909, 2911, 2915, 2918, 2927, 2929, 2929, 2946, 2947, 2949, 2954, 2958, 2960, 2962, 2965, 2969, 2970, 2972, 2972, 2974, 2975, 2979, 2980, 2984, 2986, 2990, 3001, 3006, 3010, 3014, 3016, 3018, 3021, 3024, 3024, 3031, 3031, 3046, 3055, 3072, 3084, 3086, 3088, 3090, 3112, 3114, 3129, 3133, 3140, 3142, 3144, 3146, 3149, 3157, 3158, 3160, 3162, 3168, 3171, 3174, 3183, 3200, 3203, 3205, 3212, 3214, 3216, 3218, 3240, 3242, 3251, 3253, 3257, 3260, 3268, 3270, 3272, 3274, 3277, 3285, 3286, 3294, 3294, 3296, 3299, 3302, 3311, 3313, 3314, 3328, 3331, 3333, 3340, 3342, 3344, 3346, 3396, 3398, 3400, 3402, 3406, 3412, 3415, 3423, 3427, 3430, 3439, 3450, 3455, 3458, 3459, 3461, 3478, 3482, 3505, 3507, 3515, 3517, 3517, 3520, 3526, 3530, 3530, 3535, 3540, 3542, 3542, 3544, 3551, 3558, 3567, 3570, 3571, 3585, 3642, 3648, 3662, 3664, 3673, 3713, 3714, 3716, 3716, 3718, 3722, 3724, 3747, 3749, 3749, 3751, 3773, 3776, 3780, 3782, 3782, 3784, 3789, 3792, 3801, 3804, 3807, 3840, 3840, 3864, 3865, 3872, 3881, 3893, 3893, 3895, 3895, 3897, 3897, 3902, 3911, 3913, 3948, 3953, 3972, 3974, 3991, 3993, 4028, 4038, 4038, 4096, 4169, 4176, 4253, 4256, 4293, 4295, 4295, 4301, 4301, 4304, 4346, 4348, 4680, 4682, 4685, 4688, 4694, 4696, 4696, 4698, 4701, 4704, 4744, 4746, 4749, 4752, 4784, 4786, 4789, 4792, 4798, 4800, 4800, 4802, 4805, 4808, 4822, 4824, 4880, 4882, 4885, 4888, 4954, 4957, 4959, 4969, 4977, 4992, 5007, 5024, 5109, 5112, 5117, 5121, 5740, 5743, 5759, 5761, 5786, 5792, 5866, 5870, 5880, 5888, 5900, 5902, 5908, 5920, 5940, 5952, 5971, 5984, 5996, 5998, 6000, 6002, 6003, 6016, 6099, 6103, 6103, 6108, 6109, 6112, 6121, 6155, 6157, 6160, 6169, 6176, 6264, 6272, 6314, 6320, 6389, 6400, 6430, 6432, 6443, 6448, 6459, 6470, 6509, 6512, 6516, 6528, 6571, 6576, 6601, 6608, 6618, 6656, 6683, 6688, 6750, 6752, 6780, 6783, 6793, 6800, 6809, 6823, 6823, 6832, 6845, 6912, 6987, 6992, 7001, 7019, 7027, 7040, 7155, 7168, 7223, 7232, 7241, 7245, 7293, 7296, 7304, 7312, 7354, 7357, 7359, 7376, 7378, 7380, 7418, 7424, 7673, 7675, 7957, 7960, 7965, 7968, 8005, 8008, 8013, 8016, 8023, 8025, 8025, 8027, 8027, 8029, 8029, 8031, 8061, 8064, 8116, 8118, 8124, 8126, 8126, 8130, 8132, 8134, 8140, 8144, 8147, 8150, 8155, 8160, 8172, 8178, 8180, 8182, 8188, 8255, 8256, 8276, 8276, 8305, 8305, 8319, 8319, 8336, 8348, 8400, 8412, 8417, 8417, 8421, 8432, 8450, 8450, 8455, 8455, 8458, 8467, 8469, 8469, 8472, 8477, 8484, 8484, 8486, 8486, 8488, 8488, 8490, 8505, 8508, 8511, 8517, 8521, 8526, 8526, 8544, 8584, 11264, 11310, 11312, 11358, 11360, 11492, 11499, 11507, 11520, 11557, 11559, 11559, 11565, 11565, 11568, 11623, 11631, 11631, 11647, 11670, 11680, 11686, 11688, 11694, 11696, 11702, 11704, 11710, 11712, 11718, 11720, 11726, 11728, 11734, 11736, 11742, 11744, 11775, 12293, 12295, 12321, 12335, 12337, 12341, 12344, 12348, 12353, 12438, 12441, 12447, 12449, 12538, 12540, 12543, 12549, 12591, 12593, 12686, 12704, 12730, 12784, 12799, 13312, 19893, 19968, 40943, 40960, 42124, 42192, 42237, 42240, 42508, 42512, 42539, 42560, 42607, 42612, 42621, 42623, 42737, 42775, 42783, 42786, 42888, 42891, 42943, 42946, 42950, 42999, 43047, 43072, 43123, 43136, 43205, 43216, 43225, 43232, 43255, 43259, 43259, 43261, 43309, 43312, 43347, 43360, 43388, 43392, 43456, 43471, 43481, 43488, 43518, 43520, 43574, 43584, 43597, 43600, 43609, 43616, 43638, 43642, 43714, 43739, 43741, 43744, 43759, 43762, 43766, 43777, 43782, 43785, 43790, 43793, 43798, 43808, 43814, 43816, 43822, 43824, 43866, 43868, 43879, 43888, 44010, 44012, 44013, 44016, 44025, 44032, 55203, 55216, 55238, 55243, 55291, 63744, 64109, 64112, 64217, 64256, 64262, 64275, 64279, 64285, 64296, 64298, 64310, 64312, 64316, 64318, 64318, 64320, 64321, 64323, 64324, 64326, 64433, 64467, 64829, 64848, 64911, 64914, 64967, 65008, 65019, 65024, 65039, 65056, 65071, 65075, 65076, 65101, 65103, 65136, 65140, 65142, 65276, 65296, 65305, 65313, 65338, 65343, 65343, 65345, 65370, 65382, 65470, 65474, 65479, 65482, 65487, 65490, 65495, 65498, 65500, 65536, 65547, 65549, 65574, 65576, 65594, 65596, 65597, 65599, 65613, 65616, 65629, 65664, 65786, 65856, 65908, 66045, 66045, 66176, 66204, 66208, 66256, 66272, 66272, 66304, 66335, 66349, 66378, 66384, 66426, 66432, 66461, 66464, 66499, 66504, 66511, 66513, 66517, 66560, 66717, 66720, 66729, 66736, 66771, 66776, 66811, 66816, 66855, 66864, 66915, 67072, 67382, 67392, 67413, 67424, 67431, 67584, 67589, 67592, 67592, 67594, 67637, 67639, 67640, 67644, 67644, 67647, 67669, 67680, 67702, 67712, 67742, 67808, 67826, 67828, 67829, 67840, 67861, 67872, 67897, 67968, 68023, 68030, 68031, 68096, 68099, 68101, 68102, 68108, 68115, 68117, 68119, 68121, 68149, 68152, 68154, 68159, 68159, 68192, 68220, 68224, 68252, 68288, 68295, 68297, 68326, 68352, 68405, 68416, 68437, 68448, 68466, 68480, 68497, 68608, 68680, 68736, 68786, 68800, 68850, 68864, 68903, 68912, 68921, 69376, 69404, 69415, 69415, 69424, 69456, 69600, 69622, 69632, 69702, 69734, 69743, 69759, 69818, 69840, 69864, 69872, 69881, 69888, 69940, 69942, 69951, 69956, 69958, 69968, 70003, 70006, 70006, 70016, 70084, 70089, 70092, 70096, 70106, 70108, 70108, 70144, 70161, 70163, 70199, 70206, 70206, 70272, 70278, 70280, 70280, 70282, 70285, 70287, 70301, 70303, 70312, 70320, 70378, 70384, 70393, 70400, 70403, 70405, 70412, 70415, 70416, 70419, 70440, 70442, 70448, 70450, 70451, 70453, 70457, 70459, 70468, 70471, 70472, 70475, 70477, 70480, 70480, 70487, 70487, 70493, 70499, 70502, 70508, 70512, 70516, 70656, 70730, 70736, 70745, 70750, 70751, 70784, 70853, 70855, 70855, 70864, 70873, 71040, 71093, 71096, 71104, 71128, 71133, 71168, 71232, 71236, 71236, 71248, 71257, 71296, 71352, 71360, 71369, 71424, 71450, 71453, 71467, 71472, 71481, 71680, 71738, 71840, 71913, 71935, 71935, 72096, 72103, 72106, 72151, 72154, 72161, 72163, 72164, 72192, 72254, 72263, 72263, 72272, 72345, 72349, 72349, 72384, 72440, 72704, 72712, 72714, 72758, 72760, 72768, 72784, 72793, 72818, 72847, 72850, 72871, 72873, 72886, 72960, 72966, 72968, 72969, 72971, 73014, 73018, 73018, 73020, 73021, 73023, 73031, 73040, 73049, 73056, 73061, 73063, 73064, 73066, 73102, 73104, 73105, 73107, 73112, 73120, 73129, 73440, 73462, 73728, 74649, 74752, 74862, 74880, 75075, 77824, 78894, 82944, 83526, 92160, 92728, 92736, 92766, 92768, 92777, 92880, 92909, 92912, 92916, 92928, 92982, 92992, 92995, 93008, 93017, 93027, 93047, 93053, 93071, 93760, 93823, 93952, 94026, 94031, 94087, 94095, 94111, 94176, 94177, 94179, 94179, 94208, 100343, 100352, 101106, 110592, 110878, 110928, 110930, 110948, 110951, 110960, 111355, 113664, 113770, 113776, 113788, 113792, 113800, 113808, 113817, 113821, 113822, 119141, 119145, 119149, 119154, 119163, 119170, 119173, 119179, 119210, 119213, 119362, 119364, 119808, 119892, 119894, 119964, 119966, 119967, 119970, 119970, 119973, 119974, 119977, 119980, 119982, 119993, 119995, 119995, 119997, 120003, 120005, 120069, 120071, 120074, 120077, 120084, 120086, 120092, 120094, 120121, 120123, 120126, 120128, 120132, 120134, 120134, 120138, 120144, 120146, 120485, 120488, 120512, 120514, 120538, 120540, 120570, 120572, 120596, 120598, 120628, 120630, 120654, 120656, 120686, 120688, 120712, 120714, 120744, 120746, 120770, 120772, 120779, 120782, 120831, 121344, 121398, 121403, 121452, 121461, 121461, 121476, 121476, 121499, 121503, 121505, 121519, 122880, 122886, 122888, 122904, 122907, 122913, 122915, 122916, 122918, 122922, 123136, 123180, 123184, 123197, 123200, 123209, 123214, 123214, 123584, 123641, 124928, 125124, 125136, 125142, 125184, 125259, 125264, 125273, 126464, 126467, 126469, 126495, 126497, 126498, 126500, 126500, 126503, 126503, 126505, 126514, 126516, 126519, 126521, 126521, 126523, 126523, 126530, 126530, 126535, 126535, 126537, 126537, 126539, 126539, 126541, 126543, 126545, 126546, 126548, 126548, 126551, 126551, 126553, 126553, 126555, 126555, 126557, 126557, 126559, 126559, 126561, 126562, 126564, 126564, 126567, 126570, 126572, 126578, 126580, 126583, 126585, 126588, 126590, 126590, 126592, 126601, 126603, 126619, 126625, 126627, 126629, 126633, 126635, 126651, 131072, 173782, 173824, 177972, 177984, 178205, 178208, 183969, 183984, 191456, 194560, 195101, 917760, 917999];

fn lookup_in_unicode_map(code: u32, map: &[u32]) -> bool {
  // Bail out quickly if it couldn't possibly be in the map.
  if code < map[0] {
    return false;
  }

  // Perform binary search in one of the Unicode range maps
  let mut lo = 0;
  let mut hi = map.len();
  let mut mid;

  while lo + 1 < hi {
    mid = lo + (hi - lo) / 2;
    // mid has to be even to catch a range's beginning
    mid -= mid % 2;
    if map[mid] <= code && code <= map[mid + 1] {
      return true;
    }

    if code < map[mid] {
      hi = mid;
    } else {
      lo = mid + 2;
    }
  }

  false
}

/// True if `c` is valid as a first character of an identifier.
/// See [ECMA262](https://tc39.es/ecma262/#prod-IdentifierStart) for
/// a formal definition of valid identifier name.
#[inline]
pub fn is_id_start(c: char) -> bool {
  lookup_in_unicode_map(c as u32, &UNICODE_ESNEXT_IDENTIFIER_START)
}

/// True if `c` is valid as a non-first character of an identifier.
/// See [ECMA262](https://tc39.es/ecma262/#prod-IdentifierPart) for
/// a formal definition of valid identifier name.
#[inline]
pub fn is_id_part(c: char) -> bool {
  lookup_in_unicode_map(c as u32, &UNICODE_ESNEXT_IDENTIFIER_PART)
}

/// The passed string is lexically an identifier.
#[inline]
pub fn is_ident(string: &str) -> bool {
  let mut chars = string.chars();
  if let Some(start) = chars.next() {
    is_id_start(start) && chars.all(|c| is_id_part(c))
  } else {
    false
  }
}

impl Cursor<'_> {
  /// Parses a token from the input string.
  fn advance_token(&mut self) -> Token {
    let first_char = self.bump().unwrap();
    let token_kind = match first_char {
      // Slash, comment or block comment.
      '/' => match self.first() {
        '/' => self.line_comment(),
        '*' => self.block_comment(),
        _ => Slash,
      },

      // Whitespace sequence.
      c if is_whitespace(c) => self.whitespace(),

      c if is_line_break(c) => self.line_break(),

      // Identifier (this should be checked after other variant that can
      // start as identifier).
      c if is_id_start(c) => self.ident(),

      // Numeric literal.
      '0'..='9' => {
        let literal_kind = self.number();
        let suffix_start = self.len_consumed();
        self.eat_literal_suffix();
        TokenKind::Literal { kind: literal_kind, suffix_start }
      }

      c if c == '.' && self.first().is_digit(10) => {
        let literal_kind = self.number();
        let suffix_start = self.len_consumed();
        self.eat_literal_suffix();
        TokenKind::Literal { kind: literal_kind, suffix_start }
      }

      // String literal.
      '"' | '\'' | '`' => {
        let terminated = self.string();
        let suffix_start = self.len_consumed();
        if terminated {
          self.eat_literal_suffix();
        }
        let kind = Str { terminated };
        Literal { kind, suffix_start }
      }

      // One-symbol tokens.
      ';' => Semi,
      ',' => Comma,
      '.' => Dot,
      '(' => OpenParen,
      ')' => CloseParen,
      '{' => OpenBrace,
      '}' => CloseBrace,
      '[' => OpenBracket,
      ']' => CloseBracket,
      '@' => At,
      '#' => Pound,
      '~' => Tilde,
      '?' => Question,
      ':' => Colon,
      '$' => Dollar,
      '=' => Eq,
      '!' => Bang,
      '<' => Lt,
      '>' => Gt,
      '-' => Minus,
      '&' => And,
      '|' => Or,
      '+' => Plus,
      '*' => Star,
      '^' => Caret,
      '%' => Percent,
      _ => Unknown,
    };
    Token::new(token_kind, self.len_consumed())
  }

  fn line_comment(&mut self) -> TokenKind {
    debug_assert!(self.prev() == '/' && self.first() == '/');
    self.bump();
    self.eat_while(|c| c != '\n');
    LineComment
  }

  fn block_comment(&mut self) -> TokenKind {
    debug_assert!(self.prev() == '/' && self.first() == '*');
    self.bump();

    let mut depth = 1usize;
    while let Some(c) = self.bump() {
      match c {
        '/' if self.first() == '*' => {
          self.bump();
          depth += 1;
        }
        '*' if self.first() == '/' => {
          self.bump();
          depth -= 1;
          if depth == 0 {
            // This block comment is closed, so for a construction like "/* */ */"
            // there will be a successfully parsed block comment "/* */"
            // and " */" will be processed separately.
            break;
          }
        }
        _ => (),
      }
    }

    BlockComment { terminated: depth == 0 }
  }

  fn whitespace(&mut self) -> TokenKind {
    debug_assert!(is_whitespace(self.prev()));
    self.eat_while(is_whitespace);
    Whitespace
  }

  fn line_break(&mut self) -> TokenKind {
    debug_assert!(is_line_break(self.prev()));
    self.eat_while(is_line_break);
    LineBreak
  }

  fn ident(&mut self) -> TokenKind {
    debug_assert!(is_id_start(self.prev()));
    // Start is already eaten, eat the rest of identifier.
    self.eat_while(is_id_part);
    Ident
  }

  fn number(&mut self) -> LiteralKind {
    debug_assert!(
      self.prev() == '.' && self.first().is_digit(10) || ('0' <= self.prev() && self.prev() <= '9')
    );

    let mut base = Base::Decimal;
    let mut float = false;

    if self.prev() == '0' {
      // Attempt to parse encoding base.
      let has_digits = match self.first() {
        'b' | 'B' => {
          base = Base::Binary;
          self.bump();
          self.eat_decimal_digits()
        }
        'o' | 'O' => {
          base = Base::Octal;
          self.bump();
          self.eat_decimal_digits()
        }
        'x' | 'X' => {
          base = Base::Hexadecimal;
          self.bump();
          self.eat_hexadecimal_digits()
        }
        // Not a base prefix.
        '0'..='9' | '_' | '.' | 'e' | 'E' => {
          self.eat_decimal_digits();
          true
        }
        // Just a 0.
        _ => return Numeric { base, empty_int: false, empty_exponent: false },
      };
      // Base prefix was provided, but there were no digits
      // after it, e.g. "0x".
      if !has_digits {
        return Numeric { base, empty_int: true, empty_exponent: false };
      }
    } else if self.prev() == '.' {
      float = true;
      self.eat_decimal_digits();
    } else {
      // No base prefix, parse number in the usual way.
      self.eat_decimal_digits();
    };

    match self.first() {
      // Don't be greedy if this is actually an
      // integer literal followed by field/method access
      // (`12.1`, `12.foo()`)
      '.' if !float && self.second().is_digit(10) => {
        // might have stuff after the ., and if it does, it needs to start
        // with a number
        self.bump();
        let mut empty_exponent = false;
        self.eat_decimal_digits();
        match self.first() {
          'e' | 'E' => {
            self.bump();
            empty_exponent = !self.eat_float_exponent();
          }
          _ => (),
        }
        Numeric { base, empty_int: false, empty_exponent }
      }
      'e' | 'E' => {
        self.bump();
        let empty_exponent = !self.eat_float_exponent();
        Numeric { base, empty_int: false, empty_exponent }
      }
      _ => Numeric { base, empty_int: false, empty_exponent: false },
    }
  }

  /// Eats string and returns true
  /// if it is terminated.
  fn string(&mut self) -> bool {
    let quote = self.prev();
    debug_assert!(quote == '"' || quote == '\'');
    while let Some(c) = self.bump() {
      match c {
        c if c == quote => {
          return true;
        }
        '\\' if self.first() == '\\' || self.first() == quote => {
          self.bump();
        }
        _ => (),
      }
    }
    false
  }

  fn eat_decimal_digits(&mut self) -> bool {
    let mut has_digits = false;
    loop {
      match self.first() {
        '_' => {
          self.bump();
        }
        '0'..='9' => {
          has_digits = true;
          self.bump();
        }
        _ => break,
      }
    }
    has_digits
  }

  fn eat_hexadecimal_digits(&mut self) -> bool {
    let mut has_digits = false;
    loop {
      match self.first() {
        '_' => {
          self.bump();
        }
        '0'..='9' | 'a'..='f' | 'A'..='F' => {
          has_digits = true;
          self.bump();
        }
        _ => break,
      }
    }
    has_digits
  }

  /// Eats the float exponent. Returns true if at least one digit was met,
  /// and returns false otherwise.
  fn eat_float_exponent(&mut self) -> bool {
    debug_assert!(self.prev() == 'e' || self.prev() == 'E');
    if self.first() == '-' || self.first() == '+' {
      self.bump();
    }
    self.eat_decimal_digits()
  }

  fn eat_literal_suffix(&mut self) {
    self.eat_identifier();
  }

  // Eats the identifier.
  fn eat_identifier(&mut self) {
    if !is_id_start(self.first()) {
      return;
    }
    self.bump();

    self.eat_while(is_id_part);
  }

  /// Eats symbols while predicate returns true or until the end of file is reached.
  fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
    while predicate(self.first()) && !self.is_eof() {
      self.bump();
    }
  }
}
