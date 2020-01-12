use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;


// 判断路径是否存在
fn is_exist(path: &str) -> bool {
    Path::new(path).exists()
}

fn main() -> std::io::Result<()> {

    let is_dir_exist = is_exist("assets");
    // let file_type = metadata.file_type();
    // let is_dir = file_type.is_dir();

    // 开始前清除目录
    println!("[step-01] remove all assets");
    if is_dir_exist == true {
        fs::remove_dir_all("assets")?;
    }

    // 在项目的根目录下，创建一个目录 ./assets
    println!("[step-02] create dir ./assets");
    fs::create_dir("assets")?;

    // 遍历创建深层目录
    println!("[step-03] create deep-dir ./assets/child_dir/test_dir");
    fs::create_dir_all("assets/child_dir/test_dir")?;

    // 写入一个文件
    println!("[step-04] write file ./assets/child_dir/test_dir/demo.txt");
    fs::write("assets/child_dir/test_dir/demo.txt", "hello world!\r\nthis is second line")?;

    // 读取刚写入的文件
    println!("[step-04] read file ./assets/child_dir/test_dir/demo.txt");
    let mut f = File::open("./assets/child_dir/test_dir/demo.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    println!("read result is \"{}\"", contents);


    println!("[end] fs actions end!");
    Ok(())
}