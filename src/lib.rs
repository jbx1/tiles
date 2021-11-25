#[macro_use]
extern crate lazy_static;

use crate::board::Board;
use crate::search::{SearchResult, State};

pub mod queue;
pub mod search;
pub mod board;

#[derive(Hash, Debug, Copy, Clone, Eq, PartialEq)]
struct BoardState {
    board: Board,
}

impl BoardState {
    fn new(board: Board) -> BoardState {
        BoardState { board }
    }
}

impl State for BoardState {
    fn successors(&self) -> Vec<Self> {
        let successor_boards = self.board.successors();
        let mut successor_states = Vec::with_capacity(successor_boards.len());

        for successor in self.board.successors() {
            successor_states.push(BoardState::new(successor));
        }

        successor_states
    }

    fn h(&self) -> f32 {
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

        assert!(result.is_some());

        if let Some(plan) = result {
            assert_eq!(plan.len(), 2);
            assert_eq!(*plan.last().unwrap(), GOAL);
        }
    }

    #[test]
    fn test_hard_board1() {
        let hard_board = Board::new([8, 6, 7, 2, 5, 4, 3, 0, 1]);

        println!("Starting A* search for hard board 1");
        let result = a_star_search(hard_board);

        assert!(result.is_some());

        if let Some(plan) = result {
            assert_eq!(plan.len(), 32);
            assert_eq!(*plan.last().unwrap(), GOAL);
        }
    }

    #[test]
    fn test_hard_board2() {
        let hard_board = Board::new([6, 4, 7, 8, 5, 0, 3, 2, 1]);

        println!("Starting A* search for hard board 2");
        let result = a_star_search(hard_board);

        assert!(result.is_some());

        if let Some(plan) = result {
            assert_eq!(plan.len(), 32);
            assert_eq!(*plan.last().unwrap(), GOAL);
        }
    }
}