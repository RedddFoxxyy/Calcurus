//! Implementation of a simple token parser that converts input string into tokens of chars and then performs calculation on them to give the result of type
//! Decimal, it uses Shunting Yard Algorithm to parse the tokens and operate on them to give an accurate result adhering to Operator Precedence.
//!
//! Sources:
//! https://en.wikipedia.org/wiki/Shunting_yard_algorithm
//! https://brilliant.org/wiki/shunting-yard-algorithm/
//! https://mathcenter.oxford.emory.edu/site/cs171/shuntingYardAlgorithm/
//! https://people.willamette.edu/~fruehr/353/files/ShuntingYard.pdf

use rust_decimal::{Decimal, MathematicalOps};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum Operator {
	Add,
	Sub,
	Mul,
	Div,
	Exp,
	Sqrt,
	Not,
}
impl Operator {
	fn from_char(c: &char) -> Option<Operator> {
		match c {
			'+' => Some(Operator::Add),
			'-' => Some(Operator::Sub),
			'*' => Some(Operator::Mul),
			'×' => Some(Operator::Mul),
			'/' => Some(Operator::Div),
			'÷' => Some(Operator::Div),
			'^' => Some(Operator::Exp),
			'√' => Some(Operator::Sqrt),
			'!' => Some(Operator::Not),
			_ => None,
		}
	}

	fn operate_on(&self, right: Decimal, left: Decimal) -> ParseResult {
		match self {
			Operator::Add => left.checked_add(right).ok_or(ParseErr::OutOfBounds),
			Operator::Sub => left.checked_sub(right).ok_or(ParseErr::OutOfBounds),
			Operator::Mul => left.checked_mul(right).ok_or(ParseErr::OutOfBounds),
			Operator::Div => {
				if right == Decimal::ZERO {
					Err(ParseErr::DivisionByZero)
				} else {
					left.checked_div(right).ok_or(ParseErr::OutOfBounds)
				}
			}
			// TODO: Handle Unwrap.
			Operator::Exp => left.checked_powd(right).ok_or(ParseErr::OutOfBounds),
			// TODO: Handle Sqrt.
			Operator::Sqrt => Ok(Decimal::default()),
			_ => Ok(Decimal::default()),
		}
	}
}

#[derive(Debug, PartialEq, Hash, Clone, Eq)]
enum Associativity {
	Right,
	Left,
}

#[derive(Debug, PartialEq, Hash, Clone, Eq)]
struct OpInfo {
	precedence: usize,
	associativity: Associativity,
}
impl OpInfo {
	fn from(precedence: usize, associativity: Associativity) -> OpInfo {
		Self { precedence, associativity }
	}
}

/// An Arithmetic Unit type is an enum that can have two types, either a Decimal or an Operator.
/// Where an Operator is any arithmetic operator such as '+' and, a Number is of type 'Decimal' from
/// the 'rust_decimal' crate.
#[derive(Clone, Debug, PartialEq)]
pub enum ArithmeticUnit {
	Num(Decimal),
	Op(Operator),
}

#[derive(Debug, PartialEq, Clone, Default)]
pub enum ParseErr {
	DivisionByZero,
	OutOfBounds,
	#[default]
	SyntaxErr,
	InvalidNumber,
}
impl ParseErr {
	pub fn as_str(&self) -> &'static str {
		match self {
			ParseErr::DivisionByZero => "NaN",
			ParseErr::OutOfBounds => "Out of Bounds!",
			ParseErr::SyntaxErr => "Syntax Error!",
			ParseErr::InvalidNumber => "Invalid Number!",
		}
	}
}
impl std::fmt::Display for ParseErr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.as_str())
	}
}

pub type ParseResult = Result<Decimal, ParseErr>;

/// A Basic ArithmeticUnit parser that uses shunting yard algorithm to operate on tokens(of chars) to produce the resulting output.
#[derive(Debug, PartialEq, Default)]
pub struct AUParser {
	tokens: Vec<char>,
	precedence_map: HashMap<Operator, OpInfo>,
	output: Vec<ArithmeticUnit>,
	stack: Vec<Operator>,
}
impl AUParser {
	pub fn init() -> Self {
		Self {
			precedence_map: load_operator_precedence(),
			..Default::default()
		}
	}

	pub fn set_input(&mut self, input: String) -> &Self {
		self.tokens = input.chars().collect();
		self
	}

	pub fn reset(&mut self) {
		self.tokens = vec![];
		self.output = vec![];
		self.stack = vec![];
	}

	fn shunt_tokens(&mut self) -> Result<(), ParseErr> {
		let mut stage_buffer: String = String::new();
		let mut expect_operand = true;

		for token in self.tokens.iter() {
			match token {
				'0'..='9' | '.' => {
					stage_buffer.push(*token);
					expect_operand = false;
				}
				'+' | '-' | '×' | '÷' | '*' | '/' | '^' => {
					// Handle buffered number first:
					if !stage_buffer.is_empty() {
						let decimal_num = stage_buffer.parse::<Decimal>().map_err(|_| ParseErr::InvalidNumber)?;
						self.output.push(ArithmeticUnit::Num(decimal_num));
						stage_buffer.clear();
					}

					if expect_operand && (*token == '+' || *token == '-') {
						if *token == '-' {
							self.output.push(ArithmeticUnit::Num(Decimal::ZERO));
							self.stack.push(Operator::Sub);
							expect_operand = true;
						}
						continue;
					}

					// Then Handle the operator:
					let input_operator = Operator::from_char(token).ok_or(ParseErr::SyntaxErr)?;
					if self.stack.is_empty() {
						self.stack.push(input_operator);
						expect_operand = true;
						continue;
					}
					let in_op_info = self.precedence_map.get(&input_operator).ok_or(ParseErr::SyntaxErr)?;

					while let Some(stack_operator) = self.stack.last() {
						let stack_op_info = self.precedence_map.get(stack_operator).ok_or(ParseErr::SyntaxErr)?;

						if (in_op_info.precedence < stack_op_info.precedence)
							|| (in_op_info.precedence == stack_op_info.precedence && in_op_info.associativity == Associativity::Left)
						{
							let stack_op = self.stack.pop().unwrap();
							self.output.push(ArithmeticUnit::Op(stack_op));
						} else {
							break;
						}
					}
					self.stack.push(input_operator);
					expect_operand = true;
				}
				_ => return Err(ParseErr::SyntaxErr),
			}
		}
		if !stage_buffer.is_empty() {
			let decimal_num = stage_buffer.parse::<Decimal>().map_err(|_| ParseErr::InvalidNumber)?;
			self.output.push(ArithmeticUnit::Num(decimal_num));
			stage_buffer.clear();
		}
		while let Some(operator) = self.stack.pop() {
			self.output.push(ArithmeticUnit::Op(operator));
		}
		Ok(())
	}

	fn parse_output_stack(&mut self) -> ParseResult {
		// NOTE: Maybe this Vec can be converted to a fixed size Array.
		let mut dec_stack: Vec<Decimal> = vec![];
		for au in self.output.iter() {
			if let ArithmeticUnit::Num(dec) = au {
				dec_stack.push(*dec);
			} else if let ArithmeticUnit::Op(op) = au {
				let right_reg = dec_stack.pop().ok_or(ParseErr::SyntaxErr)?;
				let left_reg = dec_stack.pop().ok_or(ParseErr::SyntaxErr)?;
				let new_val = op.operate_on(right_reg, left_reg)?;
				dec_stack.push(new_val);
			}
		}

		dec_stack.pop().ok_or(ParseErr::SyntaxErr)
	}

	pub fn calculate_result(&mut self) -> ParseResult {
		self.shunt_tokens()?;
		self.parse_output_stack()
	}
}

fn load_operator_precedence() -> HashMap<Operator, OpInfo> {
	let mut opt_precedence_map: HashMap<Operator, OpInfo> = HashMap::new();
	// opt_precedence_map.insert(',', OpInfo::from(0, Associativity::Left)); // comma
	// opt_precedence_map.insert('=', OpInfo::from(1, Associativity::Right)); // assignment
	opt_precedence_map.insert(Operator::Add, OpInfo::from(10, Associativity::Left)); // Addition
	opt_precedence_map.insert(Operator::Sub, OpInfo::from(10, Associativity::Left)); // Subtraction
	opt_precedence_map.insert(Operator::Mul, OpInfo::from(11, Associativity::Left)); // Multiplication
	opt_precedence_map.insert(Operator::Mul, OpInfo::from(11, Associativity::Left)); // Multiplication
	opt_precedence_map.insert(Operator::Div, OpInfo::from(11, Associativity::Left)); // Division
	opt_precedence_map.insert(Operator::Div, OpInfo::from(11, Associativity::Left)); // Division
	opt_precedence_map.insert(Operator::Exp, OpInfo::from(12, Associativity::Right)); // Exponentiation
	opt_precedence_map.insert(Operator::Not, OpInfo::from(13, Associativity::Right)); // Logical Not
	opt_precedence_map.insert(Operator::Sqrt, OpInfo::from(14, Associativity::Left)); // Sqrt
	// opt_precedence_map.insert('(', OpInfo::from(15, Associativity::Left)); // Parentheses
	// opt_precedence_map.insert(')', OpInfo::from(15, Associativity::Left)); // Parentheses

	opt_precedence_map
}

// TODO: Implement calculate function.
#[allow(dead_code)]
pub fn calculate(input: String) -> ParseResult {
	let mut parser = AUParser::init();
	parser.set_input(input);
	parser.calculate_result()
}

#[cfg(test)]
mod tests {
	use crate::parser::{calculate, ParseErr};
	use rust_decimal_macros::dec;

	#[test]
	fn add_two_digits() {
		let input = "10+5".to_string();
		let result = calculate(input).unwrap();

		assert_eq!(result, dec!(15));
	}

	#[test]
	fn add_three_digits() {
		let input = "10+5+5".to_string();
		let result = calculate(input).unwrap();

		assert_eq!(result, dec!(20));
	}

	#[test]
	fn multiply_two_digits() {
		let input = "22*8".to_string();
		let result = calculate(input).unwrap();

		assert_eq!(result, dec!(176));
	}

	#[test]
	fn multiply_three_digits() {
		let input = "10*5*5".to_string();
		let result = calculate(input).unwrap();

		assert_eq!(result, dec!(250));
	}

	#[test]
	fn divide_two_digits() {
		let input = "100/2".to_string();
		let result = calculate(input).unwrap();

		assert_eq!(result, dec!(50));
	}

	#[test]
	fn divide_three_digits() {
		let input = "100/2/2".to_string();
		let result = calculate(input).unwrap();

		assert_eq!(result, dec!(25));
	}

	#[test]
	fn exponentiation_two_digits() {
		let input = "100^2".to_string();
		let result = calculate(input).unwrap();

		assert_eq!(result, dec!(10000));
	}

	#[test]
	fn exponentiation_three_digits() {
		let input = "100^2^2".to_string();
		let result = calculate(input).unwrap();

		assert_eq!(result, dec!(100000000));
	}

	#[test]
	fn exponentiation_four_digits() {
		let input = "2^2^2^2".to_string();
		let result = calculate(input).unwrap();

		assert_eq!(result, dec!(65536));
	}

	#[test]
	fn multiple_digits_multiple_operators() {
		let input = "45*2/3+1".to_string();
		let result = calculate(input).unwrap();

		assert_eq!(result, dec!(31));
	}
	#[test]
	fn multiple_digits_multiple_operators_2() {
		let input = "789*3/5+145-66^4".to_string();
		let result = calculate(input).unwrap();

		assert_eq!(result, dec!(-18974117.6));
	}

	#[test]
	fn multiple_digits_multiple_operators_3() {
		let input = "139/3*8-269+66^2+55".to_string();
		let result = calculate(input).unwrap();

		assert_eq!(result, dec!(4512.6666666666666666666666667));
	}

	#[test]
	fn multiple_digits_multiple_operators_4() {
		let input = "2^2^2-6".to_string();
		let result = calculate(input).unwrap();

		assert_eq!(result, dec!(10));
	}

	#[test]
	fn floating_point_numbers() {
		let input = "55.2+78.1234-64.431+5.6893".to_string();
		let result = calculate(input).unwrap();

		assert_eq!(result, dec!(74.5817));
	}

	#[test]
	fn divide_by_zero() {
		let input = "2345/0".to_string();
		let result = calculate(input).unwrap_err();

		assert_eq!(result, ParseErr::DivisionByZero);
	}

	#[test]
	fn divide_zero_by_zero() {
		let input = "0/0".to_string();
		let result = calculate(input).unwrap_err();

		assert_eq!(result, ParseErr::DivisionByZero);
	}

	#[test]
	fn out_of_bounds() {
		let input = "2^2^2^2^2^2".to_string();
		let result = calculate(input).unwrap_err();

		assert_eq!(result, ParseErr::OutOfBounds);
	}

	#[test]
	fn syntax_error() {
		let input = "3*3+r".to_string();
		let result = calculate(input).unwrap_err();

		assert_eq!(result, ParseErr::SyntaxErr);
	}
}
