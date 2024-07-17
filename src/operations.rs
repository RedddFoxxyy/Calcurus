use macroquad::prelude::*;

pub(crate) async fn operate(vec: &Vec<f64>) -> f64 {
    let mut input: f64 = 0.0;
    let mut input_buf: f64 = 0.0;
    let mut operator: Option<f64> = None;
    let mut i = 0;
    let mut decimal_factor: f64 = 0.1;
    let mut is_decimal = false;

    while i < vec.len() {
        let current = vec[i];

        if current == 10.0 || current == 11.0 || current == 12.0 || current == 15.0 {
            // If we encounter an operator
            if operator.is_none() {
                // If there's no previous operator, move input_buf to input
                input = input_buf;
                input_buf = 0.0;
            } else {
                // If there's a previous operator, perform the operation
                match operator.unwrap() {
                    10.0 => input += input_buf,
                    11.0 => input -= input_buf,
                    12.0 => input *= input_buf,
                    15.0 => input /= input_buf,
                    _ => unreachable!(),
                }
                input_buf = 0.0;
            }
            operator = Some(current);
            is_decimal = false;
            decimal_factor = 0.1;
        } else if current == 16.0 {
            // Decimal point
            is_decimal = true;
        } else {
            // Building number in input_buf
            if is_decimal {
                input_buf += current * decimal_factor;
                decimal_factor *= 0.1;
            } else {
                input_buf = input_buf * 10.0 + current;
            }
        }

        i += 1;
    }

    // Perform final operation if there's a pending operator
    if let Some(op) = operator {
        match op {
            10.0 => input += input_buf,
            11.0 => input -= input_buf,
            12.0 => input *= input_buf,
            15.0 => input /= input_buf,
            _ => unreachable!(),
        }
    } else {
        // If there's no final operator, add the input_buf to input
        input = input_buf;
    }

    input
}

pub(crate) async fn concatenate_strings(vector: &Vec<String>) -> String {
    let mut result = String::new();
    for s in vector {
        result += &s;
    }
    result
}

pub(crate) async fn key_code_return(value: f64) -> KeyCode {
    match value {
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
    }
}
