struct Health(i32);
struct Name(&'static str);

trait ComponentVec {
    fn push_none(&mut self);
}

struct World {
    entities_count: usize,
    component_vecs: Vec<Box<dyn ComponentVec>>,
}

impl<T> ComponentVec for Vec<Option<T>> {
    fn push_none(&mut self) {
        self.push(None)
    }
}


impl World {

    fn new() -> Self {
        Self {
            entities_count: 0,
            component_vecs: Vec::new()
        }
    }

    fn new_entity(&mut self, health: Option<Health>, name: Option<Name>) {
        self.health_components.push(health);
        self.name_components.push(name);
    }


}



fn main() {
    let mut world = World::new();
    world.new_entity(Some(Health(-10)), Some(Name("Icarus")));
    world.new_entity(Some(Health(100)), Some(Name("Prometheus")));
    world.new_entity(None, Some(Name("Zeus")));

    let zip = world
        .health_components
        .iter()
        .zip(world.name_components.iter());
    
    let with_health_and_name = 
        zip.filter_map(|(health,name): (&Option<Health>, &Option<Name>)| {
            Some((health.as_ref()?, name.as_ref()?))
        });

    for (health, name) in with_health_and_name {
        if health.0 < 0 {
            println!("{} has perished!", name.0);
        } else {
            println!("{} is still healthy", name.0);
        }
    }
}
