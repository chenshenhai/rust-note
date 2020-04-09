use json;
use std::fs::File;
use std::io::Read;

fn read_file(path: &str) -> String {
    let mut f = File::open(path).expect("file not found");
    let mut content = String::new();
    f.read_to_string(&mut content)
        .expect("something went wrong reading the file");
    return content;
}

fn main() {
    let content = read_file("./json/hello.json");

    let obj = json::parse(&content).unwrap();
    println!("json.str:  {:?}", obj["str"]);
    println!("json.boolean:  {:?}", obj["boolean"]);
    println!("json.number:  {:?}", obj["number"]);
    println!("json.null:  {:?}", obj["null"]);
    println!("json.array:  {:?}", obj["array"]);
    println!("json.objArray:  {:?}", obj["objArray"]);
}
