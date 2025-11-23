use serde::{Deserialize, Serialize};

/// The sorting order.
///
/// Check the variants for more information (Should be pretty clear already though).
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SortingOrder {
    /// From top to bottom: 5 -> 4 -> 3 -> 2 -> 1
    Desc,
    /// From bottom to top: 1 -> 2 -> 3 -> 4 -> 5
    Asc,
}
