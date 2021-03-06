pub use BinOpToken::*;
pub use DelimToken::*;
pub use LitKind::*;
pub use TokenKind::*;

use crate::ast;
use crate::ptr::P;
use crate::tokenstream::TokenTree;

use rustc_data_structures::stable_hasher::{HashStable, StableHasher};
use rustc_data_structures::sync::Lrc;
use rustc_macros::HashStable_Generic;
use std::borrow::Cow;
use std::{fmt, mem};
use tscore_span::source_map::SourceMap;
use tscore_span::symbol::{kw, sym};
use tscore_span::symbol::{Ident, Symbol};
use tscore_span::{self, FileName, RealFileName, Span, DUMMY_SP};

#[derive(Clone, PartialEq, Encodable, Decodable, Debug, HashStable_Generic)]
pub struct Token {
  pub kind: TokenKind,
  pub span: Span,
}

// `TokenKind` is used a lot. Make sure it doesn't unintentionally get bigger.
#[cfg(target_arch = "x86_64")]
rustc_data_structures::static_assert_size!(TokenKind, 16);

#[derive(Clone, PartialEq, Encodable, Decodable, Debug, HashStable_Generic)]
pub enum TokenKind {
  Unknown,
  Eof,
  /// A doc comment token.
  /// `Symbol` is the doc comment's data excluding its "quotes" (`///`, `/**`, etc)
  /// similarly to symbols in string literal tokens.
  DocComment(CommentKind),
  // Punctuation
  OpenDelim(DelimToken),
  CloseDelim(DelimToken),
  Dot,
  DotDotDot,
  Semi,
  Comma,
  QuestionDot,
  Lt,
  /// `</`
  LtSlash,
  Le,
  Gt,
  Ge,
  Eq,
  EqEq,
  EqEqEq,
  NotEq,
  NotEqEq,
  /// `=>`
  Arrow,
  BinOp(BinOpToken),
  BinOpEq(BinOpToken),
  // unary
  Not,
  Tilde,
  PlusPlus,
  MinusMinus,

  Colon,
  At,
  Question,
  /// Only the JSDoc scanner produces Backtick.
  /// The normal scanner produces NoSubstitutionTemplateLiteral and related kinds.
  Backtick,
  /* Literals */
  Literal(Lit),

  /// Identifier token.
  /// Do not forget about `NtIdent` when you want to match on identifiers.
  /// It's recommended to use `Token::(ident,uninterpolate,uninterpolated_span)` to
  /// treat regular and interpolated identifiers in the same way.
  Ident(Symbol, /* is_raw */ bool),
}

#[derive(Clone, Copy, PartialEq, Encodable, Decodable, Debug, HashStable_Generic)]
pub enum CommentKind {
  Line,
  Block,
}

#[derive(Clone, PartialEq, Encodable, Decodable, Hash, Debug, Copy, HashStable_Generic)]
pub enum BinOpToken {
  Plus,
  Minus,
  Star,
  StarStar,
  Slash,
  Percent,
  /// `<<`
  Shl,
  /// `>>`
  Shr,
  /// `>>>`
  Ushr,
  /// `&`
  And,
  /// `|`
  Or,
  /// `^`
  Caret,
  AndAnd,
  OrOr,
  QuestionQuestion,
}

/// A delimiter token.
#[derive(Clone, PartialEq, Eq, Encodable, Decodable, Hash, Debug, Copy, HashStable_Generic)]
pub enum DelimToken {
  /// A round parenthesis (i.e., `(` or `)`).
  Paren,
  /// A square bracket (i.e., `[` or `]`).
  Bracket,
  /// A curly brace (i.e., `{` or `}`).
  Brace,
  /// An empty delimiter.
  NoDelim,
}

#[derive(Clone, Copy, PartialEq, Encodable, Decodable, Debug, HashStable_Generic)]
pub enum QuoteKind {
  Single,
  Double,
}

#[derive(Clone, Copy, PartialEq, Encodable, Decodable, Debug, HashStable_Generic)]
pub enum LitKind {
  Bool, // AST only, must never appear in a `Token`
  Num,
  Str(QuoteKind),
  BigInt,
  RegExp,
  Template,
}

/// A literal token.
#[derive(Clone, Copy, PartialEq, Encodable, Decodable, Debug, HashStable_Generic)]
pub struct Lit {
  pub kind: LitKind,
  pub symbol: Symbol,
  pub suffix: Option<Symbol>,
}

impl fmt::Display for Lit {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let Lit { kind, symbol, suffix } = *self;
    match kind {
      Str(quote_kind) => {
        if quote_kind == QuoteKind::Double {
          write!(f, "\"{}\"", symbol)?
        } else {
          write!(f, "'{}'", symbol)?
        }
      }
      _ => write!(f, "{}", symbol)?,
    }

    if let Some(suffix) = suffix {
      write!(f, "{}", suffix)?;
    }

    Ok(())
  }
}

impl LitKind {
  /// An English article for the literal token kind.
  /// Cause none of the the literal kind starts with vowel,
  /// so here we just return an 'a'.
  pub fn article(self) -> &'static str {
    "a"
  }

  pub fn descr(self) -> &'static str {
    match self {
      Bool => panic!("literal token contains `Lit::Bool`"),
      Num => "number",
      Str(_) => "string",
      BigInt => "BigInt",
      RegExp => "RegExp",
      Template => "template string",
    }
  }

  crate fn may_have_suffix(self) -> bool {
    matches!(self, BigInt)
  }
}

impl Lit {
  pub fn new(kind: LitKind, symbol: Symbol, suffix: Option<Symbol>) -> Lit {
    Lit { kind, symbol, suffix }
  }
}

impl Token {
  pub fn new(kind: TokenKind, span: Span) -> Self {
    Token { kind, span }
  }

  /// Some token that will be thrown away later.
  pub fn dummy() -> Self {
    Token::new(TokenKind::Question, DUMMY_SP)
  }

  /// Recovers a `Token` from an `Ident`. This creates a raw identifier if necessary.
  pub fn from_ast_ident(ident: Ident) -> Self {
    Token::new(Ident(ident.name, ident.is_raw_guess()), ident.span)
  }

  /// Return this token by value and leave a dummy token in its place.
  pub fn take(&mut self) -> Self {
    mem::replace(self, Token::dummy())
  }

  pub fn is_op(&self) -> bool {
    todo!()
  }

  pub fn is_like_plus(&self) -> bool {
    todo!()
  }

  pub fn can_begin_expr(&self) -> bool {
    todo!()
  }

  pub fn can_begin_type(&self) -> bool {
    todo!()
  }

  pub fn is_lit(&self) -> bool {
    matches!(self.kind, Literal(..))
  }

  /// Returns an identifier if this token is an identifier.
  pub fn ident(&self) -> Option<(Ident, /* is_raw */ bool)> {
    match self.kind {
      Ident(name, is_raw) => Some((Ident::new(name, self.span), is_raw)),
      _ => None,
    }
  }

  /// Returns `true` if the token is an identifier.
  pub fn is_ident(&self) -> bool {
    self.ident().is_some()
  }

  /// Returns `true` if the token is a identifier whose name is the given
  /// string slice.
  pub fn is_ident_named(&self, name: Symbol) -> bool {
    self.ident().map_or(false, |(ident, _)| ident.name == name)
  }

  /// Returns `true` if the token is either a special identifier or a keyword.
  pub fn is_reserved_ident(&self) -> bool {
    self.is_non_raw_ident_where(Ident::is_reserved)
  }

  /// Returns `true` if the token is a non-raw identifier for which `pred` holds.
  pub fn is_non_raw_ident_where(&self, pred: impl FnOnce(Ident) -> bool) -> bool {
    match self.ident() {
      Some((id, false)) => pred(id),
      _ => false,
    }
  }

  pub fn glue(&self, joint: &Token) -> Option<Token> {
    let kind = match self.kind {
      Eq => match joint.kind {
        Eq => EqEq,
        Gt => Arrow,
        EqEq => EqEqEq,
        _ => return None,
      },
      EqEq => match joint.kind {
        Eq => EqEqEq,
        _ => return None,
      },
      Lt => match joint.kind {
        Eq => Le,
        Lt => BinOp(Shl),
        Le => BinOpEq(Shl),
        BinOp(Slash) => Arrow,
        _ => return None,
      },
      Gt => match joint.kind {
        Eq => Ge,
        Gt => BinOp(Shr),
        Ge => BinOpEq(Shr),
        BinOp(Shr) => BinOp(Ushr),
        BinOpEq(Shr) => BinOpEq(Ushr),
        _ => return None,
      },
      Not => match joint.kind {
        Eq => NotEq,
        EqEq => NotEqEq,
        _ => return None,
      },
      NotEq => match joint.kind {
        Eq => NotEqEq,
        _ => return None,
      },
      BinOp(op) => match joint.kind {
        Eq => BinOpEq(op),
        BinOp(Plus) if op == Plus => PlusPlus,
        BinOp(Minus) if op == Minus => MinusMinus,
        BinOp(And) if op == And => BinOp(AndAnd),
        BinOp(Or) if op == Or => BinOp(OrOr),
        BinOp(Star) if op == Star => BinOp(StarStar),
        BinOpEq(And) if op == And => BinOpEq(AndAnd),
        BinOpEq(Or) if op == Or => BinOpEq(OrOr),
        _ => return None,
      },
      Question => match joint.kind {
        Dot => QuestionDot,
        Question => BinOp(QuestionQuestion),
        _ => return None,
      },
      _ => return None,
    };

    Some(Token::new(kind, self.span.to(joint.span)))
  }
}

impl PartialEq<TokenKind> for Token {
  fn eq(&self, rhs: &TokenKind) -> bool {
    self.kind == *rhs
  }
}
