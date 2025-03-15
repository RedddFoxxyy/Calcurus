use rust_decimal::prelude::*;
use rust_decimal_macros::*;

use crate::calcurus::state::*;

pub fn operate_on(num_object_buffer: &NumObjectBuffer, display_buffer: &mut String) {
    let mut buf1: Decimal = dec!(0);
    let mut first_num: bool = true;
    let mut buf2: Decimal;
    let mut current_operator: NumObject = NumObject::Operator("+".to_string());
    let num_object_iterator = num_object_buffer.buffer.iter();

    for num_object in num_object_iterator {
        if let &NumObject::DecNumber(num) = num_object {
            if first_num {
                buf1 = num;
                first_num = false;
            } else {
                buf2 = num;
                perform_operation(&mut buf1, &mut buf2, &mut current_operator);
            }
        } else {
            current_operator = num_object.clone();
        }
    }

    let buf1_string = buf1.to_string();
    display_buffer.clear();
    display_buffer.push_str(&buf1_string);
}

fn perform_operation(buf1: &mut Decimal, buf2: &mut Decimal, operator: &mut NumObject) {
    let operator_value = match operator {
        NumObject::Operator(operator_value) => operator_value.clone(),
        _ => unreachable!(),
    };

    match operator_value.as_str() {
        "+" => *buf1 += *buf2,
        "-" => *buf1 -= *buf2,
        "*" => *buf1 *= *buf2,
        "/" => *buf1 /= *buf2,
        _ => unreachable!(),
    }
}

// pub async fn handle_initial_num(currentnum: &Some(NumObject)) {
//     if
// }
