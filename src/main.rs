use std::collections::HashSet;
use std::fs;
use codenames_solver::solver::{Board, Color, Space};
use std::io::Write;

mod constants;

fn main() {
    let keys = fs::read_to_string("keys.txt").unwrap_or_else(|_| {
        fs::write("keys.txt", constants::KEYS).unwrap();
        constants::KEYS.to_string()
    });
    let mut keys: Vec<_> = keys
        .lines()
        .flat_map(|l| {
            let board = Board::from_str(l);
            let board_90 = board.rotate_90();
            let board_180 = board.rotate_180();
            let board_270 = board_90.rotate_180();
            [
                board,
                board_90,
                board_180,
                board_270,
            ]
        })
        .collect();
    let mut input = String::new();
    print!("Who goes first, (r)ed or (b)lue?\n> ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    let first = match input.trim().to_ascii_lowercase().as_str() {
        "r" | "red" => Color::Red,
        "b" | "blue" => Color::Blue,
        _ => panic!(),
    };
    keys = keys.into_iter().filter(|k| k.first() == first).collect();
    let mut board = Board::new(first);
    loop {
        println!("{}", board);
        println!("Valid keys: {}\n", keys.len());
        query_board_input();
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim().to_ascii_lowercase().as_str() {
            "a" | "about" => println!("{}", constants::ABOUT),
            "h" | "help" => println!("{}", constants::HELP),
            "u" | "undo" => unimplemented!(),
            "q" | "quit" => return,
            _ => ()
        }
        let mut in_chars = input.chars();
        let position = constants::POSITIONS.find(in_chars.next().unwrap()).unwrap();
        let value = Space::from_char(in_chars.next().unwrap()).unwrap();
        board.set(position, Some(value));
        keys = keys
            .into_iter()
            .filter(|k| k.get(position).unwrap() == value)
            .collect();
        let mut keys_iter = keys.iter();
        let first_key = keys_iter.next().unwrap();
        let first_spaces: HashSet<_> = (0_usize..25)
            .into_iter()
            .filter_map(|i| first_key.get(i).map(|s| (i, s)))
            .collect();
        let common_spaces = keys_iter.fold(first_spaces, |acc, k| {
            let spaces: HashSet<_> = (0_usize..25)
                .into_iter()
                .filter_map(|i| k.get(i).map(|s| (i, s)))
                .collect();
            acc.intersection(&spaces).copied().collect()
        });
        for (i, s) in common_spaces {
            board.set(i, Some(s));
        }
        if keys.len() == 1 {
            break
        }
    }
}

fn query_board_input() {
    println!(
        r"[a][b][c][d][e]
[f][g][h][i][j]
[k][l][m][n][o]
[p][q][r][s][t]
[u][v][w][x][y]"
    );
    print!("> ");
    std::io::stdout().flush().unwrap();
}
