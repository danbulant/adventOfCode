use std::{fs::File, io::{BufRead, BufReader}};

use anyhow::{anyhow, Result};
use aoc2024::{Context, Day};
use itertools::Itertools;

fn part1(ctx: &Context) -> Result<String> {
    let tuples = BufReader::new(File::open(&ctx.args.input)?).lines().map(|line| -> Result<_> {
        let line = line?;
        let nums = line.split_whitespace().map(|n| n.parse::<u32>()).filter_map(|n| n.ok()).collect_tuple::<(u32, u32)>().ok_or(anyhow!("Invalid input"))?;

        Ok(nums)
    });

    let mut first = Vec::new();
    let mut second = Vec::new();
    for res in tuples {
        let (x, y) = res?;
        first.push(x);
        second.push(y);
    }
    first.sort();
    second.sort();
    let sum: u32 = first.iter().zip(second).map(|(x, y)| x.abs_diff(y)).sum();

    Ok(sum.to_string())
}

fn main() {
    Day::new().part1(part1).run();
}