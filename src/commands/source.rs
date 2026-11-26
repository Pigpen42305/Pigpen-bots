const REPO: &str = "https://github.com/Pigpen42305/Pigpen-bots";

#[derive(clap::Args, Clone, Copy, Debug)]
pub struct Command;
impl super::Command for Command {
    fn execute(self) -> Result<super::DiscordOutput, !> {
        Ok((None, REPO.into()))
    }
}
