use super::*;

use crate::SessionGlobals;

#[test]
fn interner_tests() {
  let mut i: Interner = Interner::default();
  // first one is zero:
  assert_eq!(i.intern("dog"), Symbol::new(0));
  // re-use gets the same entry:
  assert_eq!(i.intern("dog"), Symbol::new(0));
  // different string gets a different #:
  assert_eq!(i.intern("cat"), Symbol::new(1));
  assert_eq!(i.intern("cat"), Symbol::new(1));
  // dog is still at zero
  assert_eq!(i.intern("dog"), Symbol::new(0));
}

#[test]
fn without_first_quote_test() {
  SESSION_GLOBALS.set(&SessionGlobals::new(), || {
    let i = Ident::from_str("'break");
    assert_eq!(i.without_first_quote().name, kw::Break);
  });
}
