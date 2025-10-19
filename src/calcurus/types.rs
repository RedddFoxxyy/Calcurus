use rust_decimal::Decimal;

/// An Arithmetic Unit type, is an enum that can have two types either a DecNumber or an Operator.
/// Where an Operator is any arithmetic operator such as '+' and, a Number is of type 'Decimal' from
/// the 'rust_decimal' crate.
#[derive(Clone)]
pub enum ArithmeticUnit {
	Number(Decimal),
	Operator(String),
}

/// A Buffer(Vector) of ArithmeticUnits, and the current selected/entered unit.
pub struct ArithmeticUnitBuffer {
	pub buffer: Vec<ArithmeticUnit>,
	pub current_unit: Option<ArithmeticUnit>,
}
impl ArithmeticUnitBuffer {
	pub fn default() -> Self {
		ArithmeticUnitBuffer {
			buffer: Vec::new(),
			current_unit: None,
		}
	}
}
