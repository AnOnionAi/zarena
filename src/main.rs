mod gato;

fn main() {
    let mut game = gato::Tictactoe::new();
    game.reset();
    println!("{:?}", game.get_state());
}