use maplit::btreemap;

use crate::{
    action::Action::*,
    filter::Filter::*,
    mp_import::{Casts, Entry, PeeringAction},
    AsExpr::*,
    *,
};

pub const AST: &str = r#"{"aut_nums":[{"name":"AS590","body":"remarks:\nremarks: This aut-num has been transfered as part of the ERX.\nremarks: It was present in both the ARIN and RIPE databases, so\nremarks: the information from both databases has been merged.\nremarks: If you are the mntner of this object, please update it\nremarks: to reflect the correct information.\nremarks:\nremarks: Please see the FAQ for this process:\nremarks: http://www.ripe.net/db/erx/erx-asn/group3-faq.html\nremarks:\nremarks: **** INFORMATION FROM ARIN OBJECT ****\nremarks: as-name: EASINET-AS1\ndescr: EASInet Operations Center\n Riemenschneiderstrasse 11\n D-5300 Bonn 2\n DE\nadmin-c: DUMY-RIPE\ntech-c: DUMY-RIPE\nremarks: changed: hostmaster@arin.net 19900302\nremarks: changed: hostmaster@arin.net 19910416\nremarks:\nremarks: **** INFORMATION FROM RIPE OBJECT ****\nas-name: UNSPECIFIED\ndescr: EASInet\nimport: from AS690\n action pref=100;\n accept ANY\nimport: from AS513\n action pref=100;\n accept ANY\nimport: from AS559\n action pref=100;\n accept AS559\nimport: from AS697\n action pref=100;\n accept AS697\nexport: to AS690\n announce AS590\nexport: to AS513\n announce AS590\nexport: to AS559\n announce AS590\nexport: to AS697\n announce AS590\ndefault: to AS690\n action pref=100;\n networks ANY\ndefault: to AS513\n action pref=200;\n networks ANY\nstatus: LEGACY\nnotify: stf@easi.net\nmnt-by: RIPE-NCC-AN-MNT # WARNING: maintainer added to protect object\ncreated: 2002-09-19T15:23:42Z\nlast-modified: 2017-11-15T09:12:37Z\nsource: RIPE\nremarks: ****************************\nremarks: * THIS OBJECT IS MODIFIED\nremarks: * Please note that all data that is generally regarded as personal\nremarks: * data has been removed from this object.\nremarks: * To view the original object, please query the RIPE Database at:\nremarks: * http://www.ripe.net/whois\nremarks: ****************************\n","imports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"as_expr":"AS690"},"actions":{"pref":"100"}}],"mp_filter":{"path_attr":"ANY"}},{"mp_peerings":[{"mp_peering":{"as_expr":"AS513"},"actions":{"pref":"100"}}],"mp_filter":{"path_attr":"ANY"}},{"mp_peerings":[{"mp_peering":{"as_expr":"AS559"},"actions":{"pref":"100"}}],"mp_filter":{"path_attr":"AS559"}},{"mp_peerings":[{"mp_peering":{"as_expr":"AS697"},"actions":{"pref":"100"}}],"mp_filter":{"path_attr":"AS697"}}]}},"exports":{"any":{"any":[{"mp_peerings":[{"mp_peering":{"as_expr":"AS690"}}],"mp_filter":{"path_attr":"AS590"}},{"mp_peerings":[{"mp_peering":{"as_expr":"AS513"}}],"mp_filter":{"path_attr":"AS590"}},{"mp_peerings":[{"mp_peering":{"as_expr":"AS559"}}],"mp_filter":{"path_attr":"AS590"}},{"mp_peerings":[{"mp_peering":{"as_expr":"AS697"}}],"mp_filter":{"path_attr":"AS590"}}]}}}],"as_sets":[{"name":"AS-RESTENA","body":"descr: Reseau Teleinformatique de l'Education Nationale\ndescr: Educational and research network for Luxembourg\nmembers: AS2602\nmembers: AS42909\nmembers: AS51966\nmembers: AS-LXP\nmembers: AS-VDL\ntech-c: DUMY-RIPE\nadmin-c: DUMY-RIPE\nnotify: noc@restena.lu\nmnt-by: AS2602-MNT\ncreated: 1970-01-01T00:00:00Z\nlast-modified: 2022-09-08T09:11:41Z\nsource: RIPE\nremarks: ****************************\nremarks: * THIS OBJECT IS MODIFIED\nremarks: * Please note that all data that is generally regarded as personal\nremarks: * data has been removed from this object.\nremarks: * To view the original object, please query the RIPE Database at:\nremarks: * http://www.ripe.net/whois\nremarks: ****************************\n","members":["AS2602","AS42909","AS51966","AS-LXP","AS-VDL"]}],"route_sets":[{"name":"AS13646:RS-PEERLANS","body":"descr: Internet Exchange Peering LAN Routes\nmembers: 195.66.224.0/23\nmembers: 194.68.129.0/24\nmembers: 217.29.66.0/23\nmembers: 193.149.1.0/25\nmembers: 193.149.1.128/25\nmembers: 193.148.15.0/24\nmembers: 194.31.232.0/24\nmembers: 194.42.48.0/25\nmembers: 194.53.172.0/26\nmembers: 193.203.0.0/24\nadmin-c: DUMY-RIPE\ntech-c: DUMY-RIPE\nmnt-by: ZIGGO-SERVICES-MNT\ncreated: 1970-01-01T00:00:00Z\nlast-modified: 2020-01-21T15:43:54Z\nsource: RIPE\nremarks: ****************************\nremarks: * THIS OBJECT IS MODIFIED\nremarks: * Please note that all data that is generally regarded as personal\nremarks: * data has been removed from this object.\nremarks: * To view the original object, please query the RIPE Database at:\nremarks: * http://www.ripe.net/whois\nremarks: ****************************\n","members":["195.66.224.0/23","194.68.129.0/24","217.29.66.0/23","193.149.1.0/25","193.149.1.128/25","193.148.15.0/24","194.31.232.0/24","194.42.48.0/25","194.53.172.0/26","193.203.0.0/24"]},{"name":"RS-PEERING-TESTBED","body":"","members":["184.164.224.0/19^19-24","204.9.168.0/22^22-24","138.185.228.0/22^22-24","2804:269c::/32^32-32","2804:269c::/42^44-48","2804:269c:fe00::/43^48-48","2804:269c:fe40::/43^48-48","66.180.190.0/23^23-24","208.87.172.0/23^23-24","199.36.157.0/24^24-24","103.171.218.0/23^23-24","151.216.4.0/23^23-24","102.218.96.0/23^23-24","165.140.104.0/23^23-24","201.219.252.0/23^23-24","2604:4540:0000::/48^48-48","2604:4540:0080::/44^44-48","2620:33:c000::/48^48-48","2001:df7:5380::/47^47-48","2001:7fc:2::/47^47-48","2001:43f8:df0::/47^47-48","2620:9e:6000::/47^47-48","2801:1e:1800::/47^47-48"]}],"peering_sets":[{"name":"AS8785:prng-nyiix","body":"descr: NYIIX Peering Partners\npeering: AS2516 at 198.32.160.25\npeering: AS3257 at 198.32.160.29\npeering: AS4323 at 198.32.160.35\npeering: AS4436 at 198.32.160.53\npeering: AS4513 at 198.32.160.32\npeering: AS5496 at 198.32.160.16\npeering: AS6427 at 198.32.160.12\npeering: AS6461 at 198.32.160.22\npeering: AS6660 at 198.32.160.13\npeering: AS6667 at 198.32.160.41\npeering: AS6939 at 198.32.160.61\npeering: AS8001 at 198.32.160.20\npeering: AS8002 at 198.32.160.33\npeering: AS8220 at 198.32.160.34\npeering: AS8647 at 198.32.160.14\npeering: AS8966 at 198.32.160.45\npeering: AS9156 at 198.32.160.24\npeering: AS13768 at 198.32.160.65\npeering: AS13945 at 198.32.160.37\nadmin-c: DUMY-RIPE\ntech-c: DUMY-RIPE\nmnt-by: MISTRALNOC\ncreated: 2001-10-20T18:41:03Z\nlast-modified: 2005-10-10T11:47:35Z\nsource: RIPE\nremarks: ****************************\nremarks: * THIS OBJECT IS MODIFIED\nremarks: * Please note that all data that is generally regarded as personal\nremarks: * data has been removed from this object.\nremarks: * To view the original object, please query the RIPE Database at:\nremarks: * http://www.ripe.net/whois\nremarks: ****************************\n","peerings":[{"as_expr":"AS2516","router_expr2":"198.32.160.25"},{"as_expr":"AS3257","router_expr2":"198.32.160.29"},{"as_expr":"AS4323","router_expr2":"198.32.160.35"},{"as_expr":"AS4436","router_expr2":"198.32.160.53"},{"as_expr":"AS4513","router_expr2":"198.32.160.32"},{"as_expr":"AS5496","router_expr2":"198.32.160.16"},{"as_expr":"AS6427","router_expr2":"198.32.160.12"},{"as_expr":"AS6461","router_expr2":"198.32.160.22"},{"as_expr":"AS6660","router_expr2":"198.32.160.13"},{"as_expr":"AS6667","router_expr2":"198.32.160.41"},{"as_expr":"AS6939","router_expr2":"198.32.160.61"},{"as_expr":"AS8001","router_expr2":"198.32.160.20"},{"as_expr":"AS8002","router_expr2":"198.32.160.33"},{"as_expr":"AS8220","router_expr2":"198.32.160.34"},{"as_expr":"AS8647","router_expr2":"198.32.160.14"},{"as_expr":"AS8966","router_expr2":"198.32.160.45"},{"as_expr":"AS9156","router_expr2":"198.32.160.24"},{"as_expr":"AS13768","router_expr2":"198.32.160.65"},{"as_expr":"AS13945","router_expr2":"198.32.160.37"}]}],"filter_sets":[{"name":"FLTR-EUX","body":"filter: AS8785 AND AS13285\ndescr: test filter set 1\nmnt-by: MISTRALNOC\nadmin-c: DUMY-RIPE\ntech-c: DUMY-RIPE\ncreated: 2002-08-23T22:50:47Z\nlast-modified: 2005-10-10T11:47:30Z\nsource: RIPE\nremarks: ****************************\nremarks: * THIS OBJECT IS MODIFIED\nremarks: * Please note that all data that is generally regarded as personal\nremarks: * data has been removed from this object.\nremarks: * To view the original object, please query the RIPE Database at:\nremarks: * http://www.ripe.net/whois\nremarks: ****************************\n","filters":[{"and":{"left":{"path_attr":"AS8785"},"right":{"path_attr":"AS13285"}}}]}],"as_routes":{"AS10":["45.143.208.0/22"]}}"#;

pub fn expected_ast() -> Ast {
    Ast { aut_nums: vec![AutNum { name: "AS590".into(), body: "remarks:\nremarks: This aut-num has been transfered as part of the ERX.\nremarks: It was present in both the ARIN and RIPE databases, so\nremarks: the information from both databases has been merged.\nremarks: If you are the mntner of this object, please update it\nremarks: to reflect the correct information.\nremarks:\nremarks: Please see the FAQ for this process:\nremarks: http://www.ripe.net/db/erx/erx-asn/group3-faq.html\nremarks:\nremarks: **** INFORMATION FROM ARIN OBJECT ****\nremarks: as-name: EASINET-AS1\ndescr: EASInet Operations Center\n Riemenschneiderstrasse 11\n D-5300 Bonn 2\n DE\nadmin-c: DUMY-RIPE\ntech-c: DUMY-RIPE\nremarks: changed: hostmaster@arin.net 19900302\nremarks: changed: hostmaster@arin.net 19910416\nremarks:\nremarks: **** INFORMATION FROM RIPE OBJECT ****\nas-name: UNSPECIFIED\ndescr: EASInet\nimport: from AS690\n action pref=100;\n accept ANY\nimport: from AS513\n action pref=100;\n accept ANY\nimport: from AS559\n action pref=100;\n accept AS559\nimport: from AS697\n action pref=100;\n accept AS697\nexport: to AS690\n announce AS590\nexport: to AS513\n announce AS590\nexport: to AS559\n announce AS590\nexport: to AS697\n announce AS590\ndefault: to AS690\n action pref=100;\n networks ANY\ndefault: to AS513\n action pref=200;\n networks ANY\nstatus: LEGACY\nnotify: stf@easi.net\nmnt-by: RIPE-NCC-AN-MNT # WARNING: maintainer added to protect object\ncreated: 2002-09-19T15:23:42Z\nlast-modified: 2017-11-15T09:12:37Z\nsource: RIPE\nremarks: ****************************\nremarks: * THIS OBJECT IS MODIFIED\nremarks: * Please note that all data that is generally regarded as personal\nremarks: * data has been removed from this object.\nremarks: * To view the original object, please query the RIPE Database at:\nremarks: * http://www.ripe.net/whois\nremarks: ****************************\n".into(), imports: Versions { any: Casts { any: vec![Entry { mp_peerings: vec![PeeringAction { mp_peering: Peering { as_expr: Field("AS690".into()), router_expr1: None, router_expr2: None }, actions: BTreeMap::from([("pref".into(), Assigned("100".into()))]) }], mp_filter: PathAttr("ANY".into()) }, Entry { mp_peerings: vec![PeeringAction { mp_peering: Peering { as_expr: Field("AS513".into()), router_expr1: None, router_expr2: None }, actions: BTreeMap::from([("pref".into(), Assigned("100".into()))]) }], mp_filter: PathAttr("ANY".into()) }, Entry { mp_peerings: vec![PeeringAction { mp_peering: Peering { as_expr: Field("AS559".into()), router_expr1: None, router_expr2: None }, actions: BTreeMap::from([("pref".into(), Assigned("100".into()))]) }], mp_filter: PathAttr("AS559".into()) }, Entry { mp_peerings: vec![PeeringAction { mp_peering: Peering { as_expr: Field("AS697".into()), router_expr1: None, router_expr2: None }, actions: BTreeMap::from([("pref".into(), Assigned("100".into()))]) }], mp_filter: PathAttr("AS697".into()) }], unicast: vec![], multicast: vec![] }, ipv4: Casts::default(), ipv6: Casts::default() }, exports: Versions { any: Casts { any: vec![Entry { mp_peerings: vec![PeeringAction { mp_peering: Peering { as_expr: Field("AS690".into()), router_expr1: None, router_expr2: None }, actions: BTreeMap::new() }], mp_filter: PathAttr("AS590".into()) }, Entry { mp_peerings: vec![PeeringAction { mp_peering: Peering { as_expr: Field("AS513".into()), router_expr1: None, router_expr2: None }, actions: BTreeMap::new() }], mp_filter: PathAttr("AS590".into()) }, Entry { mp_peerings: vec![PeeringAction { mp_peering: Peering { as_expr: Field("AS559".into()), router_expr1: None, router_expr2: None }, actions: BTreeMap::new() }], mp_filter: PathAttr("AS590".into()) }, Entry { mp_peerings: vec![PeeringAction { mp_peering: Peering { as_expr: Field("AS697".into()), router_expr1: None, router_expr2: None }, actions: BTreeMap::new() }], mp_filter: PathAttr("AS590".into()) }], unicast: vec![], multicast: vec![] }, ipv4: Casts::default(), ipv6: Casts::default() } }], as_sets: vec![AsOrRouteSet { name: "AS-RESTENA".into(), body: "descr: Reseau Teleinformatique de l'Education Nationale\ndescr: Educational and research network for Luxembourg\nmembers: AS2602\nmembers: AS42909\nmembers: AS51966\nmembers: AS-LXP\nmembers: AS-VDL\ntech-c: DUMY-RIPE\nadmin-c: DUMY-RIPE\nnotify: noc@restena.lu\nmnt-by: AS2602-MNT\ncreated: 1970-01-01T00:00:00Z\nlast-modified: 2022-09-08T09:11:41Z\nsource: RIPE\nremarks: ****************************\nremarks: * THIS OBJECT IS MODIFIED\nremarks: * Please note that all data that is generally regarded as personal\nremarks: * data has been removed from this object.\nremarks: * To view the original object, please query the RIPE Database at:\nremarks: * http://www.ripe.net/whois\nremarks: ****************************\n".into(), members: vec!["AS2602".into(), "AS42909".into(), "AS51966".into(), "AS-LXP".into(), "AS-VDL".into()] }], route_sets: vec![AsOrRouteSet { name: "AS13646:RS-PEERLANS".into(), body: "descr: Internet Exchange Peering LAN Routes\nmembers: 195.66.224.0/23\nmembers: 194.68.129.0/24\nmembers: 217.29.66.0/23\nmembers: 193.149.1.0/25\nmembers: 193.149.1.128/25\nmembers: 193.148.15.0/24\nmembers: 194.31.232.0/24\nmembers: 194.42.48.0/25\nmembers: 194.53.172.0/26\nmembers: 193.203.0.0/24\nadmin-c: DUMY-RIPE\ntech-c: DUMY-RIPE\nmnt-by: ZIGGO-SERVICES-MNT\ncreated: 1970-01-01T00:00:00Z\nlast-modified: 2020-01-21T15:43:54Z\nsource: RIPE\nremarks: ****************************\nremarks: * THIS OBJECT IS MODIFIED\nremarks: * Please note that all data that is generally regarded as personal\nremarks: * data has been removed from this object.\nremarks: * To view the original object, please query the RIPE Database at:\nremarks: * http://www.ripe.net/whois\nremarks: ****************************\n".into(), members: vec!["195.66.224.0/23".into(), "194.68.129.0/24".into(), "217.29.66.0/23".into(), "193.149.1.0/25".into(), "193.149.1.128/25".into(), "193.148.15.0/24".into(), "194.31.232.0/24".into(), "194.42.48.0/25".into(), "194.53.172.0/26".into(), "193.203.0.0/24".into()] }, AsOrRouteSet { name: "RS-PEERING-TESTBED".into(), body: "".into(), members: vec!["184.164.224.0/19^19-24".into(), "204.9.168.0/22^22-24".into(), "138.185.228.0/22^22-24".into(), "2804:269c::/32^32-32".into(), "2804:269c::/42^44-48".into(), "2804:269c:fe00::/43^48-48".into(), "2804:269c:fe40::/43^48-48".into(), "66.180.190.0/23^23-24".into(), "208.87.172.0/23^23-24".into(), "199.36.157.0/24^24-24".into(), "103.171.218.0/23^23-24".into(), "151.216.4.0/23^23-24".into(), "102.218.96.0/23^23-24".into(), "165.140.104.0/23^23-24".into(), "201.219.252.0/23^23-24".into(), "2604:4540:0000::/48^48-48".into(), "2604:4540:0080::/44^44-48".into(), "2620:33:c000::/48^48-48".into(), "2001:df7:5380::/47^47-48".into(), "2001:7fc:2::/47^47-48".into(), "2001:43f8:df0::/47^47-48".into(), "2620:9e:6000::/47^47-48".into(), "2801:1e:1800::/47^47-48".into()] }], peering_sets: vec![PeeringSet { name: "AS8785:prng-nyiix".into(), body: "descr: NYIIX Peering Partners\npeering: AS2516 at 198.32.160.25\npeering: AS3257 at 198.32.160.29\npeering: AS4323 at 198.32.160.35\npeering: AS4436 at 198.32.160.53\npeering: AS4513 at 198.32.160.32\npeering: AS5496 at 198.32.160.16\npeering: AS6427 at 198.32.160.12\npeering: AS6461 at 198.32.160.22\npeering: AS6660 at 198.32.160.13\npeering: AS6667 at 198.32.160.41\npeering: AS6939 at 198.32.160.61\npeering: AS8001 at 198.32.160.20\npeering: AS8002 at 198.32.160.33\npeering: AS8220 at 198.32.160.34\npeering: AS8647 at 198.32.160.14\npeering: AS8966 at 198.32.160.45\npeering: AS9156 at 198.32.160.24\npeering: AS13768 at 198.32.160.65\npeering: AS13945 at 198.32.160.37\nadmin-c: DUMY-RIPE\ntech-c: DUMY-RIPE\nmnt-by: MISTRALNOC\ncreated: 2001-10-20T18:41:03Z\nlast-modified: 2005-10-10T11:47:35Z\nsource: RIPE\nremarks: ****************************\nremarks: * THIS OBJECT IS MODIFIED\nremarks: * Please note that all data that is generally regarded as personal\nremarks: * data has been removed from this object.\nremarks: * To view the original object, please query the RIPE Database at:\nremarks: * http://www.ripe.net/whois\nremarks: ****************************\n".into(), peerings: vec![Peering { as_expr: Field("AS2516".into()), router_expr1: None, router_expr2: Some(Field("198.32.160.25".into())) }, Peering { as_expr: Field("AS3257".into()), router_expr1: None, router_expr2: Some(Field("198.32.160.29".into())) }, Peering { as_expr: Field("AS4323".into()), router_expr1: None, router_expr2: Some(Field("198.32.160.35".into())) }, Peering { as_expr: Field("AS4436".into()), router_expr1: None, router_expr2: Some(Field("198.32.160.53".into())) }, Peering { as_expr: Field("AS4513".into()), router_expr1: None, router_expr2: Some(Field("198.32.160.32".into())) }, Peering { as_expr: Field("AS5496".into()), router_expr1: None, router_expr2: Some(Field("198.32.160.16".into())) }, Peering { as_expr: Field("AS6427".into()), router_expr1: None, router_expr2: Some(Field("198.32.160.12".into())) }, Peering { as_expr: Field("AS6461".into()), router_expr1: None, router_expr2: Some(Field("198.32.160.22".into())) }, Peering { as_expr: Field("AS6660".into()), router_expr1: None, router_expr2: Some(Field("198.32.160.13".into())) }, Peering { as_expr: Field("AS6667".into()), router_expr1: None, router_expr2: Some(Field("198.32.160.41".into())) }, Peering { as_expr: Field("AS6939".into()), router_expr1: None, router_expr2: Some(Field("198.32.160.61".into())) }, Peering { as_expr: Field("AS8001".into()), router_expr1: None, router_expr2: Some(Field("198.32.160.20".into())) }, Peering { as_expr: Field("AS8002".into()), router_expr1: None, router_expr2: Some(Field("198.32.160.33".into())) }, Peering { as_expr: Field("AS8220".into()), router_expr1: None, router_expr2: Some(Field("198.32.160.34".into())) }, Peering { as_expr: Field("AS8647".into()), router_expr1: None, router_expr2: Some(Field("198.32.160.14".into())) }, Peering { as_expr: Field("AS8966".into()), router_expr1: None, router_expr2: Some(Field("198.32.160.45".into())) }, Peering { as_expr: Field("AS9156".into()), router_expr1: None, router_expr2: Some(Field("198.32.160.24".into())) }, Peering { as_expr: Field("AS13768".into()), router_expr1: None, router_expr2: Some(Field("198.32.160.65".into())) }, Peering { as_expr: Field("AS13945".into()), router_expr1: None, router_expr2: Some(Field("198.32.160.37".into())) }] }], filter_sets: vec![FilterSet { name: "FLTR-EUX".into(), body: "filter: AS8785 AND AS13285\ndescr: test filter set 1\nmnt-by: MISTRALNOC\nadmin-c: DUMY-RIPE\ntech-c: DUMY-RIPE\ncreated: 2002-08-23T22:50:47Z\nlast-modified: 2005-10-10T11:47:30Z\nsource: RIPE\nremarks: ****************************\nremarks: * THIS OBJECT IS MODIFIED\nremarks: * Please note that all data that is generally regarded as personal\nremarks: * data has been removed from this object.\nremarks: * To view the original object, please query the RIPE Database at:\nremarks: * http://www.ripe.net/whois\nremarks: ****************************\n".into(), filters: vec![And { left: Box::new(PathAttr("AS8785".into())), right: Box::new(PathAttr("AS13285".into())) }] }], as_routes: btreemap! {"AS10".into()=> vec!["45.143.208.0/22".into()]} }
}
