use std::{fs::File, io::{BufRead, BufReader}};

use anyhow::Result;
use aoc2024::{Context, Day};
use itertools::Itertools;

fn part1(ctx: &Context) -> Result<String> {
    Ok(BufReader::new(File::open(&ctx.args.input)?).lines().map(|line| {
        let line = line.unwrap();
        let mut rising = None;
        let nums = line.split_whitespace().map(|n| n.parse::<u32>().unwrap()).tuple_windows().all(|(a,b)| valid(a,b, &mut rising));
    
        nums
    }).map(|r| r as i32).sum::<i32>().to_string())
}

fn valid(a: u32, b: u32, rising: &mut Option<bool>) -> bool {
    let diff = a.abs_diff(b);
    if !(1..=3).contains(&diff) { return false }
    match &rising {
        None => {
            *rising = Some(b > a);
            true
        },
        Some(r) => {
            if *r {
                b > a
            } else {
                b < a
            }
        }
    }
}

fn part2(ctx: &Context) -> Result<String> {
    Ok(BufReader::new(File::open(&ctx.args.input)?).lines().map(|line| {
        let mut found = false;
        let mut skip = 0;
        let line = line.unwrap();
        let mut rising = None;
        let nums = line.split_whitespace().map(|n| n.parse::<u32>().unwrap()).tuple_windows().all(|(a,b,c)| {
            if skip > 0 {
                skip -= 1;
                return true
            }
            if valid(a,b,&mut rising) {
                true
            } else {
                if found { return false }
                found = true;
                skip = 1;

                valid(a, c, &mut rising)
            }
        });
    
        nums
    })
    .map(|r| r as i32).sum::<i32>().to_string())
}

fn main() {
    Day::new()
        .part1(part1)
        .part2(part2)
        .run();
}