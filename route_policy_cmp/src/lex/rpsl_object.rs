use serde::{Deserialize, Serialize};

use super::{mp_import::Versions, peering::Peering};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AutNum {
    pub name: String,
    pub body: String,
    pub imports: Versions,
    pub exports: Versions,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AsOrRouteSet {
    pub name: String,
    pub body: String,
    pub members: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct PeeringSet {
    pub name: String,
    pub body: String,
    pub peerings: Vec<Peering>,
}
