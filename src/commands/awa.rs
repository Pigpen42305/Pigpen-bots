use std::iter::repeat_n;

const MAX: i64 = 34;
const MIN: i64 = 1;
const DEFAULT: u8 = 5;

#[derive(clap::Args, Clone, Copy, Debug)]
pub struct Command {
    /// The height of the awa.
    #[arg(value_parser = clap::value_parser!(u8).range(MIN..=MAX), default_value_t = DEFAULT)]
    input: u8,

    /// Uppercase AWA.
    #[arg(short, default_value_t)]
    upper: bool,
}
impl Command {
    fn build_row(full_height: u8, row: u8, upper: bool) -> String {
        let space = repeat_n(' ', ((full_height - 1) - row) as usize);
        let src = if upper { ['A', 'W'] } else { ['a', 'w'] };
        let iter = src.into_iter().cycle().take(((row << 1) + 1) as usize);
        space.chain(iter).collect()
    }
    fn build_pyramid(self) -> String {
        let mut layers = Vec::with_capacity(self.input as usize);
        for i in 0..(self.input) {
            let new = Self::build_row(self.input, i, self.upper);
            layers.push(new);
        }
        layers.join("\n")
    }
}
impl super::Command for Command {
    fn execute(self) -> Result<super::DiscordOutput, !> {
        Ok(super::DiscordOutput {
            block: Some("text"),
            content: self.build_pyramid(),
            suppress_embeds: true,
        })
    }
}
