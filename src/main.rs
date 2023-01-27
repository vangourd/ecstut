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
    ){
        /* do stuff  */
    }

}



fn main() {
    
}
