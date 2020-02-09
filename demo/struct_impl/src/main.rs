struct Number {
    num: i64,
}

impl Number {

    fn increase(&mut self) {
        self.num += 1;
    }

    fn decrease(&mut self) {
        self.num -= 1;
    }

    fn get_result(&self) -> i64 {
        self.num
    }

    fn print(&self) {
        println!("the Number.num = {}", self.num.to_string());
    }
}

fn main() {
    let data = 123;
    println!("Number.num = {}", data.to_string());

    let mut num = Number{ num: data };
    num.increase();
    num.increase();
    num.increase();
    let result = num.get_result();
    println!("Increase 3 times, Number.num = {}", result.to_string());

    num.decrease();
    let result = num.get_result();
    println!("Decrease 1 times, Number.num = {}", result.to_string());

    num.print();
}
