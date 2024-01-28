// SPDX-License-Identifier: Apache-2.0 OR MIT
// This file is @generated by cargo-config2-internal-codegen
// (gen_de function at tools/codegen/src/main.rs).
// It is not intended for manual editing.

#![cfg_attr(rustfmt, rustfmt::skip)]
use std::path::Path;
use crate::{error::Result, merge::Merge, value::SetPath};
impl Merge for crate::de::Config {
    fn merge(&mut self, from: Self, force: bool) -> Result<()> {
        self.alias.merge(from.alias, force)?;
        self.build.merge(from.build, force)?;
        self.doc.merge(from.doc, force)?;
        self.env.merge(from.env, force)?;
        self.future_incompat_report.merge(from.future_incompat_report, force)?;
        self.net.merge(from.net, force)?;
        self.registries.merge(from.registries, force)?;
        self.registry.merge(from.registry, force)?;
        self.target.merge(from.target, force)?;
        self.term.merge(from.term, force)?;
        Ok(())
    }
}
impl SetPath for crate::de::Config {
    fn set_path(&mut self, path: &Path) {
        self.alias.set_path(path);
        self.build.set_path(path);
        self.doc.set_path(path);
        self.env.set_path(path);
        self.future_incompat_report.set_path(path);
        self.net.set_path(path);
        self.registries.set_path(path);
        self.registry.set_path(path);
        self.target.set_path(path);
        self.term.set_path(path);
    }
}
impl Merge for crate::de::BuildConfig {
    fn merge(&mut self, from: Self, force: bool) -> Result<()> {
        self.jobs.merge(from.jobs, force)?;
        self.rustc.merge(from.rustc, force)?;
        self.rustc_wrapper.merge(from.rustc_wrapper, force)?;
        self.rustc_workspace_wrapper.merge(from.rustc_workspace_wrapper, force)?;
        self.rustdoc.merge(from.rustdoc, force)?;
        self.target.merge(from.target, force)?;
        self.target_dir.merge(from.target_dir, force)?;
        self.rustflags.merge(from.rustflags, force)?;
        self.rustdocflags.merge(from.rustdocflags, force)?;
        self.incremental.merge(from.incremental, force)?;
        self.dep_info_basedir.merge(from.dep_info_basedir, force)?;
        Ok(())
    }
}
impl SetPath for crate::de::BuildConfig {
    fn set_path(&mut self, path: &Path) {
        self.jobs.set_path(path);
        self.rustc.set_path(path);
        self.rustc_wrapper.set_path(path);
        self.rustc_workspace_wrapper.set_path(path);
        self.rustdoc.set_path(path);
        self.target.set_path(path);
        self.target_dir.set_path(path);
        self.rustflags.set_path(path);
        self.rustdocflags.set_path(path);
        self.incremental.set_path(path);
        self.dep_info_basedir.set_path(path);
    }
}
impl Merge for crate::de::TargetConfig {
    fn merge(&mut self, from: Self, force: bool) -> Result<()> {
        self.linker.merge(from.linker, force)?;
        self.runner.merge(from.runner, force)?;
        self.rustflags.merge(from.rustflags, force)?;
        Ok(())
    }
}
impl SetPath for crate::de::TargetConfig {
    fn set_path(&mut self, path: &Path) {
        self.linker.set_path(path);
        self.runner.set_path(path);
        self.rustflags.set_path(path);
    }
}
impl Merge for crate::de::DocConfig {
    fn merge(&mut self, from: Self, force: bool) -> Result<()> {
        self.browser.merge(from.browser, force)?;
        Ok(())
    }
}
impl SetPath for crate::de::DocConfig {
    fn set_path(&mut self, path: &Path) {
        self.browser.set_path(path);
    }
}
impl SetPath for crate::de::EnvConfigValue {
    fn set_path(&mut self, path: &Path) {
        match self {
            Self::Value(v) => {
                v.set_path(path);
            }
            Self::Table { value, force, relative } => {
                value.set_path(path);
                force.set_path(path);
                relative.set_path(path);
            }
        }
    }
}
impl Merge for crate::de::FutureIncompatReportConfig {
    fn merge(&mut self, from: Self, force: bool) -> Result<()> {
        self.frequency.merge(from.frequency, force)?;
        Ok(())
    }
}
impl SetPath for crate::de::FutureIncompatReportConfig {
    fn set_path(&mut self, path: &Path) {
        self.frequency.set_path(path);
    }
}
impl Merge for crate::de::NetConfig {
    fn merge(&mut self, from: Self, force: bool) -> Result<()> {
        self.retry.merge(from.retry, force)?;
        self.git_fetch_with_cli.merge(from.git_fetch_with_cli, force)?;
        self.offline.merge(from.offline, force)?;
        Ok(())
    }
}
impl SetPath for crate::de::NetConfig {
    fn set_path(&mut self, path: &Path) {
        self.retry.set_path(path);
        self.git_fetch_with_cli.set_path(path);
        self.offline.set_path(path);
    }
}
impl Merge for crate::de::RegistriesConfigValue {
    fn merge(&mut self, from: Self, force: bool) -> Result<()> {
        self.index.merge(from.index, force)?;
        self.token.merge(from.token, force)?;
        self.protocol.merge(from.protocol, force)?;
        Ok(())
    }
}
impl SetPath for crate::de::RegistriesConfigValue {
    fn set_path(&mut self, path: &Path) {
        self.index.set_path(path);
        self.token.set_path(path);
        self.protocol.set_path(path);
    }
}
impl Merge for crate::de::RegistryConfig {
    fn merge(&mut self, from: Self, force: bool) -> Result<()> {
        self.default.merge(from.default, force)?;
        self.token.merge(from.token, force)?;
        Ok(())
    }
}
impl SetPath for crate::de::RegistryConfig {
    fn set_path(&mut self, path: &Path) {
        self.default.set_path(path);
        self.token.set_path(path);
    }
}
impl Merge for crate::de::TermConfig {
    fn merge(&mut self, from: Self, force: bool) -> Result<()> {
        self.quiet.merge(from.quiet, force)?;
        self.verbose.merge(from.verbose, force)?;
        self.color.merge(from.color, force)?;
        self.progress.merge(from.progress, force)?;
        Ok(())
    }
}
impl SetPath for crate::de::TermConfig {
    fn set_path(&mut self, path: &Path) {
        self.quiet.set_path(path);
        self.verbose.set_path(path);
        self.color.set_path(path);
        self.progress.set_path(path);
    }
}
impl Merge for crate::de::TermProgress {
    fn merge(&mut self, from: Self, force: bool) -> Result<()> {
        self.when.merge(from.when, force)?;
        self.width.merge(from.width, force)?;
        Ok(())
    }
}
impl SetPath for crate::de::TermProgress {
    fn set_path(&mut self, path: &Path) {
        self.when.set_path(path);
        self.width.set_path(path);
    }
}
impl SetPath for crate::de::Flags {
    fn set_path(&mut self, path: &Path) {
        self.flags.set_path(path);
    }
}
impl SetPath for crate::de::ConfigRelativePath {
    fn set_path(&mut self, path: &Path) {
        self.0.set_path(path);
    }
}
impl SetPath for crate::de::PathAndArgs {
    fn set_path(&mut self, path: &Path) {
        self.path.set_path(path);
        self.args.set_path(path);
    }
}
impl SetPath for crate::de::StringList {
    fn set_path(&mut self, path: &Path) {
        self.list.set_path(path);
    }
}
impl SetPath for crate::de::StringOrArray {
    fn set_path(&mut self, path: &Path) {
        match self {
            Self::String(v) => {
                v.set_path(path);
            }
            Self::Array(v) => {
                v.set_path(path);
            }
        }
    }
}
