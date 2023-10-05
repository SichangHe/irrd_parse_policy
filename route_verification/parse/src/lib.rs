use std::collections::BTreeMap;

use anyhow::{bail, Context, Error, Result};
use common_regex::{set::*, *};
use ipnet::IpNet;
use log::{debug, error};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};

pub mod action;
pub mod address_prefix;
pub mod aut_num;
pub mod aut_sys;
pub mod filter;
pub mod ir;
pub mod lex;
pub mod mp_import;
pub mod peering;
pub mod router_expr;
pub mod set;
#[cfg(test)]
mod tests;

pub use {
    self::lex::parse_lexed,
    action::{parse_actions, Actions},
    address_prefix::{match_ips, AddrPfxRange, RangeOperator},
    aut_num::AutNum,
    aut_sys::{is_as_set, parse_as_name, AsName},
    filter::{is_any, is_filter_set, parse_filter, Filter},
    ir::Ir,
    mp_import::{parse_imports, Casts, Entry, Versions},
    peering::{
        is_peering_set, parse_mp_peering, parse_mp_peerings, AsExpr, Peering, PeeringAction,
    },
    router_expr::{parse_router_expr, RouterExpr},
    set::{is_route_set_name, AsSet, FilterSet, PeeringSet, RouteSet, RouteSetMember},
};
