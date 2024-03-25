pub mod mrgraph;

pub use mrgraph::GraphSingleton;
pub use mrgraph::GRAPH;

pub mod error;

pub mod meritrank {
    pub struct MeritRank;
    pub struct MeritRankError;
    pub struct MyGraph;
    pub struct Weight;
    pub mod node {
        pub struct Node;
    }
}



