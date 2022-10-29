
pub enum AttrSpec {
    Single(String),
    Equa(String, String),
}

pub struct Attr(pub Vec<AttrSpec>);
