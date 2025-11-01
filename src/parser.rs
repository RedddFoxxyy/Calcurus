//! Implementation of a simple token parser that converts input string into tokens of chars and then performs calculation on them to give the result of type
//! Decimal, it uses Shunting Yard Algorithm to parse the tokens and operate on them to give accurate result adhering to Operator Precedence.
//!
//! Sources:
//! https://en.wikipedia.org/wiki/Shunting_yard_algorithm
//! https://brilliant.org/wiki/shunting-yard-algorithm/
//! https://mathcenter.oxford.emory.edu/site/cs171/shuntingYardAlgorithm/
//! https://people.willamette.edu/~fruehr/353/files/ShuntingYard.pdf

use std::collections::HashMap;
use rust_decimal::{Decimal, MathematicalOps};

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum Operator {
	Add,
	Sub,
	Mul,
	Div,
	Exp,
	Sqrt,
	Not
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
			_ => None
		}
	}

	fn operate_on(&self, right: Decimal, left: Decimal) -> Decimal {
		match self {
			Operator::Add => left + right,
			Operator::Sub => left - right,
			Operator::Mul => left * right,
			Operator::Div => left / right,
			// TODO: Handle Unwrap.
			Operator::Exp => left.checked_powd(right).unwrap(),
			// TODO: Handle Sqrt.
			Operator::Sqrt => Decimal::default(),
			_ => Decimal::default()
		}
	}
}

#[derive(Debug, PartialEq, Hash, Clone, Eq)]
enum Associativity {
	Right,
	Left
}

#[derive(Debug, PartialEq, Hash, Clone, Eq)]
struct OpInfo {
	precedence: usize,
	associativity: Associativity,
}
impl OpInfo {
	fn from(precedence: usize, associativity: Associativity) -> OpInfo {
		Self {
			precedence,
			associativity
		}
	}
}

/// An Arithmetic Unit type, is an enum that can have two types either a DecNumber or an Operator.
/// Where an Operator is any arithmetic operator such as '+' and, a Number is of type 'Decimal' from
/// the 'rust_decimal' crate.
#[derive(Clone, Debug, PartialEq)]
pub enum ArithmeticUnit {
	Num(Decimal),
	Op(Operator),
}

/// A Basic ArithmeticUnit parser that uses shunting yard algorithm to operate on tokens(of chars) to produce the resulting output.
#[derive(Debug, PartialEq, Default)]
pub struct AUParser {
	tokens: Vec<char>,
	precedence_map: HashMap<Operator, OpInfo>,
	output: Vec<ArithmeticUnit>,
	stack: Vec<Operator>,
	result: Decimal
}
impl AUParser {
	fn init() -> Self {
		Self {
			precedence_map: load_operator_precedence(),
			..Default::default()
		}
	}

	fn set_input(&mut self, input: String) -> &Self {
		self.tokens = input.chars().collect();
		self
	}
	
	fn reset(&mut self) {
		self.tokens = vec![];
		self.output = vec![];
		self.stack = vec![];
		self.result = Decimal::default();
	}

	fn shunt_tokens(&mut self) {
		let mut stage_buffer: String = String::new();
		for token in self.tokens.iter() {
			match token {
				'0'..='9' | '.' => {
					stage_buffer.push(*token);
				},
				'+' | '-' | '×' | '÷' | '^' => {
					// Handle buffered number first:
					let decimal = stage_buffer.parse::<Decimal>().unwrap();
					let arithmetic_unit = ArithmeticUnit::Num(decimal);
					self.output.push(arithmetic_unit);
					stage_buffer.clear();

					// Then Handle the operator:
					// TODO: Handle None.
					let input_operator = Operator::from_char(token).unwrap();
					if self.stack.is_empty() {
						self.stack.push(input_operator);
						continue;
					}
					let stack_operator = self.stack.last().unwrap();
					let in_op_info = self.precedence_map.get(&input_operator).unwrap();
					let stack_op_info = self.precedence_map.get(stack_operator).unwrap();

					if in_op_info.precedence < stack_op_info.precedence {
						let stack_op = self.stack.pop().unwrap();
						self.output.push(ArithmeticUnit::Op(stack_op));
					} else if in_op_info.precedence > stack_op_info.precedence {
						self.stack.push(input_operator);
					} else if in_op_info.precedence == stack_op_info.precedence {
						if  in_op_info.associativity == Associativity::Left {
							let stack_op = self.stack.pop().unwrap();
							self.output.push(ArithmeticUnit::Op(stack_op));
						} else {
							self.stack.push(input_operator);
						}
					}
				},
				_ => (),
			}
		}
		if !stage_buffer.is_empty() {
			// TODO: Handle None
			self.output.push(ArithmeticUnit::Num(stage_buffer.parse::<Decimal>().unwrap()));
			stage_buffer.clear();
		}
		while let Some(operator) = self.stack.pop() {
			self.output.push(ArithmeticUnit::Op(operator));
		}
	}

	fn parse_output_stack(&mut self) {
		// NOTE: Maybe this Vec can be converted to a fixed size Array.
		let mut dec_stack: Vec<Decimal> = vec![];
		for au in self.output.iter() {
			if let ArithmeticUnit::Num(dec) = au {
				dec_stack.push(*dec);
			} else if let ArithmeticUnit::Op(op) = au {
				let right_reg = dec_stack.pop().unwrap();
				let left_reg = dec_stack.pop().unwrap();
				let new_val = op.operate_on(right_reg, left_reg);
				dec_stack.push(new_val);
			}
		}
		self.result = dec_stack.pop().unwrap();
	}

	fn calculate_result(&mut self) -> Decimal {
		self.shunt_tokens();
		self.parse_output_stack();
		self.result
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
pub fn calculate(input: String) -> Decimal {
	let mut parser = AUParser::init();
	parser.set_input(input);
	parser.calculate_result()
}