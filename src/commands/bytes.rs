#[derive(clap::Args, Clone, Debug)]
pub struct Command {
    /// The input to show the bytes of.
    input: String,
    /// The number base to show the byte values with.
    #[arg(short, value_enum, default_value_t)]
    format: BytesFormat,
    /// Return the output in a markdown code block.
    #[arg(short, default_value_t)]
    markdown: bool,
    /// Show more verbose output.
    #[arg(short, default_value_t)]
    verbose: bool,
}
impl super::Command for Command {
    fn execute(self) -> Result<super::DiscordOutput, !> {
        let Self {
            input,
            format,
            markdown,
            verbose,
        } = self;

        let mut output_string = String::new();
        let numbers = input.into_bytes();

        if verbose {
            let string = String::from_utf8_lossy(&numbers);
            output_string.push_str(&format!("Input: {}\nOutput: ", string.trim_end()));
        }

        let output: String = {
            numbers
                .into_iter()
                .map(format.byte_fmt())
                .collect::<Vec<String>>()
                .join(" ")
        };

        output_string.push_str(&output);

        let block = if markdown { Some("text") } else { None };

        Ok((block, output_string))
    }
}

#[derive(clap::ValueEnum, Clone, Copy, Default, Debug)]
pub enum BytesFormat {
    #[default]
    Hex,
    Bin,
    Dec,
    Oct,
}
impl BytesFormat {
    pub fn byte_fmt(&self) -> fn(u8) -> String {
        match self {
            BytesFormat::Hex => |x| format!("{x:0>2X}"),
            BytesFormat::Bin => |x| format!("{x:0>8b}"),
            BytesFormat::Dec => |x| format!("{x:0>3o}"),
            BytesFormat::Oct => |x| format!("{x}"),
        }
    }
}
