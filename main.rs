use bitvec::prelude::*;
use once_cell::sync::Lazy;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::{self, Display};
use std::fs::File;
use std::io::{self, BufRead, BufReader, StdinLock, Write};
use std::path::Path;
use std::rc::Rc; // Use Rc for single-threaded shared ownership in AutoPlayer search state
use std::{env, process};

// --- Constants and Types ---

type Int = i8; // Using i8 like C++ int_t
const R: Int = 11;
const C: Int = 10;
const N: usize = (R * C) as usize;

// --- Position Struct ---

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Position {
    r: Int,
    c: Int,
}

impl Position {
    fn new(r: Int, c: Int) -> Self {
        Position { r, c }
    }

    fn idx(&self) -> usize {
        (self.r * C + self.c) as usize
    }

    fn is_valid(&self) -> bool {
        self.r >= 0 && self.r < R && self.c >= 0 && self.c < C
    }

    fn is_edge(&self) -> bool {
        self.r == 0 || self.r == R - 1 || self.c == 0 || self.c == C - 1
    }

    fn neighbors(&self) -> Vec<Position> {
        let mut ret = Vec::with_capacity(6);
        let r = self.r;
        let c = self.c;

        // Hex grid neighbor logic (same as C++)
        let positions = [
            Position::new(r, c - 1),
            Position::new(r - 1, c - ((r + 1) % 2)),
            Position::new(r - 1, c - ((r + 1) % 2) + 1),
            Position::new(r, c + 1),
            Position::new(r + 1, c - ((r + 1) % 2) + 1),
            Position::new(r + 1, c - ((r + 1) % 2)),
        ];

        for p in positions {
            if p.is_valid() {
                ret.push(p);
            }
        }
        ret
    }
}

// Use once_cell::sync::Lazy for safe static initialization
static EDGES: Lazy<Vec<Position>> = Lazy::new(|| {
    let mut ret = Vec::new();
    for r in 0..R {
        for c in 0..C {
            let p = Position::new(r, c);
            if p.is_edge() {
                ret.push(p);
            }
        }
    }
    ret
});

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.r, self.c)
    }
}

// --- Board Struct ---

// Using Clone for the AutoPlayer search state copying
#[derive(Debug, Clone)]
struct Board {
    cells: BitVec<usize, Lsb0>, // Use bitvec crate
    cat_position: Position,
}

impl Board {
    fn new() -> Self {
        Board {
            cells: bitvec![usize, Lsb0; 0; N], // Initialize N bits to 0 (false)
            cat_position: Position::new(0, 0), // Default, will be overwritten
        }
    }

    // Helper to create board from a reader (like the C++ istream operator)
    fn from_reader<R: BufRead>(reader: &mut R) -> io::Result<Self> {
        let mut board = Board::new();
        let mut cat_found = false;
        let mut line_num = 0;

        for r in 0..R {
            line_num += 1;
            let mut line = String::new();
            reader.read_line(&mut line)?;
            let chars: Vec<char> = line.trim().chars().collect(); // Trim whitespace

            // Adjust column index based on row offset for reading input
            let mut expected_c = 0;
            let mut char_idx = 0;
             if r % 2 != 0 {
                 // Skip expected offset space if present
                 if chars.get(0) == Some(&' ') {
                      char_idx += 1;
                 }
             }

            for c in 0..C {
                let p = Position::new(r, c);
                // Handle space separation
                 if expected_c > 0 { // skip expected space
                      if chars.get(char_idx) == Some(&' ') {
                           char_idx += 1;
                      } else {
                          // Allow missing space for flexibility
                      }
                 }

                let s = chars.get(char_idx).ok_or_else(|| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Unexpected end of line {} (r={}, c={})", line_num, r, c),
                    )
                })?;
                 char_idx += 1;
                 expected_c +=1;


                match s {
                    'C' => {
                        if cat_found {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!("Multiple cats found (line {})", line_num),
                            ));
                        }
                        board.cat_position = p;
                        cat_found = true;
                        board.cells.set(p.idx(), false); // Cat position is not a wall
                    }
                    '#' => {
                        board.cells.set(p.idx(), true); // Wall
                    }
                    '-' => {
                        board.cells.set(p.idx(), false); // Empty
                    }
                    _ => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!(
                                "Invalid character '{}' on line {} (r={}, c={})",
                                s, line_num, r, c
                            ),
                        ));
                    }
                }
            }
        }

        if !cat_found {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Cat 'C' not found in input",
            ));
        }

        Ok(board)
    }

    // Mark: move_cat
    // Attempts the same logic as the C++ version, fixing the layer bug
    // Returns Option<Position> -> Some(new_position) if moved, None if trapped
    fn move_cat(&mut self) -> Option<Position> {
        let mut pos_to_num_paths: HashMap<Position, usize> = HashMap::new();
        let mut layer: HashSet<Position> = HashSet::new();

        // Initialize BFS from edges
        for &p in EDGES.iter() {
            if !self.cells[p.idx()] {
                pos_to_num_paths.insert(p, 1);
                layer.insert(p);
            } else {
                pos_to_num_paths.insert(p, 0); // Wall on edge counts as 0 paths
            }
        }

        // BFS Loop moving inwards
        while !layer.is_empty() {
            // Check if cat is reached in the *current* layer
            if layer.contains(&self.cat_position) {
                let mut best_num_paths: usize = 0;
                let mut best_move: Option<Position> = None;

                for p in self.cat_position.neighbors() {
                    // Potential move 'p' should NOT be in the current layer
                    // AND must have been reached by the BFS (i.e., in pos_to_num_paths)
                    // AND must have paths leading to it (> 0)
                    if !layer.contains(&p) {
                         if let Some(&num_paths) = pos_to_num_paths.get(&p) {
                              if num_paths > best_num_paths {
                                   best_num_paths = num_paths;
                                   best_move = Some(p);
                              }
                         }
                    }
                }

                return match best_move {
                    Some(pos) => {
                        // This check corresponds to the C++ throw
                        if best_num_paths == 0 {
                            // This state should be unreachable if cat was in layer
                            eprintln!("Warning: Cat found in BFS layer but no valid outward move found.");
                            None
                        } else {
                             self.cat_position = pos; // Mutate board state
                             Some(pos)
                        }

                    }
                    None => None, // Trapped if no valid neighbor closer to edge exists
                };
            }

            // Build next layer
            let mut next_layer: HashSet<Position> = HashSet::new();
            let mut layer_updates: HashMap<Position, usize> = HashMap::new(); // Track path updates for next layer

            for &p in &layer {
                let current_paths = *pos_to_num_paths.get(&p).unwrap_or(&0);
                if current_paths == 0 { continue; } // Skip nodes with no path in

                for pp in p.neighbors() {
                    if self.cells[pp.idx()] { continue; } // Skip walls

                    // If pp hasn't been processed AT ALL yet by the BFS
                    if !pos_to_num_paths.contains_key(&pp) {
                         // Add to next layer and initialize path count based on current node p
                         next_layer.insert(pp);
                         *layer_updates.entry(pp).or_insert(0) += current_paths;
                    } else if !layer.contains(&pp) {
                        // If pp was processed in a *previous* layer, do nothing here.
                        // If pp is being reached from another node 'p' in the *same current layer*, add paths.
                        // We need to be careful here. Let's refine: Only add paths if pp *will be* in next_layer.
                         if next_layer.contains(&pp) {
                            *layer_updates.entry(pp).or_insert(0) += current_paths;
                         }
                    }
                }
            }

            // Apply the accumulated path updates for the next layer
            for (pos, count_increase) in layer_updates {
                *pos_to_num_paths.entry(pos).or_insert(0) += count_increase;
            }

            layer = next_layer; // Move to the next layer
        }

        // If loop finishes and cat wasn't found, it's trapped
        None
    }

    // move_player remains similar
    fn move_player(&mut self, p: Position) -> bool {
        if p.is_valid() && !self.cells[p.idx()] && p != self.cat_position {
            self.cells.set(p.idx(), true);
            true
        } else {
            false
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..R {
            if r % 2 != 0 {
                write!(f, " ")?; // Offset odd rows
            }
            for c in 0..C {
                let p = Position::new(r, c);
                if c != 0 {
                    write!(f, " ")?; // Space between columns
                }
                let symbol = if self.cat_position == p {
                    'C'
                } else if self.cells[p.idx()] {
                    '#'
                } else {
                    '-'
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?; // Newline after each row
        }
        Ok(())
    }
}

// --- Player Trait and Implementations ---

trait Player {
    // Use &mut self if player needs to maintain state (like AutoPlayer cache)
    fn get_move(&mut self, board: &Board) -> Position;
}

struct InteractivePlayer;

impl Player for InteractivePlayer {
    fn get_move(&mut self, _board: &Board) -> Position {
        println!("Please enter your move (r c):");
        loop {
            io::stdout().flush().expect("Failed to flush stdout"); // Ensure prompt is shown
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            let parts: Vec<&str> = input.trim().split_whitespace().collect();
            if parts.len() == 2 {
                if let (Ok(r), Ok(c)) = (parts[0].parse::<Int>(), parts[1].parse::<Int>()) {
                    let pos = Position::new(r, c);
                    if pos.is_valid() {
                         return pos;
                    } else {
                         println!("Invalid position: coordinates out of bounds.");
                    }

                } else {
                    println!("Invalid input. Please enter two numbers (e.g., '5 4').");
                }
            } else {
                println!("Invalid input format. Please enter row and column separated by a space.");
            }
        }
    }
}

// --- AutoPlayer and Search State ---

// State for the A* search used by AutoPlayer
#[derive(Debug, Clone)]
struct SearchState {
    board: Board,
    // Use Option for cached value, None means not calculated yet
    cached_min_cat_moves: Option<usize>,
    num_moves: usize, // Player moves made to reach this state
    previous: Option<Rc<SearchState>>, // Use Rc for shared ownership
    move_taken: Option<Position>,      // Player move that led to this state
}

impl SearchState {
    // Calculate minimum cat moves to edge from this state's board
    // This is a BFS from the cat's position outward
    fn calculate_min_cat_moves(&self) -> usize {
        let mut visited: HashSet<Position> = HashSet::new();
        let mut queue: VecDeque<(Position, usize)> = VecDeque::new(); // (Position, distance)

        visited.insert(self.board.cat_position);
        queue.push_back((self.board.cat_position, 0));

        while let Some((current_pos, distance)) = queue.pop_front() {
            if current_pos.is_edge() {
                return distance; // Found shortest path to edge
            }

            for neighbor in current_pos.neighbors() {
                // Check not visited AND not a wall on the board
                if !self.board.cells[neighbor.idx()] && visited.insert(neighbor) {
                    queue.push_back((neighbor, distance + 1));
                }
            }
        }

        N // Cat is trapped, return a large value indicating this
    }

    // Lazily calculate and cache the min_cat_moves
    fn min_cat_moves(&mut self) -> usize {
        if self.cached_min_cat_moves.is_none() {
            self.cached_min_cat_moves = Some(self.calculate_min_cat_moves());
        }
        self.cached_min_cat_moves.unwrap()
    }

    // Priority for the Max-Heap (BinaryHeap): Higher score is better (cat further from edge)
    fn priority(&mut self) -> usize {
        self.min_cat_moves()
    }
}

// Implement Ord, PartialOrd, Eq, PartialEq for BinaryHeap (Max-Heap)
// Note: We compare based on priority, which calls min_cat_moves (mutates cache)
// This is generally discouraged, but acceptable if priority() is stable after first call.
// A better approach might be to ensure priority is calculated *before* insertion.
impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // We need mutable access to calculate priority if not cached.
        // This is tricky with Ord. Let's assume priority is pre-calculated or
        // use a wrapper type in the heap.
        // For simplicity here, assume cached value exists or use default.
        let self_prio = self.cached_min_cat_moves.unwrap_or(0);
        let other_prio = other.cached_min_cat_moves.unwrap_or(0);
        self_prio.cmp(&other_prio) // Higher distance = higher priority
    }
}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SearchState {
    fn eq(&self, other: &Self) -> bool {
        // Equality might be based on board state, or priority for heap purposes
        self.cached_min_cat_moves == other.cached_min_cat_moves // Simple equality for heap ordering
    }
}

impl Eq for SearchState {}


struct AutoPlayer {
    cached_moves: VecDeque<Position>,
}

impl AutoPlayer {
     fn new() -> Self {
          AutoPlayer { cached_moves: VecDeque::new() }
     }
}

impl Player for AutoPlayer {
    fn get_move(&mut self, board: &Board) -> Position {
        if let Some(pos) = self.cached_moves.pop_front() { // Use FIFO for cached plan
            return pos;
        }

        // A* Search (using BFS distance as heuristic)
        let mut pq = BinaryHeap::new(); // Max-heap based on Ord implementation
        let mut initial_state = SearchState {
            board: board.clone(),
            cached_min_cat_moves: None,
            num_moves: 0,
            previous: None,
            move_taken: None,
        };
        // Pre-calculate priority before inserting into heap
        initial_state.min_cat_moves();
        pq.push(initial_state);

        // Limit search depth/iterations to prevent infinite loops in impossible scenarios
        let mut iterations = 0;
        let max_iterations = 50000; // Adjust as needed

        while let Some(mut current_state) = pq.pop() {
            iterations += 1;
            if iterations > max_iterations {
                 eprintln!("AutoPlayer search limit reached, giving up and picking arbitrary move.");
                 // Fallback: find *any* valid move
                 for r in 0..R { for c in 0..C { let p = Position::new(r, c); if !board.cells[p.idx()] && p != board.cat_position { return p; } } }
                 return board.cat_position; // Should not happen if board has empty cells
            }


            // Simulate cat's move *after* player's hypothetical move led to current_state
            // We need a mutable copy to simulate the cat's move
            let mut board_after_player = current_state.board.clone();
            let cat_move_result = board_after_player.move_cat(); // Mutates board_after_player

             // Recalculate heuristic *after* cat moves
             current_state.board = board_after_player; // Update state's board
             current_state.cached_min_cat_moves = None; // Invalidate cache
             let current_priority = current_state.min_cat_moves(); // Recalculate

            // Check if this state is a winning state (cat is trapped)
            if current_priority == N || cat_move_result.is_none() {
                eprintln!("AutoPlayer found winning sequence after {} iterations!", iterations);
                let mut path = VecDeque::new();
                let mut current_rc = Rc::new(current_state); // Wrap in Rc to share for reconstruction
                while let Some(pos) = current_rc.move_taken {
                    path.push_front(pos);
                    if let Some(prev) = &current_rc.previous {
                        current_rc = prev.clone(); // Move up the chain
                    } else {
                        break;
                    }
                }
                self.cached_moves = path;
                return self.cached_moves.pop_front().expect("Path should not be empty");
            }

            // If cat escaped in simulation, this path is bad (heuristic is 0) - prune?
             if current_state.board.cat_position.is_edge() {
                  continue; // Don't explore states where cat has already escaped
             }


            // Explore next possible player moves from the state *after* the cat moved
            let shared_current_ptr = Rc::new(current_state); // Share ownership for 'previous' links

            for r in 0..R {
                for c in 0..C {
                    let p = Position::new(r, c);
                    // Check if player can place wall at 'p' in the board *after* cat moved
                    if !shared_current_ptr.board.cells[p.idx()] && p != shared_current_ptr.board.cat_position {
                        let mut next_board = shared_current_ptr.board.clone();
                        next_board.cells.set(p.idx(), true); // Player places wall

                        let mut next_state = SearchState {
                            board: next_board,
                            cached_min_cat_moves: None, // Calculate later
                            num_moves: shared_current_ptr.num_moves + 1,
                            previous: Some(shared_current_ptr.clone()),
                            move_taken: Some(p),
                        };
                         // Pre-calculate heuristic/priority
                         next_state.min_cat_moves();
                        pq.push(next_state);
                    }
                }
            }
        }

         eprintln!("AutoPlayer failed to find a solution. Picking arbitrary move.");
         // Fallback if queue empties (should indicate player cannot win)
         for r in 0..R { for c in 0..C { let p = Position::new(r, c); if !board.cells[p.idx()] && p != board.cat_position { return p; } } }
         return board.cat_position; // Should not happen
    }
}


// --- Main Game Logic ---

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        process::exit(1);
    }

    let input_path = Path::new(&args[1]);
    let file = File::open(input_path)?;
    let mut reader = BufReader::new(file);

    let mut board = Board::from_reader(&mut reader)?;

    println!("Initial Board:");
    println!("{}", board);

    // Choose player type
    // let mut player: Box<dyn Player> = Box::new(InteractivePlayer);
     let mut player: Box<dyn Player> = Box::new(AutoPlayer::new());

    loop {
        // 1. Check if cat escaped edge (before player move)
        if board.cat_position.is_edge() {
            println!("Failed! The cat ran away!");
            break;
        }

        // 2. Get Player's move
        println!("Player's turn...");
        let player_move = loop {
            let mv = player.get_move(&board);
            if board.move_player(mv) {
                println!("Player places wall at: {}", mv);
                break mv; // Valid move applied
            } else {
                // This should ideally not happen with AutoPlayer if logic is correct
                 // For InteractivePlayer, re-prompt
                eprintln!("Invalid move: {} (either occupied, the cat, or out of bounds). Try again.", mv);
                 // If player is AutoPlayer and gives invalid move, it's a bug. Exit?
                 // For now, assume InteractivePlayer might retry. If AutoPlayer fails, loop might continue forever.
                 // Maybe add logic here for AutoPlayer failure.
            }
        };

        println!("Board after player's move:");
        println!("{}", board);

        // 3. Check if player move trapped the cat (cat has no moves)
        // We check this by trying to move the cat.

        // 4. Move Cat
        println!("Cat's turn...");
        match board.move_cat() { // move_cat mutates the board
            Some(cat_new_pos) => {
                println!("Cat moved to: {}", cat_new_pos);
                println!("Board after cat's move:");
                println!("{}", board);
                // Loop continues
            }
            None => {
                // Cat couldn't move
                println!("Succeeded! You trapped the cat!");
                break; // Player wins
            }
        }
         // 5. Check if cat reached edge *after* its move
         if board.cat_position.is_edge() {
              println!("Failed! The cat reached the edge after moving!");
              break; // Cat wins
         }

    }

    Ok(())
}