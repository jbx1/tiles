use std::fmt::{Display, Formatter, Result};
use std::collections::HashMap;

pub const GOAL: Board = Board { tiles: [1, 2, 3, 4, 5, 6, 7, 8, 0], zero: 0 };

lazy_static! {
    static ref GOAL_MAP: HashMap<i8, usize> = {
        let mut goal_positions = HashMap::with_capacity(GOAL.tiles.len());
        for (index, tile) in GOAL.tiles.iter().enumerate() {
            goal_positions.insert(*tile, index);
        }

        goal_positions
    };
}

#[derive(Debug, Copy, Clone, Hash, Eq)]
pub struct Board {
    tiles: [i8; 9],
    zero: usize,
}

impl Board {
    pub fn new(tiles: [i8; 9]) -> Board {
        Board { tiles, zero: find_zero(tiles) }
    }

    pub fn is_goal(&self) -> bool {
        *self == GOAL
    }

    fn swap(&self, pos1: usize, pos2: usize) -> Board {
        assert!(pos1 < self.tiles.len());
        assert!(pos2 < self.tiles.len());

        if pos1 == pos2 {
            Board::new(self.tiles)
        } else {
            let mut swapped = self.tiles;
            let temp = swapped[pos1];
            swapped[pos1] = swapped[pos2];
            swapped[pos2] = temp;

            Board::new(swapped)
        }
    }

    /// Returns the successors of the current board configuration.
    pub fn successors(&self) -> Vec<Board> {
        let mut successors = Vec::with_capacity(self.successor_count());

        //left
        if self.zero % 3 != 2 {
            successors.push(self.swap(self.zero, self.zero + 1));
        }

        //up
        if self.zero <= 5 {
            successors.push(self.swap(self.zero, self.zero + 3));
        }

        //down
        if self.zero >= 3 {
            successors.push(self.swap(self.zero, self.zero - 3));
        }

        //right
        if self.zero % 3 != 0 {
            successors.push(self.swap(self.zero, self.zero - 1));
        }

        successors
    }

    /// Returns how many successors this board configuration should have
    /// Position 4 has 4 places to move, odd positions have 3 places, and the rest have 2
    fn successor_count(&self) -> usize {
        match self.zero {
            4 => 4,
            n if (n % 2 == 1) => 3,
            _ => 2
        }
    }

    /// Calculates the manhattan distance from GOAL
    pub fn manhattan_dist(&self) -> i32 {
        let mut distance = 0;
        for (index, tile) in self.tiles.iter().enumerate() {
            let goal_tile_pos = GOAL_MAP.get(&tile).unwrap();
            distance += manhattan_dist_positions(index, *goal_tile_pos);
        }

        distance
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.tiles == other.tiles
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut board_str = String::new();
        for (index, tile) in self.tiles.iter().enumerate() {
            board_str.push_str(&tile.to_string());
            if index % 3 == 2 {
                board_str.push_str("\r\n");
            } else {
                board_str.push_str(" ");
            }
        };

        write!(f, "{}", board_str)
    }
}

fn find_zero(tiles: [i8; 9]) -> usize {
    //we should always find 0, so panic if not
    tiles.iter().position(|&tile| tile == 0).unwrap()
}

fn manhattan_dist_positions(pos1: usize, pos2: usize) -> i32 {
    if pos1 == pos2 {
        0
    } else {
        let (x_pos1, y_pos1) = to_coordinates(pos1);
        let (x_pos2, y_pos2) = to_coordinates(pos2);

        (x_pos2 - x_pos1).abs() + (y_pos2 - y_pos1).abs()
    }
}

fn to_coordinates(pos: usize) -> (i32, i32) {
    ((pos % 3) as i32, (pos / 3) as i32)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality() {
        assert_eq!(Board::new([0, 1, 2, 3, 4, 5, 6, 7, 8]), GOAL);
    }

    #[test]
    fn test_inequality() {
        assert_ne!(Board::new([1, 0, 2, 3, 4, 5, 6, 7, 8]), GOAL);
    }

    #[test]
    fn test_goal() {
        assert!(Board::new([0, 1, 2, 3, 4, 5, 6, 7, 8]).is_goal());
    }

    #[test]
    fn test_not_goal() {
        assert_eq!(Board::new([1, 0, 2, 3, 4, 5, 6, 7, 8]).is_goal(), false);
    }

    #[test]
    fn test_zero_pos() {
        assert_eq!(find_zero([0, 1, 2, 3, 4, 5, 6, 7, 8]), 0);
        assert_eq!(find_zero([1, 0, 2, 3, 4, 5, 6, 7, 8]), 1);
        assert_eq!(find_zero([1, 8, 2, 3, 4, 5, 6, 7, 0]), 8);
    }

    #[test]
    fn test_board_initialisation() {
        let board = Board::new([0, 1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(board.zero, 0);

        let board = Board::new([1, 0, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(board.zero, 1);

        let board = Board::new([1, 8, 2, 3, 4, 5, 6, 7, 0]);
        assert_eq!(board.zero, 8);
    }

    #[test]
    #[should_panic]
    fn test_invalid_board_initialisation() {
        Board::new([9, 1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_swap() {
        let board = Board::new([0, 1, 2, 3, 4, 5, 6, 7, 8]);
        let swapped = board.swap(0, 5);

        assert_eq!(swapped, Board::new([5, 1, 2, 3, 4, 0, 6, 7, 8]));
    }

    #[test]
    fn test_swap_noop() {
        let board = Board::new([0, 1, 2, 3, 4, 5, 6, 7, 8]);
        let swapped = board.swap(5, 5);

        assert_eq!(swapped, Board::new([0, 1, 2, 3, 4, 5, 6, 7, 8]));
    }

    #[test]
    #[should_panic]
    fn test_swap_pos1_out_of_range() {
        let board = Board::new([0, 1, 2, 3, 4, 5, 6, 7, 8]);
        board.swap(9, 7);
    }

    #[test]
    #[should_panic]
    fn test_swap_pos2_out_of_range() {
        let board = Board::new([0, 1, 2, 3, 4, 5, 6, 7, 8]);
        board.swap(7, 9);
    }

    #[test]
    fn test_successor_count() {
        assert_eq!(Board::new([0, 1, 2, 3, 4, 5, 6, 7, 8]).successor_count(), 2);
        assert_eq!(Board::new([1, 0, 2, 3, 4, 5, 6, 7, 8]).successor_count(), 3);
        assert_eq!(Board::new([1, 2, 0, 3, 4, 5, 6, 7, 8]).successor_count(), 2);
        assert_eq!(Board::new([1, 2, 3, 0, 4, 5, 6, 7, 8]).successor_count(), 3);
        assert_eq!(Board::new([1, 2, 3, 4, 0, 5, 6, 7, 8]).successor_count(), 4);
        assert_eq!(Board::new([1, 2, 3, 4, 5, 0, 6, 7, 8]).successor_count(), 3);
        assert_eq!(Board::new([1, 2, 3, 4, 5, 6, 0, 7, 8]).successor_count(), 2);
        assert_eq!(Board::new([1, 2, 3, 4, 5, 6, 7, 0, 8]).successor_count(), 3);
        assert_eq!(Board::new([1, 2, 3, 4, 5, 6, 7, 8, 0]).successor_count(), 2);
    }

    #[test]
    fn test_successor_0() {
        let successors = Board::new([0, 1, 2, 3, 4, 5, 6, 7, 8]).successors();
        assert_eq!(successors.len(), 2);
        assert!(successors.contains(&Board::new([1, 0, 2, 3, 4, 5, 6, 7, 8])));
        assert!(successors.contains(&Board::new([3, 1, 2, 0, 4, 5, 6, 7, 8])));
    }

    #[test]
    fn test_successor_1() {
        let successors = Board::new([1, 0, 2, 3, 4, 5, 6, 7, 8]).successors();
        assert_eq!(successors.len(), 3);
        assert!(successors.contains(&Board::new([0, 1, 2, 3, 4, 5, 6, 7, 8])));
        assert!(successors.contains(&Board::new([1, 2, 0, 3, 4, 5, 6, 7, 8])));
        assert!(successors.contains(&Board::new([1, 4, 2, 3, 0, 5, 6, 7, 8])));
    }

    #[test]
    fn test_successor_4() {
        let successors = Board::new([1, 2, 3, 4, 0, 5, 6, 7, 8]).successors();
        assert_eq!(successors.len(), 4);
        assert!(successors.contains(&Board::new([1, 0, 3, 4, 2, 5, 6, 7, 8])));
        assert!(successors.contains(&Board::new([1, 2, 3, 4, 7, 5, 6, 0, 8])));
        assert!(successors.contains(&Board::new([1, 2, 3, 0, 4, 5, 6, 7, 8])));
        assert!(successors.contains(&Board::new([1, 2, 3, 4, 5, 0, 6, 7, 8])));
    }

    #[test]
    fn test_successor_6() {
        let successors = Board::new([1, 2, 3, 4, 5, 0, 6, 7, 8]).successors();
        assert_eq!(successors.len(), 3);
        assert!(successors.contains(&Board::new([1, 2, 3, 4, 0, 5, 6, 7, 8])));
        assert!(successors.contains(&Board::new([1, 2, 0, 4, 5, 3, 6, 7, 8])));
        assert!(successors.contains(&Board::new([1, 2, 3, 4, 5, 8, 6, 7, 0])));
    }

    #[test]
    fn test_successor_8() {
        let successors = Board::new([1, 2, 3, 4, 5, 6, 7, 8, 0]).successors();
        assert_eq!(successors.len(), 2);
        assert!(successors.contains(&Board::new([1, 2, 3, 4, 5, 6, 7, 0, 8])));
        assert!(successors.contains(&Board::new([1, 2, 3, 4, 5, 0, 7, 8, 6])));
    }
}