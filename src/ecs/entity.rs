use crate::ecs::query::{QueryType, Query};
pub struct Entity {
    // I will need to use .iter()
    pub components: Vec<Box<dyn Component>>
}

impl Entity {
    pub fn contains_component(&self, c: &str) -> bool {
        for component in self.components.iter() {
            if component.id() == c {
                return true
            }
        }
        false
    }

    pub fn satisfies_query(&self, query: QueryType) -> bool {
        let satisfied = true;
        for condition in query {
            match condition {
                Query::With(c) => {
                    if !self.contains_component(c) {return false}
                }
                Query::Get(..) => ()
            }
            
        }
        return satisfied;
    }

    pub fn query(&self, query: QueryType) -> Option<Vec<Box<dyn Component>>> {
        
    }
}

pub trait Component {

    fn n(&self)->i32 {0}
    fn id(&self)->String;
}