#[allow(non_snake_case)]
#[rustfmt::skip]
pub mod CharacterCodes {
  pub const NullCharacter: char = '\u{0}';
  pub const MaxAsciiCharacter: char = '\u{7F}';
  
  pub const LineSeparator: char = '\u{2028}';
  pub const ParagraphSeparator: char = '\u{2029}';
  pub const NextLine: char = '\u{0085}';

  // Unicode 3.0 space characters
  pub const Space: char = ' ';
  pub const NonBreakingSpace: char = '\u{00A0}';
  pub const EnQuad: char = '\u{2000}';
  pub const EmQuad: char = '\u{2001}';
  pub const EnSpace: char = '\u{2002}';
  pub const EmSpace: char = '\u{2003}';
  pub const ThreePerEmSpace: char = '\u{2004}';
  pub const FourPerEmSpace: char = '\u{2005}';
  pub const SixPerEmSpace: char = '\u{2006}';
  pub const FigureSpace: char = '\u{2007}';
  pub const PunctuationSpace: char = '\u{2008}';
  pub const ThinSpace: char = '\u{2009}';
  pub const HairSpace: char = '\u{200A}';
  pub const ZeroWidthSpace: char = '\u{200B}';
  pub const NarrowNoBreakSpace: char = '\u{202F}';
  pub const IdeographicSpace: char = '\u{3000}';
  pub const MathematicalSpace: char = '\u{205F}';
  pub const Ogham: char = '\u{1680}';

  pub const _0: char = '0';
  pub const _1: char = '1';
  pub const _2: char = '2';
  pub const _3: char = '3';
  pub const _4: char = '4';
  pub const _5: char = '5';
  pub const _6: char = '6';
  pub const _7: char = '7';
  pub const _8: char = '8';
  pub const _9: char = '9';

  pub const a: char = 'a';
  pub const b: char = 'b';
  pub const c: char = 'c';
  pub const d: char = 'd';
  pub const e: char = 'e';
  pub const f: char = 'f';
  pub const g: char = 'g';
  pub const h: char = 'h';
  pub const i: char = 'i';
  pub const j: char = 'j';
  pub const k: char = 'k';
  pub const l: char = 'l';
  pub const m: char = 'm';
  pub const n: char = 'n';
  pub const o: char = 'o';
  pub const p: char = 'p';
  pub const q: char = 'q';
  pub const r: char = 'r';
  pub const s: char = 's';
  pub const t: char = 't';
  pub const u: char = 'u';
  pub const v: char = 'v';
  pub const w: char = 'w';
  pub const x: char = 'x';
  pub const y: char = 'y';
  pub const z: char = 'z';

  pub const A: char = 'A';
  pub const B: char = 'B';
  pub const C: char = 'C';
  pub const D: char = 'D';
  pub const E: char = 'E';
  pub const F: char = 'F';
  pub const G: char = 'G';
  pub const H: char = 'H';
  pub const I: char = 'I';
  pub const J: char = 'J';
  pub const K: char = 'K';
  pub const L: char = 'L';
  pub const M: char = 'M';
  pub const N: char = 'N';
  pub const O: char = 'O';
  pub const P: char = 'P';
  pub const Q: char = 'Q';
  pub const R: char = 'R';
  pub const S: char = 'S';
  pub const T: char = 'T';
  pub const U: char = 'U';
  pub const V: char = 'V';
  pub const W: char = 'W';
  pub const X: char = 'X';
  pub const Y: char = 'Y';
  pub const Z: char = 'Z';

  pub const LineFeed: char = '\n';
  pub const CarriageReturn: char = '\r';
  pub const UnderScore: char = '\u{5F}';   // _
  pub const Dollar: char = '\u{24}';       // $
  pub const Ampersand: char = '\u{26}';    // &
  pub const Asterisk: char = '\u{2A}';     // *
  pub const At: char = '\u{40}';           // @
  pub const Backslash: char = '\u{5C}';    // \
  pub const Backtick: char = '\u{60}';     // `
  pub const Bar: char = '\u{7C}';          // |
  pub const Caret: char = '\u{5E}';        // ^
  pub const CloseBrace: char = '\u{7D}';   // }
  pub const CloseBracket: char = '\u{5D}'; // ]
  pub const CloseParen: char = '\u{29}';   // )
  pub const Colon: char = '\u{3A}';        // :
  pub const Comma: char = '\u{2C}';        // ,
  pub const Dot: char = '\u{2E}';          // .
  pub const DoubleQuote: char = '\u{22}';  // "
  pub const Equals: char = '\u{3D}';       // =
  pub const Exclamation: char = '\u{21}';  // !
  pub const GreaterThan: char = '\u{3E}';  // >
  pub const Hash: char = '\u{23}';         // #
  pub const LessThan: char = '\u{3C}';     // <
  pub const Minus: char = '\u{2D}';        // -
  pub const OpenBrace: char = '\u{7B}';    // {
  pub const OpenBracket: char = '\u{5B}';  // [
  pub const OpenParen: char = '\u{28}';    // (
  pub const Percent: char = '\u{25}';      // %
  pub const Plus: char = '\u{2B}';         // +
  pub const Question: char = '\u{3F}';     // ?
  pub const Semicolon: char = '\u{3B}';    // ;
  pub const SingleQuote: char = '\u{27}';  // '
  pub const Slash: char = '\u{2F}';        // /
  pub const Tilde: char = '\u{7E}';        // ~

  pub const Backspace: char = '\u{08}';    // \b
  pub const FormFeed: char = '\u{0C}';     // \f
  pub const Tab: char = '\u{09}';          // \t
  pub const VerticalTab: char = '\u{0B}';  // \v

  pub const ByteOrderMark: char = '\u{FEFF}';
}
