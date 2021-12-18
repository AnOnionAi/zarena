# Zarena 
🦀 Rust Game Collection with Reninforcement Learning gym environments. 
This library aims to serve the same purpose as OpenSpiel, except in Rust to make it easier to use & maintain. The current games are gato, blackjack, chess & poker texas hold'em. All of these additionally support Web Assembly. You can play gato & chess against our Artificial Intelligence at [Zeti Games](https://zeti.ai/playground) 

## Configurations

Depending on the cargo file you want. You must change your cargo.toml to match that build.

`Cargo.py.toml` -> Python Build
`Cargo.rs.toml` -> Development Build
`Cargo.wa.toml` -> Web Assembly Build
`Cargo.toml` -> The actual file that Rust will build on. Copy from py/rs/wa to this file. 

## Commands
If you don't have Rust, no worries. Download Rust for Linux or Windows Subsystem. [If you need more help.](https://www.rust-lang.org/tools/install)

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

#### Download the C compiler

`sudo apt-get update && sudo apt-get install build-essential`

#### Install poetry

`curl -sSL https://raw.githubusercontent.com/python-poetry/poetry/master/get-poetry.py | python -`

#### Install Maturin via Poetry 

`poetry install`

#### Build the Maturin Develop Build 

`poetry run maturin develop`

#### Build the Maturin Test Build 

`poetry run maturin build`

#### Build the Maturin Production Build. The Python Wheel & source distribution. 

`poetry run maturin build --release`

#### Build the Web Assembly file

`wasm-pack build --target web -- --features wasm`

### Usage

You can import the Python classes directly, or create pre-defined environments with `gym` in this case it is also necessary to import the class:


```python
# import gym to use training environments
import gym
# from zarena import the training environment of your choice, for: 
# option 1.- use the python class directly 
# option 2.- register the environment in gym and use it with gym.make(environment_name)
from zarena import gym_chess

env = gym_chess.ChessEnv() # Option 1
env = gym.make('ChessEnv-v3') # Option 2

# reset the environment and get the initial state observation
observation = env.reset()

# obtain legal actions
actions = env.legal_actions()

# select action according to a criterion, in this case random
action = random.choice(actions)

# pass it to the env and get the next state observation, reward, if the game is over and environment information
observation, reward, done, info = env.step(action)

# get the player to play
env.to_play()

# properly close the game
env.close()

# display the game ovservation
env.render()
```

## Environments id

- Tictactoe: `TictactoeR-v2`
- Chess: `ChessEnv-v3`
- Blackjack: `BlackjackR-v1`
- Poker: `PokerR-v1`

## Testing

Run all the tests with `pytest`.

## Code linting and fixing

Python code is formatted with [black](https://github.com/psf/black).

Rust code is formatted with `cargo fmt`.


## Building the Rust code

The environment uses a chess engine implemented in Rust that uses [PyO3](https://github.com/PyO3/pyo3) Maturin to bind to the Python interpreter. Rust is an amazing compiled language and this project holds 2 configurations:

- `Cargo.py.toml` is used to build the library into a Python module with maturin
- `Cargo.rs.toml` is used to build directly with `cargo` in Rust to access the library in the `main.rs` script for development
- `Cargo.wa.toml` is used to build to build for Javascript with Web Assembly. The games can be played via Web Assembly on Zeti's website https://zeti.ai 

Note: we haven't found a way to specify the Cargo toml file to either process, so copy the contents of the config you want to use into `Cargo.toml` to make it work.

## Game of Gato
The game of Xs & Os


### API
    
#### Initialize environment

```python
>>> env = BlackjackEnv(n_players=1)
```

- `n_players`: specify the number of players `2<=n_players<=7` (default: `1`)

#### Set actions

```python
>>> env.step(action)
```

- `action`: mark a position, could be `0<=action<=8`
```shell
> 0 | 1 | 2 
> 3 | 4 | 5 
> 6 | 7 | 8 
```

#### Available opponents
    
* random
* expert

<img src="https://i.imgur.com/qqK1mBc.jpeg" alt="gata" height="400"/>

#### Notes:

Tests not implemented yet. 

## Blackjack

### API
    
#### Initialize environment

```python
>>> env = BlackjackEnv(n_players=1)
```

- `n_players`: specify the number of players `2<=n_players<=7` (default: `1`)


#### Set actions

```python
>>> env.step(action)
```

- `action`: can be
    * `0` -> stand
    * `1` -> HIT
    * `2` -> double down
    * `3` -> pull apart (currently disabled)

![21](https://black-jack.com/es/wp-content/uploads/sites/5/2019/02/blackjack-3.jpg)

#### Notes:

Tests not implemented yet. 

## Chess


#### See the chess board and moves
    
Visualise the current state of the chess game:

```python

env.render()

```

```shell
    -------------------------
 8 |  ♖  ♘  ♗  ♕  ♔  ♗  ♘  ♖ |
 7 |  ♙  ♙  ♙  ♙  ♙  ♙  ♙  ♙ |
 6 |  .  .  .  .  .  .  .  . |
 5 |  .  .  .  .  .  .  .  . |
 4 |  .  .  .  .  .  .  .  . |
 3 |  .  .  .  .  .  .  .  . |
 2 |  ♟  ♟  ♟  ♟  ♟  ♟  ♟  ♟ |
 1 |  ♜  ♞  ♝  ♛  ♚  ♝  ♞  ♜ |
    -------------------------
      a  b  c  d  e  f  g  h
```


You can also visualise multiple moves:

```python
>>> moves = env.possible_moves
>>> env.render_moves(moves[10:12] + moves[16:18])
```

### API

#### Initialize environment

```python
>>> env = ChessEnv(player_color="WHITE", opponent="random", log=True, initial_state=DEFAULT_BOARD)
```

- `opponent`: can be `"random"`, `"none"` or a function. Tells the environment whether to use a bot that picks a random move, play against self or use a specific bot policy (default: `"random"`)
- `log`: `True` or `False`, specifies whether to log every move and render every new state (default: `True`)
- `initial_state`: initial board positions, the default value is the default chess starting board. You can specify a custom board. View scripts `gym_chess/test/` for some examples
- `player_color`: `"WHITE"` or `"BLACK"`, only useful if playing against a bot (default: `"WHITE"`)


```python
>>> env.get_possible_moves(state=state, player="WHITE", attack=False)
```

This method will calculate the possible moves. By default they are calculated at the current state for the current player (`state.current_player`).

- `state`: (optional) state for which to calculate the moves
- `player`: (optional) "WHITE" or "BLACK", specifies the player

#### Move specification:

Moves are encoded as either:
- a tuple of coordinates `((from_x, from_y), (to_x, to_y))`
- or a string e.g. `"CASTLE_KING_SIDE_WHITE"`, `"CASTLE_QUEEN_SIDE_BLACK"`, `"RESIGN"`

Moves are pre-calculated for every new state and stored in `possible_moves`.


#### Get State 

```python
>>> print(env.state['board'])
```

```shell
[[-3, -5, -4, -2, -1, -4, -5, -3],
 [-6, -6, -6, -6, -6, -6, -6, -6],
 [0, 0, 0, 0, 0, 0, 0, 0],
 [0, 0, 0, 0, 0, 0, 0, 0],
 [0, 0, 0, 0, 0, 0, 0, 0],
 [0, 0, 0, 0, 0, 0, 0, 0],
 [6, 6, 6, 6, 6, 6, 6, 6],
 [3, 5, 4, 2, 1, 4, 5, 3]]
```

Every integer represents a piece. Positive pieces are white and negative ones are black.

Piece IDs are stored in constants that can be imported.

```python
from gym_chess.envs.chess import (
    KING_ID,
    QUEEN_ID,
    ROOK_ID,
    BISHOP_ID,
    KNIGHT_ID,
    PAWN_ID,
)
```

The schema is:

```python
EMPTY_SQUARE_ID = 0
KING_ID = 1
QUEEN_ID = 2
ROOK_ID = 3
BISHOP_ID = 4
KNIGHT_ID = 5
PAWN_ID = 6
```

Additional information can be found in other attributes of the environment:

```python
env.current_player
env.white_king_castle_possible
env.white_queen_castle_possible
env.black_king_castle_possible
env.black_queen_castle_possible
env.white_king_on_the_board
env.black_king_on_the_board
```

![Fischer](https://upload.wikimedia.org/wikipedia/commons/thumb/9/9d/Bobby_Fischer_1960_in_Leipzig_in_color.jpg/375px-Bobby_Fischer_1960_in_Leipzig_in_color.jpg)

#### Notes:

En-passant has not been implemented yet. 

## Poker


### API
    
#### Initialize environment

```python
>>> env = PokerEnv(n_players=2, infinite_game=True)
```

- `n_players`: specify the number of players `2<=n_players<=9` (default: `2`)
- `infinite_game`: `True` or `False`, specify if players get their starting credit back after each round (default: `True`)

#### Set actions

```python
>>> env.step(action)
```

- `action`: can be
    * `0` -> small blind
    * `1` -> big blind
    * `2` -> fold
    * `3` -> check
    * `4` -> bet
    * `5` -> call
    * `6` -> raise to 25
    * `7` -> raise to 50
    * `8` -> raise to 100
    * `9` -> raise to 500
    * `10` -> raise to 1000
    * `11` -> all in

![alt text](https://media.wired.com/photos/5fbe703e534553a88817f988/master/w_640,c_limit/Sec_poker_914262206.jpg)

#### Notes:

Tests not implemented yet. 

## References

- https://github.com/PyO3/maturin
- https://github.com/genyrosk/gym-chess (Thanks to genyrosk for gym-chess)
- https://github.com/deepmind/open_spiel

## Contrbutions
Pull Request Are Welcomed! 

## License 
MIT 

## Social
[Discord](https://zetiai.slack.com/archives/C01G0HRJWPK/p1637616128002100)
[Twitter](https://twitter.com/ZetiAi)
[Youtube](https://www.youtube.com/channel/UC4f1XKeAqBsTuKHXLywXpEQ)
[Facebook](https://www.facebook.com/ZetiAI/)
