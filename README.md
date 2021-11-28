# Zarena 
🦀 Rust Game Collection with Reninforcement Learning gym environments. 
This library aims to serve the same purpose as OpenSpiel, except in Rust and more moedular. The current games are gato, blackjack, chess & poker. All of these additionally support Web Assembly. You can play gato & chess against our Ai at https://zeti.ai/playground 

# Configurations

Depending on the cargo file you want. You must change your cargo.toml to match that build.

# Commands
Download Rust for Linux or Windows Subsystem 

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Download the C compiler

`sudo apt-get update && sudo apt-get install build-essential`

Install poetry

`curl -sSL https://raw.githubusercontent.com/python-poetry/poetry/master/get-poetry.py | python -`

Install Maturin via Poetry 

`poetry install`

Build the Maturin Develop Build 

`poetry run maturin develop`

Build the Maturin Test Build 

`poetry run maturin build`

Build the Maturin Production Build. The Python Wheel & source distribution. 

`poetry run maturin build --release`

Build the WASM 

`wasm-pack build --target web -- --features wasm`

## Usage

You can import the Python classes directly, or create pre-defined environments with `gym`:


```python

import gym
from gym_chess import Zarena.ChessEnv

env = ChessEnv() # Option 1
env = gym.make('ChessEnv') # Option 2

# current state
state = env.state

# select a move and convert it into an action
moves = env.possible_moves
action = env.move_to_actions(move)

# or select an action directly
actions = env.possible_actions

# pass it to the env and get the next state
new_state, reward, done, info = env.step(action)

```

Reset the environment:

``` python

initial_state = env.reset()

# Testing

Run all the tests with `pytest`.

# Code linting and fixing

Python code is formatted with [black](https://github.com/psf/black).

Rust code is formatted with `cargo fmt`.


# Building the Rust code

The environment uses a chess engine implemented in Rust that uses [PyO3](https://github.com/PyO3/pyo3) Maturin to bind to the Python interpreter. Rust is an amazing compiled language and this project holds 2 configurations:

- `Cargo.py.toml` is used to build the library into a Python module with maturin
- `Cargo.rs.toml` is used to build directly with `cargo` in Rust to access the library in the `main.rs` script for development
- `Cargo.wa.toml` is used to build to build for Javascript with Web Assembly. The games can be played via Web Assembly on Zeti's website https://zeti.ai 

Note: we haven't found a way to specify the Cargo toml file to either process, so copy the contents of the config you want to use into `Cargo.toml` to make it work.

# Game of Gato
The game of Xs & Os

![alt text](https://imgur.com/a/NmOfLmo)

https://imgur.com/a/NmOfLmo

# Blackjack

![alt text](https://black-jack.com/es/wp-content/uploads/sites/5/2019/02/blackjack-3.jpg)

# Chess

![alt text](https://upload.wikimedia.org/wikipedia/commons/thumb/9/9d/Bobby_Fischer_1960_in_Leipzig_in_color.jpg/375px-Bobby_Fischer_1960_in_Leipzig_in_color.jpg)

<table style="text-align:center;border-spacing:0pt;font-family:'Arial Unicode MS'; border-collapse:collapse; border-color: black; border-style: solid; border-width: 0pt 0pt 0pt 0pt">
<tr>
<td style="width:12pt">8</td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 1pt 0pt 0pt 1pt"><span style="font-size:150%;">♜</span></td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 1pt 0pt 0pt 0pt" bgcolor="silver"><span style="font-size:150%;">♞</span></td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 1pt 0pt 0pt 0pt"><span style="font-size:150%;">♝</span></td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 1pt 0pt 0pt 0pt" bgcolor="silver"><span style="font-size:150%;">♛</span></td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 1pt 0pt 0pt 0pt"><span style="font-size:150%;">♚</span></td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 1pt 0pt 0pt 0pt" bgcolor="silver"><span style="font-size:150%;">♝</span></td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 1pt 0pt 0pt 0pt"><span style="font-size:150%;">♞</span></td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 1pt 1pt 0pt 0pt" bgcolor="silver"><span style="font-size:150%;">♜</span></td>
</tr>
<tr>
<td style="width:12pt">7</td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 0pt 0pt 0pt 1pt" bgcolor="silver"><span style="font-size:150%;">♟</span></td>
<td style="width:24pt; height:24pt;"><span style="font-size:150%;">♟</span></td>
<td style="width:24pt; height:24pt;" bgcolor="silver"><span style="font-size:150%;">♟</span></td>
<td style="width:24pt; height:24pt;"><span style="font-size:150%;">♟</span></td>
<td style="width:24pt; height:24pt;" bgcolor="silver"><span style="font-size:150%;">♟</span></td>
<td style="width:24pt; height:24pt;"><span style="font-size:150%;">♟</span></td>
<td style="width:24pt; height:24pt;" bgcolor="silver"><span style="font-size:150%;">♟</span></td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 0pt 1pt 0pt 0pt"><span style="font-size:150%;">♟</span></td>
</tr>
<tr>
<td style="width:12pt">6</td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 0pt 0pt 0pt 1pt"><span style="font-size:150%;"><br /></span></td>
<td style="width:24pt; height:24pt;" bgcolor="silver"></td>
<td style="width:24pt; height:24pt;"></td>
<td style="width:24pt; height:24pt;" bgcolor="silver"></td>
<td style="width:24pt; height:24pt;"></td>
<td style="width:24pt; height:24pt;" bgcolor="silver"></td>
<td style="width:24pt; height:24pt;"></td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 0pt 1pt 0pt 0pt" bgcolor="silver"></td>
</tr>
<tr>
<td style="width:12pt">5</td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 0pt 0pt 0pt 1pt" bgcolor="silver"><span style="font-size:150%;"><br /></span></td>
<td style="width:24pt; height:24pt;"></td>
<td style="width:24pt; height:24pt;" bgcolor="silver"></td>
<td style="width:24pt; height:24pt;"></td>
<td style="width:24pt; height:24pt;" bgcolor="silver"></td>
<td style="width:24pt; height:24pt;"></td>
<td style="width:24pt; height:24pt;" bgcolor="silver"></td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 0pt 1pt 0pt 0pt"></td>
</tr>
<tr>
<td style="width:12pt">4</td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 0pt 0pt 0pt 1pt"><span style="font-size:150%;"><br /></span></td>
<td style="width:24pt; height:24pt;" bgcolor="silver"></td>
<td style="width:24pt; height:24pt;"></td>
<td style="width:24pt; height:24pt;" bgcolor="silver"></td>
<td style="width:24pt; height:24pt;"></td>
<td style="width:24pt; height:24pt;" bgcolor="silver"></td>
<td style="width:24pt; height:24pt;"></td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 0pt 1pt 0pt 0pt" bgcolor="silver"></td>
</tr>
<tr>
<td style="width:12pt">3</td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 0pt 0pt 0pt 1pt" bgcolor="silver"><span style="font-size:150%;"><br /></span></td>
<td style="width:24pt; height:24pt;"></td>
<td style="width:24pt; height:24pt;" bgcolor="silver"></td>
<td style="width:24pt; height:24pt;"></td>
<td style="width:24pt; height:24pt;" bgcolor="silver"></td>
<td style="width:24pt; height:24pt;"></td>
<td style="width:24pt; height:24pt;" bgcolor="silver"></td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 0pt 1pt 0pt 0pt"></td>
</tr>
<tr>
<td style="width:12pt">2</td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 0pt 0pt 0pt 1pt"><span style="font-size:150%;">♙</span></td>
<td style="width:24pt; height:24pt;" bgcolor="silver"><span style="font-size:150%;">♙</span></td>
<td style="width:24pt; height:24pt;"><span style="font-size:150%;">♙</span></td>
<td style="width:24pt; height:24pt;" bgcolor="silver"><span style="font-size:150%;">♙</span></td>
<td style="width:24pt; height:24pt;"><span style="font-size:150%;">♙</span></td>
<td style="width:24pt; height:24pt;" bgcolor="silver"><span style="font-size:150%;">♙</span></td>
<td style="width:24pt; height:24pt;"><span style="font-size:150%;">♙</span></td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 0pt 1pt 0pt 0pt" bgcolor="silver"><span style="font-size:150%;">♙</span></td>
</tr>
<tr>
<td style="width:12pt">1</td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 0pt 0pt 1pt 1pt" bgcolor="silver"><span style="font-size:150%;">♖</span></td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 0pt 0pt 1pt 0pt"><span style="font-size:150%;">♘</span></td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 0pt 0pt 1pt 0pt" bgcolor="silver"><span style="font-size:150%;">♗</span></td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 0pt 0pt 1pt 0pt"><span style="font-size:150%;">♕</span></td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 0pt 0pt 1pt 0pt" bgcolor="silver"><span style="font-size:150%;">♔</span></td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 0pt 0pt 1pt 0pt"><span style="font-size:150%;">♗</span></td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 0pt 0pt 1pt 0pt" bgcolor="silver"><span style="font-size:150%;">♘</span></td>
<td style="width:24pt; height:24pt; border-collapse:collapse; border-color: black; border-style: solid; border-width: 0pt 1pt 1pt 0pt"><span style="font-size:150%;">♖</span></td>
</tr>
<tr>
<td></td>
<td>a</td>
<td>b</td>
<td>c</td>
<td>d</td>
<td>e</td>
<td>f</td>
<td>g</td>
<td>h</td>
</tr>
</table>


## Visualise the chess board and moves

Visualise the current state of the chess game:

``` python

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

moves = env.possible_moves
env.render_moves(moves[10:12] + moves[16:18])

```

## API

### Initialize environment

#### `ChessEnv(player_color="WHITE", opponent="random", log=True, initial_state=DEFAULT_BOARD)`

- `opponent`: can be `"random"`, `"none"` or a function. Tells the environment whether to use a bot that picks a random move, play against self or use a specific bot policy (default: `"random"`)
- `log`: `True` or `False`, specifies whether to log every move and render every new state (default: `True`)
- `initial_state`: initial board positions, the default value is the default chess starting board. You can specify a custom board. View scripts `gym_chess/test/v1` for some examples
- `player_color`: `"WHITE"` or `"BLACK"`, only useful if playing against a bot (default: `"WHITE"`)


#### `env.get_possible_moves(state=state, player="WHITE", attack=False)`

This method will calculate the possible moves. By default they are calculated at the current state for the current player (`state.current_player`).

- `state`: (optional) state for which to calculate the moves
- `player`: (optional) "WHITE" or "BLACK", specifies the player

## Move specification:

Moves are encoded as either:
- a tuple of coordinates `((from_x, from_y), (to_x, to_y))`
- or a string e.g. `"CASTLE_KING_SIDE_WHITE"`, `"CASTLE_QUEEN_SIDE_BLACK"`, `"RESIGN"`

Moves are pre-calculated for every new state and stored in `possible_moves`.


## State and differences between v1 and v2

```python
>>> print(env.state['board']) # v2
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

# Notes:

En-passant has not been implemented yet. 

# Poker

![alt text](https://media.wired.com/photos/5fbe703e534553a88817f988/master/w_640,c_limit/Sec_poker_914262206.jpg)

# References

- https://github.com/PyO3/maturin
- https://github.com/werner-duvaud/muzero-general
- https://github.com/genyrosk/gym-chess (Thanks to genyrosk for gym-chess)
- https://github.com/deepmind/open_spiel

# Contrbutions
Pull Request Are Welcomed! 

# License 
MIT 