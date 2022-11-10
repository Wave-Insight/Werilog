
#[derive(Debug)]
pub enum AttrSpec {
    Single(String),
    Equa(String, String),
}

#[derive(Debug)]
pub struct Attr(pub Vec<AttrSpec>);
