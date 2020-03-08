mod calc;
use calc::calc_expression;

fn main() {
    // 3+(2-5)*6 => 325-6*+ => -15
    // ["3", "2", "5", "-", "6", "*", "+"]
    let expr = "3+(2-5)*6".to_string();
    let result = calc_expression(&expr);
    let expect = "-15";
    assert_eq!(result, expect);

    // 11+(22-33)*44+(55-66)+77/88 = -483.125
    let expr = "11+(22-33)*44+(55-66)+77/88".to_string();
    let result = calc_expression(&expr);
    let expect = "-483.125";
    assert_eq!(result, expect);
    println!("success!");
}
