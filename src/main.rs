use ndarray::Array2;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

// Size of the maze
static DIM: usize = 16;

// Code words for the four possible directions
static BASE: u8 = 2;
static L2R: u8 = BASE.pow(0); // code for possible "left to right" move
static R2L: u8 = BASE.pow(1); // code for possible "right to left" move
static U2D: u8 = BASE.pow(2); // code for possible "up to down" move
static D2U: u8 = BASE.pow(3); // code for possible "down to up" move

// Random seed
static USE_RANDOM_SEED: bool = false;
static SEED: u64 = 421;


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

  // Random choice for the entrance and the goal
  let pos_entrance = rng.gen_range(0..DIM);
  let pos_goal = rng.gen_range(0..DIM);

  // Random choice for the first separation
  let orientation = if rng.gen_range(0..2) > 0 {
    true // vertical
  } else {
    false // horizontal
  };

  // Uppermost call to the recursive process
  create_wall_with_door(&mut maze, 0, DIM-1, 0, DIM-1, &orientation, &mut rng);

  println!("---\nFinal maze:\n{}\n", maze);

  println!("Orientation: {}, Entrance: {}, Goal: {}", orientation, pos_entrance, pos_goal);

  // Draw the maze in ascii-art
  draw_maze_in_ascii(&maze, &orientation, &pos_entrance, &pos_goal);

}


// ----------------------------------------------------------------
fn draw_maze_in_ascii(maze: &Array2<u8>, orientation: &bool, pos_entrance: &usize, pos_goal: &usize) {

  for y in 0..DIM {
    for x in 0..DIM {

      // Check for entrance
      let e: &str = if *orientation && x == *pos_entrance && y == 0 {
        "  "
      } else {
        "##"
      };
      print_ascii_line(maze[[y, x]], 0, e);
    }

    print!("#\n");

    for x in 0..DIM {

      // Check for entrance
      let f: &str = if !*orientation && y == *pos_entrance && x == 0 {
        " "
      } else {
        "#"
      };
      print_ascii_line(maze[[y, x]], 1, f);
    }

    // Check for goal
    if !*orientation && y == *pos_goal {
      print!(" \n");
    } else {
      print!("#\n");
    }
  }

  for x in 0..DIM {

    // Check for goal
    if *orientation && x == *pos_goal {
      print!("#  ");
    } else {
      print!("###");
    }
  }

  print!("#\n");
}


// ----------------------------------------------------------------
fn print_ascii_line(code: u8, line: u8, c: &str) {

  /*
  // Annotate each cell with its codename
  let b: u8 = code % 10;
  let a: u8 = (code - b) / 10;
  */

  if code == 1 && line == 0 {
    print!("#{}", c);
  } else if code == 1 && line == 1 {
    print!("{}  ", c); // {}{}", a, b);
  } else if code == 2 && line == 0 {
    print!("#{}", c);
  } else if code == 2 && line == 1 {
    print!("   "); // {}{}", a, b);
  } else if code == 3 && line == 0 {
    print!("#{}", c);
  } else if code == 3 && line == 1 {
    print!("   "); // {}{}", a, b);
  } else if code == 4 && line == 0 {
    print!("#{}", c);
  } else if code == 4 && line == 1 {
    print!("{}  ", c); // {}{}", a, b);
  } else if code == 5 && line == 0 {
    print!("#{}", c);
  } else if code == 5 && line == 1 {
    print!("{}  ", c); // {}{}", a, b);
  } else if code == 6 && line == 0 {
    print!("#{}", c);
  } else if code == 6 && line == 1 {
    print!("   "); // {}{}", a, b);
  } else if code == 7 && line == 0 {
    print!("#{}", c);
  } else if code == 7 && line == 1 {
    print!("   "); // {}{}", a, b);
  } else if code == 8 && line == 0 {
    print!("#  ");
  } else if code == 8 && line == 1 {
    print!("{}  ", c); // {}{}", a, b);
  } else if code == 9 && line == 0 {
    print!("#  ");
  } else if code == 9 && line == 1 {
    print!("{}  ", c); // {}{}", a, b);
  } else if code == 10 && line == 0 {
    print!("#  ");
  } else if code == 10 && line == 1 {
    print!("   "); // {}{}", a, b);
  } else if code == 11 && line == 0 {
    print!("#  ");
  } else if code == 11 && line == 1 {
    print!("   "); // {}{}", a, b);
  } else if code == 12 && line == 0 {
    print!("#  ");
  } else if code == 12 && line == 1 {
    print!("{}  ", c); // {}{}", a, b);
  } else if code == 13 && line == 0 {
    print!("#  ");
  } else if code == 13 && line == 1 {
    print!("{}  ", c); // {}{}", a, b);
  } else if code == 14 && line == 0 {
    print!("#  ");
  } else if code == 14 && line == 1 {
    print!("   "); // {}{}", a, b);
  } else if code == 15 && line == 0 {
    print!("#  ");
  } else if code == 15 && line == 1 {
    print!("   "); // {}{}", a, b);
  } else {
    print!("???");
  }
}


// ----------------------------------------------------------------
fn create_wall_with_door(
  maze: &mut Array2<u8>, wall_min: usize, wall_max: usize, door_min: usize, door_max: usize, orientation: &bool,
  rng: &mut StdRng) {

  println!(
    "---\norientation: {}, wall_min: {}, wall_max: {}, door_min: {}, door_max: {}",
    orientation, wall_min, wall_max, door_min, door_max);

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

    println!("pos_wall={}, pos_door={}\n{}", pos_wall, pos_door, maze);

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
