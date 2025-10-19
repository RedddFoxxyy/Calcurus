//! The Basic Working and logic of operation of this calculator is that, there is an empty buffer where we can push and pop Arithmetic Units ( see types.rs ).
//! When we type any number/operator on the calculator, these values are pushed to this buffer and during this various edge cases are also checked.
//! Once the user clicks the 'equal to' key, instead of pushing this to buffer, a function is called to operate on this buffer, and this function operates
//! on each unit one by one.
//!
//! To handle a number with multiple digits, a number is only pushed to the buffer once user clicks on an operator or when equal to sign is clicked.
//! However, this does not affect the handling of visual buffer ( the display of calculator ). The clicked key character gets pushed to visual buffer immediately.

// TODO: First handle inputs directly into the buffer, and then do the operation on all the units one by one.
use rust_decimal::prelude::*;
use rust_decimal_macros::*;

use crate::calcurus::{types::*, state::*};

pub(crate) fn handle_key_click(state: &mut Calcurus, button_id: String) {
	handle_delete_keys(state, &button_id);

	// TODO: Replace unwrap with unwrap_or_else.
	let button_id_char = button_id.chars().next().unwrap();
	match button_id_char {
		'0'..='9' | '.' => handle_num_keys(state, button_id_char),

		// TODO: Add handling case for '√'
		'+' | '-' | '×' | '÷' | '^' => {
			// If the num_string_buffer is empty, only allow input of + or - to give
			// sign to the number.
			if state.current_input_buffer.is_empty() {
				if button_id_char == '+' || button_id_char == '-' {
					state.push_current_input(&button_id_char);
				}
				return;
			}
			// Get the current number ( as Decimal ) present in the num string buffer before clearing the
			// num_string_buffer for the next number after the operator.
			let new_num: Decimal = state.current_input_buffer.parse().unwrap();
			// Clear the num_string_buffer for the next number.
			state.current_input_buffer.clear();
			// Depending upon the presence of current_object, make the current number as the current object if current
			// object is none else push the current object to num_buffer, push the operator to num buffer and then make
			// the new_num as the current object
			if state.unit_buf.current_unit.is_none() {
				state.unit_buf.current_unit = Some(ArithmeticUnit::Number(new_num));
				let operator = ArithmeticUnit::Operator(button_id);
				state.unit_buf.buffer.push(operator);
				state.display_buffer.push(button_id_char);
			} else {
				let current_num_object = state.unit_buf.current_unit.clone().unwrap();
				state.unit_buf.current_unit = None;
				let operator = ArithmeticUnit::Operator(button_id);
				state.unit_buf.buffer.push(current_num_object);
				state.unit_buf.buffer.push(operator);
				state.display_buffer.push(button_id_char);
			}
		}
		'=' => {
			// Parse and add the current number to num_buffer before operating
			if !state.current_input_buffer.is_empty() {
				let final_num = state.current_input_buffer.parse::<Decimal>().unwrap();
				state.current_input_buffer.clear();

				let num_object = ArithmeticUnit::Number(final_num);
				if state.unit_buf.current_unit.is_some() {
					let current_obj = state.unit_buf.current_unit.take().unwrap();
					state.unit_buf.buffer.push(current_obj);
				}
				state.unit_buf.buffer.push(num_object);
			}
			operate_on_buffer(state);
			if state.unit_buf.buffer.len() == 1 {
				let current_num_object: ArithmeticUnit = state.unit_buf.buffer[0].clone();

				if let ArithmeticUnit::Number(current_num) = current_num_object {
					let current_num_string = current_num.to_string();
					state.current_input_buffer.push_str(&current_num_string);
				}

				state.unit_buf.buffer.clear();
			}
		}

		_ => (),
	}
}
fn handle_delete_keys(state: &mut Calcurus, button_id: &str) {
	if button_id == "Clr" {
		state.unit_buf.buffer.clear();
		state.display_buffer.clear();
		state.current_input_buffer.clear();
		state.is_output_dec = true;
	} else if button_id == "Bck" {
		if state.is_output_dec {
			if state.current_input_buffer.is_empty() {
				state.unit_buf.buffer.pop();
				state.display_buffer.pop();
			} else {
				state.display_buffer.pop();
				state.current_input_buffer.pop();
			}
		} else {
			state.unit_buf.buffer.clear();
			state.display_buffer.clear();
			state.current_input_buffer.clear();
			state.is_output_dec = true;
		}
	}
}

fn handle_num_keys(state: &mut Calcurus, button_id_char: char) {
	if !state.is_output_dec {
		state.unit_buf.buffer.clear();
		state.display_buffer.clear();
		state.current_input_buffer.clear();
		state.push_current_input(&button_id_char);
		state.is_output_dec = true;
	} else {
		state.push_current_input(&button_id_char);
	}
}

fn operate_on_buffer(app_state: &mut Calcurus) {
	let mut first_num: bool = true;
	let mut buf1: Decimal = dec!(0);
	let mut buf2: Decimal;

	let mut current_operator: ArithmeticUnit = ArithmeticUnit::Operator("+".to_string());
	let num_object_iterator = app_state.unit_buf.buffer.iter();

	for num_object in num_object_iterator {
		if let &ArithmeticUnit::Number(num) = num_object {
			if first_num {
				buf1 = num;
				first_num = false;
			} else {
				buf2 = num;
				app_state.is_output_dec = perform_calculation(
					&mut buf1,
					&mut buf2,
					&mut current_operator,
					&mut app_state.display_buffer,
				);
			}
		} else {
			current_operator = num_object.clone();
		}
	}

	let buf1_string = buf1.to_string();
	if app_state.is_output_dec {
		let buf1_dec = buf1_string.parse::<Decimal>().unwrap();
		let buf1_num_object = ArithmeticUnit::Number(buf1_dec);

		app_state.unit_buf.buffer.clear();
		app_state.unit_buf.buffer.push(buf1_num_object);
		// num_object_buffer.current_object = Some(buf1_num_object);

		app_state.display_buffer.clear();
		app_state.display_buffer.push_str(&buf1_string);
	}
}

fn perform_calculation(
	buf1: &mut Decimal,
	buf2: &mut Decimal,
	operator: &mut ArithmeticUnit,
	display_buffer: &mut String,
) -> bool {
	let operator_value = match operator {
		ArithmeticUnit::Operator(operator_value_inner) => operator_value_inner.clone(),
		_ => unreachable!(),
	};

	match operator_value.as_str() {
		"+" => *buf1 += *buf2,
		"-" => *buf1 -= *buf2,
		"×" => *buf1 *= *buf2,
		"÷" => {
			if *buf2 == dec!(0) {
				display_buffer.clear();
				display_buffer.push_str("Cannot Divide By 0!");
				return false;
			}
			*buf1 /= *buf2
		}
		"^" => {
			let temp_dec = *buf1;
			let output_check = temp_dec.checked_powd(*buf2);
			if output_check.is_none() {
				display_buffer.clear();
				display_buffer.push_str("Value out of bounds!");
				return false;
			}
			*buf1 = output_check.unwrap();
		}
		_ => unreachable!(),
	}
	true
}
