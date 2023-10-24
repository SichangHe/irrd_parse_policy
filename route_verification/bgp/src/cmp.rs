use std::net::{Ipv4Addr, Ipv6Addr};

use as_path_regex::interpreter::{InterpretErr::*, Interpreter};
use ipnet::*;
use parse::*;

use super::*;

use {
    as_regex::AsRegex, AllReportCase::*, AnyReportCase::*, AsPathEntry::*, Report::*, ReportItem::*,
};

pub mod as_regex;
mod compliance;
mod filter;
mod hill;
mod peering;

pub(crate) use filter::*;
pub use {compliance::*, peering::*};

pub const RECURSION_LIMIT: isize = 0x100;

/// All information needed for a route to be compared to [`QueryIr`].
/// The main usage is to generate [`Report`]s with [`check`](#method.check).
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Compare {
    /// IP prefix propagated.
    pub prefix: IpNet,
    /// AS path for the propagation.
    pub as_path: Vec<AsPathEntry>,
    /// Recursion limit when checking against [`QueryIr`].
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

    /// Check `self` against RPSL policy `query` and generate reports.
    /// Depending on which [`Verbosity`] `self.verbosity` is set to,
    /// the reports have different levels of details.
    /// If `verbosity.stop_at_first`, stops at the first report.
    pub fn check(&self, query: &QueryIr) -> Vec<Report> {
        if self.as_path.len() == 1 {
            return self.check_last_export(query).into_iter().collect();
        }

        let mut reports = Vec::with_capacity(self.as_path.len() << 1);
        let path = self.as_path.iter();
        // Iterate the pairs in `as_path` from right to left, with overlaps.
        for ((index, from), to) in path.clone().enumerate().rev().zip(path.rev().skip(1)) {
            if let (Seq(from), Seq(to)) = (from, to) {
                let r = self.check_pair(query, *from, *to, &self.as_path[index..]);
                if !r.is_empty() {
                    reports.extend(r);
                    if self.verbosity.stop_at_first {
                        break;
                    }
                }
            } else {
                reports.extend(self.verbosity.record_set.then(|| AsPathPairWithSet {
                    from: from.clone(),
                    to: to.clone(),
                }));
            }
        }
        reports.shrink_to_fit();
        reports
    }

    pub fn check_last_export(&self, query: &QueryIr) -> Option<Report> {
        match self.as_path.last()? {
            Seq(from) => match query.aut_nums.get(from) {
                Some(from_an) => self.check_export(query, from_an, *from, None, &[]),
                None => self.verbosity.show_unrec.then(|| {
                    let items = aut_num_unrecorded_items(*from);
                    UnrecSingleExport { from: *from, items }
                }),
            },
            Set(from) => self
                .verbosity
                .record_set
                .then(|| SetSingleExport { from: from.clone() }),
        }
    }

    /// `prev_path` is previous path for `to`.
    pub fn check_pair(
        &self,
        query: &QueryIr,
        from: u64,
        to: u64,
        prev_path: &[AsPathEntry],
    ) -> Vec<Report> {
        let from_report = match query.aut_nums.get(&from) {
            Some(from_an) => self.check_export(query, from_an, from, Some(to), prev_path),
            None => self.verbosity.show_unrec.then(|| {
                let items = aut_num_unrecorded_items(from);
                UnrecExport { from, to, items }
            }),
        };
        let from_report = match (from_report, self.verbosity.stop_at_first) {
            (Some(r), true) => return vec![r],
            (from_report, _) => from_report,
        };
        let to_report = match query.aut_nums.get(&to) {
            Some(to_an) => self.check_import(query, to_an, from, to, prev_path),
            None => self.verbosity.show_unrec.then(|| {
                let items = aut_num_unrecorded_items(to);
                UnrecImport { from, to, items }
            }),
        };
        [from_report, to_report].into_iter().flatten().collect()
    }

    pub fn check_export(
        &self,
        query: &QueryIr,
        from_an: &AutNum,
        from: u64,
        to: Option<u64>,
        prev_path: &[AsPathEntry],
    ) -> Option<Report> {
        if from_an.exports.is_default() {
            return self.verbosity.show_skips.then(|| {
                let items = vec![SkipExportEmpty];
                match to {
                    Some(to) => SkipExport { from, to, items },
                    None => SkipSingleExport { from, items },
                }
            });
        }
        let mut report = match (Compliance {
            cmp: self,
            query,
            accept_num: to,
            self_num: from,
            export: true,
            prev_path: &prev_path[prev_path.len().min(1)..],
        })
        .check(&from_an.exports)
        {
            None => {
                return self.verbosity.show_success.then_some(match to {
                    Some(to) => OkExport { from, to },
                    None => OkSingleExport { from },
                })
            }
            Some(report) => report,
        };
        report.shrink_to_fit();
        match report {
            SkipAnyReport(items) => self.verbosity.show_skips.then_some(match to {
                Some(to) => SkipExport { from, to, items },
                None => SkipSingleExport { from, items },
            }),
            UnrecAnyReport(items) => self.verbosity.show_unrec.then_some(match to {
                Some(to) => UnrecExport { from, to, items },
                None => UnrecSingleExport { from, items },
            }),
            MehAnyReport(items) => self.verbosity.show_meh.then_some(match to {
                Some(to) => MehExport { from, to, items },
                None => MehSingleExport { from, items },
            }),
            BadAnyReport(items) => Some(match to {
                Some(to) => BadExport { from, to, items },
                None => BadSingleExport { from, items },
            }),
        }
    }

    pub fn check_import(
        &self,
        query: &QueryIr,
        to_an: &AutNum,
        from: u64,
        to: u64,
        prev_path: &[AsPathEntry],
    ) -> Option<Report> {
        if to_an.imports.is_default() {
            return self.verbosity.show_skips.then(|| SkipImport {
                from,
                to,
                items: vec![SkipImportEmpty],
            });
        }
        let mut report = match (Compliance {
            cmp: self,
            query,
            accept_num: Some(from),
            self_num: to,
            export: false,
            prev_path,
        })
        .check(&to_an.imports)
        {
            None => return self.verbosity.show_success.then_some(OkImport { from, to }),
            Some(report) => report,
        };
        report.shrink_to_fit();
        match report {
            SkipAnyReport(items) => {
                self.verbosity
                    .show_skips
                    .then_some(SkipImport { from, to, items })
            }
            UnrecAnyReport(items) => {
                self.verbosity
                    .show_unrec
                    .then_some(UnrecImport { from, to, items })
            }
            MehAnyReport(items) => self
                .verbosity
                .show_meh
                .then_some(MehImport { from, to, items }),
            BadAnyReport(items) => Some(BadImport { from, to, items }),
        }
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
    vec![UnrecordedAutNum(aut_num)]
}
