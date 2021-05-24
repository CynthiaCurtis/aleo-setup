use setup1_contributor::{
    cli::{Command, Options},
    commands::{contribute_subcommand, generate_keys},
};

use i18n_embed::{DesktopLanguageRequester, LanguageRequester};
use structopt::StructOpt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup localization of the `age` dependency.
    let age_localizer = age::localizer();
    let language_requester = DesktopLanguageRequester::new();
    age_localizer.select(&language_requester.requested_languages())?;

    let opts = Options::from_args();

    match opts.subcommand {
        Command::Generate(generate_opts) => generate_keys(generate_opts),
        Command::Contribute(contribute_opts) => {
            contribute_subcommand(&contribute_opts).await?;
        }
    }

    Ok(())
}