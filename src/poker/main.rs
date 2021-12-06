use gym_poker::{Poker};
// to read input
use std::io;
use std::io::Write;
use std::str::FromStr;
use std::num::ParseIntError; 

fn read_input() -> Result<u32,ParseIntError> {
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    let input = input.trim();
    let s: u32 = u32::from_str(&input)?;
    Ok(s)
}

fn main() {
    let mut poker = Poker::new(vec![100_000, 200_000], true);
    poker.reset();
    loop {
        poker.render();
        println!("Player {} select {:?}", poker.to_play(), actions_to_string(&poker.legal_actions()));
        let mut option;
        loop {
            if let Ok(e) = read_input() {
                option = e;
                if option <= 11 && poker.legal_actions().contains(&(option as u8)) {
                    break;
                } else {
                    println!("Enter a valid option, please");
                }
            } else {
                println!("Enter a valid option, please");
            }
        }
        let res = poker.step(option as u8, true);
        println!("Reward: {:?}", res.1);
        println!("Observation: {:?}: ", res.0);
        println!("All done: {}", res.2);
        if res.2 {
            break;
        }
    }
}

fn actions_to_string(actions: &Vec<u8>) -> String {
    let mut s = String::new();
    for action in actions.iter() {
        s.push_str(action_to_string(action));
    }
    s
}

fn action_to_string(action: &u8) -> &str {
    match action {
        11 => "(11).all in",
        10 => "(10).raise to $1000, ",
        9 => "(9).raise to $500, ",
        8 => "(8).raise to $100, ",
        7 => "(7).raise to $50, ",
        6 => "(6).raise to $25, ",
        5 => "(5).call, ",
        4 => "(4).bet, ",
        3 => "(3).check, ",
        2 => "(2).fold, ",
        1 => "(1).big blind",
        0 => "(0).small blind",
        _ => "Unknow action"
    }
}