fn divide(num1: f32, num2: f32) -> Option<f32> {
    if num2 == 0.0 {
        None
    } else {
        Some(num1 / num2)
    }
}

fn get_option_result(op: Option<f32>) -> String {
    let &mut result;
    match op {
        None => result = "Error".to_string(),
        Some(x) => result = x.to_string(),
    }
    result
}

fn main() {
    let op_1 = divide(10.0, 4.0);
    let result_1 = get_option_result(op_1);
    println!("result_1 = {}", result_1);

    let op_2 = divide(10.0, 4.0);
    let result_2 = get_option_result(op_2);
    println!("result_2 = {}", result_2);
}
