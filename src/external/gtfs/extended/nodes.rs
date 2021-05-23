use serde::{Deserialize, Serialize};

use crate::external::gtfsdb::Table;

/// ノードID (ex: 1)
pub type NodeId = i32;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
pub struct Node {
    /// ノードID
    pub node_id: NodeId,
    /// ノード名
    pub node_name: String,
    /// ノード読み仮名
    pub node_ruby: String,
}

impl Table for Node {
    fn table_name() -> &'static str {
        "nodes"
    }

    fn column_names() -> &'static [&'static str] {
        &["node_id", "node_name", "node_ruby"]
    }

    fn create_sql() -> &'static str {
        "
        node_id int,
        node_name text,
        node_ruby text,
        PRIMARY KEY(node_id)
        "
    }
}
