// PURPOSE: GraphColorVO — 3-color DFS graph traversal state VO for cycle detection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GraphColorVO {
    #[default]
    White,
    Gray,
    Black,
}
