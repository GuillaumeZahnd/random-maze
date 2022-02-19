use ndarray::Array2;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use colored::Colorize;


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~ The following parameters can be modified ~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// Size of the maze
static DIM: usize = 7;

// Random seed
static USE_RANDOM_SEED: bool = false;
static SEED: u64 = 101;

// Intermediate display
static SHOW_GENERATION_PROCESS: bool = false;
static SHOW_SOLVING_PROCESS: bool = false;


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
static ENT_SYMB: &'static str = "◆"; // ("E") entrance
static GOA_SYMB: &'static str = "♥"; // ("G") goal
static WAL_SYMB: &'static str = "■"; // ("#") wall
static OPN_SYMB: &'static str = " "; // opening
static NOD_SYMB: &'static str = "○"; // (".") node
static ALV_SYMB: &'static str = "a"; // node in the alive region
static NAR_SYMB: &'static str = "n"; // node in the narrow band
static FAR_SYMB: &'static str = "f"; // node in the far away region
static PATH_L2R_SYMB: &'static str = "→"; // (">") node corresponding to a "left to right" move of the shortest path
static PATH_R2L_SYMB: &'static str = "←"; // ("<") node corresponding to a "right to left" move of the shortest path
static PATH_U2D_SYMB: &'static str = "↓"; // ("v") node corresponding to an "up to down" move of the shortest path
static PATH_D2U_SYMB: &'static str = "↑"; // ("^") node corresponding to a "down to up" move of the shortest path
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
static ENT_CODE: u8 = 100; // entrance
static GOA_CODE: u8 = 101; // goal
static WAL_CODE: u8 = 102; // wall
static OPN_CODE: u8 = 103; // opening
static NOD_CODE: u8 = 104; // node
static ALV_CODE: u8 = 105; // node in the alive region
static NAR_CODE: u8 = 106; // node in the narrow band
static FAR_CODE: u8 = 107; // node in the far away region
static PATH_L2R_CODE: u8 = 200; // node corresponding to a "left to right" move of the shortest path
static PATH_R2L_CODE: u8 = 201; // node corresponding to a "right to left" move of the shortest path
static PATH_U2D_CODE: u8 = 202; // node corresponding to an "up to down" move of the shortest path
static PATH_D2U_CODE: u8 = 203; // node corresponding to a "down to up" move of the shortest path
static HEX_1_CODE: u8 = 1; // hexadecimal code for a possible move
static HEX_2_CODE: u8 = 2; // hexadecimal code for a possible move
static HEX_3_CODE: u8 = 3; // hexadecimal code for a possible move
static HEX_4_CODE: u8 = 4; // hexadecimal code for a possible move
static HEX_5_CODE: u8 = 5; // hexadecimal code for a possible move
static HEX_6_CODE: u8 = 6; // hexadecimal code for a possible move
static HEX_7_CODE: u8 = 7; // hexadecimal code for a possible move
static HEX_8_CODE: u8 = 8; // hexadecimal code for a possible move
static HEX_9_CODE: u8 = 9; // hexadecimal code for a possible move
static HEX_A_CODE: u8 = 10; // hexadecimal code for a possible move
static HEX_B_CODE: u8 = 11; // hexadecimal code for a possible move
static HEX_C_CODE: u8 = 12; // hexadecimal code for a possible move
static HEX_D_CODE: u8 = 13; // hexadecimal code for a possible move
static HEX_E_CODE: u8 = 14; // hexadecimal code for a possible move
static HEX_F_CODE: u8 = 15; // hexadecimal code for a possible move

// 2D coordinates
struct Pos {
  x: usize,
  y: usize
}


// ----------------------------------------------------------------
fn main() {

  // Create a DIM*DIM array
  let mut maze = Array2::<u8>::zeros((DIM, DIM));

  // Create a random number generator
  let mut rng: StdRng = if USE_RANDOM_SEED {
    SeedableRng::seed_from_u64(SEED)
  } else {
    StdRng::from_entropy()
  };

  // Random choice for the first separation
  let orientation = if rng.gen_range(0..2) > 0 {
    true // vertical
  } else {
    false // horizontal
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

  // Create the maze: uppermost call to the recursive process
  create_wall_with_door(&mut maze, 0, DIM-1, 0, DIM-1, &orientation, &mut rng);

  // Print the naked maze in ascii
  let symbol_code = get_codenames_for_naked_maze();
  print_maze_in_ascii(&maze, &orientation, &pos_entrance, &pos_goal, &symbol_code);

  // Print the maze with hexa code
  print_maze_in_ascii(&maze, &orientation, &pos_entrance, &pos_goal, &maze);

  // Solve the maze
  solve_maze(&maze, &pos_entrance, &pos_goal, &orientation, &mut rng);
}


// ----------------------------------------------------------------
// Random search on the narrow band (not a depth-first search, not a breadth-first search, not a cost-first search)
fn solve_maze(maze: &Array2<u8>, pos_entrance: &Pos, pos_goal: &Pos, orientation: &bool, rng: &mut StdRng) {

  let mut alive = Array2::<u8>::zeros((DIM, DIM)).mapv(|_| false);
  let mut narrow = Array2::<u8>::zeros((DIM, DIM)).mapv(|_| false);
  let mut far = Array2::<u8>::zeros((DIM, DIM)).mapv(|_| true);
  let mut vec_narrow = vec![];
  let mut backtracking = Array2::<u8>::zeros((DIM, DIM));
  let mut backtracking_x = Array2::<usize>::zeros((DIM, DIM));
  let mut backtracking_y = Array2::<usize>::zeros((DIM, DIM));
  let mut vec_backtracking = vec![];
  let mut goal_has_been_reached: bool = false;
  let mut nb_iter_solve: usize = 0;

  vec_narrow.push(Pos{x: pos_entrance.x, y: pos_entrance.y});
  narrow[[pos_entrance.y, pos_entrance.x]] = true;
  far[[pos_entrance.y, pos_entrance.x]] = false;

  while !goal_has_been_reached {

    nb_iter_solve += 1;

    // Select a node from the narrow region
    let idx_cell = if narrow[[pos_goal.y, pos_goal.x]] {
      // Select the goal, if the goal is in the narrow region
      vec_narrow.iter().position(|r| r.x == pos_goal.x && r.y == pos_goal.y).unwrap()
    } else {
      // Else, select a random node in the narrow region
      rng.gen_range(0..vec_narrow.iter().len())
    };

    let pos = Pos{x: vec_narrow[idx_cell].x, y: vec_narrow[idx_cell].y};

    alive[[pos.y, pos.x]]= !alive[[pos.y, pos.x]]; // Switch from "false" to "true"
    narrow[[pos.y, pos.x]]= !narrow[[pos.y, pos.x]]; // Switch from "true" to "false"
    vec_narrow.remove(idx_cell);

    // Trigger the end of the search
    if alive[[pos_goal.y, pos_goal.x]] {
      goal_has_been_reached = true;
      println!(
        "Maze solved in {} iteration ({}% of the nodes have been visited)", nb_iter_solve, 100*nb_iter_solve/(DIM*DIM));
    }

    // Update the narrow band and the far away region in function of the new alive node
    let mut v = get_possible_moves(&maze, &pos);
    v = refine_moves_based_on_far_region(v, &far);
    let nb_possible_moves = v.iter().len();
    if nb_possible_moves > 0 {
      for idx in 0..nb_possible_moves {
        if far[[v[idx].y, v[idx].x]] {
          far[[v[idx].y, v[idx].x]] = !far[[v[idx].y, v[idx].x]];
          narrow[[v[idx].y, v[idx].x]] = !narrow[[v[idx].y, v[idx].x]];
          vec_narrow.push(Pos{x: v[idx].x, y: v[idx].y});
          backtracking_x[[v[idx].y, v[idx].x]] = pos.x;
          backtracking_y[[v[idx].y, v[idx].x]] = pos.y;
        }
      }
    }

    // Display the front propagation
    if SHOW_SOLVING_PROCESS {
      let symbol_code = get_codenames_for_alv_nar_far_regions(&alive, &narrow, &far);
      print_maze_in_ascii(&maze, &orientation, &pos_entrance, &pos_goal, &symbol_code);
    }

  }

  // Backtracking
  for y in 0..DIM {
    for x in 0..DIM {
      backtracking[[y, x]] = NOD_CODE;
    }
  }
  let symbol_code = get_codenames_for_alv_nar_far_regions(&alive, &narrow, &far);
  print_maze_in_ascii(&maze, &orientation, &pos_entrance, &pos_goal, &symbol_code);
  backtracking[[pos_goal.y, pos_goal.x]] = if *orientation { // TODO there is a borrow here
    PATH_L2R_CODE
  } else {
    PATH_U2D_CODE
  };
  vec_backtracking.push(Pos{x: pos_goal.x, y: pos_goal.y});
  let mut cont: bool = true;
  let mut current_pos = Pos{x: pos_goal.x, y: pos_goal.y};
  while cont {
    if current_pos.x == pos_entrance.x && current_pos.y == pos_entrance.y {
      cont = false;
    } else {

      // Retrieve the previous position
      let mut previous_pos = Pos{
        x: backtracking_x[[current_pos.y, current_pos.x]], y: backtracking_y[[current_pos.y, current_pos.x]]};

      // Encode the path direction (either L2R, R2L, U2D, or D2U) in the backtracking array
      backtracking[[previous_pos.y, previous_pos.x]] = if current_pos.x == previous_pos.x +1 && current_pos.y == previous_pos.y {
        PATH_L2R_CODE
      } else if previous_pos.x > 0 && current_pos.x == previous_pos.x -1 && current_pos.y == previous_pos.y {
        PATH_R2L_CODE
      } else if current_pos.x == previous_pos.x && current_pos.y == previous_pos.y +1 {
        PATH_U2D_CODE
      } else if previous_pos.y > 0 && current_pos.x == previous_pos.x && current_pos.y == previous_pos.y -1 {
        PATH_D2U_CODE
      } else {
        panic!("Impossible path");
      };

      // Update the current position
      current_pos = previous_pos;

      // Update the backtracking vector // TODO --> This is currently not used
      vec_backtracking.push(Pos{x: current_pos.x, y: current_pos.y});
    }
  }
  print_maze_in_ascii(&maze, &orientation, &pos_entrance, &pos_goal, &backtracking);

}


// ----------------------------------------------------------------
fn refine_moves_based_on_far_region(mut v: Vec<Pos>, far: &Array2<bool>) -> Vec<Pos> {
  let mut idx_remove_proof = 0;
  for _idx in 0..v.iter().len() {
    if !far[[v[idx_remove_proof].y, v[idx_remove_proof].x]] {
      v.remove(idx_remove_proof);
    } else {
      idx_remove_proof +=1;
    }
  }
  v
}


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
fn get_codenames_for_alv_nar_far_regions(alive: &Array2<bool>, narrow: &Array2<bool>, far: &Array2<bool>) -> Array2<u8> {
  let mut codenames = Array2::<u8>::zeros((DIM, DIM));
  for y in 0..DIM {
    for x in 0..DIM {
      codenames[[y, x]] = if alive[[y, x]] {
          ALV_CODE
        } else if narrow[[y, x]] {
          NAR_CODE
        } else if far[[y, x]] {
          FAR_CODE
        } else {
          panic!("Impossible configuration");
        };
    }
  }
  codenames
}


// ----------------------------------------------------------------
fn print_maze_in_ascii(
  maze: &Array2<u8>, orientation: &bool, pos_entrance: &Pos, pos_goal: &Pos, symbol_code: &Array2<u8>) {
  //alive: &Array2<bool>, narrow: &Array2<bool>, far: &Array2<bool>) {
  for y in 0..DIM {
    for x in 0..DIM {
      // Check for entrance
      let opn_or_wal: &str = if !*orientation && x == pos_entrance.x && y == 0 {
        ENT_SYMB
      } else {
        WAL_SYMB
      };
       print_node_in_ascii(maze[[y, x]], 0, opn_or_wal, WAL_SYMB);
    }
    print!("{}\n", WAL_SYMB);
    for x in 0..DIM {
      // Check for entrance
      let opn_or_wal: &str = if *orientation && y == pos_entrance.y && x == 0 {
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
      print_node_in_ascii(maze[[y, x]], 1, opn_or_wal, symb);

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
fn  print_node_in_ascii(code: u8, line: u8, opn_or_wal: &str, symb: &str) {

  let colored_opn_or_wal = if opn_or_wal == ENT_SYMB {
    opn_or_wal.red()
  } else {
    opn_or_wal.normal()
  };

  if line == 0 {
    if code == 1 {
      print!("{}{}", WAL_SYMB, colored_opn_or_wal);
    } else if code == 2 {
      print!("{}{}", WAL_SYMB, colored_opn_or_wal);
    } else if code == 3 {
      print!("{}{}", WAL_SYMB, colored_opn_or_wal);
    } else if code == 4 {
      print!("{}{}", WAL_SYMB, colored_opn_or_wal);
    } else if code == 5 {
      print!("{}{}", WAL_SYMB, colored_opn_or_wal);
    } else if code == 6 {
      print!("{}{}", WAL_SYMB, colored_opn_or_wal);
    } else if code == 7 {
      print!("{}{}", WAL_SYMB, colored_opn_or_wal);
    } else if code == 8 {
      print!("{}{}", WAL_SYMB, OPN_SYMB);
    } else if code == 9 {
      print!("{}{}", WAL_SYMB, OPN_SYMB);
    } else if code == 10 {
      print!("{}{}", WAL_SYMB, OPN_SYMB);
    } else if code == 11 {
      print!("{}{}", WAL_SYMB, OPN_SYMB);
    } else if code == 12 {
      print!("{}{}", WAL_SYMB, OPN_SYMB);
    } else if code == 13 {
      print!("{}{}", WAL_SYMB, OPN_SYMB);
    } else if code == 14 {
      print!("{}{}", WAL_SYMB, OPN_SYMB);
    } else if code == 15 {
      print!("{}{}", WAL_SYMB, OPN_SYMB);
    } else {
      panic!("Impossible code: {}", code);
    }
  } else if line == 1 {

    // Glorious hack to print a symbol in a semantically-specific color
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
    } else if symb == HEX_1_SYMB || symb == HEX_2_SYMB || symb == HEX_3_SYMB || symb == HEX_4_SYMB || symb == HEX_5_SYMB ||
      symb == HEX_6_SYMB || symb == HEX_7_SYMB || symb == HEX_8_SYMB || symb == HEX_9_SYMB ||
      symb == HEX_A_SYMB || symb == HEX_B_SYMB || symb == HEX_C_SYMB || symb == HEX_D_SYMB || symb == HEX_E_SYMB || symb == HEX_F_SYMB {
      symb.green()
    } else {
      panic!("Impossible symbol: {}", symb);
    };

    if code == 1 {
      print!("{}{}", colored_opn_or_wal, colored_symb);
    } else if code == 2 {
      print!("{}{}", OPN_SYMB, colored_symb);
    } else if code == 3 {
      print!("{}{}", OPN_SYMB, colored_symb);
    } else if code == 4 {
      print!("{}{}", colored_opn_or_wal, colored_symb);
    } else if code == 5 {
      print!("{}{}", colored_opn_or_wal, colored_symb);
    } else if code == 6 {
      print!("{}{}", OPN_SYMB, colored_symb);
    } else if code == 7 {
      print!("{}{}", OPN_SYMB, colored_symb);
    } else if code == 8 {
      print!("{}{}", colored_opn_or_wal, colored_symb);
    } else if code == 9 {
      print!("{}{}", colored_opn_or_wal, colored_symb);
    } else if code == 10 {
      print!("{}{}", OPN_SYMB, colored_symb);
    } else if code == 11 {
      print!("{}{}", OPN_SYMB, colored_symb);
    } else if code == 12 {
      print!("{}{}", colored_opn_or_wal, colored_symb);
    } else if code == 13 {
      print!("{}{}", colored_opn_or_wal, colored_symb);
    } else if code == 14 {
      print!("{}{}", OPN_SYMB, colored_symb);
    } else if code == 15 {
      print!("{}{}", OPN_SYMB, colored_symb);
    } else {
      panic!("Impossible code: {}", code);
    }
  } else {
    panic!("Impossible line: {}", line);
  }
}


// ----------------------------------------------------------------
fn create_wall_with_door(
  maze: &mut Array2<u8>, wall_min: usize, wall_max: usize, door_min: usize, door_max: usize, orientation: &bool,
  rng: &mut StdRng) {

  /*
  println!(
    "---\norientation: {}, wall_min: {}, wall_max: {}, door_min: {}, door_max: {}",
    orientation, wall_min, wall_max, door_min, door_max);
  */

  if wall_max > wall_min {
    // Subdivision of the room

    let pos_wall = rng.gen_range(wall_min..wall_max);
    let pos_wall_plus_one = pos_wall +1;

    let pos_door = if door_max > door_min {
      rng.gen_range(door_min..door_max)
    } else {
      door_min
    };

    // Determine, in function of the current orientation, the door value and the next wall orientation
    match orientation {
      // vertical separation, horizontal move
      true => {
        maze[[pos_door, pos_wall]] += L2R;
        maze[[pos_door, pos_wall +1]] += R2L;
      },
      // horizontal separation, vertical move
      false => {
        maze[[pos_wall, pos_door]] += U2D;
        maze[[pos_wall +1, pos_door]] += D2U;
      }
    }

    /*
    // Display the intermediate steps
    if SHOW_GENERATION_PROCESS {
      let symbol_code = get_codenames_for_naked_maze();
      print_maze_in_ascii(&maze, &orientation, &pos_entrance, &pos_goal, &symbol_code);
    }
    */

    if door_max > door_min {
    // Recursive call on two subdivided rooms with flipped orientation
      create_wall_with_door(maze, door_min, door_max, wall_min, pos_wall, &!orientation, rng);
      create_wall_with_door(maze, door_min, door_max, pos_wall_plus_one, wall_max, &!orientation, rng);
    } else if door_max == door_min {
      // Recursive call on two subdivided rooms with same orientation
      create_wall_with_door(maze, wall_min, pos_wall, door_min, door_max, &orientation, rng);
      create_wall_with_door(maze, pos_wall_plus_one, wall_max, door_min, door_max, &orientation, rng);
    }

  } else if wall_max == wall_min && door_max > door_min {
    // Recursive call on the same room with flipped orientation
    create_wall_with_door(maze, door_min, door_max, wall_min, wall_max, &!orientation, rng);
  }

  // Implicit stop condition: "wall_max == wall_min && door_max == door_min"

}


// ----------------------------------------------------------------
// Each symbol {L2R, R2L, U2D, D2U} appears exactly eight times in the encoding scheme
// The encoding scheme is made up fifteen different values ranging from 1 to 15
// No need to encode a "zero" symbol because this would correspond to a non-existing fully-closed unit-sized room
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
