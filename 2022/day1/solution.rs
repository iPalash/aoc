use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut elf_totals = Vec::new();
    let mut current_total = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            elf_totals.push(current_total);
            current_total = 0;
        } else {
            current_total += line.trim().parse::<i32>().unwrap();
        }
    }
    elf_totals.push(current_total);

    let max_elf_total = *elf_totals.iter().max().unwrap();
    println!("{}", max_elf_total);
}

fn part2() {
    let file = File::open("input").unwrap();
    let reader = BufReader::new(file);

    let mut elf_totals = Vec::new();
    let mut current_total = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.trim().is_empty() {
            elf_totals.push(current_total);
            current_total = 0;
        } else {
            current_total += line.trim().parse::<i32>().unwrap();
        }
    }
    elf_totals.push(current_total);
    elf_totals.sort_by(|a, b| b.cmp(a));
    let max_elf_total: i32 = elf_totals.iter().take(3).sum();
    println!("{}", max_elf_total);
}


fn main() {
    part1();
    part2();
}
