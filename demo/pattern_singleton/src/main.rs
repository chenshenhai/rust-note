use std::sync::Arc;

pub struct Singleton {
    pub name: String,
}

pub fn create_singleton(name: &str) -> Arc<Singleton> {
    static mut SINGLESTON: Option<Arc<Singleton>> = None;
    unsafe {
        return Arc::clone(SINGLESTON.get_or_insert_with(|| {
            println!("init singleton");
            Arc::new(Singleton {
                name: name.to_string(),
            })
        }));
    }
}

fn main() {
    let name1 = "aaa";
    let a = create_singleton(&name1);
    println!("{}", a.name);

    let name2 = "bbb";
    let b = create_singleton(&name2);
    println!("{}", b.name);
}