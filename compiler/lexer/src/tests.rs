use super::*;

use expect_test::{expect, Expect};

#[test]
fn test_valid_shebang() {
  let input = "#!/usr/bin/node\nlet x = 5;";
  assert_eq!(strip_shebang(input), Some(15));
}

#[test]
fn test_shebang_second_line() {
  // Because shebangs are interpreted by the kernel, they must be on the first line
  let input = "\n#!/bin/bash";
  assert_eq!(strip_shebang(input), None);
}

#[test]
fn test_shebang_space() {
  let input = "#!    /bin/bash";
  assert_eq!(strip_shebang(input), Some(input.len()));
}

#[test]
fn test_shebang_empty_shebang() {
  let input = "#!    \n[attribute(foo)]";
  assert_eq!(strip_shebang(input), None);
}

#[test]
fn test_invalid_shebang_comment() {
  let input = "#!//bin/ami/a/comment\n[";
  assert_eq!(strip_shebang(input), None)
}

#[test]
fn test_invalid_shebang_another_comment() {
  let input = "#!/*bin/ami/a/comment*/\n[attribute";
  assert_eq!(strip_shebang(input), None)
}

#[test]
fn test_shebang_valid_rust_after() {
  let input = "#!/*bin/ami/a/comment*/\npub fn main() {}";
  assert_eq!(strip_shebang(input), Some(23))
}

#[test]
fn test_shebang_followed_by_attrib() {
  let input = "#!/bin/rust-scripts\n#![allow_unused(true)]";
  assert_eq!(strip_shebang(input), Some(19));
}

fn check_lexing(src: &str, expect: Expect) {
  let actual: String = tokenize(src).map(|token| format!("{:?}\n", token)).collect();
  expect.assert_eq(&actual)
}

#[test]
fn smoke_test() {
  check_lexing(
    "/* my source file */ function main() { console.log(\"hello world.\"); }\n",
    expect![[r#"
            Token { kind: BlockComment { terminated: true }, len: 20 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 8 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 4 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: OpenBrace, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Ident, len: 7 }
            Token { kind: Dot, len: 1 }
            Token { kind: Ident, len: 3 }
            Token { kind: OpenParen, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true }, suffix_start: 14 }, len: 14 }
            Token { kind: CloseParen, len: 1 }
            Token { kind: Semi, len: 1 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: CloseBrace, len: 1 }
            Token { kind: LineBreak, len: 1 }
        "#]],
  )
}

#[test]
fn comment_flavors() {
  check_lexing(
    r"
// line
//// line as well
/* block */
/**/
/*** also block */
/** outer doc block */
",
    expect![[r#"
            Token { kind: LineBreak, len: 1 }
            Token { kind: LineComment, len: 7 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: LineComment, len: 17 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: BlockComment { terminated: true }, len: 11 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: BlockComment { terminated: true }, len: 4 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: BlockComment { terminated: true }, len: 18 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: BlockComment { terminated: true }, len: 22 }
            Token { kind: LineBreak, len: 1 }
        "#]],
  )
}

#[test]
fn nested_block_comments() {
  check_lexing(
    "/* /* */ */'a'",
    expect![[r#"
            Token { kind: BlockComment { terminated: true }, len: 11 }
            Token { kind: Literal { kind: Str { terminated: true }, suffix_start: 3 }, len: 3 }
        "#]],
  )
}

#[test]
fn characters() {
  check_lexing(
    "'a' ' ' '\\n'",
    expect![[r#"
            Token { kind: Literal { kind: Str { terminated: true }, suffix_start: 3 }, len: 3 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true }, suffix_start: 3 }, len: 3 }
            Token { kind: Whitespace, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true }, suffix_start: 4 }, len: 4 }
        "#]],
  );
}

#[test]
fn literals() {
  check_lexing(
    r####"
'a'
"a"
1234
0b101
0B101
0x
0xABC
0X1AB
0888
0777
0o77
0O77
.123
0.123
2e
2e2
2e-2
0n
123n
0o123n
0b123n
0x123n
0123n
"####,
    expect![[r#"
            Token { kind: LineBreak, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true }, suffix_start: 3 }, len: 3 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: Literal { kind: Str { terminated: true }, suffix_start: 3 }, len: 3 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: Literal { kind: Numberic { base: Decimal, empty_int: false, empty_exponent: false }, suffix_start: 4 }, len: 4 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: Literal { kind: Numberic { base: Binary, empty_int: false, empty_exponent: false }, suffix_start: 5 }, len: 5 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: Literal { kind: Numberic { base: Binary, empty_int: false, empty_exponent: false }, suffix_start: 5 }, len: 5 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: Literal { kind: Numberic { base: Hexadecimal, empty_int: true, empty_exponent: false }, suffix_start: 2 }, len: 2 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: Literal { kind: Numberic { base: Hexadecimal, empty_int: false, empty_exponent: false }, suffix_start: 5 }, len: 5 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: Literal { kind: Numberic { base: Hexadecimal, empty_int: false, empty_exponent: false }, suffix_start: 5 }, len: 5 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: Literal { kind: Numberic { base: Decimal, empty_int: false, empty_exponent: false }, suffix_start: 4 }, len: 4 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: Literal { kind: Numberic { base: Decimal, empty_int: false, empty_exponent: false }, suffix_start: 4 }, len: 4 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: Literal { kind: Numberic { base: Octal, empty_int: false, empty_exponent: false }, suffix_start: 4 }, len: 4 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: Literal { kind: Numberic { base: Octal, empty_int: false, empty_exponent: false }, suffix_start: 4 }, len: 4 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: Literal { kind: Numberic { base: Decimal, empty_int: false, empty_exponent: false }, suffix_start: 4 }, len: 4 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: Literal { kind: Numberic { base: Decimal, empty_int: false, empty_exponent: false }, suffix_start: 5 }, len: 5 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: Literal { kind: Numberic { base: Decimal, empty_int: false, empty_exponent: true }, suffix_start: 2 }, len: 2 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: Literal { kind: Numberic { base: Decimal, empty_int: false, empty_exponent: false }, suffix_start: 3 }, len: 3 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: Literal { kind: Numberic { base: Decimal, empty_int: false, empty_exponent: false }, suffix_start: 4 }, len: 4 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: Literal { kind: Numberic { base: Decimal, empty_int: false, empty_exponent: false }, suffix_start: 1 }, len: 2 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: Literal { kind: Numberic { base: Decimal, empty_int: false, empty_exponent: false }, suffix_start: 3 }, len: 4 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: Literal { kind: Numberic { base: Octal, empty_int: false, empty_exponent: false }, suffix_start: 5 }, len: 6 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: Literal { kind: Numberic { base: Binary, empty_int: false, empty_exponent: false }, suffix_start: 5 }, len: 6 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: Literal { kind: Numberic { base: Hexadecimal, empty_int: false, empty_exponent: false }, suffix_start: 5 }, len: 6 }
            Token { kind: LineBreak, len: 1 }
            Token { kind: Literal { kind: Numberic { base: Decimal, empty_int: false, empty_exponent: false }, suffix_start: 4 }, len: 5 }
            Token { kind: LineBreak, len: 1 }
        "#]],
  )
}
