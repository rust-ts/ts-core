pub const NONE: u8 = 0;
/** An element from a named property of the JSX.IntrinsicElements interface */
pub const IntrinsicNamedElement: u8 = 1 << 0;
/** An element inferred from the string index signature of the JSX.IntrinsicElements interface */
pub const IntrinsicIndexedElement: u8 = 1 << 1;

pub const IntrinsicElement: u8 = IntrinsicNamedElement | IntrinsicIndexedElement;
