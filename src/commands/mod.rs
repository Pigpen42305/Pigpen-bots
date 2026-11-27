mod awa;
mod bytes;
mod privacy_policy;
mod source;
mod terms_of_service;

pub struct DiscordOutput {
    pub block: Option<&'static str>,
    pub content: String,
    pub suppress_embeds: bool,
}

trait Command: clap::Args {
    fn execute(self) -> Result<DiscordOutput, !>;
}

#[derive(clap::Subcommand, Clone, Debug)]
#[non_exhaustive]
pub enum Commands {
    /// Show the bytes of some text.
    Bytes(bytes::Command),
    /// Summon an awawa pyramid...
    Awa(awa::Command),
    /// Get a link to this bot's repository!
    Source(source::Command),
    /// Get a link to this bot's privacy policy.
    PrivacyPolicy(privacy_policy::Command),
    /// Get a link to this bot's terms of service.
    TermsOfService(terms_of_service::Command),
}
impl Commands {
    pub fn execute(self) -> Result<DiscordOutput, !> {
        #[allow(unreachable_patterns)]
        match self {
            Commands::Bytes(cmd) => cmd.execute(),
            Commands::Awa(cmd) => cmd.execute(),
            Commands::Source(cmd) => cmd.execute(),
            Commands::PrivacyPolicy(cmd) => cmd.execute(),
            Commands::TermsOfService(cmd) => cmd.execute(),
            _ => unreachable!(),
        }
    }
}
