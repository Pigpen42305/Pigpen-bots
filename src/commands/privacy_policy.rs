const PP: &str = "https://pigpen42305.github.io/Pigpen-bots/privacy-policy.html";

#[derive(clap::Args, Clone, Copy, Debug)]
pub struct Command {
    /// Show the embed.
    #[arg(short, default_value_t)]
    embeds: bool,
}
impl super::Command for Command {
    fn execute(self) -> Result<super::DiscordOutput, !> {
        Ok(super::DiscordOutput {
            block: None,
            content: PP.into(),
            suppress_embeds: !self.embeds,
        })
    }
}
