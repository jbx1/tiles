use tiles::board::Board;

fn process_plan(plan_opt: Option<Vec<Board>>) {
    match plan_opt {
        Some(plan) => {
            println!("Found plan of {} steps", plan.len());
            for board in plan {
                println!("{}", board);
            }
        }

        None => println!("Plan not found!")
    }
}

fn main() {

    //todo: read the board from the commandline
    let hard_board = Board::new([8, 6, 7, 2, 5, 4, 3, 0, 1]);

    println!("Starting A* search for hard board");
    process_plan(tiles::a_star_search(hard_board));
}
