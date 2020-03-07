mod calc;
use calc::calc_expression;

fn main() {
    let expr = "1+(2-3)*4+(5-5)".to_string();
    let result = calc_expression(&expr);
    let expect = "-3";
    assert_eq!(result, expect);
    println!("success!");
}
