// define trait
trait Tool {
    fn show_name(&self);
}

// define type
struct Car;
struct Bike;
struct ToolFactory;

impl Tool for Car {
    fn  show_name(&self) {
        println!("This is a car!")
    }
}

impl Tool for Bike {
    fn  show_name(&self) {
        println!("This is a bike!")
    }
}

impl ToolFactory {
    fn create_tool(&self, tool_name: &str) -> Box<dyn Tool> {
        if tool_name == "car" {
            Box::new(Car)
        } else {
            Box::new(Bike)
        }
    }   
    fn new() -> Self {
        ToolFactory
    }
}

fn main() {
    let factory = ToolFactory::new();

    let tool1 = factory.create_tool("car");
    tool1.show_name();

    let tool2 = factory.create_tool("bike");
    tool2.show_name();
}
