#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Change {
    Added,
    Removed,
    Modified,
    Unchanged,
}
