struct Health(i32);
struct Name(&'static str);

struct World {
    health_components: Vec<Option<Health>>,
    name_components: Vec<Option<Name>>,
}

impl World {

    fn new() -> Self {
        Self {
            health_components: Vec::new(),
            name_components:Vec::new(),
        }
    }


}



fn main() {
    println!("Hello, world!");
}
