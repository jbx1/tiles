#[macro_use]
extern crate lazy_static;

use crate::board::Board;
use crate::search::{SearchResult, State};

pub mod queue;
pub mod search;
pub mod board;

#[derive(Hash, Debug, Copy, Clone, Eq, PartialEq)]
struct BoardState {
    board: Board
}

impl BoardState {
    fn new(board: Board) -> BoardState {
        BoardState { board }
    }
}

impl State for BoardState {
    fn successors(&self) -> Vec<Self> {
        self.board.successors()
            .iter()
            .map(|board| BoardState::new(*board))
            .collect()
    }

    fn h(&self) -> f32 {
        //todo: cache this once computed, or move it out completely
        self.board.manhattan_dist() as f32
    }
}

fn goal_check(candidate: &BoardState) -> bool {
    candidate.board == board::GOAL
}

pub fn breadth_first_search(board: Board) -> Option<Vec<Board>> {
    let initial_state = BoardState::new(board);
    let result = search::breadth_first_search(&initial_state, goal_check);
    process_result(result)
}

pub fn ehc_search(board: Board) -> Option<Vec<Board>> {
    let initial_state = BoardState::new(board);
    let result = search::ehc_search(&initial_state, goal_check);
    process_result(result)
}

pub fn ehc_steepest_search(board: Board) -> Option<Vec<Board>> {
    let initial_state = BoardState::new(board);
    let result = search::ehc_steepest_search(&initial_state, goal_check);
    process_result(result)
}

pub fn greedy_best_first_search(board: Board) -> Option<Vec<Board>> {
    let initial_state = BoardState::new(board);
    let result = search::greedy_best_first_search(&initial_state, goal_check);
    process_result(result)
}

pub fn a_star_search(board: Board) -> Option<Vec<Board>> {
    let initial_state = BoardState::new(board);
    let result = search::a_star_search(&initial_state, goal_check);
    process_result(result)
}

fn process_result(result: SearchResult<BoardState>) -> Option<Vec<Board>> {
    println!("{:?}", result.statistics);
    match result.plan {
        Some(plan_states) => {
            let mut plan = Vec::with_capacity(plan_states.len());
            for state in plan_states {
                plan.push(state.board);
            }

            Some(plan)
        }

        None => None
    }
}

#[cfg(test)]
mod tests {
    use crate::board::GOAL;

    use super::*;

    #[test]
    fn test_easy_board() {
        let hard_board = Board::new([1, 2, 3, 4, 5, 6, 7, 0, 8]);

        println!("Starting A* search for hard board 1");
        let result = a_star_search(hard_board);

        expect_plan(result, 2);
    }

    #[test]
    fn test_hard_board1_a_star() {
        let hard_board = Board::new([8, 6, 7, 2, 5, 4, 3, 0, 1]);

        println!("Starting A* search for hard board 1:\n{}", hard_board);
        let result = a_star_search(hard_board);

        expect_plan(result, 32);
    }

    #[test]
    fn test_hard_board2_a_star() {
        let hard_board = Board::new([6, 4, 7, 8, 5, 0, 3, 2, 1]);

        println!("Starting A* search for hard board 2:\n{}", hard_board);
        let result = a_star_search(hard_board);

        expect_plan(result, 32);
    }

    #[test]
    fn test_hard_board1_ehc() {
        let tiles = [8, 6, 7, 2, 5, 4, 3, 0, 1];
        println!("Tiles: {:?}", tiles);
        let hard_board = Board::new(tiles);

        println!("Starting EHC search for hard board 1:\n{}", hard_board);
        let result = ehc_search(hard_board);

        expect_plan(result, 46);
    }

    #[test]
    fn test_hard_board2_ehc() {
        let tiles = [6, 4, 7, 8, 5, 0, 3, 2, 1];
        println!("Tiles: {:?}", tiles);
        let hard_board = Board::new(tiles);

        println!("Starting EHC search for hard board 2:\n{}", hard_board);
        let result = ehc_search(hard_board);

        expect_plan(result, 46);
    }

    #[test]
    fn test_hard_board1_ehc_steepest() {
        let tiles = [8, 6, 7, 2, 5, 4, 3, 0, 1];
        println!("Tiles: {:?}", tiles);
        let hard_board = Board::new(tiles);

        println!("Starting EHC search for hard board 1:\n{}", hard_board);
        let result = ehc_steepest_search(hard_board);

        expect_plan(result, 46);
    }

    #[test]
    fn test_hard_board2_ehc_steepest() {
        let tiles = [6, 4, 7, 8, 5, 0, 3, 2, 1];
        println!("Tiles: {:?}", tiles);
        let hard_board = Board::new(tiles);

        println!("Starting EHC search for hard board 2:\n{}", hard_board);
        let result = ehc_steepest_search(hard_board);

        expect_plan(result, 46);
    }

    fn expect_plan(result: Option<Vec<Board>>, len: usize) {
        assert!(result.is_some());

        if let Some(plan) = result {
            let goal_state = plan.last().unwrap();
            assert_eq!(plan.len(), len);
            assert_eq!(*goal_state, GOAL);
            println!("Plan length: {:?}", plan.len());
            println!("Goal board state found:\n{}", goal_state);
        }
    }
}