use std::collections::BTreeMap;

use crate::{
    parse::{
        aut_num::AutNum,
        lex::{parse_aut_num_name, parse_lexed},
        set::{AsSet, RouteSet},
    },
    test::lex::dump::expected_dump,
};

pub use super::*;

#[test]
fn parse_name() {
    assert_eq!(parse_aut_num_name("AS2340").unwrap(), 2340);
    assert_eq!(parse_aut_num_name("AS2340 ").unwrap(), 2340);
    assert!(parse_aut_num_name("AS-2340").is_err());
    assert!(parse_aut_num_name("jfwoe").is_err());
}

#[test]
fn parse_dump() {
    let lexed = expected_dump();
    let (aut_nums, as_sets, route_sets) = parse_lexed(lexed);
    assert_eq!(aut_nums, expected_aut_nums());
    assert_eq!(as_sets, expected_as_sets());
    assert_eq!(route_sets, expected_route_sets());
}

fn expected_aut_nums() -> BTreeMap<usize, AutNum> {
    let body = "remarks:\nremarks: This aut-num has been transfered as part of the ERX.\nremarks: It was present in both the ARIN and RIPE databases, so\nremarks: the information from both databases has been merged.\nremarks: If you are the mntner of this object, please update it\nremarks: to reflect the correct information.\nremarks:\nremarks: Please see the FAQ for this process:\nremarks: http://www.ripe.net/db/erx/erx-asn/group3-faq.html\nremarks:\nremarks: **** INFORMATION FROM ARIN OBJECT ****\nremarks: as-name: EASINET-AS1\ndescr: EASInet Operations Center\n Riemenschneiderstrasse 11\n D-5300 Bonn 2\n DE\nadmin-c: DUMY-RIPE\ntech-c: DUMY-RIPE\nremarks: changed: hostmaster@arin.net 19900302\nremarks: changed: hostmaster@arin.net 19910416\nremarks:\nremarks: **** INFORMATION FROM RIPE OBJECT ****\nas-name: UNSPECIFIED\ndescr: EASInet\nimport: from AS690\n action pref=100;\n accept ANY\nimport: from AS513\n action pref=100;\n accept ANY\nimport: from AS559\n action pref=100;\n accept AS559\nimport: from AS697\n action pref=100;\n accept AS697\nexport: to AS690\n announce AS590\nexport: to AS513\n announce AS590\nexport: to AS559\n announce AS590\nexport: to AS697\n announce AS590\ndefault: to AS690\n action pref=100;\n networks ANY\ndefault: to AS513\n action pref=200;\n networks ANY\nstatus: LEGACY\nnotify: stf@easi.net\nmnt-by: RIPE-NCC-AN-MNT # WARNING: maintainer added to protect object\ncreated: 2002-09-19T15:23:42Z\nlast-modified: 2017-11-15T09:12:37Z\nsource: RIPE\nremarks: ****************************\nremarks: * THIS OBJECT IS MODIFIED\nremarks: * Please note that all data that is generally regarded as personal\nremarks: * data has been removed from this object.\nremarks: * To view the original object, please query the RIPE Database at:\nremarks: * http://www.ripe.net/whois\nremarks: ****************************\n".into();
    BTreeMap::from([(
        590,
        AutNum {
            body,
            errors: vec![],
        },
    )])
}

fn expected_as_sets() -> BTreeMap<String, AsSet> {
    BTreeMap::from([])
}

fn expected_route_sets() -> BTreeMap<String, RouteSet> {
    BTreeMap::from([])
}
