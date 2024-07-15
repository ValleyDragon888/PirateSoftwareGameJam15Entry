use super::entity::Component;

pub enum Query<'a> {
    Get(Vec<&'a str>),
    With(&'a str)
}

pub type QueryType<'a> = Vec<Query<'a>>;