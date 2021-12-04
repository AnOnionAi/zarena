use gym_tictactoe::{Tictactoe};
fn main() {
    // let mut rewards = Vec::new();
    let mut wins = [0;3];
    for _ in 0..10000 {
        let mut game = Tictactoe::new();
        let mut done = false;
        let mut reward = 0.0;
        while !done {
            let action;
            if game.to_play() == 0 {
                action = game.expert_action();
            } else {
                action = game.random_action();
            }
            let (_, _reward, _done) = game.step(action);
            done = _done;
            reward = _reward;
            // game.print();
            // print!("Observation: {:?}", observation);
            // println!();
        }
        // println!("Game over {}", reward);
        // rewards.push(reward);
        if reward == 0.5 {
            wins[2] += 1;
        } else {
            wins[game.to_play() as usize] += 1;
        }
    }
    let total_wins = wins[0] as f32 + wins[1] as f32 + wins[2] as f32;
    println!("X wins {}% - O wins {}% - Draw {}%", wins[1] as f32 / total_wins, wins[0] as f32 / total_wins, wins[2] as f32 / total_wins);
}
