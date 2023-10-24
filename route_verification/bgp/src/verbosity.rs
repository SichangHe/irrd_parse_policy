use super::*;

#[allow(unused)] // For the doc.
use {Report::*, ReportItem::*};

/// Verbosity level.
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Verbosity {
    /// Stop checking the AS path at the first [`Report`].
    pub stop_at_first: bool,
    /// Report meh, or special cases.
    pub show_meh: bool,
    /// Report unrecorded.
    pub show_unrec: bool,
    /// Report skips.
    pub show_skips: bool,
    /// Report success.
    pub show_success: bool,
    /// Report error information for each RPSL policy entry.
    pub per_entry_err: bool,
    /// All errors.
    pub all_err: bool,
    /// Record [`AsPathPairWithSet`], [`SetSingleExport`].
    pub record_set: bool,
    /// Record [`SkipCommunityCheckUnimplemented`].
    pub record_community: bool,
    /// Mark routes from customer to provider as special.
    pub special_uphill: bool,
    /// Check for pseudo customer sets.
    pub check_customer: bool,
    /// Check for peers that only specify imports from providers.
    pub check_import_only_provider: bool,
}

impl std::fmt::Debug for Verbosity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = f.debug_set();
        let Verbosity {
            stop_at_first,
            show_meh,
            show_unrec,
            show_skips,
            show_success,
            per_entry_err,
            all_err,
            record_set,
            record_community,
            special_uphill,
            check_customer,
            check_import_only_provider,
        } = self;
        for (is_true, tag) in [
            (stop_at_first, "stop_at_first"),
            (show_meh, "show_meh"),
            (show_unrec, "show_unrec"),
            (show_skips, "show_skips"),
            (show_success, "show_success"),
            (per_entry_err, "per_entry_err"),
            (all_err, "all_err"),
            (record_set, "record_set"),
            (record_community, "record_community"),
            (special_uphill, "special_uphill"),
            (check_customer, "check_customer"),
            (check_import_only_provider, "check_import_only_provider"),
        ] {
            if *is_true {
                result.entry(&tag);
            }
        }
        result.finish()
    }
}

impl Verbosity {
    /// Report all errors, skips, and success but not the details.
    pub const fn minimum_all() -> Self {
        Self {
            stop_at_first: false,
            show_meh: true,
            show_unrec: true,
            show_skips: true,
            show_success: true,
            special_uphill: true,
            check_customer: true,
            check_import_only_provider: true,
            ..Self::least()
        }
    }

    pub const fn least() -> Self {
        Self {
            stop_at_first: true,
            show_meh: false,
            show_unrec: false,
            show_skips: false,
            show_success: false,
            per_entry_err: false,
            all_err: false,
            record_set: false,
            record_community: false,
            special_uphill: false,
            check_customer: false,
            check_import_only_provider: false,
        }
    }
}

impl Default for Verbosity {
    fn default() -> Self {
        Self::least()
    }
}

pub trait VerbosityReport {
    fn get_verbosity(&self) -> Verbosity;

    fn meh_import(&self, from: u64, to: u64, mut items: ReportItems, reason: ReportItem) -> Report {
        if self.get_verbosity().show_meh {
            items.push(reason)
        }
        MehImport { from, to, items }
    }

    fn meh_export(&self, from: u64, to: u64, mut items: ReportItems, reason: ReportItem) -> Report {
        if self.get_verbosity().show_meh {
            items.push(reason)
        }
        MehExport { from, to, items }
    }

    fn skip_any_report<F>(&self, reason: F) -> AnyReport
    where
        F: Fn() -> ReportItem,
    {
        if self.get_verbosity().show_skips {
            skip_any_report(reason())
        } else {
            empty_skip_any_report()
        }
    }

    fn skip_any_reports<F>(&self, reasons: F) -> AnyReport
    where
        F: Fn() -> ReportItems,
    {
        if self.get_verbosity().show_skips {
            skip_any_reports(reasons())
        } else {
            empty_skip_any_report()
        }
    }

    fn unrec_any_report<F>(&self, reason: F) -> AnyReport
    where
        F: Fn() -> ReportItem,
    {
        if self.get_verbosity().show_skips {
            unrec_any_report(reason())
        } else {
            empty_unrec_any_report()
        }
    }

    fn special_any_report<F>(&self, reason: F) -> AnyReport
    where
        F: Fn() -> ReportItem,
    {
        if self.get_verbosity().show_meh {
            special_any_report(reason())
        } else {
            empty_meh_any_report()
        }
    }

    fn bad_any_report<F>(&self, reason: F) -> AnyReport
    where
        F: Fn() -> ReportItem,
    {
        if self.get_verbosity().all_err {
            bad_any_report(reason())
        } else {
            empty_bad_any_report()
        }
    }

    fn skip_all_report<F>(&self, reason: F) -> AllReport
    where
        F: Fn() -> ReportItem,
    {
        if self.get_verbosity().show_skips {
            skip_all_report(reason())
        } else {
            empty_skip_all_report()
        }
    }

    fn bad_all_report<F>(&self, reason: F) -> AllReport
    where
        F: Fn() -> ReportItem,
    {
        if self.get_verbosity().all_err {
            bad_all_report(reason())
        } else {
            empty_bad_all_report()
        }
    }
}
