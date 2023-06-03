use serde::{Deserialize, Serialize};
use Report::*;
use ReportItem::*;

/// Use this in an `Option`, and use `None` to indicate "good."
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Report {
    Neutral(Vec<ReportItem>),
    Bad(Vec<ReportItem>),
}

impl Report {
    pub fn skip(reason: String) -> Self {
        Neutral(vec![Skip(reason)])
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ReportItem {
    Skip(String),
    NoMatch(String),
    BadRpsl(String),
}

pub type ReportItems = Vec<ReportItem>;

pub trait JoinReportItems {
    fn join(self, other: Option<ReportItems>) -> Self;
}

impl JoinReportItems for Option<ReportItems> {
    fn join(self, other: Option<ReportItems>) -> Self {
        match self {
            Some(mut self_reports) => match other {
                Some(other_reports) => {
                    self_reports.extend(other_reports);
                    Some(self_reports)
                }
                None => Some(self_reports),
            },
            None => other,
        }
    }
}

/// Useful if all of the reports need to succeed.
/// - `Ok(Some(skips))` indicates skip.
/// - `Ok(None)` indicates success.
/// - `Err(errors)` indicates failure.
pub type AllReport = Result<Option<ReportItems>, ReportItems>;

pub fn skip_all_report(reason: String) -> AllReport {
    let skips = vec![Skip(reason)];
    Ok(Some(skips))
}

pub fn no_match_all_report(reason: String) -> AllReport {
    let errors = vec![NoMatch(reason)];
    Err(errors)
}

/// Useful if any of the reports succeeding is enough.
/// - `Some((errors, true))` indicates failure.
/// - `Some((skips, false))` indicates skip.
/// - `None` indicates success.
pub type AnyReport = Option<(ReportItems, bool)>;

pub fn skip_any_report(reason: String) -> AnyReport {
    let skips = vec![Skip(reason)];
    Some((skips, false))
}

pub fn no_match_any_report(reason: String) -> AnyReport {
    let errors = vec![NoMatch(reason)];
    Some((errors, true))
}

pub fn bad_rpsl_any_report(reason: String) -> AnyReport {
    let errors = vec![BadRpsl(reason)];
    Some((errors, true))
}

pub trait ToAnyReport {
    fn to_any(self) -> AnyReport;
}

impl ToAnyReport for AllReport {
    fn to_any(self) -> AnyReport {
        match self {
            Ok(Some(skips)) => Some((skips, false)),
            Ok(None) => None,
            Err(errors) => Some((errors, true)),
        }
    }
}

pub trait ToAllReport {
    fn to_all(self) -> AllReport;
}

impl ToAllReport for AnyReport {
    fn to_all(self) -> AllReport {
        match self {
            Some((errors, true)) => Err(errors),
            Some((skips, false)) => Ok(Some(skips)),
            None => Ok(None),
        }
    }
}

impl ToAllReport for Option<ReportItems> {
    fn to_all(self) -> AllReport {
        Ok(self)
    }
}

pub struct AnyReportAggregater {
    pub report_items: ReportItems,
    pub all_fail: bool,
}

impl AnyReportAggregater {
    pub fn new() -> Self {
        Self {
            report_items: vec![],
            all_fail: true,
        }
    }

    pub fn join(&mut self, (report_items, fail): (ReportItems, bool)) {
        self.report_items.extend(report_items);
        self.all_fail = self.all_fail || fail;
    }
}

impl ToAnyReport for AnyReportAggregater {
    fn to_any(self) -> AnyReport {
        Some((self.report_items, self.all_fail))
    }
}

impl Default for AnyReportAggregater {
    fn default() -> Self {
        Self::new()
    }
}

impl From<(ReportItems, bool)> for AnyReportAggregater {
    fn from((report_items, all_fail): (ReportItems, bool)) -> Self {
        Self {
            report_items,
            all_fail,
        }
    }
}
