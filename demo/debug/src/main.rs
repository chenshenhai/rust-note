fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn main() {
    let x = 123;
    let y = 456;
    let result = add(x, y);

    let result = add(result, result);
    println!("result = {}", result);
}
