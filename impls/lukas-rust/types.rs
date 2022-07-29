#[derive(Debug)]
pub enum MalType {
    MalList(Vec<MalType>),
    MalAtom(String),
}
