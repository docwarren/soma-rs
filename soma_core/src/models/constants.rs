#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnvType {
    SUBSTITUTION,
    INSERTION,
    DELETION,
}