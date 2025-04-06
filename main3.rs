use bitvec::prelude::*;
use once_cell::sync::Lazy; // Or use std::sync::OnceLock
use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead, BufReader, Write};
use std::ops::{Index, IndexMut};
use std::rc::Rc; // Using Rc as the C++ code seems single-threaded with shared_ptr
use std::str::FromStr;

// Type alias equivalent to using int_t = int8_t;
type IntT = i8;

// Constants
const R: IntT = 11;
const C: IntT = 10;
const N: usize = (R as usize) * (C as usize); // usize for indexing bitvec

// --- Position Struct ---
#[derive(Debug, Copy, Clone)]
struct Position {
    r: IntT,
    c: IntT,
}

impl Position {
    fn new(r: IntT, c: IntT) -> Self {
        Position { r, c }
    }

    // Calculates the linear index for the position.
    // Potential issue: If R or C were large, r * C could overflow i8.
    // Here R*C = 110, which fits within i8::MAX (127).
    fn idx(&self) -> usize {
        (self.r * C + self.c) as usize // Cast to usize for indexing
    }

    // Checks if the position is within the board boundaries.
    fn is_valid(&self) -> bool {
        self.r >= 0 && self.r < R && self.c >= 0 && self.c < C
    }

    // Checks if the position is on the edge of the board.
    fn is_edge(&self) -> bool {
        self.r == 0 || self.r == R - 1 || self.c == 0 || self.c == C - 1
    }

    // Generates valid neighbor positions based on the hex grid logic.
    fn neighbors(&self) -> Vec<Position> {
        let mut ret = Vec::new();
        let r = self.r;
        let c = self.c;

        // Helper closure equivalent
        let maybe_insert = |ret: &mut Vec<Position>, p: Position| {
            if p.is_valid() {
                ret.push(p);
            }
        };

        // Explicitly cast r to i32 for modulo operation to avoid potential
        // issues if r were negative (though it shouldn't be here).
        // Using rem_euclid for potentially more intuitive modulo behavior.
        let offset = (r as i32 + 1).rem_euclid(2) as IntT;

        maybe_insert(&mut ret, Position::new(r, c - 1));
        maybe_insert(&mut ret, Position::new(r - 1, c - offset));
        maybe_insert(&mut ret, Position::new(r - 1, c - offset + 1));
        maybe_insert(&mut ret, Position::new(r, c + 1));
        maybe_insert(&mut ret, Position::new(r + 1, c - offset + 1));
        maybe_insert(&mut ret, Position::new(r + 1, c - offset));

        ret
    }

    // Static list of edge positions, initialized lazily.
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
}

// Implement traits needed for HashMap keys and HashSet elements
impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.c == other.c
    }
}
impl Eq for Position {}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.r.hash(state);
        self.c.hash(state);
    }
}

// Implement Ord and PartialOrd based on idx() for consistent ordering like C++'s operator<
impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        self.idx().cmp(&other.idx())
    }
}

// Implement Display for printing Position (equivalent to C++ operator<<)
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Position(r={}, c={})", self.r, self.c)
    }
}

// Implement FromStr for parsing Position from "r c" string format
// This replaces the C++ operator>> for Position used in InteractivePlayer
impl FromStr for Position {
    type Err = String; // Simple error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split_whitespace().collect();
        if parts.len() != 2 {
            return Err(format!("Invalid input: Expected 'r c', got '{}'", s));
        }
        let r = parts[0]
            .parse::<IntT>()
            .map_err(|e| format!("Invalid row '{}': {}", parts[0], e))?;
        let c = parts[1]
            .parse::<IntT>()
            .map_err(|e| format!("Invalid column '{}': {}", parts[1], e))?;
        Ok(Position::new(r, c))
    }
}

// --- Error Handling ---
// Equivalent to C++ throw_error function, using panic!
// Includes file and line information automatically.
fn throw_error(s: char, r: IntT, c: IntT, file: &str, line: u32) -> ! {
    panic!(
        "Unexpected on {}:{}: '{}' at r={}, c={}",
        file, line, s, r, c
    );
}

// --- Board Struct ---
#[derive(Clone)] // Clone needed for AutoPlayer search states
struct Board {
    // Use bitvec for efficient boolean storage, similar to std::bitset
    cells: BitVec<u8, Lsb0>, // Lsb0 is a common bit order
    cat_position: Position,
}

impl Board {
    // Creates an empty board (all false, cat at 0,0 - will be overwritten by parsing)
    fn new() -> Self {
        Board {
            cells: bitvec![u8, Lsb0; 0; N], // Initialize with N zeros
            cat_position: Position::new(0, 0), // Default position
        }
    }

    // Accessor for bitvec using Position index - mimics C++ board.cells[p.idx()]
    // Note: bitvec doesn't have direct Index/IndexMut like std::bitset values,
    // but provides `get`/`set`/`get_mut`. We can create helper methods.
    fn get_cell(&self, p: Position) -> bool {
        // Use get() which returns Option<&bool>, unwrap_or(false) if index is out of bounds
        // Although idx() should always be valid if p is valid.
        *self.cells.get(p.idx()).unwrap_or(&false)
    }

    fn set_cell(&mut self, p: Position, value: bool) {
        // Use set() which panics on out of bounds, similar to C++ bitset::operator[]
        self.cells.set(p.idx(), value);
    }

    // Logic ported from C++ Board::move_cat
    // Returns Option<Position> representing the cat's new position, or None if trapped.
    fn move_cat(&mut self) -> Option<Position> {
        let mut pos2num_paths: HashMap<Position, i32> = HashMap::new(); // Using i32 for path counts
        let mut layer: HashSet<Position> = HashSet::new();

        for &p in Position::EDGES.iter() {
            if self.get_cell(p) {
                pos2num_paths.insert(p, 0);
            } else {
                pos2num_paths.insert(p, 1);
                layer.insert(p);
            }
        }

        loop {
            if layer.is_empty() {
                // Cat not found - implies cat is trapped
                return None;
            }

            if layer.contains(&self.cat_position) {
                // Found shortest path(s) *to* the cat from the edge. Now find where the cat
                // should move *away* from the current layer towards an edge along a calculated path.
                let mut best_num_paths: i32 = 0;
                let mut best_move: Option<Position> = None; // Use Option for clarity

                for p in self.cat_position.neighbors() {
                    // Skip neighbors already blocked or part of the current path layer "frontier"
                    if self.get_cell(p) || layer.contains(&p) {
                        continue;
                    }

                    // If neighbor 'p' has a path calculated (meaning it's closer to an edge
                    // than the current cat position in this BFS) and has more paths than
                    // the current best, choose it.
                    if let Some(&num_paths) = pos2num_paths.get(&p) {
                         if num_paths > best_num_paths {
                            best_move = Some(p);
                            best_num_paths = num_paths;
                        }
                    }
                    // C++ version implicitly handles case where find returns end() - here we use if let Some
                }


                // Check if a valid move was found
                if let Some(chosen_move) = best_move {
                     if best_num_paths == 0 {
                         // This condition seems problematic in the C++ code as well.
                         // If best_move is Some, best_num_paths should be > 0 based on the loop logic.
                         // If best_move is None, this panic won't be reached.
                         // Replicating the C++ panic condition just in case.
                         panic!("sth bad happened oops: best_num_paths is 0 but a move was found");
                     }
                    self.cat_position = chosen_move;
                    return Some(chosen_move);
                } else {
                    // If no valid neighbor leads towards an edge (all blocked or closer to edge)
                    // This can happen if the cat is surrounded but not technically on the layer yet.
                    // The original C++ code would panic here ("sth bad happened oops").
                    // Returning None seems more reasonable, indicating the cat is trapped.
                    // However, to match C++, we panic.
                    panic!("sth bad happened oops: No valid move found for the cat");
                    // Alternative: return None;
                }
            }

            // Calculate next layer for BFS
            let mut next_layer = HashSet::new();
            for &p in &layer {
                for pp in p.neighbors() {
                    if self.get_cell(pp) {
                        continue; // Skip blocked cells
                    }

                    // Original C++ logic:
                    // if (next_layer.count(pp) || !pos2num_paths.count(pp)) { ... }
                    // This seems intended to add paths if pp is already in next_layer OR
                    // if pp hasn't been visited yet (!pos2num_paths.count(pp)).
                    // Let's replicate:

                    let pp_is_newly_discovered = !pos2num_paths.contains_key(&pp);
                    let pp_in_next_layer = next_layer.contains(&pp);

                    if pp_in_next_layer || pp_is_newly_discovered {
                        // Add paths from p to pp. Initialize if pp is new.
                        let paths_from_p = *pos2num_paths.get(&p).unwrap_or(&0); // Should always exist for p in layer
                        let current_paths_to_pp = pos2num_paths.entry(pp).or_insert(0);
                        // Potential overflow if path counts get huge, but i32 should be sufficient.
                        *current_paths_to_pp = current_paths_to_pp.saturating_add(paths_from_p);

                        // Insert pp into the next layer frontier if it wasn't already.
                        // This condition differs slightly from C++'s `next_layer.insert(pp)`
                        // which happens regardless of whether it was already there. HashSet handles duplicates.
                         next_layer.insert(pp);

                    }
                     // C++ logic seems slightly off here? It adds paths even if pp is already processed
                     // beyond the next_layer? Let's try to match the C++ logic exactly:
                     /*
                     let paths_from_p = *pos2num_paths.get(&p).unwrap_or(&0);
                     let pp_in_next = next_layer.contains(&pp);
                     let pp_known = pos2num_paths.contains_key(&pp);

                     if pp_in_next || !pp_known {
                        // This condition matches C++ `if (next_layer.count(pp) || !pos2num_paths.count(pp))`
                         let current_paths_to_pp = pos2num_paths.entry(pp).or_insert(0);
                         *current_paths_to_pp = current_paths_to_pp.saturating_add(paths_from_p);
                         next_layer.insert(pp); // Idempotent insert
                     }
                     */

                }
            }
            layer = next_layer;
        }
    }

    // Logic ported from C++ Board::move_player
    // Returns true if the move was valid and applied, false otherwise.
    fn move_player(&mut self, p: Position) -> bool {
        if p.is_valid() && !self.get_cell(p) && !(p == self.cat_position) {
            self.set_cell(p, true);
            true
        } else {
            false
        }
    }

    // Equivalent to C++ operator>> for Board (parsing from a reader)
    // Returns Result<Board, Box<dyn Error>> for better error handling than panic!
    fn from_reader<R: BufRead>(reader: R) -> Result<Board, Box<dyn Error>> {
        let mut board = Board::new();
        let mut cat_found = false;
        let mut line_no: u32 = 0; // Keep track for error messages

        for (r_idx, line_result) in reader.lines().enumerate() {
            line_no = r_idx as u32 + 1;
            let line = line_result?;
            let r = r_idx as IntT;

            if r >= R {
                // Handle case of too many rows in input
                 return Err(format!("Too many rows in input (expected {}, got at least {})", R, r + 1).into());
            }


            // Adjust expected character count based on row (for staggered grid input)
             let expected_chars = C as usize;
             // C++ reads char by char ignoring whitespace. Rust reads line by line.
             // We need to handle the staggered input format carefully if it matters.
             // Assuming input is like:
             // - - - - - - - - - -
             //  - - - - C - - - - -
             // - - - - - - - - - -
             // The C++ >> operator skips whitespace. We'll filter whitespace.
             let chars: Vec<char> = line.chars().filter(|c| !c.is_whitespace()).collect();

             if chars.len() != expected_chars {
                 return Err(format!("Line {}: Expected {} non-whitespace characters, found {}", line_no, expected_chars, chars.len()).into());
             }


            for (c_idx, s) in chars.iter().enumerate() {
                 let c = c_idx as IntT;
                let p = Position::new(r, c);

                match s {
                    'C' => {
                        if cat_found {
                            // Use throw_error equivalent for consistency with C++
                            throw_error(*s, r, c, file!(), line!());
                            // Alternative: return Err(...)
                        }
                        board.cat_position = p;
                        cat_found = true;
                        board.set_cell(p, false); // Cat position is not blocked
                    }
                    '#' => {
                        board.set_cell(p, true); // Blocked cell
                    }
                    '-' => {
                        board.set_cell(p, false); // Empty cell
                    }
                    _ => {
                        // Use throw_error equivalent
                        throw_error(*s, r, c, file!(), line!());
                        // Alternative: return Err(...)
                    }
                }
            }
        }
         // Check if we read enough rows
         if (line_no as IntT) < R {
              return Err(format!("Too few rows in input (expected {}, got {})", R, line_no).into());
         }


        if !cat_found {
            // Use throw_error equivalent (passing dummy coords)
            throw_error('C', -1, -1, file!(), line!());
            // Alternative: return Err("Cat 'C' not found in input".into())
        }

        Ok(board)
    }
}

// Implement Display for printing Board (equivalent to C++ operator<<)
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for r in 0..R {
            // Add space for odd rows for hex grid visualization
            if r % 2 != 0 {
                write!(f, " ")?;
            }
            for c in 0..C {
                let p = Position::new(r, c);
                // Add space between columns
                if c > 0 {
                    write!(f, " ")?;
                }
                // Print C, #, or -
                if self.cat_position == p {
                    write!(f, "C")?;
                } else if self.get_cell(p) {
                    write!(f, "#")?;
                } else {
                    write!(f, "-")?;
                }
            }
            // Newline after each row
            writeln!(f)?;
        }
        Ok(())
    }
}

// --- Player Trait and Implementations ---

// Trait defining the player interface (equivalent to C++ BasePlayer)
// Use &mut self if the player needs to modify internal state (like AutoPlayer cache)
trait BasePlayer {
    fn get_move(&mut self, board: &Board) -> Position;
}

// Interactive Player
struct InteractivePlayer;

impl BasePlayer for InteractivePlayer {
    fn get_move(&mut self, _board: &Board) -> Position {
        println!("Please enter your move (r c):");
        loop {
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => match Position::from_str(&input) {
                    Ok(pos) => return pos,
                    Err(e) => eprintln!("Invalid input: {}. Please try again.", e),
                },
                Err(e) => {
                    eprintln!("Error reading input: {}. Please try again.", e);
                }
            }
        }
    }
}

// --- AutoPlayer ---

// Struct to hold state during the search in AutoPlayer
// Needs Ord, Eq for BinaryHeap
#[derive(Clone)] // Clone needed for pushing onto heap/storing Rc
struct SearchState {
    board: Board,
    // Use Cell for interior mutability of the cache, similar to C++ mutable
    cached_min_cat_moves: Cell<IntT>,
    num_moves: IntT,
    previous: Option<Rc<SearchState>>, // Use Rc for shared ownership of history
    move_made: Option<Position>,       // The player move that led to this state
}

impl SearchState {
    // Associated function equivalent to C++ AutoPlayer::get_min_cat_moves
    // Performs a BFS from cat position to find shortest path to edge.
    fn calculate_min_cat_moves(board: &Board) -> IntT {
        let mut visited: HashSet<Position> = HashSet::new();
        let mut current: HashSet<Position> = HashSet::new();

        if board.get_cell(board.cat_position) {
             // Cat starts on a blocked cell? Should not happen if input/logic is correct.
             // Return max distance as it's trapped immediately.
             return N as IntT;
        }

        visited.insert(board.cat_position);
        current.insert(board.cat_position);

        let mut num_cat_moves: IntT = 0;

        while !current.is_empty() {
            let mut next_current = HashSet::new();
            for &p in &current {
                if p.is_edge() {
                    return num_cat_moves; // Found edge
                }

                for pp in p.neighbors() {
                    // Check if neighbor is valid, not blocked, and not visited
                    if pp.is_valid() && !board.get_cell(pp) && !visited.contains(&pp) {
                         // Check visited *before* inserting into next_current for efficiency
                         if next_current.insert(pp) {
                              // Only mark visited when adding to the next layer
                              visited.insert(pp);
                         }
                    }
                    // C++ logic inserts into visited *after* processing the layer.
                    // Let's match that for identical behavior.
                     /*
                     if pp.is_valid() && !board.get_cell(pp) && !visited.contains(&pp) {
                         next_current.insert(pp);
                     }
                     */
                }
            }

             current = next_current;
             // C++ inserts *after* the inner loops.
             // visited.extend(current.iter()); // Add all elements from the new layer to visited
             // This seems wrong, the C++ code inserts before iterating neighbours?
             // Let's re-check C++: `visited.insert(current.begin(), current.end());` happens *after* the inner loops
             // but before the `++num_cat_moves`. Okay, the Rust version above with insert inside loop was wrong.
             // Let's correct to match C++:
             /*
             let mut next_current = HashSet::new();
             for &p in &current {
                  if p.is_edge() { return num_cat_moves; }
                  for pp in p.neighbors() {
                       if pp.is_valid() && !board.get_cell(pp) && !visited.contains(&pp) {
                           next_current.insert(pp);
                       }
                  }
             }
             // Now add all newly found nodes to visited *before* the next iteration
             visited.extend(next_current.iter()); // Add all elements from the new layer to visited
             current = next_current;
             */
             // Let's re-re-check C++: visited is only used to *check* if a neighbor should be added to `new_current`.
             // `visited` is updated *after* the layer is processed with `visited.insert(current.begin(), current.end())`.
             // This means a node added in this layer won't prevent *another* node in the same layer from adding it again.
             // The BFS seems slightly non-standard. Let's match the C++ precisely:

             let mut next_current_for_iteration = HashSet::new();
             for &p in &current {
                 if p.is_edge() {
                     return num_cat_moves;
                 }
                 for pp in p.neighbors() {
                     // Add to next_current if valid, empty, and *not already visited*
                     if pp.is_valid() && !board.get_cell(pp) && !visited.contains(&pp) {
                         next_current_for_iteration.insert(pp);
                     }
                 }
             }
             // Update visited *after* processing all nodes in the current layer
             visited.extend(next_current_for_iteration.iter());
             current = next_current_for_iteration;


            // Only increment if we are about to process a non-empty next layer
             if !current.is_empty() {
                 num_cat_moves = num_cat_moves.saturating_add(1); // Avoid overflow
                 // Check potential i8 overflow
                 if num_cat_moves >= N as IntT { return N as IntT; } // Cannot take more steps than cells
             } else {
                 // If current becomes empty here, it means no path to edge found
                 break;
             }
        }

        // If loop finishes without finding an edge
        N as IntT // Return max distance indicating trapped
    }

    fn min_cat_moves(&self) -> IntT {
        if self.cached_min_cat_moves.get() == -1 {
            self.cached_min_cat_moves
                .set(Self::calculate_min_cat_moves(&self.board));
        }
        self.cached_min_cat_moves.get()
    }

    // Priority: Higher minimum moves for the cat is better (means cat is more trapped)
    // This matches the C++ `operator<` which puts states with *lower* min_cat_moves first
    // when used with std::priority_queue (max heap based on operator<).
    // Rust's BinaryHeap is also a max-heap. So we implement Ord accordingly.
    fn priority(&self) -> IntT {
        self.min_cat_moves()
    }
}

impl PartialEq for SearchState {
    fn eq(&self, other: &Self) -> bool {
        self.priority() == other.priority()
            // Add other fields if needed for equality, though heap only uses Ord
            && self.num_moves == other.num_moves
    }
}
impl Eq for SearchState {}

impl PartialOrd for SearchState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Order based on priority (min_cat_moves). Higher priority = higher value = better state.
// BinaryHeap keeps the maximum element, so this works directly.
impl Ord for SearchState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority().cmp(&other.priority())
        // Tie-breaking (optional, C++ version doesn't specify):
        // .then_with(|| self.num_moves.cmp(&other.num_moves).reverse()) // Fewer moves is better
    }
}

// Implement Display for debugging SearchState
impl fmt::Display for SearchState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "State(")?;
        writeln!(f, "  priority={},", self.priority())?;
        writeln!(f, "  num_moves={},", self.num_moves)?;
        writeln!(f, "  previous={},", if self.previous.is_some() { "yes" } else { "no" })?;
        write!(f, "  move=")?;
        if let Some(mv) = self.move_made {
            write!(f, "{}", mv)?;
        } else {
            write!(f, "none")?;
        }
        writeln!(f, ",")?;
        writeln!(f, "  min_cat_moves={},", self.min_cat_moves())?; // Use cached value
        writeln!(f, ") with board:")?;
        write!(f, "{}", self.board)?; // Display the board state
        Ok(())
    }
}


struct AutoPlayer {
    // Use VecDeque for efficient front/back operations like std::deque
    cached_moves: VecDeque<Position>,
}

impl AutoPlayer {
    fn new() -> Self {
        AutoPlayer {
            cached_moves: VecDeque::new(),
        }
    }
}

impl BasePlayer for AutoPlayer {
    // Logic ported from C++ AutoPlayer::get_move
    fn get_move(&mut self, board: &Board) -> Position {
        // Return cached move if available (note: C++ pops back, Rust pops front/back)
        // C++ pushes back, then pops back (LIFO). Rust should push_front, pop_front (FIFO)
        // or push_back, pop_back (LIFO). Let's match C++ LIFO.
        if let Some(mv) = self.cached_moves.pop_back() {
             eprintln!("Using cached move: {}", mv); // Added for clarity
            return mv;
        }

        // Priority queue (max heap based on Ord implementation of SearchState)
        let mut pq: BinaryHeap<SearchState> = BinaryHeap::new();

        // Initial state
        let first_state = SearchState {
            board: board.clone(), // Clone the initial board state
            cached_min_cat_moves: Cell::new(-1),
            num_moves: 0,
            previous: None,
            move_made: None,
        };
        // Calculate initial priority eagerly to avoid weirdness on first pop?
        // first_state.min_cat_moves(); // Calculate now
        pq.push(first_state);

        let mut first_iteration = true; // Flag to skip cat move on first pop

        while let Some(current_state_rc_base) = pq.pop() {
             // We need owned state to modify board (move_cat) and create next states.
             // Since SearchState is Clone, this is okay. We store Rc in 'previous'.
             let mut current_state = current_state_rc_base; // Now owned

            // Wrap in Rc *after* popping and before potentially using as 'previous'
            let current_ptr = Rc::new(current_state.clone()); // Clone for Rc, keep current_state mutable


            // Equivalent of std::clog << "Popping top..."
            eprintln!(
                "Popping top of {} states:", // Approximate size before pop
                pq.len() + 1
            );


             // Move cat *after* player's hypothetical move (except on first state)
            if !first_iteration {
                // move_cat requires &mut Board
                if current_state.board.move_cat().is_none() {
                     // If cat is trapped immediately after player's last hypothetical move,
                     // this path is not useful for finding *player* moves.
                     // This state shouldn't be expanded? Or maybe this means we won?
                     // The C++ code doesn't explicitly check the return of move_cat here.
                     // It relies on the min_cat_moves check later. Let's stick to that.
                     // Re-evaluate this if logic seems wrong.
                     let _ = current_state.board.move_cat(); // Ignore result for now, matching C++ implicitly
                }
                // Recalculate cached moves after cat potentially moved
                 current_state.cached_min_cat_moves.set(-1); // Invalidate cache
            } else {
                 first_iteration = false;
            }


             // Log the current state (after potential cat move)
             eprintln!("{}", *current_ptr); // Log the Rc'd version (or current_state)


             // Check for win condition (cat is trapped)
             // Recalculate if cache was invalidated
             if current_state.min_cat_moves() == N as IntT {
                 eprintln!("Solution found!");
                 let mut path = VecDeque::new(); // Use VecDeque to push front easily
                 let mut temp_ptr = Some(current_ptr.clone()); // Start with the winning state

                 // Walk back through the 'previous' links
                 while let Some(state_rc) = temp_ptr {
                     if let Some(mv) = state_rc.move_made {
                         path.push_front(mv); // Add move to the front of the path
                     }
                     // Move to the previous state in the chain
                     temp_ptr = state_rc.previous.clone(); // Clone the Option<Rc<...>>
                 }


                 self.cached_moves = path; // Store the calculated path

                 // Return the first move from the path
                 if let Some(mv) = self.cached_moves.pop_front() { // First move is at the front
                     return mv;
                 } else {
                     // Should not happen if a solution was found with moves
                     panic!("Solution found but no moves in reconstructed path!");
                 }
             }


             // Expand current state: Generate next possible player moves
             for r in 0..R {
                 for c in 0..C {
                     let p = Position::new(r, c);
                     // Check if player can move to this position
                     if !current_state.board.get_cell(p) && !(p == current_state.board.cat_position) {
                         // Create the next state based on this move
                         let mut next_board = current_state.board.clone();
                         next_board.set_cell(p, true); // Apply player move

                         let next_state = SearchState {
                             board: next_board,
                             cached_min_cat_moves: Cell::new(-1), // Needs recalculation
                             num_moves: current_state.num_moves.saturating_add(1),
                             previous: Some(current_ptr.clone()), // Link to current state
                             move_made: Some(p),                   // Record the move made
                         };
                         // next_state.min_cat_moves(); // Optionally pre-calculate priority

                         pq.push(next_state);
                     }
                 }
             }
             // Check if heap gets too large - could add a limit
             // if pq.len() > SOME_LIMIT { eprintln!("Warning: Search space large!"); }
        }

        // Should not be reached if a solution always exists or cat escapes
        panic!("AutoPlayer search failed: Priority queue became empty.");
    }
}

// --- Main Function ---

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        // Use eprint! for stderr, like std::clog
        eprintln!("Usage: {} <input_file>", args[0]);
        // Return an error instead of exit code 1 for more Rust-idiomatic feel
        return Err("Input file argument required".into());
    }

    // Read board from file
    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);
    let mut board = Board::from_reader(reader)?; // Use the new parsing function

    println!("Initial board:");
    println!("{}", board); // Print initial state using Display impl

    // Choose player type
    // let mut player: Box<dyn BasePlayer> = Box::new(InteractivePlayer);
    let mut player: Box<dyn BasePlayer> = Box::new(AutoPlayer::new());

    // Game loop
    loop {
        // Check if cat reached the edge (loss condition)
        if board.cat_position.is_edge() {
            println!("Failed! The cat ran away!");
            break;
        }

        // Get and validate player's move
        let player_move;
        loop {
            let mv = player.get_move(&board);
            if board.move_player(mv) {
                println!("Accepted player's move: {}", mv);
                player_move = mv; // Keep track if needed, though not used later
                break; // Valid move accepted
            } else {
                // This part is only relevant for InteractivePlayer or flawed AutoPlayer
                eprintln!("Invalid move: {}. Try again.", mv);
                 // If AutoPlayer produces invalid move repeatedly, it's a bug.
                 // For InteractivePlayer, loop continues. Let's assume AutoPlayer is correct.
                 // If using InteractivePlayer, this loop correctly prompts again.
            }
        }

        println!("Board after player's move:");
        println!("{}", board);

        // Attempt to move the cat
        match board.move_cat() {
            Some(cat_move) => {
                println!("Cat moved: {}", cat_move);
                println!("Board after cat's move:");
                println!("{}", board);
            }
            None => {
                // Cat couldn't move - implies player wins
                println!("Succeeded! You trapped the cat!");
                break;
            }
        }
        // Add a small delay maybe?
        // std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ok(()) // Indicate successful execution
}