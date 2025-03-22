use rust_decimal::prelude::*;
use rust_decimal_macros::*;

use crate::calcurus::{defines::*, state::*};

pub(crate) fn handle_key_click(state: &mut Calcurus, button_id: String) {
	handle_delete_keys(state, &button_id);

	// TODO: Replace unwrap with unwrap_or_else.
	let button_id_char = button_id.chars().next().unwrap();
	match button_id_char {
		'0'..='9' | '.' => handle_num_keys(state, button_id_char),

		// TODO: Add handling case for '√'
		'+' | '-' | '×' | '÷' | '^' => {
			if state.num_string_buffer.is_empty() {
				if button_id_char == '+' || button_id_char == '-' {
					state.num_string_buffer.push(button_id_char);
					state.display_buffer.push(button_id_char);
				}
				return;
			}
			let new_num = state.num_string_buffer.parse::<Decimal>().unwrap();
			state.num_string_buffer.clear();
			if state.num_buffer.current_object.is_none() {
				state.num_buffer.current_object = Some(NumObject::DecNumber(new_num));
				let operator = NumObject::Operator(button_id);
				state.num_buffer.push(operator);
				state.display_buffer.push(button_id_char);
			} else {
				let current_num_object = state.num_buffer.current_object.clone().unwrap();
				state.num_buffer.current_object = None;
				let operator = NumObject::Operator(button_id);
				state.num_buffer.push(current_num_object);
				state.num_buffer.push(operator);
				state.display_buffer.push(button_id_char);
			}
		}
		'=' => {
			// Parse and add the current number to num_buffer before operating
			if !state.num_string_buffer.is_empty() {
				let final_num = state.num_string_buffer.parse::<Decimal>().unwrap();
				state.num_string_buffer.clear();

				let num_object = NumObject::DecNumber(final_num);
				if state.num_buffer.current_object.is_some() {
					let current_obj = state.num_buffer.current_object.take().unwrap();
					state.num_buffer.push(current_obj);
				}
				state.num_buffer.push(num_object);
			}
			operate_on_buffer(state);
			if state.num_buffer.buffer.len() == 1 {
				let current_num_object: NumObject = state.num_buffer.buffer[0].clone();

				if let NumObject::DecNumber(current_num) = current_num_object {
					let current_num_string = current_num.to_string();
					state.num_string_buffer.push_str(&current_num_string);
				}

				state.num_buffer.clear();
			}
		}

		_ => (), // Handle all other cases
	}
}
fn handle_delete_keys(state: &mut Calcurus, button_id: &str) {
	if button_id == "Clr" {
		state.num_buffer.clear();
		state.display_buffer.clear();
		state.num_string_buffer.clear();
		state.is_output_dec = true;
	} else if button_id == "Bck" {
		if state.is_output_dec {
			if state.num_string_buffer.is_empty() {
				state.num_buffer.pop();
				state.display_buffer.pop();
			} else {
				state.display_buffer.pop();
				state.num_string_buffer.pop();
			}
		} else {
			state.num_buffer.clear();
			state.display_buffer.clear();
			state.num_string_buffer.clear();
			state.is_output_dec = true;
		}
	}
}

fn handle_num_keys(state: &mut Calcurus, button_id_char: char) {
	if !state.is_output_dec {
		state.num_buffer.clear();
		state.display_buffer.clear();
		state.num_string_buffer.clear();
		state.num_string_buffer.push(button_id_char);
		state.display_buffer.push(button_id_char);
		state.is_output_dec = true;
	} else {
		state.num_string_buffer.push(button_id_char);
		state.display_buffer.push(button_id_char);
	}
}

fn operate_on_buffer(app_state: &mut Calcurus) {
	let mut first_num: bool = true;
	let mut buf1: Decimal = dec!(0);
	let mut buf2: Decimal;

	let mut current_operator: NumObject = NumObject::Operator("+".to_string());
	let num_object_iterator = app_state.num_buffer.buffer.iter();

	for num_object in num_object_iterator {
		if let &NumObject::DecNumber(num) = num_object {
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
		let buf1_num_object = NumObject::DecNumber(buf1_dec);

		app_state.num_buffer.clear();
		app_state.num_buffer.push(buf1_num_object);
		// num_object_buffer.current_object = Some(buf1_num_object);

		app_state.display_buffer.clear();
		app_state.display_buffer.push_str(&buf1_string);
	}
}

fn perform_calculation(
	buf1: &mut Decimal,
	buf2: &mut Decimal,
	operator: &mut NumObject,
	display_buffer: &mut String,
) -> bool {
	let operator_value = match operator {
		NumObject::Operator(operator_value_inner) => operator_value_inner.clone(),
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
