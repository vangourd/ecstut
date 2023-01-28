struct Health(i32);
struct Name(&'static str);

trait ComponentVec {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn push_none(&mut self);
}


impl<T: 'static> ComponentVec for Vec<Option<T>> {

    fn as_any(&self) -> &dyn std::any::Any {
        self as &dyn std::any::Any
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }

    fn push_none(&mut self) {
        self.push(None)
    }


}


struct World {
    entities_count: usize,
    component_vecs: Vec<Box<dyn ComponentVec>>,
}


impl World {

    fn new() -> Self {
        Self {
            entities_count: 0,
            component_vecs: Vec::new()
        }
    }

    fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }
        self.entities_count += 1;
        entity_id
    }

    fn add_component_to_entity<ComponentType: 'static>(
        &mut self,
        entity: usize,
        component: ComponentType,
    ) {
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec
                .as_any_mut()
                .downcast_mut::<Vec<Option<ComponentType>>>() {
                    component_vec[entity] = Some(component);
                    return;
                }
        }
        let mut new_component_vec: Vec<Option<ComponentType>> = 
            Vec::with_capacity(self.entities_count);
        
        for _ in 0..self.entities_count {
            new_component_vec.push(None);
        }

        new_component_vec[entity] = Some(component);
        self.component_vecs.push(Box::new(new_component_vec));
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
