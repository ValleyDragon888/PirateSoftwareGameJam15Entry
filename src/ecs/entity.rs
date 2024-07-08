
pub struct Entity {
    // I will need to use .iter()
    pub components: Vec<Box<dyn Component>>
}

pub trait Component {
    fn id(&self)->i32 {0}
}