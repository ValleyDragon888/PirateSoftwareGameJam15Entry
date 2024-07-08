pub struct Entity {
    // I will need to use .iter()
    components: Vec<Box<dyn Component>>
}

pub trait Component {
    fn id(&self);
}