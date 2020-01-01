mod mod_nest; // 声明自定义 嵌套模块 mod_nest.rs 

fn main() {
    let x: u32 = 1;
    let y: u32 = 2;
    let z: u32 = mod_nest::math::add(1, 2);
    println!("The {} + {} = {}", x, y, z);
}
