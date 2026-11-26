mod awa;
mod bytes;
mod source;

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
}
impl Commands {
    pub fn execute(self) -> Result<DiscordOutput, !> {
        #[allow(unreachable_patterns)]
        match self {
            Commands::Bytes(cmd) => cmd.execute(),
            Commands::Awa(cmd) => cmd.execute(),
            Commands::Source(cmd) => cmd.execute(),
            _ => unreachable!(),
        }
    }
}
