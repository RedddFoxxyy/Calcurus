use rust_decimal::prelude::*;
use rust_decimal_macros::*;

use crate::calcurus::state::*;

pub fn operate(
    // num_object_buffer: &mut NumObjectBuffer,
    // display_buffer: &mut String,
    app_state: &mut Calcurus,
) {
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
        "*" => *buf1 *= *buf2,
        "/" => {
            if *buf2 == dec!(0) {
                display_buffer.clear();
                display_buffer.push_str("Cannot Divide By 0!");
                return false;
            }
            *buf1 /= *buf2
        }
        _ => unreachable!(),
    }
    true
}
