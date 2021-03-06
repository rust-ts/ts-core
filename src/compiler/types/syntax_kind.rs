pub const Unknown: u16 = 0;
pub const EndOfFileToken: u16 = 1;
pub const SingleLineCommentTrivia: u16 = 2;
pub const MultiLineCommentTrivia: u16 = 3;
pub const NewLineTrivia: u16 = 4;
pub const WhitespaceTrivia: u16 = 5;
// We detect and preserve #! on the first line
pub const ShebangTrivia: u16 = 6;
// We detect and provide better error recovery when we encounter a git merge marker.  This
// allows us to edit files with git-conflict markers in them in a much more pleasant manner.
pub const ConflictMarkerTrivia: u16 = 7;
// Literals
pub const NumericLiteral: u16 = 8;
pub const BigIntLiteral: u16 = 9;
pub const StringLiteral: u16 = 10;
pub const JsxText: u16 = 11;
pub const JsxTextAllWhiteSpaces: u16 = 12;
pub const RegularExpressionLiteral: u16 = 13;
pub const NoSubstitutionTemplateLiteral: u16 = 14;
// Pseudo-literals
pub const TemplateHead: u16 = 15;
pub const TemplateMiddle: u16 = 16;
pub const TemplateTail: u16 = 17;
// Punctuation
pub const OpenBraceToken: u16 = 18;
pub const CloseBraceToken: u16 = 19;
pub const OpenParenToken: u16 = 20;
pub const CloseParenToken: u16 = 21;
pub const OpenBracketToken: u16 = 22;
pub const CloseBracketToken: u16 = 23;
pub const DotToken: u16 = 24;
pub const DotDotDotToken: u16 = 25;
pub const SemicolonToken: u16 = 26;
pub const CommaToken: u16 = 27;
pub const QuestionDotToken: u16 = 28;
pub const LessThanToken: u16 = 29;
pub const LessThanSlashToken: u16 = 30;
pub const GreaterThanToken: u16 = 31;
pub const LessThanEqualsToken: u16 = 32;
pub const GreaterThanEqualsToken: u16 = 33;
pub const EqualsEqualsToken: u16 = 34;
pub const ExclamationEqualsToken: u16 = 35;
pub const EqualsEqualsEqualsToken: u16 = 36;
pub const ExclamationEqualsEqualsToken: u16 = 37;
pub const EqualsGreaterThanToken: u16 = 38;
pub const PlusToken: u16 = 39;
pub const MinusToken: u16 = 40;
pub const AsteriskToken: u16 = 41;
pub const AsteriskAsteriskToken: u16 = 42;
pub const SlashToken: u16 = 43;
pub const PercentToken: u16 = 44;
pub const PlusPlusToken: u16 = 45;
pub const MinusMinusToken: u16 = 46;
pub const LessThanLessThanToken: u16 = 47;
pub const GreaterThanGreaterThanToken: u16 = 48;
pub const GreaterThanGreaterThanGreaterThanToken: u16 = 49;
pub const AmpersandToken: u16 = 50;
pub const BarToken: u16 = 51;
pub const CaretToken: u16 = 52;
pub const ExclamationToken: u16 = 53;
pub const TildeToken: u16 = 54;
pub const AmpersandAmpersandToken: u16 = 55;
pub const BarBarToken: u16 = 56;
pub const QuestionToken: u16 = 57;
pub const ColonToken: u16 = 58;
pub const AtToken: u16 = 59;
pub const QuestionQuestionToken: u16 = 60;
/** Only the JSDoc scanner produces BacktickToken. The normal scanner produces NoSubstitutionTemplateLiteral and related kinds. */
pub const BacktickToken: u16 = 61;
// Assignments
pub const EqualsToken: u16 = 62;
pub const PlusEqualsToken: u16 = 63;
pub const MinusEqualsToken: u16 = 64;
pub const AsteriskEqualsToken: u16 = 65;
pub const AsteriskAsteriskEqualsToken: u16 = 66;
pub const SlashEqualsToken: u16 = 67;
pub const PercentEqualsToken: u16 = 68;
pub const LessThanLessThanEqualsToken: u16 = 69;
pub const GreaterThanGreaterThanEqualsToken: u16 = 70;
pub const GreaterThanGreaterThanGreaterThanEqualsToken: u16 = 71;
pub const AmpersandEqualsToken: u16 = 72;
pub const BarEqualsToken: u16 = 73;
pub const BarBarEqualsToken: u16 = 74;
pub const AmpersandAmpersandEqualsToken: u16 = 75;
pub const QuestionQuestionEqualsToken: u16 = 76;
pub const CaretEqualsToken: u16 = 77;
// Identifiers and PrivateIdentifiers
pub const Identifier: u16 = 78;
pub const PrivateIdentifier: u16 = 79;
// Reserved words
pub const BreakKeyword: u16 = 80;
pub const CaseKeyword: u16 = 81;
pub const CatchKeyword: u16 = 82;
pub const ClassKeyword: u16 = 83;
pub const ConstKeyword: u16 = 84;
pub const ContinueKeyword: u16 = 85;
pub const DebuggerKeyword: u16 = 86;
pub const DefaultKeyword: u16 = 87;
pub const DeleteKeyword: u16 = 88;
pub const DoKeyword: u16 = 89;
pub const ElseKeyword: u16 = 90;
pub const EnumKeyword: u16 = 91;
pub const ExportKeyword: u16 = 92;
pub const ExtendsKeyword: u16 = 93;
pub const FalseKeyword: u16 = 94;
pub const FinallyKeyword: u16 = 95;
pub const ForKeyword: u16 = 96;
pub const FunctionKeyword: u16 = 97;
pub const IfKeyword: u16 = 98;
pub const ImportKeyword: u16 = 99;
pub const InKeyword: u16 = 100;
pub const InstanceOfKeyword: u16 = 101;
pub const NewKeyword: u16 = 102;
pub const NullKeyword: u16 = 103;
pub const ReturnKeyword: u16 = 104;
pub const SuperKeyword: u16 = 105;
pub const SwitchKeyword: u16 = 106;
pub const ThisKeyword: u16 = 107;
pub const ThrowKeyword: u16 = 108;
pub const TrueKeyword: u16 = 109;
pub const TryKeyword: u16 = 110;
pub const TypeOfKeyword: u16 = 111;
pub const VarKeyword: u16 = 112;
pub const VoidKeyword: u16 = 113;
pub const WhileKeyword: u16 = 114;
pub const WithKeyword: u16 = 115;
// Strict mode reserved words
pub const ImplementsKeyword: u16 = 116;
pub const InterfaceKeyword: u16 = 117;
pub const LetKeyword: u16 = 118;
pub const PackageKeyword: u16 = 119;
pub const PrivateKeyword: u16 = 120;
pub const ProtectedKeyword: u16 = 121;
pub const PublicKeyword: u16 = 122;
pub const StaticKeyword: u16 = 123;
pub const YieldKeyword: u16 = 124;
// Contextual keywords
pub const AbstractKeyword: u16 = 125;
pub const AsKeyword: u16 = 126;
pub const AssertsKeyword: u16 = 127;
pub const AnyKeyword: u16 = 128;
pub const AsyncKeyword: u16 = 129;
pub const AwaitKeyword: u16 = 130;
pub const BooleanKeyword: u16 = 131;
pub const ConstructorKeyword: u16 = 132;
pub const DeclareKeyword: u16 = 133;
pub const GetKeyword: u16 = 134;
pub const InferKeyword: u16 = 135;
pub const IsKeyword: u16 = 136;
pub const KeyOfKeyword: u16 = 137;
pub const ModuleKeyword: u16 = 138;
pub const NamespaceKeyword: u16 = 139;
pub const NeverKeyword: u16 = 140;
pub const ReadonlyKeyword: u16 = 141;
pub const RequireKeyword: u16 = 142;
pub const NumberKeyword: u16 = 143;
pub const ObjectKeyword: u16 = 144;
pub const SetKeyword: u16 = 145;
pub const StringKeyword: u16 = 146;
pub const SymbolKeyword: u16 = 147;
pub const TypeKeyword: u16 = 148;
pub const UndefinedKeyword: u16 = 149;
pub const UniqueKeyword: u16 = 150;
pub const UnknownKeyword: u16 = 151;
pub const FromKeyword: u16 = 152;
pub const GlobalKeyword: u16 = 153;
pub const BigIntKeyword: u16 = 154;
pub const OfKeyword: u16 = 155; // LastKeyword and LastToken and LastContextualKeyword

// Parse tree nodes

// Names
pub const QualifiedName: u16 = 156;
pub const ComputedPropertyName: u16 = 157;
// Signature elements
pub const TypeParameter: u16 = 158;
pub const Parameter: u16 = 159;
pub const Decorator: u16 = 160;
// TypeMember
pub const PropertySignature: u16 = 161;
pub const PropertyDeclaration: u16 = 162;
pub const MethodSignature: u16 = 163;
pub const MethodDeclaration: u16 = 164;
pub const Constructor: u16 = 165;
pub const GetAccessor: u16 = 166;
pub const SetAccessor: u16 = 167;
pub const CallSignature: u16 = 168;
pub const ConstructSignature: u16 = 169;
pub const IndexSignature: u16 = 170;
// Type
pub const TypePredicate: u16 = 171;
pub const TypeReference: u16 = 172;
pub const FunctionType: u16 = 173;
pub const ConstructorType: u16 = 174;
pub const TypeQuery: u16 = 175;
pub const TypeLiteral: u16 = 176;
pub const ArrayType: u16 = 177;
pub const TupleType: u16 = 178;
pub const OptionalType: u16 = 179;
pub const RestType: u16 = 180;
pub const UnionType: u16 = 181;
pub const IntersectionType: u16 = 182;
pub const ConditionalType: u16 = 183;
pub const InferType: u16 = 184;
pub const ParenthesizedType: u16 = 185;
pub const ThisType: u16 = 186;
pub const TypeOperator: u16 = 187;
pub const IndexedAccessType: u16 = 188;
pub const MappedType: u16 = 189;
pub const LiteralType: u16 = 190;
pub const NamedTupleMember: u16 = 191;
pub const ImportType: u16 = 192;
// Binding patterns
pub const ObjectBindingPattern: u16 = 193;
pub const ArrayBindingPattern: u16 = 194;
pub const BindingElement: u16 = 195;
// Expression
pub const ArrayLiteralExpression: u16 = 196;
pub const ObjectLiteralExpression: u16 = 197;
pub const PropertyAccessExpression: u16 = 198;
pub const ElementAccessExpression: u16 = 199;
pub const CallExpression: u16 = 200;
pub const NewExpression: u16 = 201;
pub const TaggedTemplateExpression: u16 = 202;
pub const TypeAssertionExpression: u16 = 203;
pub const ParenthesizedExpression: u16 = 204;
pub const FunctionExpression: u16 = 205;
pub const ArrowFunction: u16 = 206;
pub const DeleteExpression: u16 = 207;
pub const TypeOfExpression: u16 = 208;
pub const VoidExpression: u16 = 209;
pub const AwaitExpression: u16 = 210;
pub const PrefixUnaryExpression: u16 = 211;
pub const PostfixUnaryExpression: u16 = 212;
pub const BinaryExpression: u16 = 213;
pub const ConditionalExpression: u16 = 214;
pub const TemplateExpression: u16 = 215;
pub const YieldExpression: u16 = 216;
pub const SpreadElement: u16 = 217;
pub const ClassExpression: u16 = 218;
pub const OmittedExpression: u16 = 219;
pub const ExpressionWithTypeArguments: u16 = 220;
pub const AsExpression: u16 = 221;
pub const NonNullExpression: u16 = 222;
pub const MetaProperty: u16 = 223;
pub const SyntheticExpression: u16 = 224;

// Misc
pub const TemplateSpan: u16 = 225;
pub const SemicolonClassElement: u16 = 226;
// Element
pub const Block: u16 = 227;
pub const EmptyStatement: u16 = 228;
pub const VariableStatement: u16 = 229;
pub const ExpressionStatement: u16 = 230;
pub const IfStatement: u16 = 231;
pub const DoStatement: u16 = 232;
pub const WhileStatement: u16 = 233;
pub const ForStatement: u16 = 234;
pub const ForInStatement: u16 = 235;
pub const ForOfStatement: u16 = 236;
pub const ContinueStatement: u16 = 237;
pub const BreakStatement: u16 = 238;
pub const ReturnStatement: u16 = 239;
pub const WithStatement: u16 = 240;
pub const SwitchStatement: u16 = 241;
pub const LabeledStatement: u16 = 242;
pub const ThrowStatement: u16 = 243;
pub const TryStatement: u16 = 244;
pub const DebuggerStatement: u16 = 245;
pub const VariableDeclaration: u16 = 246;
pub const VariableDeclarationList: u16 = 247;
pub const FunctionDeclaration: u16 = 248;
pub const ClassDeclaration: u16 = 249;
pub const InterfaceDeclaration: u16 = 250;
pub const TypeAliasDeclaration: u16 = 251;
pub const EnumDeclaration: u16 = 252;
pub const ModuleDeclaration: u16 = 253;
pub const ModuleBlock: u16 = 254;
pub const CaseBlock: u16 = 255;
pub const NamespaceExportDeclaration: u16 = 256;
pub const ImportEqualsDeclaration: u16 = 257;
pub const ImportDeclaration: u16 = 258;
pub const ImportClause: u16 = 259;
pub const NamespaceImport: u16 = 260;
pub const NamedImports: u16 = 261;
pub const ImportSpecifier: u16 = 262;
pub const ExportAssignment: u16 = 263;
pub const ExportDeclaration: u16 = 264;
pub const NamedExports: u16 = 265;
pub const NamespaceExport: u16 = 266;
pub const ExportSpecifier: u16 = 267;
pub const MissingDeclaration: u16 = 268;

// Module references
pub const ExternalModuleReference: u16 = 269;

// JSX
pub const JsxElement: u16 = 270;
pub const JsxSelfClosingElement: u16 = 271;
pub const JsxOpeningElement: u16 = 272;
pub const JsxClosingElement: u16 = 273;
pub const JsxFragment: u16 = 274;
pub const JsxOpeningFragment: u16 = 275;
pub const JsxClosingFragment: u16 = 276;
pub const JsxAttribute: u16 = 277;
pub const JsxAttributes: u16 = 278;
pub const JsxSpreadAttribute: u16 = 279;
pub const JsxExpression: u16 = 280;

// Clauses
pub const CaseClause: u16 = 281;
pub const DefaultClause: u16 = 282;
pub const HeritageClause: u16 = 283;
pub const CatchClause: u16 = 284;

// Property assignments
pub const PropertyAssignment: u16 = 285;
pub const ShorthandPropertyAssignment: u16 = 286;
pub const SpreadAssignment: u16 = 287;

// Enum
pub const EnumMember: u16 = 288;
// Unparsed
pub const UnparsedPrologue: u16 = 289;
pub const UnparsedPrepend: u16 = 290;
pub const UnparsedText: u16 = 291;
pub const UnparsedInternalText: u16 = 292;
pub const UnparsedSyntheticReference: u16 = 293;

// Top-level nodes
pub const SourceFile: u16 = 294;
pub const Bundle: u16 = 295;
pub const UnparsedSource: u16 = 296;
pub const InputFiles: u16 = 297;

// JSDoc nodes
pub const JSDocTypeExpression: u16 = 298;
// The * type
pub const JSDocAllType: u16 = 299;
// The ? type
pub const JSDocUnknownType: u16 = 300;
pub const JSDocNullableType: u16 = 301;
pub const JSDocNonNullableType: u16 = 302;
pub const JSDocOptionalType: u16 = 303;
pub const JSDocFunctionType: u16 = 304;
pub const JSDocVariadicType: u16 = 305;
// https://jsdoc.app/about-namepaths.html
pub const JSDocNamepathType: u16 = 306;
pub const JSDocComment: u16 = 307;
pub const JSDocTypeLiteral: u16 = 308;
pub const JSDocSignature: u16 = 309;
pub const JSDocTag: u16 = 310;
pub const JSDocAugmentsTag: u16 = 311;
pub const JSDocImplementsTag: u16 = 312;
pub const JSDocAuthorTag: u16 = 313;
pub const JSDocDeprecatedTag: u16 = 314;
pub const JSDocClassTag: u16 = 315;
pub const JSDocPublicTag: u16 = 316;
pub const JSDocPrivateTag: u16 = 317;
pub const JSDocProtectedTag: u16 = 318;
pub const JSDocReadonlyTag: u16 = 319;
pub const JSDocCallbackTag: u16 = 320;
pub const JSDocEnumTag: u16 = 321;
pub const JSDocParameterTag: u16 = 322;
pub const JSDocReturnTag: u16 = 323;
pub const JSDocThisTag: u16 = 324;
pub const JSDocTypeTag: u16 = 325;
pub const JSDocTemplateTag: u16 = 326;
pub const JSDocTypedefTag: u16 = 327;
pub const JSDocPropertyTag: u16 = 328;

// Synthesized list
pub const SyntaxList: u16 = 329;

// Transformation nodes
pub const NotEmittedStatement: u16 = 330;
pub const PartiallyEmittedExpression: u16 = 331;
pub const CommaListExpression: u16 = 332;
pub const MergeDeclarationMarker: u16 = 333;
pub const EndOfDeclarationMarker: u16 = 334;
pub const SyntheticReferenceExpression: u16 = 335;

// Enum value count
pub const Count: u16 = 336;

// Markers
pub const FirstAssignment: u16 = EqualsToken;
pub const LastAssignment: u16 = CaretEqualsToken;
pub const FirstCompoundAssignment: u16 = PlusEqualsToken;
pub const LastCompoundAssignment: u16 = CaretEqualsToken;
pub const FirstReservedWord: u16 = BreakKeyword;
pub const LastReservedWord: u16 = WithKeyword;
pub const FirstKeyword: u16 = BreakKeyword;
pub const LastKeyword: u16 = OfKeyword;
pub const FirstFutureReservedWord: u16 = ImplementsKeyword;
pub const LastFutureReservedWord: u16 = YieldKeyword;
pub const FirstTypeNode: u16 = TypePredicate;
pub const LastTypeNode: u16 = ImportType;
pub const FirstPunctuation: u16 = OpenBraceToken;
pub const LastPunctuation: u16 = CaretEqualsToken;
pub const FirstToken: u16 = Unknown;
pub const LastToken: u16 = LastKeyword;
pub const FirstTriviaToken: u16 = SingleLineCommentTrivia;
pub const LastTriviaToken: u16 = ConflictMarkerTrivia;
pub const FirstLiteralToken: u16 = NumericLiteral;
pub const LastLiteralToken: u16 = NoSubstitutionTemplateLiteral;
pub const FirstTemplateToken: u16 = NoSubstitutionTemplateLiteral;
pub const LastTemplateToken: u16 = TemplateTail;
pub const FirstBinaryOperator: u16 = LessThanToken;
pub const LastBinaryOperator: u16 = CaretEqualsToken;
pub const FirstStatement: u16 = VariableStatement;
pub const LastStatement: u16 = DebuggerStatement;
pub const FirstNode: u16 = QualifiedName;
pub const FirstJSDocNode: u16 = JSDocTypeExpression;
pub const LastJSDocNode: u16 = JSDocPropertyTag;
pub const FirstJSDocTagNode: u16 = JSDocTag;
pub const LastJSDocTagNode: u16 = JSDocPropertyTag;
/* @internal */
pub const FirstContextualKeyword: u16 = AbstractKeyword;
/* @internal */
pub const LastContextualKeyword: u16 = OfKeyword;

#[repr(u16)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TriviaSyntaxKind {
  SingleLineCommentTrivia = SingleLineCommentTrivia,
  MultiLineCommentTrivia = MultiLineCommentTrivia,
  NewLineTrivia = NewLineTrivia,
  WhitespaceTrivia = WhitespaceTrivia,
  ShebangTrivia = ShebangTrivia,
  ConflictMarkerTrivia = ConflictMarkerTrivia,
}

#[repr(u16)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LiteralSyntaxKind {
  NumericLiteral = NumericLiteral,
  BigIntLiteral = BigIntLiteral,
  StringLiteral = StringLiteral,
  JsxText = JsxText,
  JsxTextAllWhiteSpaces = JsxTextAllWhiteSpaces,
  RegularExpressionLiteral = RegularExpressionLiteral,
  NoSubstitutionTemplateLiteral = NoSubstitutionTemplateLiteral,
}

#[repr(u16)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PseudoLiteralSyntaxKind {
  TemplateHead = TemplateHead,
  TemplateMiddle = TemplateMiddle,
  TemplateTail = TemplateTail,
}

#[repr(u16)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PunctuationSyntaxKind {
  OpenBraceToken = OpenBraceToken,
  CloseBraceToken = CloseBraceToken,
  OpenParenToken = OpenParenToken,
  CloseParenToken = CloseParenToken,
  OpenBracketToken = OpenBracketToken,
  CloseBracketToken = CloseBracketToken,
  DotToken = DotToken,
  DotDotDotToken = DotDotDotToken,
  SemicolonToken = SemicolonToken,
  CommaToken = CommaToken,
  QuestionDotToken = QuestionDotToken,
  LessThanToken = LessThanToken,
  LessThanSlashToken = LessThanSlashToken,
  GreaterThanToken = GreaterThanToken,
  LessThanEqualsToken = LessThanEqualsToken,
  GreaterThanEqualsToken = GreaterThanEqualsToken,
  EqualsEqualsToken = EqualsEqualsToken,
  ExclamationEqualsToken = ExclamationEqualsToken,
  EqualsEqualsEqualsToken = EqualsEqualsEqualsToken,
  ExclamationEqualsEqualsToken = ExclamationEqualsEqualsToken,
  EqualsGreaterThanToken = EqualsGreaterThanToken,
  PlusToken = PlusToken,
  MinusToken = MinusToken,
  AsteriskToken = AsteriskToken,
  AsteriskAsteriskToken = AsteriskAsteriskToken,
  SlashToken = SlashToken,
  PercentToken = PercentToken,
  PlusPlusToken = PlusPlusToken,
  MinusMinusToken = MinusMinusToken,
  LessThanLessThanToken = LessThanLessThanToken,
  GreaterThanGreaterThanToken = GreaterThanGreaterThanToken,
  GreaterThanGreaterThanGreaterThanToken = GreaterThanGreaterThanGreaterThanToken,
  AmpersandToken = AmpersandToken,
  BarToken = BarToken,
  CaretToken = CaretToken,
  ExclamationToken = ExclamationToken,
  TildeToken = TildeToken,
  AmpersandAmpersandToken = AmpersandAmpersandToken,
  BarBarToken = BarBarToken,
  QuestionQuestionToken = QuestionQuestionToken,
  QuestionToken = QuestionToken,
  ColonToken = ColonToken,
  AtToken = AtToken,
  BacktickToken = BacktickToken,
  EqualsToken = EqualsToken,
  PlusEqualsToken = PlusEqualsToken,
  MinusEqualsToken = MinusEqualsToken,
  AsteriskEqualsToken = AsteriskEqualsToken,
  AsteriskAsteriskEqualsToken = AsteriskAsteriskEqualsToken,
  SlashEqualsToken = SlashEqualsToken,
  PercentEqualsToken = PercentEqualsToken,
  LessThanLessThanEqualsToken = LessThanLessThanEqualsToken,
  GreaterThanGreaterThanEqualsToken = GreaterThanGreaterThanEqualsToken,
  GreaterThanGreaterThanGreaterThanEqualsToken = GreaterThanGreaterThanGreaterThanEqualsToken,
  AmpersandEqualsToken = AmpersandEqualsToken,
  BarEqualsToken = BarEqualsToken,
  CaretEqualsToken = CaretEqualsToken,
}

#[repr(u16)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum KeywordSyntaxKind {
  AbstractKeyword = AbstractKeyword,
  AnyKeyword = AnyKeyword,
  AsKeyword = AsKeyword,
  AssertsKeyword = AssertsKeyword,
  AsyncKeyword = AsyncKeyword,
  AwaitKeyword = AwaitKeyword,
  BigIntKeyword = BigIntKeyword,
  BooleanKeyword = BooleanKeyword,
  BreakKeyword = BreakKeyword,
  CaseKeyword = CaseKeyword,
  CatchKeyword = CatchKeyword,
  ClassKeyword = ClassKeyword,
  ConstKeyword = ConstKeyword,
  ConstructorKeyword = ConstructorKeyword,
  ContinueKeyword = ContinueKeyword,
  DebuggerKeyword = DebuggerKeyword,
  DeclareKeyword = DeclareKeyword,
  DefaultKeyword = DefaultKeyword,
  DeleteKeyword = DeleteKeyword,
  DoKeyword = DoKeyword,
  ElseKeyword = ElseKeyword,
  EnumKeyword = EnumKeyword,
  ExportKeyword = ExportKeyword,
  ExtendsKeyword = ExtendsKeyword,
  FalseKeyword = FalseKeyword,
  FinallyKeyword = FinallyKeyword,
  ForKeyword = ForKeyword,
  FromKeyword = FromKeyword,
  FunctionKeyword = FunctionKeyword,
  GetKeyword = GetKeyword,
  GlobalKeyword = GlobalKeyword,
  IfKeyword = IfKeyword,
  ImplementsKeyword = ImplementsKeyword,
  ImportKeyword = ImportKeyword,
  InferKeyword = InferKeyword,
  InKeyword = InKeyword,
  InstanceOfKeyword = InstanceOfKeyword,
  InterfaceKeyword = InterfaceKeyword,
  IsKeyword = IsKeyword,
  KeyOfKeyword = KeyOfKeyword,
  LetKeyword = LetKeyword,
  ModuleKeyword = ModuleKeyword,
  NamespaceKeyword = NamespaceKeyword,
  NeverKeyword = NeverKeyword,
  NewKeyword = NewKeyword,
  NullKeyword = NullKeyword,
  NumberKeyword = NumberKeyword,
  ObjectKeyword = ObjectKeyword,
  OfKeyword = OfKeyword,
  PackageKeyword = PackageKeyword,
  PrivateKeyword = PrivateKeyword,
  ProtectedKeyword = ProtectedKeyword,
  PublicKeyword = PublicKeyword,
  ReadonlyKeyword = ReadonlyKeyword,
  RequireKeyword = RequireKeyword,
  ReturnKeyword = ReturnKeyword,
  SetKeyword = SetKeyword,
  StaticKeyword = StaticKeyword,
  StringKeyword = StringKeyword,
  SuperKeyword = SuperKeyword,
  SwitchKeyword = SwitchKeyword,
  SymbolKeyword = SymbolKeyword,
  ThisKeyword = ThisKeyword,
  ThrowKeyword = ThrowKeyword,
  TrueKeyword = TrueKeyword,
  TryKeyword = TryKeyword,
  TypeKeyword = TypeKeyword,
  TypeOfKeyword = TypeOfKeyword,
  UndefinedKeyword = UndefinedKeyword,
  UniqueKeyword = UniqueKeyword,
  UnknownKeyword = UnknownKeyword,
  VarKeyword = VarKeyword,
  VoidKeyword = VoidKeyword,
  WhileKeyword = WhileKeyword,
  WithKeyword = WithKeyword,
  YieldKeyword = YieldKeyword,
}

#[repr(u16)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ModifierSyntaxKind {
  AbstractKeyword = AbstractKeyword,
  AsyncKeyword = AsyncKeyword,
  ConstKeyword = ConstKeyword,
  DeclareKeyword = DeclareKeyword,
  DefaultKeyword = DefaultKeyword,
  ExportKeyword = ExportKeyword,
  PrivateKeyword = PrivateKeyword,
  ProtectedKeyword = ProtectedKeyword,
  PublicKeyword = PublicKeyword,
  ReadonlyKeyword = ReadonlyKeyword,
  StaticKeyword = StaticKeyword,
}

#[repr(u16)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum KeywordTypeSyntaxKind {
  AnyKeyword = AnyKeyword,
  BigIntKeyword = BigIntKeyword,
  BooleanKeyword = BooleanKeyword,
  NeverKeyword = NeverKeyword,
  NumberKeyword = NumberKeyword,
  ObjectKeyword = ObjectKeyword,
  StringKeyword = StringKeyword,
  SymbolKeyword = SymbolKeyword,
  UndefinedKeyword = UndefinedKeyword,
  UnknownKeyword = UnknownKeyword,
  VoidKeyword = VoidKeyword,
}

/* @internal */
#[repr(u16)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TypeNodeSyntaxKind {
  AbstractKeyword = AbstractKeyword,
  AnyKeyword = AnyKeyword,
  AsKeyword = AsKeyword,
  AssertsKeyword = AssertsKeyword,
  AsyncKeyword = AsyncKeyword,
  AwaitKeyword = AwaitKeyword,
  BigIntKeyword = BigIntKeyword,
  BooleanKeyword = BooleanKeyword,
  BreakKeyword = BreakKeyword,
  CaseKeyword = CaseKeyword,
  CatchKeyword = CatchKeyword,
  ClassKeyword = ClassKeyword,
  ConstKeyword = ConstKeyword,
  ConstructorKeyword = ConstructorKeyword,
  ContinueKeyword = ContinueKeyword,
  DebuggerKeyword = DebuggerKeyword,
  DeclareKeyword = DeclareKeyword,
  DefaultKeyword = DefaultKeyword,
  DeleteKeyword = DeleteKeyword,
  DoKeyword = DoKeyword,
  ElseKeyword = ElseKeyword,
  EnumKeyword = EnumKeyword,
  ExportKeyword = ExportKeyword,
  ExtendsKeyword = ExtendsKeyword,
  FalseKeyword = FalseKeyword,
  FinallyKeyword = FinallyKeyword,
  ForKeyword = ForKeyword,
  FromKeyword = FromKeyword,
  FunctionKeyword = FunctionKeyword,
  GetKeyword = GetKeyword,
  GlobalKeyword = GlobalKeyword,
  IfKeyword = IfKeyword,
  ImplementsKeyword = ImplementsKeyword,
  ImportKeyword = ImportKeyword,
  InferKeyword = InferKeyword,
  InKeyword = InKeyword,
  InstanceOfKeyword = InstanceOfKeyword,
  InterfaceKeyword = InterfaceKeyword,
  IsKeyword = IsKeyword,
  KeyOfKeyword = KeyOfKeyword,
  LetKeyword = LetKeyword,
  ModuleKeyword = ModuleKeyword,
  NamespaceKeyword = NamespaceKeyword,
  NeverKeyword = NeverKeyword,
  NewKeyword = NewKeyword,
  NullKeyword = NullKeyword,
  NumberKeyword = NumberKeyword,
  ObjectKeyword = ObjectKeyword,
  OfKeyword = OfKeyword,
  PackageKeyword = PackageKeyword,
  PrivateKeyword = PrivateKeyword,
  ProtectedKeyword = ProtectedKeyword,
  PublicKeyword = PublicKeyword,
  ReadonlyKeyword = ReadonlyKeyword,
  RequireKeyword = RequireKeyword,
  ReturnKeyword = ReturnKeyword,
  SetKeyword = SetKeyword,
  StaticKeyword = StaticKeyword,
  StringKeyword = StringKeyword,
  SuperKeyword = SuperKeyword,
  SwitchKeyword = SwitchKeyword,
  SymbolKeyword = SymbolKeyword,
  ThisKeyword = ThisKeyword,
  ThrowKeyword = ThrowKeyword,
  TrueKeyword = TrueKeyword,
  TryKeyword = TryKeyword,
  TypeKeyword = TypeKeyword,
  TypeOfKeyword = TypeOfKeyword,
  UndefinedKeyword = UndefinedKeyword,
  UniqueKeyword = UniqueKeyword,
  UnknownKeyword = UnknownKeyword,
  VarKeyword = VarKeyword,
  VoidKeyword = VoidKeyword,
  WhileKeyword = WhileKeyword,
  WithKeyword = WithKeyword,
  YieldKeyword = YieldKeyword,
  TypePredicate = TypePredicate,
  TypeReference = TypeReference,
  FunctionType = FunctionType,
  ConstructorType = ConstructorType,
  TypeQuery = TypeQuery,
  TypeLiteral = TypeLiteral,
  ArrayType = ArrayType,
  TupleType = TupleType,
  NamedTupleMember = NamedTupleMember,
  OptionalType = OptionalType,
  RestType = RestType,
  UnionType = UnionType,
  IntersectionType = IntersectionType,
  ConditionalType = ConditionalType,
  InferType = InferType,
  ParenthesizedType = ParenthesizedType,
  ThisType = ThisType,
  TypeOperator = TypeOperator,
  IndexedAccessType = IndexedAccessType,
  MappedType = MappedType,
  LiteralType = LiteralType,
  ImportType = ImportType,
  ExpressionWithTypeArguments = ExpressionWithTypeArguments,
  JSDocTypeExpression = JSDocTypeExpression,
  JSDocAllType = JSDocAllType,
  JSDocUnknownType = JSDocUnknownType,
  JSDocNonNullableType = JSDocNonNullableType,
  JSDocNullableType = JSDocNullableType,
  JSDocOptionalType = JSDocOptionalType,
  JSDocFunctionType = JSDocFunctionType,
  JSDocVariadicType = JSDocVariadicType,
  JSDocNamepathType = JSDocNamepathType,
  JSDocSignature = JSDocSignature,
  JSDocTypeLiteral = JSDocTypeLiteral,
}

#[repr(u16)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TokenSyntaxKind {
  Unknown = Unknown,
  EndOfFileToken = EndOfFileToken,
  SingleLineCommentTrivia = SingleLineCommentTrivia,
  MultiLineCommentTrivia = MultiLineCommentTrivia,
  NewLineTrivia = NewLineTrivia,
  WhitespaceTrivia = WhitespaceTrivia,
  ShebangTrivia = ShebangTrivia,
  ConflictMarkerTrivia = ConflictMarkerTrivia,
  NumericLiteral = NumericLiteral,
  BigIntLiteral = BigIntLiteral,
  StringLiteral = StringLiteral,
  JsxText = JsxText,
  JsxTextAllWhiteSpaces = JsxTextAllWhiteSpaces,
  RegularExpressionLiteral = RegularExpressionLiteral,
  NoSubstitutionTemplateLiteral = NoSubstitutionTemplateLiteral,
  TemplateHead = TemplateHead,
  TemplateMiddle = TemplateMiddle,
  TemplateTail = TemplateTail,
  OpenBraceToken = OpenBraceToken,
  CloseBraceToken = CloseBraceToken,
  OpenParenToken = OpenParenToken,
  CloseParenToken = CloseParenToken,
  OpenBracketToken = OpenBracketToken,
  CloseBracketToken = CloseBracketToken,
  DotToken = DotToken,
  DotDotDotToken = DotDotDotToken,
  SemicolonToken = SemicolonToken,
  CommaToken = CommaToken,
  QuestionDotToken = QuestionDotToken,
  LessThanToken = LessThanToken,
  LessThanSlashToken = LessThanSlashToken,
  GreaterThanToken = GreaterThanToken,
  LessThanEqualsToken = LessThanEqualsToken,
  GreaterThanEqualsToken = GreaterThanEqualsToken,
  EqualsEqualsToken = EqualsEqualsToken,
  ExclamationEqualsToken = ExclamationEqualsToken,
  EqualsEqualsEqualsToken = EqualsEqualsEqualsToken,
  ExclamationEqualsEqualsToken = ExclamationEqualsEqualsToken,
  EqualsGreaterThanToken = EqualsGreaterThanToken,
  PlusToken = PlusToken,
  MinusToken = MinusToken,
  AsteriskToken = AsteriskToken,
  AsteriskAsteriskToken = AsteriskAsteriskToken,
  SlashToken = SlashToken,
  PercentToken = PercentToken,
  PlusPlusToken = PlusPlusToken,
  MinusMinusToken = MinusMinusToken,
  LessThanLessThanToken = LessThanLessThanToken,
  GreaterThanGreaterThanToken = GreaterThanGreaterThanToken,
  GreaterThanGreaterThanGreaterThanToken = GreaterThanGreaterThanGreaterThanToken,
  AmpersandToken = AmpersandToken,
  BarToken = BarToken,
  CaretToken = CaretToken,
  ExclamationToken = ExclamationToken,
  TildeToken = TildeToken,
  AmpersandAmpersandToken = AmpersandAmpersandToken,
  BarBarToken = BarBarToken,
  QuestionQuestionToken = QuestionQuestionToken,
  QuestionToken = QuestionToken,
  ColonToken = ColonToken,
  AtToken = AtToken,
  BacktickToken = BacktickToken,
  EqualsToken = EqualsToken,
  PlusEqualsToken = PlusEqualsToken,
  MinusEqualsToken = MinusEqualsToken,
  AsteriskEqualsToken = AsteriskEqualsToken,
  AsteriskAsteriskEqualsToken = AsteriskAsteriskEqualsToken,
  SlashEqualsToken = SlashEqualsToken,
  PercentEqualsToken = PercentEqualsToken,
  LessThanLessThanEqualsToken = LessThanLessThanEqualsToken,
  GreaterThanGreaterThanEqualsToken = GreaterThanGreaterThanEqualsToken,
  GreaterThanGreaterThanGreaterThanEqualsToken = GreaterThanGreaterThanGreaterThanEqualsToken,
  AmpersandEqualsToken = AmpersandEqualsToken,
  BarEqualsToken = BarEqualsToken,
  CaretEqualsToken = CaretEqualsToken,
  Identifier = Identifier,
  AbstractKeyword = AbstractKeyword,
  AnyKeyword = AnyKeyword,
  AsKeyword = AsKeyword,
  AssertsKeyword = AssertsKeyword,
  AsyncKeyword = AsyncKeyword,
  AwaitKeyword = AwaitKeyword,
  BigIntKeyword = BigIntKeyword,
  BooleanKeyword = BooleanKeyword,
  BreakKeyword = BreakKeyword,
  CaseKeyword = CaseKeyword,
  CatchKeyword = CatchKeyword,
  ClassKeyword = ClassKeyword,
  ConstKeyword = ConstKeyword,
  ConstructorKeyword = ConstructorKeyword,
  ContinueKeyword = ContinueKeyword,
  DebuggerKeyword = DebuggerKeyword,
  DeclareKeyword = DeclareKeyword,
  DefaultKeyword = DefaultKeyword,
  DeleteKeyword = DeleteKeyword,
  DoKeyword = DoKeyword,
  ElseKeyword = ElseKeyword,
  EnumKeyword = EnumKeyword,
  ExportKeyword = ExportKeyword,
  ExtendsKeyword = ExtendsKeyword,
  FalseKeyword = FalseKeyword,
  FinallyKeyword = FinallyKeyword,
  ForKeyword = ForKeyword,
  FromKeyword = FromKeyword,
  FunctionKeyword = FunctionKeyword,
  GetKeyword = GetKeyword,
  GlobalKeyword = GlobalKeyword,
  IfKeyword = IfKeyword,
  ImplementsKeyword = ImplementsKeyword,
  ImportKeyword = ImportKeyword,
  InferKeyword = InferKeyword,
  InKeyword = InKeyword,
  InstanceOfKeyword = InstanceOfKeyword,
  InterfaceKeyword = InterfaceKeyword,
  IsKeyword = IsKeyword,
  KeyOfKeyword = KeyOfKeyword,
  LetKeyword = LetKeyword,
  ModuleKeyword = ModuleKeyword,
  NamespaceKeyword = NamespaceKeyword,
  NeverKeyword = NeverKeyword,
  NewKeyword = NewKeyword,
  NullKeyword = NullKeyword,
  NumberKeyword = NumberKeyword,
  ObjectKeyword = ObjectKeyword,
  OfKeyword = OfKeyword,
  PackageKeyword = PackageKeyword,
  PrivateKeyword = PrivateKeyword,
  ProtectedKeyword = ProtectedKeyword,
  PublicKeyword = PublicKeyword,
  ReadonlyKeyword = ReadonlyKeyword,
  RequireKeyword = RequireKeyword,
  ReturnKeyword = ReturnKeyword,
  SetKeyword = SetKeyword,
  StaticKeyword = StaticKeyword,
  StringKeyword = StringKeyword,
  SuperKeyword = SuperKeyword,
  SwitchKeyword = SwitchKeyword,
  SymbolKeyword = SymbolKeyword,
  ThisKeyword = ThisKeyword,
  ThrowKeyword = ThrowKeyword,
  TrueKeyword = TrueKeyword,
  TryKeyword = TryKeyword,
  TypeKeyword = TypeKeyword,
  TypeOfKeyword = TypeOfKeyword,
  UndefinedKeyword = UndefinedKeyword,
  UniqueKeyword = UniqueKeyword,
  UnknownKeyword = UnknownKeyword,
  VarKeyword = VarKeyword,
  VoidKeyword = VoidKeyword,
  WhileKeyword = WhileKeyword,
  WithKeyword = WithKeyword,
  YieldKeyword = YieldKeyword,
}

#[repr(u16)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum JsxTokenSyntaxKind {
  LessThanSlashToken = LessThanSlashToken,
  EndOfFileToken = EndOfFileToken,
  ConflictMarkerTrivia = ConflictMarkerTrivia,
  JsxText = JsxText,
  JsxTextAllWhiteSpaces = JsxTextAllWhiteSpaces,
  OpenBraceToken = OpenBraceToken,
  LessThanToken = LessThanToken,
}

#[repr(u16)]
#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum JSDocSyntaxKind {
  EndOfFileToken = EndOfFileToken,
  WhitespaceTrivia = WhitespaceTrivia,
  AtToken = AtToken,
  NewLineTrivia = NewLineTrivia,
  AsteriskToken = AsteriskToken,
  OpenBraceToken = OpenBraceToken,
  CloseBraceToken = CloseBraceToken,
  LessThanToken = LessThanToken,
  GreaterThanToken = GreaterThanToken,
  OpenBracketToken = OpenBracketToken,
  CloseBracketToken = CloseBracketToken,
  EqualsToken = EqualsToken,
  CommaToken = CommaToken,
  DotToken = DotToken,
  Identifier = Identifier,
  BacktickToken = BacktickToken,
  Unknown = Unknown,
  AbstractKeyword = AbstractKeyword,
  AnyKeyword = AnyKeyword,
  AsKeyword = AsKeyword,
  AssertsKeyword = AssertsKeyword,
  AsyncKeyword = AsyncKeyword,
  AwaitKeyword = AwaitKeyword,
  BigIntKeyword = BigIntKeyword,
  BooleanKeyword = BooleanKeyword,
  BreakKeyword = BreakKeyword,
  CaseKeyword = CaseKeyword,
  CatchKeyword = CatchKeyword,
  ClassKeyword = ClassKeyword,
  ConstKeyword = ConstKeyword,
  ConstructorKeyword = ConstructorKeyword,
  ContinueKeyword = ContinueKeyword,
  DebuggerKeyword = DebuggerKeyword,
  DeclareKeyword = DeclareKeyword,
  DefaultKeyword = DefaultKeyword,
  DeleteKeyword = DeleteKeyword,
  DoKeyword = DoKeyword,
  ElseKeyword = ElseKeyword,
  EnumKeyword = EnumKeyword,
  ExportKeyword = ExportKeyword,
  ExtendsKeyword = ExtendsKeyword,
  FalseKeyword = FalseKeyword,
  FinallyKeyword = FinallyKeyword,
  ForKeyword = ForKeyword,
  FromKeyword = FromKeyword,
  FunctionKeyword = FunctionKeyword,
  GetKeyword = GetKeyword,
  GlobalKeyword = GlobalKeyword,
  IfKeyword = IfKeyword,
  ImplementsKeyword = ImplementsKeyword,
  ImportKeyword = ImportKeyword,
  InferKeyword = InferKeyword,
  InKeyword = InKeyword,
  InstanceOfKeyword = InstanceOfKeyword,
  InterfaceKeyword = InterfaceKeyword,
  IsKeyword = IsKeyword,
  KeyOfKeyword = KeyOfKeyword,
  LetKeyword = LetKeyword,
  ModuleKeyword = ModuleKeyword,
  NamespaceKeyword = NamespaceKeyword,
  NeverKeyword = NeverKeyword,
  NewKeyword = NewKeyword,
  NullKeyword = NullKeyword,
  NumberKeyword = NumberKeyword,
  ObjectKeyword = ObjectKeyword,
  OfKeyword = OfKeyword,
  PackageKeyword = PackageKeyword,
  PrivateKeyword = PrivateKeyword,
  ProtectedKeyword = ProtectedKeyword,
  PublicKeyword = PublicKeyword,
  ReadonlyKeyword = ReadonlyKeyword,
  RequireKeyword = RequireKeyword,
  ReturnKeyword = ReturnKeyword,
  SetKeyword = SetKeyword,
  StaticKeyword = StaticKeyword,
  StringKeyword = StringKeyword,
  SuperKeyword = SuperKeyword,
  SwitchKeyword = SwitchKeyword,
  SymbolKeyword = SymbolKeyword,
  ThisKeyword = ThisKeyword,
  ThrowKeyword = ThrowKeyword,
  TrueKeyword = TrueKeyword,
  TryKeyword = TryKeyword,
  TypeKeyword = TypeKeyword,
  TypeOfKeyword = TypeOfKeyword,
  UndefinedKeyword = UndefinedKeyword,
  UniqueKeyword = UniqueKeyword,
  UnknownKeyword = UnknownKeyword,
  VarKeyword = VarKeyword,
  VoidKeyword = VoidKeyword,
  WhileKeyword = WhileKeyword,
  WithKeyword = WithKeyword,
  YieldKeyword = YieldKeyword,
}
