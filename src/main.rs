use ndarray::Array2;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use colored::Colorize;


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~ The following parameters control the process tuning ~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// Size of the maze
static DIM: usize = 7;

// Maze orientation ("horizontal", "vertical", "random")
static MAZE_ORIENTATION: &'static str = "random";

// Intermediate display
static SHOW_GENERATION_PROCESS: bool = false;
static SHOW_SOLVING_PROCESS: bool = false;

// Display the different steps of the process in the console either in ascii-art (either fancy, or pure)
static USE_FANCY_ASCII: bool = true;

// Random seed for reproducibility
static USE_RANDOM_SEED: bool = false;
static SEED: u64 = 893; // Japanese speakers will know what this number stands for ;)


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~ The following parameters shall not be modified ~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// Code-words for the four possible directions
static BASE: u8 = 2;
static L2R: u8 = BASE.pow(0); // (:=1) code for possible "left to right" move
static R2L: u8 = BASE.pow(1); // (:=2) code for possible "right to left" move
static U2D: u8 = BASE.pow(2); // (:=4) code for possible "up to down" move
static D2U: u8 = BASE.pow(3); // (:=8) code for possible "down to up" move

// Symbols to draw the maze in ASCII-art
static ENT_SYMB: &'static str = if USE_FANCY_ASCII {"◆"} else {"E"}; // ("E") entrance
static GOA_SYMB: &'static str = if USE_FANCY_ASCII {"♥"} else {"G"}; // ("G") goal
static WAL_SYMB: &'static str = if USE_FANCY_ASCII {"■"} else {"#"}; // ("#") wall
static NOD_SYMB: &'static str = if USE_FANCY_ASCII {"○"} else {"."}; // (".") generic node
static OPN_SYMB: &'static str = " "; // opening
static ALV_SYMB: &'static str = "a"; // node in the alive region
static NAR_SYMB: &'static str = "n"; // node in the narrow band
static FAR_SYMB: &'static str = "f"; // node in the far away region
static PATH_L2R_SYMB: &'static str = if USE_FANCY_ASCII {"→"} else {">"}; // "left to right" move of the shortest path
static PATH_R2L_SYMB: &'static str = if USE_FANCY_ASCII {"←"} else {"<"}; // "right to left" move of the shortest path
static PATH_U2D_SYMB: &'static str = if USE_FANCY_ASCII {"↓"} else {"v"}; // "up to down" move of the shortest path
static PATH_D2U_SYMB: &'static str = if USE_FANCY_ASCII {"↑"} else {"^"}; // "down to up" move of the shortest path
static HEX_1_SYMB: &'static str = "1"; // hexadecimal code for a possible move
static HEX_2_SYMB: &'static str = "2"; // hexadecimal code for a possible move
static HEX_3_SYMB: &'static str = "3"; // hexadecimal code for a possible move
static HEX_4_SYMB: &'static str = "4"; // hexadecimal code for a possible move
static HEX_5_SYMB: &'static str = "5"; // hexadecimal code for a possible move
static HEX_6_SYMB: &'static str = "6"; // hexadecimal code for a possible move
static HEX_7_SYMB: &'static str = "7"; // hexadecimal code for a possible move
static HEX_8_SYMB: &'static str = "8"; // hexadecimal code for a possible move
static HEX_9_SYMB: &'static str = "9"; // hexadecimal code for a possible move
static HEX_A_SYMB: &'static str = "A"; // hexadecimal code for a possible move
static HEX_B_SYMB: &'static str = "B"; // hexadecimal code for a possible move
static HEX_C_SYMB: &'static str = "C"; // hexadecimal code for a possible move
static HEX_D_SYMB: &'static str = "D"; // hexadecimal code for a possible move
static HEX_E_SYMB: &'static str = "E"; // hexadecimal code for a possible move
static HEX_F_SYMB: &'static str = "F"; // hexadecimal code for a possible move

// The purpose using these integer codes is that they can be stored in an array, and link to the corresponding string
static WAL_CODE: u8 = 100;      // wall
static NOD_CODE: u8 = 101;      // generic node
static OPN_CODE: u8 = 102;      // opening
static ALV_CODE: u8 = 103;      // node in the alive region
static NAR_CODE: u8 = 104;      // node in the narrow band
static FAR_CODE: u8 = 105;      // node in the far away region
static PATH_L2R_CODE: u8 = 200; // node corresponding to a "left to right" move of the shortest path
static PATH_R2L_CODE: u8 = 201; // node corresponding to a "right to left" move of the shortest path
static PATH_U2D_CODE: u8 = 202; // node corresponding to an "up to down" move of the shortest path
static PATH_D2U_CODE: u8 = 203; // node corresponding to a "down to up" move of the shortest path
static HEX_1_CODE: u8 = 1;      // hexadecimal code for a possible move
static HEX_2_CODE: u8 = 2;      // hexadecimal code for a possible move
static HEX_3_CODE: u8 = 3;      // hexadecimal code for a possible move
static HEX_4_CODE: u8 = 4;      // hexadecimal code for a possible move
static HEX_5_CODE: u8 = 5;      // hexadecimal code for a possible move
static HEX_6_CODE: u8 = 6;      // hexadecimal code for a possible move
static HEX_7_CODE: u8 = 7;      // hexadecimal code for a possible move
static HEX_8_CODE: u8 = 8;      // hexadecimal code for a possible move
static HEX_9_CODE: u8 = 9;      // hexadecimal code for a possible move
static HEX_A_CODE: u8 = 10;     // hexadecimal code for a possible move
static HEX_B_CODE: u8 = 11;     // hexadecimal code for a possible move
static HEX_C_CODE: u8 = 12;     // hexadecimal code for a possible move
static HEX_D_CODE: u8 = 13;     // hexadecimal code for a possible move
static HEX_E_CODE: u8 = 14;     // hexadecimal code for a possible move
static HEX_F_CODE: u8 = 15;     // hexadecimal code for a possible move

// 2D coordinates
struct Pos {
  x: usize,
  y: usize
}


// ----------------------------------------------------------------
// Generate a random "DIM"x"DIM" maze via recursive splitting
// Store the possible moves from each node via an unambiguous hexadecimal encoding
// Solve the maze via front-propagation and extract the corresponding path via backtracking
// Display the different processes and results in the console in ascii-art
// ----------------------------------------------------------------
fn main() {

  // Create a DIM*DIM maze array
  let mut maze = Array2::<u8>::zeros((DIM, DIM));
  let mut maze_opposite = Array2::<u8>::zeros((DIM, DIM));

  // Create a pseudo-random number generator
  let mut rng: StdRng = if USE_RANDOM_SEED {
    SeedableRng::seed_from_u64(SEED)
  } else {
    StdRng::from_entropy()
  };

  // Determine the maze orientation (true: horizontal; false: vertical)
  let orientation = if MAZE_ORIENTATION == "horizontal" {
    true
  } else if MAZE_ORIENTATION == "vertical" {
    false
  } else if MAZE_ORIENTATION == "random" {
    if rng.gen_range(0..2) > 0 {
      true
    } else {
      false
    }
  } else {
    panic!("Impossible orientation");
  };

  // Random choice for the entrance
  let pos_entrance = if orientation {
    Pos{x: 0, y: rng.gen_range(0..DIM)} // entrance on the left-side wall
  } else {
    Pos{x: rng.gen_range(0..DIM), y: 0} // entrance on the top-side wall
  };

  // Random choice for the goal
  let pos_goal = if orientation {
    Pos{x: DIM-1, y: rng.gen_range(0..DIM)} // goal on the right-side wall
  } else {
    Pos{x: rng.gen_range(0..DIM), y: DIM-1} // goal on the bottom-side wall
  };

  // Keep track of the intermediate states to show the generation process
  // This variable is "opposite" to the maze because in this process, maze_opposite are added instead of connections
  for x in 0..DIM {
    for y in 0..DIM {
      maze_opposite[[y, x]] = HEX_F_CODE;
      if x == 0 {
        maze_opposite[[y, x]] -= R2L;
      }
      if x == DIM -1 {
        maze_opposite[[y, x]] -= L2R;
      }
      if y == 0 {
        maze_opposite[[y, x]] -= D2U;
      }
      if y == DIM -1 {
        maze_opposite[[y, x]] -= U2D;
      }
    }
  }
  if SHOW_GENERATION_PROCESS {
    println!(
      "[generation] iteration: 0");
    let pos_dummy = Pos{x: DIM, y: DIM}; // Unreachable position for the entrance and goal, so they are not displayed
    print_maze_in_ascii(&maze_opposite, &orientation, &pos_dummy, &pos_dummy, &maze_opposite);
  }

  // Create the maze: uppermost call to the recursive process (independent of the position of the entrance and goal)
  let mut nb_iter_create: usize = 0;
  recursive_region_splitting(&mut maze, &mut maze_opposite, 0, DIM-1, 0, DIM-1, &orientation, &mut nb_iter_create, &mut rng);
  println!("[generation] Maze ({}x{} nodes) generated in {} iterations", DIM, DIM, nb_iter_create);

  // Print the naked maze in ascii
  let symbol_code = get_codenames_for_naked_maze();
  print_maze_in_ascii(&maze, &orientation, &pos_entrance, &pos_goal, &symbol_code);

  // Print the maze with hexa code
  println!("[encoding] Hexadecimal code defining the possible moves from each node");
  print_maze_in_ascii(&maze, &orientation, &pos_entrance, &pos_goal, &maze);

  // Solve the maze
  solve_maze(&maze, &pos_entrance, &pos_goal, &orientation, &mut rng);
}


// ----------------------------------------------------------------
// Recursively generate the maze by splitting a region in two sub-regions...
// ...and creating a connection between two neighbor nodes from each region
// The implicit stop condition for this recursive process is: "wall_max == wall_min && door_max == door_min"
// ----------------------------------------------------------------
fn recursive_region_splitting(
  maze: &mut Array2<u8>, maze_opposite: &mut Array2<u8>, wall_min: usize, wall_max: usize, door_min: usize,
  door_max: usize, orientation: &bool, nb_iter_create: &mut usize, rng: &mut StdRng) {

  // Increment the number of iterations
  *nb_iter_create += 1;

  // Subdivide the room
  if wall_max > wall_min {

    // Randomly determine the wall position
    let wall_pos = rng.gen_range(wall_min..wall_max);
    let wall_pos_plus_one = wall_pos +1;

    // Randomly determine the door position
    let door_pos = if door_max > door_min {
      rng.gen_range(door_min..door_max)
    } else {
      door_min
    };

    // Determine the code-word for the bi-directional move through the door in function of the current orientation
    match orientation {
      true => { // vertical separation, horizontal move
        maze[[door_pos, wall_pos]] += L2R;
        maze[[door_pos, wall_pos +1]] += R2L;
        // The following is only needed to display the intermediate steps during maze generation
        for y in door_min..door_pos {
          maze_opposite[[y, wall_pos]] -= L2R;
          maze_opposite[[y, wall_pos +1]] -= R2L;
        }
        for y in door_pos+1..door_max+1 {
          maze_opposite[[y, wall_pos]] -= L2R;
          maze_opposite[[y, wall_pos +1]] -= R2L;
        }
      },
      false => { // horizontal separation, vertical move
        maze[[wall_pos, door_pos]] += U2D;
        maze[[wall_pos +1, door_pos]] += D2U;
        // The following is only needed to display the intermediate steps during maze generation
        for x in door_min..door_pos {
          maze_opposite[[wall_pos, x]] -= U2D;
          maze_opposite[[wall_pos +1, x]] -= D2U;
        }
        for x in door_pos+1..door_max+1 {
          maze_opposite[[wall_pos, x]] -= U2D;
          maze_opposite[[wall_pos +1, x]] -= D2U;
        }
      }
    }

    // Display the intermediate steps
    if SHOW_GENERATION_PROCESS {
      println!(
        "[generation] iteration: {} | orientation: {} | wall: [{}, {}] --> {} | door: [{}, {}] --> {}",
        nb_iter_create, orientation, wall_min, wall_max, wall_pos, door_min, door_max, door_pos);
      let pos_dummy = Pos{x: DIM, y: DIM}; // Unreachable position for the entrance and goal, so they are not displayed
      print_maze_in_ascii(&maze_opposite, &orientation, &pos_dummy, &pos_dummy, &maze_opposite);
    }

    // Two recursive calls, on the regions in both sides of the wall
    if door_max > door_min {
      // Call with flipped orientation: the room is large enough to be subdivided along the other orientation
      recursive_region_splitting(maze, maze_opposite, door_min, door_max, wall_min, wall_pos, &!orientation, nb_iter_create, rng);
      recursive_region_splitting(maze, maze_opposite, door_min, door_max, wall_pos_plus_one, wall_max, &!orientation, nb_iter_create, rng);
    } else if door_max == door_min {
      // Call with same orientation: the room cannot be subdivided along the other orientation
      recursive_region_splitting(maze, maze_opposite, wall_min, wall_pos, door_min, door_max, &orientation, nb_iter_create, rng);
      recursive_region_splitting(maze, maze_opposite, wall_pos_plus_one, wall_max, door_min, door_max, &orientation, nb_iter_create, rng);
    }

  // Recursive call on the same room with flipped orientation
  } else if wall_max == wall_min && door_max > door_min {
    recursive_region_splitting(maze, maze_opposite, door_min, door_max, wall_min, wall_max, &!orientation, nb_iter_create, rng);
  }
}


// ----------------------------------------------------------------
// Routine to display the maze in the console in ascii-art
// ----------------------------------------------------------------
fn print_maze_in_ascii(
  maze: &Array2<u8>, orientation: &bool, pos_entrance: &Pos, pos_goal: &Pos, symbol_code: &Array2<u8>) {

  for y in 0..DIM {

    // Top half of the node
    for x in 0..DIM {
      // Check for entrance
      let wall_or_entrance: &str = if !*orientation && x == pos_entrance.x && y == 0 {
        ENT_SYMB
      } else {
        WAL_SYMB
      };
      print_ascii_node_top_half(maze[[y, x]], wall_or_entrance);
    }
    print!("{}\n", WAL_SYMB);

    // Bottom half of the node
    for x in 0..DIM {
      // Check for entrance
      let wall_or_entrance: &str = if *orientation && y == pos_entrance.y && x == 0 {
        ENT_SYMB
      } else {
        WAL_SYMB
      };

      // This Fugly routine juggles between u8 codenames (nicely storable in a 2D array)...
      // ...and the desired corresponding strings (seemingly impossible to store)
      let symb: &str = if symbol_code[[y, x]] == WAL_CODE {
        WAL_SYMB
      } else if symbol_code[[y, x]] == OPN_CODE {
        OPN_SYMB
      } else if symbol_code[[y, x]] == NOD_CODE {
        NOD_SYMB
      } else if symbol_code[[y, x]] == ALV_CODE {
        ALV_SYMB
      } else if symbol_code[[y, x]] == NAR_CODE {
        NAR_SYMB
      } else if symbol_code[[y, x]] == FAR_CODE {
        FAR_SYMB
      } else if symbol_code[[y, x]] == PATH_L2R_CODE {
        PATH_L2R_SYMB
      } else if symbol_code[[y, x]] == PATH_R2L_CODE {
        PATH_R2L_SYMB
      } else if symbol_code[[y, x]] == PATH_U2D_CODE {
        PATH_U2D_SYMB
      } else if symbol_code[[y, x]] == PATH_D2U_CODE {
        PATH_D2U_SYMB
      } else if symbol_code[[y, x]] == HEX_1_CODE {
        HEX_1_SYMB
      } else if symbol_code[[y, x]] == HEX_2_CODE {
        HEX_2_SYMB
      } else if symbol_code[[y, x]] == HEX_3_CODE {
        HEX_3_SYMB
      } else if symbol_code[[y, x]] == HEX_4_CODE {
        HEX_4_SYMB
      } else if symbol_code[[y, x]] == HEX_5_CODE {
        HEX_5_SYMB
      } else if symbol_code[[y, x]] == HEX_6_CODE {
        HEX_6_SYMB
      } else if symbol_code[[y, x]] == HEX_7_CODE {
        HEX_7_SYMB
      } else if symbol_code[[y, x]] == HEX_8_CODE {
        HEX_8_SYMB
      } else if symbol_code[[y, x]] == HEX_9_CODE {
        HEX_9_SYMB
      } else if symbol_code[[y, x]] == HEX_A_CODE {
        HEX_A_SYMB
      } else if symbol_code[[y, x]] == HEX_B_CODE {
        HEX_B_SYMB
      } else if symbol_code[[y, x]] == HEX_C_CODE {
        HEX_C_SYMB
      } else if symbol_code[[y, x]] == HEX_D_CODE {
        HEX_D_SYMB
      } else if symbol_code[[y, x]] == HEX_E_CODE {
        HEX_E_SYMB
      } else if symbol_code[[y, x]] == HEX_F_CODE {
        HEX_F_SYMB
      } else {
        panic!("Impossible configuration");
      };
      print_ascii_node_bot_half(maze[[y, x]], wall_or_entrance, symb);

    }

    // Check for goal
    if *orientation && y == pos_goal.y {
      print!("{}\n", GOA_SYMB.red());
    } else {
      print!("{}\n", WAL_SYMB);
    }
  }

  // Check for goal
  for x in 0..DIM {
    if !*orientation && x == pos_goal.x {
      print!("{}{}", WAL_SYMB, GOA_SYMB.red());
    } else {
      print!("{}{}", WAL_SYMB, WAL_SYMB);
    }
  }
  print!("{}\n\n", WAL_SYMB);
}


// ----------------------------------------------------------------
// Routine to print the top-part of a given node
// ----------------------------------------------------------------
fn print_ascii_node_top_half(code: u8, wall_or_entrance: &str) {

  // Glorious hack to print symbols in a semantically-specific color
  // Nodes with hexa code (1, 2, 3, 4, 5, 6, 7) can possibly have their top side facing a wall...
  // ...except when in contact with the entrance
  let wall_or_entrance_color = if wall_or_entrance == ENT_SYMB {
    wall_or_entrance.red()
  } else {
    wall_or_entrance.normal()
  };

  if code == HEX_1_CODE {
    print!("{}{}", WAL_SYMB, wall_or_entrance_color);
  } else if code == HEX_2_CODE {
    print!("{}{}", WAL_SYMB, wall_or_entrance_color);
  } else if code == HEX_3_CODE {
    print!("{}{}", WAL_SYMB, wall_or_entrance_color);
  } else if code == HEX_4_CODE {
    print!("{}{}", WAL_SYMB, wall_or_entrance_color);
  } else if code == HEX_5_CODE {
    print!("{}{}", WAL_SYMB, wall_or_entrance_color);
  } else if code == HEX_6_CODE {
    print!("{}{}", WAL_SYMB, wall_or_entrance_color);
  } else if code == HEX_7_CODE {
    print!("{}{}", WAL_SYMB, wall_or_entrance_color);
  } else if code == HEX_8_CODE {
    print!("{}{}", WAL_SYMB, OPN_SYMB);
  } else if code == HEX_9_CODE {
    print!("{}{}", WAL_SYMB, OPN_SYMB);
  } else if code == HEX_A_CODE {
    print!("{}{}", WAL_SYMB, OPN_SYMB);
  } else if code == HEX_B_CODE {
    print!("{}{}", WAL_SYMB, OPN_SYMB);
  } else if code == HEX_C_CODE {
    print!("{}{}", WAL_SYMB, OPN_SYMB);
  } else if code == HEX_D_CODE {
    print!("{}{}", WAL_SYMB, OPN_SYMB);
  } else if code == HEX_E_CODE {
    print!("{}{}", WAL_SYMB, OPN_SYMB);
  } else if code == HEX_F_CODE {
    print!("{}{}", WAL_SYMB, OPN_SYMB);
  } else {
    panic!("Impossible code: {}", code);
  }
}


// ----------------------------------------------------------------
// Routine to print the bottom part of a given node
// ----------------------------------------------------------------
fn print_ascii_node_bot_half(code: u8, wall_or_entrance: &str, symb: &str) {

  // Glorious hack to print symbols in a semantically-specific color
  // Nodes with hexa code (1, 4, 5, 8, 9, C, D) can possibly have their left side facing a wall...
  // ...except when in contact with the entrance
  let wall_or_entrance_color = if wall_or_entrance == ENT_SYMB {
    wall_or_entrance.red()
  } else {
    wall_or_entrance.normal()
  };

  // Glorious hack to print symbols in a semantically-specific color
  let colored_symb = if symb == NOD_SYMB {
    symb.blue()
  } else if symb == ALV_SYMB {
    symb.yellow()
  } else if symb == NAR_SYMB {
    symb.magenta()
  } else if symb == FAR_SYMB {
    symb.cyan()
  } else if symb == PATH_L2R_SYMB || symb == PATH_R2L_SYMB || symb == PATH_U2D_SYMB || symb == PATH_D2U_SYMB {
    symb.red()
  } else if symb == HEX_1_SYMB || symb == HEX_2_SYMB || symb == HEX_3_SYMB || symb == HEX_4_SYMB ||
    symb == HEX_5_SYMB || symb == HEX_6_SYMB || symb == HEX_7_SYMB || symb == HEX_8_SYMB ||
    symb == HEX_9_SYMB || symb == HEX_A_SYMB || symb == HEX_B_SYMB || symb == HEX_C_SYMB ||
    symb == HEX_D_SYMB || symb == HEX_E_SYMB || symb == HEX_F_SYMB {
    symb.green()
  } else {
    panic!("Impossible symbol: {}", symb);
  };

  if code == HEX_1_CODE {
    print!("{}{}", wall_or_entrance_color, colored_symb);
  } else if code == HEX_2_CODE {
    print!("{}{}", OPN_SYMB, colored_symb);
  } else if code == HEX_3_CODE {
    print!("{}{}", OPN_SYMB, colored_symb);
  } else if code == HEX_4_CODE {
    print!("{}{}", wall_or_entrance_color, colored_symb);
  } else if code == HEX_5_CODE {
    print!("{}{}", wall_or_entrance_color, colored_symb);
  } else if code == HEX_6_CODE {
    print!("{}{}", OPN_SYMB, colored_symb);
  } else if code == HEX_7_CODE {
    print!("{}{}", OPN_SYMB, colored_symb);
  } else if code == HEX_8_CODE {
    print!("{}{}", wall_or_entrance_color, colored_symb);
  } else if code == HEX_9_CODE {
    print!("{}{}", wall_or_entrance_color, colored_symb);
  } else if code == HEX_A_CODE {
    print!("{}{}", OPN_SYMB, colored_symb);
  } else if code == HEX_B_CODE {
    print!("{}{}", OPN_SYMB, colored_symb);
  } else if code == HEX_C_CODE {
    print!("{}{}", wall_or_entrance_color, colored_symb);
  } else if code == HEX_D_CODE {
    print!("{}{}", wall_or_entrance_color, colored_symb);
  } else if code == HEX_E_CODE {
    print!("{}{}", OPN_SYMB, colored_symb);
  } else if code == HEX_F_CODE {
    print!("{}{}", OPN_SYMB, colored_symb);
  } else {
    panic!("Impossible code: {}", code);
  }
}


// ----------------------------------------------------------------
// From a given code-value "maze[[pos.y, pos.x]] = a*L2R + b*R2L +c*U2D + d*D2U", s.t. {a, b, c, d} are boolean,...
// ...retrieve the individual components {L2R, R2L, U2D, and/or D2U} and store them in the vector "possible_moves"
// The encoding scheme consists of fifteen different values ranging from 1 to 15, describing the possible moves
// No need to encode a "zero" symbol because this would correspond to a non-existing fully-closed unit-sized room
// Interestingly, each symbol {L2R, R2L, U2D, D2U} appears exactly eight times in the encoding scheme
// ----------------------------------------------------------------
fn get_possible_moves(maze: &Array2<u8>, pos: &Pos) -> Vec<Pos> {
  let mut possible_moves = vec![];
  if maze[[pos.y, pos.x]] == L2R {
    possible_moves.push(Pos{x: pos.x +1, y: pos.y});
  } else if maze[[pos.y, pos.x]] == R2L {
    possible_moves.push(Pos{x: pos.x -1, y: pos.y});
  } else if maze[[pos.y, pos.x]] == R2L + L2R {
    possible_moves.push(Pos{x: pos.x -1, y: pos.y});
    possible_moves.push(Pos{x: pos.x +1, y: pos.y});
  } else if maze[[pos.y, pos.x]] == U2D {
    possible_moves.push(Pos{x: pos.x, y: pos.y +1});
  } else if maze[[pos.y, pos.x]] == U2D + L2R {
    possible_moves.push(Pos{x: pos.x +1, y: pos.y});
    possible_moves.push(Pos{x: pos.x, y: pos.y +1});
  } else if maze[[pos.y, pos.x]] == U2D + R2L {
    possible_moves.push(Pos{x: pos.x -1, y: pos.y});
    possible_moves.push(Pos{x: pos.x, y: pos.y +1});
  } else if maze[[pos.y, pos.x]] == U2D + R2L + L2R {
    possible_moves.push(Pos{x: pos.x -1, y: pos.y});
    possible_moves.push(Pos{x: pos.x +1, y: pos.y});
    possible_moves.push(Pos{x: pos.x, y: pos.y +1});
  } else if maze[[pos.y, pos.x]] == D2U {
    possible_moves.push(Pos{x: pos.x, y: pos.y -1});
  } else if maze[[pos.y, pos.x]] == D2U + L2R {
    possible_moves.push(Pos{x: pos.x +1, y: pos.y});
    possible_moves.push(Pos{x: pos.x, y: pos.y -1});
  } else if maze[[pos.y, pos.x]] == D2U + R2L {
    possible_moves.push(Pos{x: pos.x -1, y: pos.y});
    possible_moves.push(Pos{x: pos.x, y: pos.y -1});
  } else if maze[[pos.y, pos.x]] == D2U + R2L + L2R {
    possible_moves.push(Pos{x: pos.x -1, y: pos.y});
    possible_moves.push(Pos{x: pos.x +1, y: pos.y});
    possible_moves.push(Pos{x: pos.x, y: pos.y -1});
  } else if maze[[pos.y, pos.x]] == D2U + U2D {
    possible_moves.push(Pos{x: pos.x, y: pos.y -1});
    possible_moves.push(Pos{x: pos.x, y: pos.y +1});
  } else if maze[[pos.y, pos.x]] == D2U + U2D + L2R {
    possible_moves.push(Pos{x: pos.x +1, y: pos.y});
    possible_moves.push(Pos{x: pos.x, y: pos.y -1});
    possible_moves.push(Pos{x: pos.x, y: pos.y +1});
  } else if maze[[pos.y, pos.x]] == D2U + U2D + R2L{
    possible_moves.push(Pos{x: pos.x -1, y: pos.y});
    possible_moves.push(Pos{x: pos.x, y: pos.y -1});
    possible_moves.push(Pos{x: pos.x, y: pos.y +1});
  } else if maze[[pos.y, pos.x]] == D2U + U2D + R2L + L2R {
    possible_moves.push(Pos{x: pos.x -1, y: pos.y});
    possible_moves.push(Pos{x: pos.x +1, y: pos.y});
    possible_moves.push(Pos{x: pos.x, y: pos.y -1});
    possible_moves.push(Pos{x: pos.x, y: pos.y +1});
  } else {
    panic!("Impossible move: {}", maze[[pos.y, pos.x]]);
  }
  possible_moves
}


// ----------------------------------------------------------------
// The possible candidate moves from a given node have first been determined based on the presence of walls and/or doors
// Here, the possible moves are further restricted to exclude candidate moves that do not land in the "far" region
// ----------------------------------------------------------------
fn refine_moves_based_on_far_region(mut candidate_moves: Vec<Pos>, far_region: &Array2<bool>) -> Vec<Pos> {
  let mut idx = 0; // This index is different to the one used in the for loop so it can adapt when elements are removed
  for _idx_bis in 0..candidate_moves.iter().len() {
    if !far_region[[candidate_moves[idx].y, candidate_moves[idx].x]] {
      candidate_moves.remove(idx); // Remove candidate moves that land in a region that was already explored
    } else {
      idx +=1;
    }
  }
  candidate_moves
}


// ----------------------------------------------------------------
// Prepare a lookup table of "codenames" so each node of the maze is associated with their corresponding ascii symbol
// Here, the simple floorplan of the maze is displayed, therefore each node is represented by "NOD_SYMB"
// ----------------------------------------------------------------
fn get_codenames_for_naked_maze() -> Array2<u8> {
  let mut codenames = Array2::<u8>::zeros((DIM, DIM));
  for y in 0..DIM {
    for x in 0..DIM {
      codenames[[y, x]] = NOD_CODE;
    }
  }
  codenames
}


// ----------------------------------------------------------------
// Prepare a lookup table of "codenames" so each node of the maze is associated with their corresponding ascii symbol
// Here, the current state of the front propagation is displayed, using "ALV_SYMB", "NAR_SYMB", or "FAR_SYMB"
// ----------------------------------------------------------------
fn get_codenames_for_alv_nar_far_regions(
  alv_region: &Array2<bool>, nar_region: &Array2<bool>, far_region: &Array2<bool>) -> Array2<u8> {
  let mut codenames = Array2::<u8>::zeros((DIM, DIM));
  for y in 0..DIM {
    for x in 0..DIM {
      codenames[[y, x]] = if alv_region[[y, x]] {
        ALV_CODE
      } else if nar_region[[y, x]] {
        NAR_CODE
      } else if far_region[[y, x]] {
        FAR_CODE
      } else {
        panic!("Impossible alv/nar/far configuration");
      };
    }
  }
  codenames
}


// ----------------------------------------------------------------
// Random search on the narrow band (not a depth-first search, not a breadth-first search, not a cost-first search)
// ----------------------------------------------------------------
fn solve_maze(maze: &Array2<u8>, pos_entrance: &Pos, pos_goal: &Pos, orientation: &bool, rng: &mut StdRng) {

  let mut alv_region = Array2::<u8>::zeros((DIM, DIM)).mapv(|_| false);
  let mut nar_region = Array2::<u8>::zeros((DIM, DIM)).mapv(|_| false);
  let mut far_region = Array2::<u8>::zeros((DIM, DIM)).mapv(|_| true);
  let mut vec_narrow = vec![];
  let mut backtracking_x = Array2::<usize>::zeros((DIM, DIM));
  let mut backtracking_y = Array2::<usize>::zeros((DIM, DIM));
  let mut goal_has_been_reached: bool = false;
  let mut nb_iter_solve: usize = 0;

  // Place the entrance
  vec_narrow.push(Pos{x: pos_entrance.x, y: pos_entrance.y});
  nar_region[[pos_entrance.y, pos_entrance.x]] = true;
  far_region[[pos_entrance.y, pos_entrance.x]] = false;

  // Start searching
  while !goal_has_been_reached {

    // Increase the number of steps
    nb_iter_solve += 1;

    //Select a node from the narrow region
    let idx_cell = if nar_region[[pos_goal.y, pos_goal.x]] {
      // Select the goal, if the goal is in the narrow region
      vec_narrow.iter().position(|r| r.x == pos_goal.x && r.y == pos_goal.y).unwrap()
    } else {
      // Else, select a random node in the narrow region
      rng.gen_range(0..vec_narrow.iter().len())
    };

    // Instanciate a new position object with the newly-selected node
    let pos = Pos{x: vec_narrow[idx_cell].x, y: vec_narrow[idx_cell].y};
    alv_region[[pos.y, pos.x]]= !alv_region[[pos.y, pos.x]]; // Switch from "false" to "true"
    nar_region[[pos.y, pos.x]]= !nar_region[[pos.y, pos.x]]; // Switch from "true" to "false"
    vec_narrow.remove(idx_cell);

    // Trigger the end of the search if the goal has been reached
    if alv_region[[pos_goal.y, pos_goal.x]] {
      goal_has_been_reached = true;
    }

    // Update the narrow band and the far away region in function of the new alive node
    let mut v = get_possible_moves(&maze, &pos);
    v = refine_moves_based_on_far_region(v, &far_region);
    let nb_possible_moves = v.iter().len();
    if nb_possible_moves > 0 {
      for idx in 0..nb_possible_moves {
        if far_region[[v[idx].y, v[idx].x]] {
          far_region[[v[idx].y, v[idx].x]] = !far_region[[v[idx].y, v[idx].x]];
          nar_region[[v[idx].y, v[idx].x]] = !nar_region[[v[idx].y, v[idx].x]];
          vec_narrow.push(Pos{x: v[idx].x, y: v[idx].y});
          backtracking_x[[v[idx].y, v[idx].x]] = pos.x;
          backtracking_y[[v[idx].y, v[idx].x]] = pos.y;
        }
      }
    }

    // Display the front propagation
    if SHOW_SOLVING_PROCESS {
      println!("[propagation] iteration: {} | alive region: {}% | narrow band: {}% | far-away region: {}%",
      nb_iter_solve,
      100*count_nb_of_true(&alv_region)/(DIM*DIM),
      100*count_nb_of_true(&nar_region)/(DIM*DIM),
      100*count_nb_of_true(&far_region)/(DIM*DIM));
      let symbol_code = get_codenames_for_alv_nar_far_regions(&alv_region, &nar_region, &far_region);
      print_maze_in_ascii(&maze, &orientation, &pos_entrance, &pos_goal, &symbol_code);
    }
  }

  // Display only the last step of the front propagation
  if !SHOW_SOLVING_PROCESS {
    println!(
      "[propagation] Maze solved in {} iteration ({}% of the nodes have been visited)",
      nb_iter_solve, 100*nb_iter_solve/(DIM*DIM));
    let symbol_code = get_codenames_for_alv_nar_far_regions(&alv_region, &nar_region, &far_region);
    print_maze_in_ascii(&maze, &orientation, &pos_entrance, &pos_goal, &symbol_code);
  }

  // Extract the path via backtracking
  conduct_backtracking(&maze, &backtracking_x, &backtracking_y, &pos_entrance, &pos_goal, &orientation);
}


// ----------------------------------------------------------------
// Extract the (unique and therefore shortest) path from the entrance to the goal via backtracking
// ----------------------------------------------------------------
fn conduct_backtracking(
  maze: &Array2<u8>, backtracking_x: &Array2<usize>, backtracking_y: &Array2<usize>, pos_entrance: &Pos, pos_goal: &Pos,
  orientation: &bool) {

  // Initialize the backtracking array with the code-word for generic nodes
  let mut backtracking = Array2::<u8>::zeros((DIM, DIM));
  for y in 0..DIM {
    for x in 0..DIM {
      backtracking[[y, x]] = NOD_CODE;
    }
  }

  // Start at the goal position
  let mut path_length: usize = 1;
  let mut pos_current = Pos{x: pos_goal.x, y: pos_goal.y};
  let mut shortest_path = vec![];
  shortest_path.push(Pos{x: pos_current.x, y: pos_current.y});
  backtracking[[pos_current.y, pos_current.x]] = if *orientation {
    PATH_L2R_CODE
  } else {
    PATH_U2D_CODE
  };

  // Iteratively backtrack the path from the goal to the entrance using the connections stored during front propagation
  let mut entrance_has_been_reached: bool = false;
  while !entrance_has_been_reached {

    // Check if entrance has been reached, otherwise continue the bactracking
    if pos_current.x == pos_entrance.x && pos_current.y == pos_entrance.y {
      entrance_has_been_reached = true;
    } else {
      // Increase the path length
      path_length += 1;

      // Retrieve the previous position
      let pos_prev = Pos{
        x: backtracking_x[[pos_current.y, pos_current.x]], y: backtracking_y[[pos_current.y, pos_current.x]]};

      // Encode the path direction (either L2R:">", R2L:"<", U2D:"v", or D2U:"^") in the backtracking array
      backtracking[[pos_prev.y, pos_prev.x]] = if pos_current.x == pos_prev.x +1 && pos_current.y == pos_prev.y {
        PATH_L2R_CODE
      } else if pos_prev.x > 0 && pos_current.x == pos_prev.x -1 && pos_current.y == pos_prev.y {
        PATH_R2L_CODE
      } else if pos_current.x == pos_prev.x && pos_current.y == pos_prev.y +1 {
        PATH_U2D_CODE
      } else if pos_prev.y > 0 && pos_current.x == pos_prev.x && pos_current.y == pos_prev.y -1 {
        PATH_D2U_CODE
      } else {
        panic!("Impossible path");
      };

      // Update the current position
      pos_current = pos_prev;

      // Update the backtracking vector
      shortest_path.push(Pos{x: pos_current.x, y: pos_current.y});
    }
  }

  // Print the maze with the shortest path
  println!("[backtracking] Path length: {}", path_length);
  print_maze_in_ascii(&maze, &orientation, &pos_entrance, &pos_goal, &backtracking);

  // Reverse the order of the path so it goes from the entrance to the goal, and print the step-by-step solution
  shortest_path.reverse();
  print!("Path:");
  for idx in 0..path_length {
    if idx % 10 == 0 {
      print!("\n");
    }
    print!("{}:({},{}) ", idx, shortest_path[idx].x, shortest_path[idx].y);
  }
  print!("\n");
}


// ----------------------------------------------------------------
// Count the number of "True" in a boolean vector
// ----------------------------------------------------------------
fn count_nb_of_true(alv_nar_far_array: &Array2<bool>) -> usize {
  let mut nb_of_true = 0;
  for x in 0..DIM {
    for y in 0..DIM {
      if alv_nar_far_array[[y, x]] == true {
        nb_of_true +=1;
      }
    }
  }
  nb_of_true
}
