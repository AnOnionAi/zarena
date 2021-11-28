use gym_blackjack::TwentyOne;
// to read input
use std::io;
use std::io::Write;
use std::num::ParseIntError;
use std::str::FromStr;

fn read_input() -> Result<u32, ParseIntError> {
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .ok()
        .expect("Failed to read line");
    let input = input.trim();
    let edad: u32 = u32::from_str(&input)?;
    Ok(edad)
}

fn main() {
    println!("New PLay");
    println!("Number of players:");
    let mut option;
    loop {
        if let Ok(e) = read_input() {
            option = e;
            if option <= 7 {
                break;
            }
            println!("7 is the maximum number of players");
        } else {
            println!("invalid option");
        }
    }
    let n_players = option as usize;
    let mut game = TwentyOne::new(n_players);
    game.reset();
    loop {
        for player in 1..game.get_total_players() {
            println!("{:?}", game.render());
            println!("{:?}", game.legal_actions());
            println!("Player {} select a bet: 4->$1, 5->$5, 6->$10, 7->$25, 8->$50, 9->$100, 10->$500, 11->$1000", player);
            let mut option;
            loop {
                if let Ok(e) = read_input() {
                    option = e;
                    if option <= 11 {
                        break;
                    } else {
                        println!("Enter a valid option, please");
                    }
                } else {
                    println!("Enter a valid option, please");
                }
            }
            let result = game.step(option as u8, true);
            println!(
                "Observation {:?} current player {} ",
                result.0,
                game.to_play()
            );
            println!("Reward {:?} ", result.1);
        }
        'outer: loop {
            loop {
                println!("{:?}", game.render());
                let legal_actions = game.legal_actions();
                println!("{:?}", legal_actions);
                print!("Player: {} ", game.to_play());
                if legal_actions[0] {
                    print!("Stand: 0  ");
                }
                if legal_actions[1] {
                    print!("HIT: 1  ");
                }
                if legal_actions[2] {
                    print!("Double down: 2  ");
                }
                println!();
                let mut option;
                loop {
                    if let Ok(e) = read_input() {
                        option = e;
                        if option <= 4 && legal_actions[option as usize] {
                            break;
                        }
                        println!("Enter a valid option, please");
                    } else {
                        println!("Enter a valid option, please");
                    }
                }
                let done = game.step(option as u8, true);
                println!("Observation {:?} current player {}", done.0, game.to_play());
                println!("Reward {:?} ", done.1);
                if done.2 {
                    println!("<-----------------Round Finished------------------->");
                    game.render();
                    let (_, rewards, _) = &done;
                    for player in 0..game.get_total_players() {
                        print!("Player {}: {} ", player, rewards[player as usize]);
                    }
                    println!("");
                    println!("");
                    game.reset();
                    break 'outer;
                }
            }
        }
    }
}
