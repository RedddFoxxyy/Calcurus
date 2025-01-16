use macroquad::prelude::*;
use rust_decimal::prelude::*;

pub(crate) async fn operate(vec: &[u8]) -> f64 {
    let mut input: Decimal = 0_u8.into();
    let mut input_buf: Decimal = 0_u8.into();
    let mut operator: Option<u8> = None;
    let mut i = 0;
    let mut decimal_factor = Decimal::from_f64(0.1).unwrap();
    let mut is_decimal = false;

    while i < vec.len() {
        let current = vec[i];

        if current == 10 || current == 11 || current == 12 || current == 15 {
            // If we encounter an operator
            if operator.is_none() {
                // If there's no previous operator, move input_buf to input
                input = input_buf;
                input_buf = 0_u8.into();
            } else {
                // If there's a previous operator, perform the operation
                match operator.unwrap() {
                    10 => input += input_buf,
                    11 => input -= input_buf,
                    12 => input *= input_buf,
                    15 => input /= input_buf,
                    _ => unreachable!(),
                }
                input_buf = 0_u8.into();
            }
            operator = Some(current);
            is_decimal = false;
            decimal_factor = Decimal::from_f64(0.1).unwrap();
        } else if current == 16 {
            // Decimal point
            is_decimal = true;
        } else {
            // Building number in input_buf
            let current_dec: Decimal = current.into();
            if is_decimal {
                input_buf += current_dec * decimal_factor;
                decimal_factor *= Decimal::from_f64(0.1).unwrap();
            } else {
                let power_shift: Decimal = 10.into();
                input_buf = (input_buf * power_shift) + current_dec;
            }
        }

        i += 1;
    }

    // Perform final operation if there's a pending operator
    if let Some(op) = operator {
        match op {
            10 => input += input_buf,
            11 => input -= input_buf,
            12 => input *= input_buf,
            15 => input /= input_buf,
            _ => unreachable!(),
        }
    } else {
        // If there's no final operator, add the input_buf to input
        input = input_buf;
    }
    input.to_f64().unwrap()
}

pub(crate) async fn concatenate_strings(vector: &Vec<String>) -> String {
    let mut result = String::new();
    for s in vector {
        result += s;
    }
    result
}

pub(crate) async fn key_check(value: u8) -> bool {
    /*match value {
        10.0 => KeyCode::KpAdd,
        11.0 => KeyCode::Minus,
        12.0 => KeyCode::KpMultiply,
        15.0 => KeyCode::KpDivide,
        16.0 => KeyCode::Period,
        14.0 => KeyCode::Enter,
        13.0 => KeyCode::Delete,
        18.0 => KeyCode::Backspace,
        1.0 => KeyCode::Key1,
        2.0 => KeyCode::Key2,
        3.0 => KeyCode::Key3,
        4.0 => KeyCode::Key4,
        5.0 => KeyCode::Key5,
        6.0 => KeyCode::Key6,
        7.0 => KeyCode::Key7,
        8.0 => KeyCode::Key8,
        9.0 => KeyCode::Key9,
        0.0 => KeyCode::Key0,
        _ => unreachable!(),
    }*/
    match value {
        10 => is_key_pressed(KeyCode::KpAdd),
        11 => is_key_pressed(KeyCode::Minus) || is_key_pressed(KeyCode::KpSubtract),
        12 => is_key_pressed(KeyCode::KpMultiply),
        15 => is_key_pressed(KeyCode::KpDivide) || is_key_pressed(KeyCode::Slash),
        16 => is_key_pressed(KeyCode::Period) || is_key_pressed(KeyCode::KpDecimal),
        14 => {
            is_key_pressed(KeyCode::Enter)
                || is_key_pressed(KeyCode::KpEnter)
                || is_key_pressed(KeyCode::KpEqual)
        }
        13 => is_key_pressed(KeyCode::Delete),
        18 => is_key_pressed(KeyCode::Backspace),
        1 => is_key_pressed(KeyCode::Key1) || is_key_pressed(KeyCode::Kp1),
        2 => is_key_pressed(KeyCode::Key2) || is_key_pressed(KeyCode::Kp2),
        3 => is_key_pressed(KeyCode::Key3) || is_key_pressed(KeyCode::Kp3),
        4 => is_key_pressed(KeyCode::Key4) || is_key_pressed(KeyCode::Kp4),
        5 => is_key_pressed(KeyCode::Key5) || is_key_pressed(KeyCode::Kp5),
        6 => is_key_pressed(KeyCode::Key6) || is_key_pressed(KeyCode::Kp6),
        7 => is_key_pressed(KeyCode::Key7) || is_key_pressed(KeyCode::Kp7),
        8 => is_key_pressed(KeyCode::Key8) || is_key_pressed(KeyCode::Kp8),
        9 => is_key_pressed(KeyCode::Key9) || is_key_pressed(KeyCode::Kp9),
        0 => is_key_pressed(KeyCode::Key0) || is_key_pressed(KeyCode::Kp0),
        _ => false,
    }
}

pub(crate) fn f64_to_u8_vec(num: f64, input_buffer: &mut Vec<u8>, decimal: &mut bool) {
    let string_num = num.to_string();

    for char in string_num.chars() {
        match char {
            '0'..='9' => input_buffer.push(char as u8 - b'0'),
            '.' => {
                input_buffer.push(16);
                *decimal = true;
            }
            '-' => input_buffer.push(11),
            '+' => input_buffer.push(10),
            _ => {} // Ignore other characters
        }
    }
}
