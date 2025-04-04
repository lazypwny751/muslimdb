pub(crate) enum Tokens {
	Plus,			// +
	Tire,			// -
	Asterix,		// * (oburix)
	Slash,			// /
	BackSlash,		// \
	Greater,		// >
	Less,			// <
	And,			// &
	Question,		// ?
	Module,			// %
	Equal,			// =
	DoubleQuot,		// "
	SingleQuot,		// '
	True,			// true
	False,			// false
	Word,			// any word without qutations.
	String(String),	// raw string betwen two double quotations. 
	Number(i32),	// regular number 32 byte integer.
}

impl Tokens {
	pub fn lex(ctx: &[u8]) -> Vec<Self> {
		let mut tokens: Vec<Self> = vec![];

		tokens
	}
}
