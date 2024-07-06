use rand::Rng;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Game {
    pub id: uuid::Uuid,
    pub created: time::OffsetDateTime,
    pub updated: time::OffsetDateTime,
    pub title: String,
    pub board: Vec<Vec<char>>,
    pub player_board: Vec<Vec<char>>,
    pub state: State,
    pub duration_seconds: i32,
    pub elapsed_seconds: i32,
    pub score: i32,
    pub resumed_timestamp: Option<time::OffsetDateTime>,
}

#[derive(Debug, Clone, Copy)]
pub struct Point(i32, i32);

impl Game {
    pub fn new(rows: i32, cols: i32, mines: i32) -> Self {
        assert!(rows > 0 && cols > 0 && mines > 0);
        let (board, player_board) = Self::new_boards(rows, cols, mines);
        Self {
            id: uuid::Uuid::new_v4(),
            created: time::OffsetDateTime::now_utc(),
            updated: time::OffsetDateTime::now_utc(),
            title: "".to_string(),
            board: board,
            player_board: player_board,
            state: State::New,
            duration_seconds: 0,
            elapsed_seconds: 0,
            score: 0,
            resumed_timestamp: None,
        }
    }

    pub fn new_point(&self, point: (i32, i32)) -> Option<Point> {
        let p = Point(point.0, point.1);
        if Self::inside_board(self.board.len() as i32, self.board[0].len() as i32, p) {
            return Some(Point(point.0, point.1));
        }
        None
    }

    pub fn get_board_view(&self) -> Vec<Vec<char>> {
        let mut board_view = vec![];
        for i in 0..self.board.len() {
            let mut row = vec![];
            for j in 0..self.board[i].len() {
                if self.player_board[i][j] == 'v' {
                    row.push(self.board[i][j]);
                } else if self.player_board[i][j] == 'h' {
                    row.push(' ');
                } else {
                    row.push(self.player_board[i][j]);
                }
            }
            board_view.push(row);
        }
        board_view
    }

    fn inside_board(rows: i32, cols: i32, point: Point) -> bool {
        point.0 >= 0 && point.0 < rows && point.1 >= 0 && point.1 < cols
    }

    fn adjacent_points(rows: i32, cols: i32, point: Point) -> Vec<Point> {
        let mut adjacent_points = vec![];
        let directions = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        for direction in directions {
            let new_point = Point(point.0 + direction.0, point.1 + direction.1);
            if Self::inside_board(rows, cols, new_point) {
                adjacent_points.push(new_point);
            }
        }
        adjacent_points
    }

    fn fill_adjacent(board: &mut Vec<Vec<char>>, rows: i32, cols: i32, point: Point) -> () {
        if board[point.0 as usize][point.1 as usize] == 'x' {
            return;
        }
        for p in Self::adjacent_points(rows, cols, point) {
            let val = board[p.0 as usize][p.1 as usize];
            if val != 'x' {
                let digit = val.to_digit(10).unwrap() + 1;
                board[p.0 as usize][p.1 as usize] = digit.to_string().chars().next().unwrap();
            }
        }
    }

    fn new_boards(rows: i32, cols: i32, mines: i32) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
        assert!(mines < rows * cols);
        let player_board = vec![vec!['h'; cols as usize]; rows as usize];
        let mut board = vec![vec!['0'; cols as usize]; rows as usize];
        for _ in 0..mines {
            let point = (
                rand::thread_rng().gen_range(0..rows),
                rand::thread_rng().gen_range(0..cols),
            );
            let mut mine_set = false;
            while !mine_set {
                if board[point.0 as usize][point.1 as usize] != 'x' {
                    board[point.0 as usize][point.1 as usize] = 'x';
                    mine_set = true;
                }
            }
        }
        for i in 0..rows {
            for j in 0..cols {
                Self::fill_adjacent(&mut board, rows, cols, Point(i, j));
            }
        }
        (board, player_board)
    }

    pub fn reveal_at(&mut self, point: Point) -> () {
        if self.player_board[point.0 as usize][point.1 as usize] == 'v' {
            return;
        }
        self.player_board[point.0 as usize][point.1 as usize] = 'v';
        if self.board[point.0 as usize][point.1 as usize] == '0' {
            for p in
                Self::adjacent_points(self.board.len() as i32, self.board[0].len() as i32, point)
            {
                self.reveal_at(p);
            }
        }
    }

    pub fn is_all_revealed(&self) -> bool {
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                if self.board[i][j] != 'x' && self.player_board[i][j] != 'v' {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_mine_at(&self, point: Point) -> bool {
        self.board[point.0 as usize][point.1 as usize] == 'x'
    }

    pub fn mark_flag_at(&mut self, point: Point) -> () {
        self.player_board[point.0 as usize][point.1 as usize] = '!';
    }

    pub fn mark_question_at(&mut self, point: Point) -> () {
        self.player_board[point.0 as usize][point.1 as usize] = '?';
    }
}

#[derive(Debug, Clone)]
pub enum State {
    New = 0,
    Started = 1,
    Paused = 2,
    Timeout = 3,
    Won = 4,
    Lost = 5,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::New => write!(f, "new"),
            State::Started => write!(f, "started"),
            State::Paused => write!(f, "paused"),
            State::Timeout => write!(f, "timeout"),
            State::Won => write!(f, "won"),
            State::Lost => write!(f, "lost"),
        }
    }
}
