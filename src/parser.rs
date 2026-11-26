use crate::{
    PREFIX,
    commands::{Commands, DiscordOutput},
};

#[derive(clap::Parser, Clone, Debug)]
#[clap(no_binary_name = true, name = PREFIX)]
pub struct BotParser {
    #[command(subcommand)]
    command: Commands,
}
impl BotParser {
    pub fn execute(self) -> Result<DiscordOutput, !> {
        match self.command {
            x => x.execute(),
        }
    }
}
