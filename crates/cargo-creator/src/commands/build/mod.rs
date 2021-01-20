pub mod android;
pub mod apple;
mod build_context;

pub use build_context::*;

use android::AndroidBuildCommand;
use apple::AppleBuildCommand;

use crate::error::Result;
use clap::Clap;
use creator_tools::Config;
use std::path::PathBuf;

#[derive(Clap, Clone, Debug)]
pub enum BuildCommand {
    Android(AndroidBuildCommand),
    Apple(AppleBuildCommand),
}

impl BuildCommand {
    pub fn handle_command(&self, config: &Config, current_dir: PathBuf) -> Result<()> {
        match &self {
            Self::Android(cmd) => cmd.run(config, current_dir),
            Self::Apple(cmd) => cmd.run(config, current_dir),
        }
    }
}

#[derive(Clap, Clone, Debug)]
pub struct SharedBuildCommand {
    /// Build the specified example.
    #[clap(long)]
    pub example: Option<String>,
    /// Space or comma separated list of features to activate. These features only apply to the current
    /// directory's package. Features of direct dependencies may be enabled with `<dep-name>/<feature-name>` syntax.
    /// This flag may be specified multiple times, which enables all specified features.
    #[clap(long)]
    pub features: Vec<String>,
    /// Activate all available features of selected package.
    #[clap(long)]
    pub all_features: bool,
    /// Do not activate the `default` feature of the current directory's package.
    #[clap(long)]
    pub no_default_features: bool,
    /// Build optimized artifact with the `release` profile.
    #[clap(long)]
    pub release: bool,
    /// Directory for generated artifact and intermediate files.
    #[clap(long)]
    pub target_dir: Option<PathBuf>,
}
