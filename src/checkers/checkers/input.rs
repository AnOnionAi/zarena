use super::board::BoardPosition;

#[derive(Debug, PartialEq, Eq)]
pub enum InputError {
    TooFewTokens,
    InvalidTokens { tokens: Vec<TokenError> },
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenError {
    MissingFile { token: String },
    MissingRank { token: String },
    ZeroRank { token: String },
    InvalidCharacter { token: String, char_index: usize },
}

///
/// Parse a move from a string
///
pub fn parse_move(the_move: &str) -> Result<Vec<BoardPosition>, InputError> {
    let results: Vec<_> = the_move.split_whitespace().map(token_validator).collect();

    let (ok_iter, err_iter): (Vec<_>, Vec<_>) = results
        .into_iter()
        .map(|result| match result {
            Ok(v) => (Some(v), None),
            Err(e) => (None, Some(e)),
        })
        .unzip();

    let errors: Vec<_> = err_iter.into_iter().filter_map(|error| error).collect();

    if !errors.is_empty() {
        return Err(InputError::InvalidTokens { tokens: errors });
    }

    let positions: Vec<_> = ok_iter
        .into_iter()
        .filter_map(|position| position)
        .collect();

    if positions.len() < 2 {
        return Err(InputError::TooFewTokens);
    }

    Ok(positions)
}

//
// Determines whether a position string is valid.
// Expects a strict sequence of alphabetic characters (rank)
// followed by a sequence of numeric characters (file).
//
fn token_validator(token: &str) -> Result<BoardPosition, TokenError> {
    let (file, rank) = parse_file_rank(token)?;

    if file.is_empty() {
        return Err(TokenError::MissingFile {
            token: token.to_string(),
        });
    }
    if rank.is_empty() {
        return Err(TokenError::MissingRank {
            token: token.to_string(),
        });
    }

    let row: usize = rank.parse::<usize>().unwrap();
    let col: usize = file_to_row_position(&file);

    if row == 0 {
        return Err(TokenError::ZeroRank {
            token: token.to_string(),
        });
    }

    Ok(BoardPosition::new(row - 1, col - 1))
}

enum ParseState {
    File,
    Rank,
}

//
// Parse a string and return a tuple containing
// the file and rank, respectively
//
fn parse_file_rank(token: &str) -> Result<(String, String), TokenError> {
    let mut file: String = String::new();
    let mut rank: String = String::new();

    let mut iter = token.chars().enumerate();
    let mut char_opt = iter.next();
    let mut parse_state = ParseState::File;

    while char_opt.is_some() {
        let (index, ch) = char_opt.unwrap();
        match parse_state {
            ParseState::File => {
                if ch.is_alphabetic() {
                    file.push(ch);
                    char_opt = iter.next();
                } else if ch.is_numeric() {
                    parse_state = ParseState::Rank;
                } else {
                    return Err(TokenError::InvalidCharacter {
                        token: token.to_string(),
                        char_index: index,
                    });
                }
            }
            ParseState::Rank => {
                if ch.is_numeric() {
                    rank.push(ch);
                    char_opt = iter.next();
                } else {
                    return Err(TokenError::InvalidCharacter {
                        token: token.to_string(),
                        char_index: index,
                    });
                }
            }
        }
    }

    Ok((file, rank))
}

//
// Convert string of alphabetic characters to an index
//
fn file_to_row_position(file: &str) -> usize {
    let mut row: usize = 0;
    let alphabet_length = 26;

    for c in file.chars() {
        row = row * alphabet_length + char_to_position(c);
    }

    row
}

//
// Convert a single alphabetic character to number
// Case insensitive [a-z] -> [1-26]
//
fn char_to_position(c: char) -> usize {
    debug_assert!(c.is_alphabetic());

    match c {
        'A'..='Z' => (c as usize) - ('A' as usize) + 1,
        'a'..='z' => (c as usize) - ('a' as usize) + 1,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod test {

    use super::BoardPosition;
    use super::*;

    fn test_parse_move(the_move: &str, exp_result: Vec<BoardPosition>) {
        let result = parse_move(the_move).ok().unwrap();

        assert_eq!(exp_result, result);
    }

    ptest!(test_parse_move[
        test_parse_move_a1_a1("a1 a1", vec![BoardPosition::new(0, 0), BoardPosition::new(0, 0)]),
        test_parse_move_a2_a1("a2 a1", vec![BoardPosition::new(1, 0), BoardPosition::new(0, 0)]),
        test_parse_move_a1_a2("a1 a2", vec![BoardPosition::new(0, 0), BoardPosition::new(1, 0)]),
        test_parse_move_a2_a2("a2 a2", vec![BoardPosition::new(1, 0), BoardPosition::new(1, 0)]),
        test_parse_move_aa1_aa1("aa1 aa1", vec![BoardPosition::new(0, 26), BoardPosition::new(0, 26)]),
        test_parse_move_aa1_ab1("aa1 ab1", vec![BoardPosition::new(0, 26), BoardPosition::new(0, 27)]),
        test_parse_move_ab1_aa1("ab1 aa1", vec![BoardPosition::new(0, 27), BoardPosition::new(0, 26)]),
        test_parse_move_yy99_zz99("yy99 zz99", vec![BoardPosition::new(98, 674), BoardPosition::new(98, 701)]),
        test_parse_move_aaa99_aaa99("aaa99 aaa99", vec![BoardPosition::new(98, 702), BoardPosition::new(98, 702)]),
        test_parse_move_xfd13_ahh37("xfd13 ahh37", vec![BoardPosition::new(12, 16383), BoardPosition::new(36, 891)]),
        test_parse_move_xx123_yy456_zz789("xx123 yy456 zz789", vec![BoardPosition::new(122, 647), BoardPosition::new(455, 674), BoardPosition::new(788, 701)])
    ]);

    fn test_parse_move_fail(the_move: &str, exp_result: InputError) {
        let result = parse_move(the_move).err().unwrap();

        assert_eq!(exp_result, result);
    }

    ptest!(test_parse_move_fail[
        test_parse_move_fail_too_few_tokens_empty("", InputError::TooFewTokens),
        test_parse_move_fail_too_few_tokens_a1("a1", InputError::TooFewTokens),
        test_parse_move_fail_too_few_tokens_z9("z9", InputError::TooFewTokens),

        test_parse_move_fail_missing_rank_a1_a("a1 a",
            InputError::InvalidTokens {
                tokens : vec![TokenError::MissingRank {
                    token : "a".to_string() } ] }),

        test_parse_move_fail_missing_file_a1_1("a1 1",
            InputError::InvalidTokens {
                tokens : vec![TokenError::MissingFile {
                    token : "1".to_string() } ] }),

        test_parse_move_fail_missing_rank_file_a_1("a 1",
            InputError::InvalidTokens {
                tokens : vec![
                    TokenError::MissingRank {
                        token : "a".to_string() },
                    TokenError::MissingFile {
                        token : "1".to_string() } ] }),

        test_parse_move_fail_zero_rank("a1 a0",
            InputError::InvalidTokens {
                tokens : vec![
                    TokenError::ZeroRank {
                        token : "a0".to_string() } ] }),

        test_parse_move_fail_invalid_character("a1 a$",
            InputError::InvalidTokens {
                tokens : vec![
                    TokenError::InvalidCharacter {
                        token : "a$".to_string(),
                        char_index : 1 } ] }),

        test_parse_move_fail_invalid_characters("#1 a$",
            InputError::InvalidTokens {
                tokens : vec![
                    TokenError::InvalidCharacter {
                        token : "#1".to_string(),
                        char_index : 0 },
                    TokenError::InvalidCharacter {
                        token : "a$".to_string(),
                        char_index : 1 } ] })
    ]);
}
