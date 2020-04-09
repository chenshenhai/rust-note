use json;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut f = File::open("./json/hello.json").expect("file not found");
    let mut content = String::new();
    f.read_to_string(&mut content)
        .expect("something went wrong reading the file");

    let obj = json::parse(&content).unwrap();
    println!("json.str:  {:?}", obj["str"]);
    println!("json.boolean:  {:?}", obj["boolean"]);
    println!("json.number:  {:?}", obj["number"]);
    println!("json.null:  {:?}", obj["null"]);
    println!("json.array:  {:?}", obj["array"]);
    println!("json.objArray:  {:?}", obj["objArray"]);
}
