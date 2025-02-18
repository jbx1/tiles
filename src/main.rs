use tiles::board::Board;
use std::env;
use std::process::exit;

fn process_plan(plan_opt: Option<Vec<Board>>) {
    match plan_opt {
        Some(plan) => {
            println!("Found plan of {} steps", plan.len() - 1);
            for board in plan {
                println!("{}", board);
            }
        }

        None => println!("Plan not found!")
    }
}

fn help() {
    println!("Specify your initial board configuration as a sequence of numbers from 0 to 8 (inclusive) separated by space, as command line arguments.");
    println!("The number 0 represent the empty blank space.");
    println!("For example: 1 2 5 3 4 6 7 8 0 represents the board");
    println!("  1 2 5");
    println!("  3 4 6");
    println!("  7 8 0");
}

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        help();
        exit(0);
    }

    assert_eq!(args.len(), 10, "Expecting 9 arguments in the range [0..8] (inclusive).");

    let mut tiles: [i8; 9]= [0; 9];

    for (index, arg) in args.iter().enumerate() {
        if index > 0 {
            match arg.parse::<i8>() {
                Ok(n) if n >= 0 && n <= 8 => tiles[index-1] = n,
                _ => panic!("Invalid argument: {} - Expecting 9 numeric arguments in the range [0..8] (inclusive).", arg)
            }
        }
    }
    //todo: explore using command line parameters such as CLAP https://docs.rs/clap/latest/clap/

    let board = Board::new(tiles);

    println!("Starting A* search with manhattan distance heuristic");
    process_plan(tiles::a_star_search(board, tiles::manhattan_distance_heuristic));

    println!("Starting Greedy Best First search");
    process_plan(tiles::greedy_best_first_search(board, tiles::displaced_tiles_heuristic));

    println!("Starting EHC search with manhattan distance heuristic");
    process_plan(tiles::ehc_search(board, tiles::manhattan_distance_heuristic));

    println!("Starting Breadth First Search");
    process_plan(tiles::breadth_first_search(board));
}
