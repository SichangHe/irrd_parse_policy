use std::net::{Ipv4Addr, Ipv6Addr};

use ipnet::*;
use parse::*;

use super::*;

use {AsPathEntry::*, MatchProblem::*, Report::*, ReportItem::*, SkipReason::*};

pub const RECURSION_LIMIT: isize = 0x100;

/// All information needed for a route to be compared to [`QueryDump`].
/// The main usage is to generate [`Report`]s with [`check`](#method.check).
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Compare {
    /// IP prefix propagated.
    pub prefix: IpNet,
    /// AS path for the propagation.
    pub as_path: Vec<AsPathEntry>,
    /// Recursion limit when checking against [`QueryDump`].
    /// Default to [`RECURSION_LIMIT`]
    pub recursion_limit: isize,
    /// [`Verbosity`] level when generating report.
    pub verbosity: Verbosity,
}

impl Compare {
    pub fn new(prefix: IpNet, mut as_path: Vec<AsPathEntry>) -> Self {
        as_path.dedup();
        as_path.shrink_to_fit();
        Self {
            prefix,
            as_path,
            recursion_limit: RECURSION_LIMIT,
            verbosity: Verbosity::default(),
        }
    }

    /// Set `self.verbosity`.
    pub fn verbosity(self, verbosity: Verbosity) -> Self {
        Self { verbosity, ..self }
    }

    /// Create [`Compare`] from a line of table dump generated by `bgpdump`
    /// on a MRT file.
    pub fn with_line_dump(line: &str) -> Result<Self> {
        let (prefix, as_path, _, _) = parse_table_dump(line)?;
        Ok(Self::new(prefix, as_path))
    }

    /// Check `self` against RPSL policy `dump` and generate reports.
    /// Depending on which [`Verbosity`] `self.verbose` is set to,
    /// the reports have different levels of details.
    /// If `verbosity.stop_at_err`, stops at the first erroneous AS pair.
    pub fn check(&self, dump: &QueryDump) -> Vec<Report> {
        if self.as_path.len() == 1 {
            return self.check_last_export(dump).into_iter().collect();
        }

        let mut reports = Vec::with_capacity(self.as_path.len() << 1);
        let reverse_as_path = self.as_path.iter().rev();
        // Iterate the pairs in `as_path` from right to left, with overlaps.
        for (from, to) in reverse_as_path.clone().zip(reverse_as_path.skip(1)) {
            if let (Seq(from), Seq(to)) = (from, to) {
                let r = self.check_pair(dump, *from, *to);
                if !r.is_empty() {
                    reports.extend(r);
                    if self.verbosity.stop_at_first {
                        break;
                    }
                }
            } else {
                reports.extend(self.verbosity.show_skips.then(|| AsPathPairWithSet {
                    from: from.clone(),
                    to: to.clone(),
                }));
            }
        }
        reports.shrink_to_fit();
        reports
    }

    pub fn check_last_export(&self, dump: &QueryDump) -> Option<Report> {
        match self.as_path.last()? {
            Seq(from) => match dump.aut_nums.get(from) {
                Some(from_an) => self.check_export(dump, from_an, *from, None),
                None => self.verbosity.show_skips.then(|| {
                    let items = aut_num_unrecorded_items(*from);
                    NeutralSingleExport { from: *from, items }
                }),
            },
            Set(from) => self
                .verbosity
                .show_skips
                .then(|| SetSingleExport { from: from.clone() }),
        }
    }

    pub fn check_pair(&self, dump: &QueryDump, from: u64, to: u64) -> Vec<Report> {
        let from_report = match dump.aut_nums.get(&from) {
            Some(from_an) => self.check_export(dump, from_an, from, Some(to)),
            None => self.verbosity.show_skips.then(|| {
                let items = aut_num_unrecorded_items(from);
                NeutralExport { from, to, items }
            }),
        };
        let from_report = match (from_report, self.verbosity.stop_at_first) {
            (Some(r), true) => return vec![r],
            (from_report, _) => from_report,
        };
        let to_report = match dump.aut_nums.get(&to) {
            Some(to_an) => self.check_import(dump, to_an, from, to),
            None => self.verbosity.show_skips.then(|| {
                let items = aut_num_unrecorded_items(to);
                NeutralImport { from, to, items }
            }),
        };
        [from_report, to_report].into_iter().flatten().collect()
    }

    pub fn check_export(
        &self,
        dump: &QueryDump,
        from_an: &AutNum,
        from: u64,
        to: Option<u64>,
    ) -> Option<Report> {
        if from_an.exports.is_default() {
            return self.verbosity.show_skips.then(|| {
                let items = vec![Skip(ExportEmpty)];
                match to {
                    Some(to) => NeutralExport { from, to, items },
                    None => NeutralSingleExport { from, items },
                }
            });
        }
        let (mut items, fail) = match self.check_compliant(dump, &from_an.exports, to) {
            None => {
                return self.verbosity.show_success.then_some(match to {
                    Some(to) => GoodExport { from, to },
                    None => GoodSingleExport { from },
                })
            }
            Some(report) => report,
        };
        items.shrink_to_fit();
        if fail {
            Some(match to {
                Some(to) => BadExport { from, to, items },
                None => BadSingeExport { from, items },
            })
        } else {
            self.verbosity.show_skips.then_some(match to {
                Some(to) => NeutralExport { from, to, items },
                None => NeutralSingleExport { from, items },
            })
        }
    }

    pub fn check_import(
        &self,
        dump: &QueryDump,
        to_an: &AutNum,
        from: u64,
        to: u64,
    ) -> Option<Report> {
        if to_an.imports.is_default() {
            return self.verbosity.show_skips.then(|| NeutralImport {
                from,
                to,
                items: vec![Skip(ImportEmpty)],
            });
        }
        let (mut items, fail) = match self.check_compliant(dump, &to_an.imports, Some(from)) {
            None => {
                return self
                    .verbosity
                    .show_success
                    .then_some(GoodImport { from, to })
            }
            Some(report) => report,
        };
        items.shrink_to_fit();
        if fail {
            Some(BadImport { from, to, items })
        } else {
            self.verbosity
                .show_skips
                .then_some(NeutralImport { from, to, items })
        }
    }

    pub fn check_compliant(
        &self,
        dump: &QueryDump,
        policy: &Versions,
        accept_num: Option<u64>,
    ) -> AnyReport {
        let mut aggregator: AnyReportAggregator = match self.prefix {
            IpNet::V4(_) => self.check_casts(dump, &policy.ipv4, accept_num),
            IpNet::V6(_) => self.check_casts(dump, &policy.ipv6, accept_num),
        }?
        .into();
        aggregator.join(self.check_casts(dump, &policy.any, accept_num)?);
        aggregator.to_any()
    }

    pub fn check_casts(
        &self,
        dump: &QueryDump,
        casts: &Casts,
        accept_num: Option<u64>,
    ) -> AnyReport {
        let mut aggregator = AnyReportAggregator::new();
        let specific_cast = match is_multicast(&self.prefix) {
            true => &casts.multicast,
            false => &casts.unicast,
        };
        for entry in [specific_cast, &casts.any].into_iter().flatten() {
            aggregator.join(self.check_entry(dump, entry, accept_num).to_any()?);
        }
        aggregator.to_any()
    }

    pub fn check_entry(
        &self,
        dump: &QueryDump,
        entry: &Entry,
        accept_num: Option<u64>,
    ) -> AllReport {
        let peering_report = match accept_num {
            Some(accept_num) => self
                .check_peering_actions(dump, &entry.mp_peerings, accept_num)
                .to_all()
                .map_err(|mut report| {
                    if self.verbosity.per_entry_err {
                        report.push(NoMatch(Peering));
                    }
                    report
                })?,
            None => None,
        };
        let filter_report = CheckFilter {
            dump,
            compare: self,
            verbosity: self.verbosity,
        }
        .check(&entry.mp_filter, self.recursion_limit)
        .to_all()
        .map_err(|mut report| {
            if self.verbosity.per_entry_err {
                report.push(NoMatch(Filter));
            }
            report
        })?;
        peering_report.join(filter_report).to_all()
    }

    pub fn check_peering_actions<'a, I>(
        &self,
        dump: &QueryDump,
        peerings: I,
        accept_num: u64,
    ) -> AnyReport
    where
        I: IntoIterator<Item = &'a PeeringAction>,
    {
        let mut aggregator = AnyReportAggregator::new();
        for peering_actions in peerings.into_iter() {
            let report = self.check_peering_action(dump, peering_actions, accept_num);
            aggregator.join(report.to_any()?);
        }
        aggregator.to_any()
    }

    pub fn check_peering_action(
        &self,
        dump: &QueryDump,
        peering_actions: &PeeringAction,
        accept_num: u64,
    ) -> AllReport {
        CheckPeering {
            dump,
            compare: self,
            accept_num,
            verbosity: self.verbosity,
        }
        .check(&peering_actions.mp_peering, self.recursion_limit)
        // Skipped.
        /* ?
        .join(self.check_actions(&peering_actions.actions)?)
        .to_all()
        */
    }

    /// We skip community checks, but this could be an enhancement.
    /// <https://github.com/SichangHe/parse_rpsl_policy/issues/16>.
    pub fn check_actions(&self, _actions: &Actions) -> AllReport {
        Ok(None)
    }

    pub fn goes_through_num(&self, num: u64) -> bool {
        self.as_path.iter().any(|p| p.contains_num(num))
    }
}

impl VerbosityReport for Compare {
    fn get_verbosity(&self) -> Verbosity {
        self.verbosity
    }
}

pub const MULTICAST_V4: Result<Ipv4Net, PrefixLenError> =
    Ipv4Net::new(Ipv4Addr::new(224, 0, 0, 0), 4);
pub const MULTICAST_V6: Result<Ipv6Net, PrefixLenError> =
    Ipv6Net::new(Ipv6Addr::new(0xff00, 0, 0, 0, 0, 0, 0, 0), 8);

/// Check if `prefix` is multicast.
pub fn is_multicast(prefix: &IpNet) -> bool {
    match prefix {
        IpNet::V4(prefix) => MULTICAST_V4
            .expect("MULTICAST_V4 is for sure Ok")
            .contains(prefix),
        IpNet::V6(prefix) => MULTICAST_V6
            .expect("MULTICAST_V6 is for sure Ok")
            .contains(prefix),
    }
}

fn aut_num_unrecorded_items(aut_num: u64) -> Vec<ReportItem> {
    vec![Skip(AutNumUnrecorded(aut_num))]
}
