//! What marks this build as Ray's own (branch `ray/master`) and not the official app.
//!
//! Only what a person reads carries the mark: the settings screen and
//! `herdr-app --version` (and, through the latter, the bundle's Info.plist, see
//! `scripts/install-macos`). Everything that compares or records versions
//! (self-update, release notes, cloned layouts, telemetry) keeps using the plain
//! `CARGO_PKG_VERSION`, so the mark can never make an update check or a notes
//! lookup misbehave.

/// Our own revision on top of the upstream release. Bump it (r1 -> r2) when a
/// build with new personal changes should look different from the previous one.
pub const REVISION: &str = "r2";

/// The version to show a person: the release number, a dot, then [`REVISION`],
/// e.g. `0.1.11.r1`.
///
/// Returns the text for display only; never feed it to a version comparison.
// TEST:src/fork.rs[version_is_the_release_number_plus_our_revision]
pub fn version() -> String {
    format!("{}.{REVISION}", env!("CARGO_PKG_VERSION"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_the_release_number_plus_our_revision() {
        let shown = version();
        // The release number stays whole at the front (upstream merges bump only that part).
        assert_eq!(shown.strip_prefix(env!("CARGO_PKG_VERSION")), Some(format!(".{REVISION}").as_str()));
        // The revision is an "r" and a number, the form people look for.
        assert!(REVISION.strip_prefix('r').is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit())));
    }
}
