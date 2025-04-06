use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};
use std::cmp::Ordering;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::rc::Rc;
use std::cell::RefCell;

const R: i8 = 11;
const C: i8 = 10;
const N: usize = (R as usize) * (C as usize);

type IntT = i8;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
struct Position {
    r: IntT,
    c: IntT,
}

impl Position {
    fn new(r: IntT, c: IntT) -> Self {
        Self { r, c }
    }

    fn idx(&self) -> usize {
        (self.r as usize) * (C as usize) + (self.c as usize)
    }

    fn is_valid(&self) -> bool {
        self.r >= 0 && self.r < R && self.c >= 0 && self.c < C
    }

    fn is_edge(&self) -> bool {
        self.r == 0 || self.r == R - 1 || self.c == 0 || self.c == C - 1
    }

    fn neighbors(&self) -> Vec<Position> {
        let mut ret = vec![];
        let mut maybe_insert = |p: Position| {
            if p.is_valid() {
                ret.push(p);
            }
        };

        maybe_insert(Position::new(self.r, self.c - 1));
        maybe_insert(Position::new(self.r - 1, self.c - ((self.r + 1) % 2)));
        maybe_insert(Position::new(self.r - 1, self.c - ((self.r + 1) % 2) + 1));
        maybe_insert(Position::new(self.r, self.c + 1));
        maybe_insert(Position::new(self.r + 1, self.c - ((self.r + 1) % 2) + 1));
        maybe_insert(Position::new(self.r + 1, self.c - ((self.r + 1) % 2)));

        ret
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Position(r={}, c={})", self.r, self.c)
    }
}

#[derive(Clone)]
struct Board {
    cells: Vec<bool>,
    cat_position: Position,
}

impl Board {
    fn new() -> Self {
        Self {
            cells: vec![false; N],
            cat_position: Position::new(0, 0),
        }
    }

    fn move_cat(&mut self) -> Option<Position> {
        let mut pos2num_paths: HashMap<Position, i32> = HashMap::new();
        let mut layer: HashSet<Position> = HashSet::new();

        for r in 0..R {
            for c in 0..C {
                let p = Position::new(r, c);
                if p.is_edge() && !self.cells[p.idx()] {
                    pos2num_paths.insert(p, 1);
                    layer.insert(p);
                }
            }
        }

        while !layer.is_empty() {
            if layer.contains(&self.cat_position) {
                let mut best_num_paths = 0;
                let mut best_move = None;

                for p in self.cat_position.neighbors() {
                    if layer.contains(&p) {
                        continue;
                    }
                    if let Some(&num) = pos2num_paths.get(&p) {
                        if num > best_num_paths {
                            best_move = Some(p);
                            best_num_paths = num;
                        }
                    }
                }

                if let Some(best) = best_move {
                    self.cat_position = best;
                    return Some(best);
                } else {
                    panic!("sth bad happened oops");
                }
            }

            let mut next_layer = HashSet::new();
            for p in &layer {
                for pp in p.neighbors() {
                    if self.cells[pp.idx()] {
                        continue;
                    }
                    if next_layer.contains(&pp) {
                        continue;
                    }
                    let entry = pos2num_paths.entry(pp).or_insert(0);
                    *entry += pos2num_paths[p];
                    next_layer.insert(pp);
                }
            }
            layer = next_layer;
        }
        None
    }

    fn move_player(&mut self, p: Position) -> bool {
        if p.is_valid() && !self.cells[p.idx()] && p != self.cat_position {
            self.cells[p.idx()] = true;
            return true;
        }
        false
    }
}

trait BasePlayer {
    fn get_move(&mut self, board: &Board) -> Position;
}

struct InteractivePlayer;

impl BasePlayer for InteractivePlayer {
    fn get_move(&mut self, _board: &Board) -> Position {
        println!("Please enter your move (r c):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let nums: Vec<i8> = input
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        Position::new(nums[0], nums[1])
    }
}

struct AutoPlayer {
    cached_moves: VecDeque<Position>,
}

impl AutoPlayer {
    fn new() -> Self {
        Self {
            cached_moves: VecDeque::new(),
        }
    }

    fn get_min_cat_moves(board: &Board) -> i8 {
        let mut visited: HashSet<Position> = HashSet::new();
        let mut current: HashSet<Position> = HashSet::new();

        visited.insert(board.cat_position);
        current.insert(board.cat_position);

        let mut num_cat_moves = 0;
        loop {
            if current.is_empty() {
                return N as i8;
            }

            if current.iter().any(|p| p.is_edge()) {
                return num_cat_moves;
            }

            let mut new_current = HashSet::new();
            for p in &current {
                for pp in p.neighbors() {
                    if !visited.contains(&pp) && !board.cells[pp.idx()] {
                        new_current.insert(pp);
                    }
                }
            }

            current = new_current;
            visited.extend(current.iter());
            num_cat_moves += 1;
        }
    }

    #[derive(Clone)]
    struct SearchState {
        board: Board,
        cached_min_cat_moves: i8,
        num_moves: i8,
        previous: Option<Rc<RefCell<SearchState>>>,
        mv: Option<Position>,
    }

    impl SearchState {
        fn new(board: Board) -> Self {
            Self {
                cached_min_cat_moves: -1,
                board,
                num_moves: 0,
                previous: None,
                mv: None,
            }
        }

        fn min_cat_moves(&mut self) -> i8 {
            if self.cached_min_cat_moves == -1 {
                self.cached_min_cat_moves = AutoPlayer::get_min_cat_moves(&self.board);
            }
            self.cached_min_cat_moves
        }
    }

    impl Ord for SearchState {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cached_min_cat_moves.cmp(&self.cached_min_cat_moves)
        }
    }

    impl PartialOrd for SearchState {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for SearchState {
        fn eq(&self, other: &Self) -> bool {
            self.cached_min_cat_moves == other.cached_min_cat_moves
        }
    }

    impl Eq for SearchState {}
}

impl BasePlayer for AutoPlayer {
    fn get_move(&mut self, board: &Board) -> Position {
        if let Some(p) = self.cached_moves.pop_back() {
            return p;
        }

        let mut pq = BinaryHeap::new();
        let mut first_state = AutoPlayer::SearchState::new(board.clone());
        first_state.cached_min_cat_moves = AutoPlayer::get_min_cat_moves(board);
        pq.push(Rc::new(RefCell::new(first_state)));

        let mut first = true;

        while let Some(current_ptr) = pq.pop() {
            let mut current = current_ptr.borrow_mut();

            if !first {
                current.board.move_cat();
            }
            first = false;

            if current.min_cat_moves() == N as i8 {
                let mut ptr = Some(current_ptr.clone());
                while let Some(state_ptr) = ptr {
                    let state = state_ptr.borrow();
                    if let Some(mv) = state.mv {
                        self.cached_moves.push_front(mv);
                    }
                    ptr = state.previous.clone();
                }

                return self.cached_moves.pop_back().unwrap();
            }

            for r in 0..R {
                for c in 0..C {
                    let p = Position::new(r, c);
                    if !current.board.cells[p.idx()] && p != current.board.cat_position {
                        let mut next_board = current.board.clone();
                        next_board.cells[p.idx()] = true;
                        let mut next_state = AutoPlayer::SearchState::new(next_board);
                        next_state.num_moves = current.num_moves + 1;
                        next_state.previous = Some(current_ptr.clone());
                        next_state.mv = Some(p);
                        next_state.cached_min_cat_moves = AutoPlayer::get_min_cat_moves(&next_state.board);
                        pq.push(Rc::new(RefCell::new(next_state)));
                    }
                }
            }
        }
        panic!("No move found");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        return;
    }

    let file = File::open(&args[1]).unwrap();
    let reader = BufReader::new(file);
    let mut board = Board::new();

    let mut cat_found = false;
    for (r, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let tokens: Vec<char> = line.chars().filter(|c| !c.is_whitespace()).collect();
        for (c, &s) in tokens.iter().enumerate() {
            let pos = Position::new(r as i8, c as i8);
            match s {
                'C' => {
                    if cat_found {
                        panic!("Multiple cats found at {} {}", r, c);
                    }
                    board.cat_position = pos;
                    board.cells[pos.idx()] = false;
                    cat_found = true;
                }
                '#' => board.cells[pos.idx()] = true,
                '-' => board.cells[pos.idx()] = false,
                _ => panic!("Unexpected char '{}' at {} {}", s, r, c),
            }
        }
    }

    if !cat_found {
        panic!("No cat found on board");
    }

    let mut player: Box<dyn BasePlayer> = Box::new(AutoPlayer::new());

    loop {
        if board.cat_position.is_edge() {
            println!("Failed! The cat ran away!");
            break;
        }

        loop {
            let mv = player.get_move(&board);
            if board.move_player(mv) {
                println!("Accepted player's move: {}", mv);
                break;
            }
        }

        if let Some(cat_mv) = board.move_cat() {
            println!("Cat moved: {}", cat_mv);
        } else {
            println!("Succeeded! You trapped the cat!");
            break;
        }
    }
}
