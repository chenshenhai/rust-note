mod mod1; // 声明自定义模块 mod1.rs 

fn main() {
    let x: u32 = 1;
    let y: u32 = 2;
    let z: u32 = mod1::add(1, 2);
    println!("The {} + {} = {}", x, y, z);
}
