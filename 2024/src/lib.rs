use anyhow::Result;
use clap::Parser;
use colored_diff::PrettyDifference;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Which part to run (empty for both)
    #[arg(short, long)]
    pub part: Option<u8>,

    /// Which input file to use
    /// Will try to find an output file by replacing the extension with .out
    #[arg(short, long)]
    pub input: String,
}

#[derive(Debug)]
pub struct Context {
    pub args: Args,
    pub day: u8,
    pub output1: String,
    pub output2: String,
}

impl Context {
    fn new() -> Self {
        let args = Args::parse();
        let day = std::env::current_exe()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .parse()
            .unwrap();
        let mut output1 = args.input.clone();
        let mut output2 = args.input.clone();
        if let Some(pos) = output1.rfind('.') {
            output1.replace_range(pos.., ".part1.out");
            output2.replace_range(pos.., ".part2.out");
        }
        Self { args, day, output1, output2 }
    }

    fn output(&self, part: u8) -> &str {
        match part {
            1 => &self.output1,
            2 => &self.output2,
            _ => unreachable!()
        }
    }

    fn read_output(&self, part: u8) -> Option<String> {
        std::fs::read_to_string(&self.output(part)).ok()
    }
}

type Part = fn(&Context) -> Result<String>;

#[derive(Debug, Default)]
pub struct Day {
    part1: Option<Part>,
    part2: Option<Part>,
}

impl Day {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn part1(mut self, f: Part) -> Self {
        self.part1 = Some(f);
        self
    }

    pub fn part2(mut self, f: Part) -> Self {
        self.part2 = Some(f);
        self
    }

    fn has_part(&self, part: u8) -> bool {
        match part {
            1 => self.part1.is_some(),
            2 => self.part2.is_some(),
            _ => unreachable!()
        }
    }

    fn verify_part(&self, part: u8, ctx: &Context) -> Result<()> {
        println!("Day {} Part {}", ctx.day, part);
        let expected = ctx.read_output(part);
        let start = std::time::Instant::now();
        let result = match part {
            1 => self.part1.as_ref().unwrap()(ctx),
            2 => self.part2.as_ref().unwrap()(ctx),
            _ => unreachable!()
        }?;
        let elapsed = start.elapsed();
        println!("Elapsed: {:?}", elapsed);
        if let Some(expected) = expected {
            println!("{}", PrettyDifference {
                expected: &expected,
                actual: &result
            });
            if result != expected {
                std::process::exit(1);
            }
        } else {
            println!("{}", result);
            let path = ctx.output(part).to_string() + ".run";
            std::fs::write(&path, &result)?;
            println!("Output written to {} - no example output provided, nothing to diff.", path);
        }
        println!("OK");
        Ok(())
    }
    
    pub fn run(self) {
        let ctx = Context::new();
        match ctx.args.part {
            Some(i) => self.verify_part(i, &ctx).unwrap(),
            _ => {
                for i in 1..=2 {
                    if !self.has_part(i) {
                        continue;
                    }
                    self.verify_part(i, &ctx).unwrap();
                }
            }
        }
    }
}