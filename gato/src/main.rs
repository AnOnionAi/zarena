use gym_gato::{gato};
use rand::Rng;
fn main() {
    let mut rng = rand::thread_rng();
    for _ in 0..10 {
        let mut game = gato::new();
        println!("{:}", game);
        let mut done = false;
        let mut reward = 0;
        while !done {
            let (_, _reward, _done) = game.step(game.legal_actions()[rng.gen_range(0..game.legal_actions().len())]);
            done = _done;
            reward = _reward;
            println!("{:}", game);
            println!("{}", reward);
        }
        println!("{}", reward);
    }
}
