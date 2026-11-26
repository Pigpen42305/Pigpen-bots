const REPO: &str = "https://github.com/Pigpen42305/Pigpen-bots";

#[derive(clap::Args, Clone, Copy, Debug)]
pub struct Command {
    /// Show the GitHub embed.
    #[arg(short, default_value_t)]
    embeds: bool,
}
impl super::Command for Command {
    fn execute(self) -> Result<super::DiscordOutput, !> {
        Ok(super::DiscordOutput {
            block: None,
            content: REPO.into(),
            suppress_embeds: !self.embeds,
        })
    }
}
