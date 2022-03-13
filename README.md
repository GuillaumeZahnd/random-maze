# README

![random_maze_art_01.jpg](../readme_images/art/random_maze_art_01.jpg?raw=true)

## Yes, but why?

This repo is a toy project for me to play with an algorithm I know (front propagation) in a language I did not know (Rust).

![random_maze_art_02.jpg](../readme_images/art/random_maze_art_02.jpg?raw=true)

## Algorithm to generate the maze

Let us consider a ```NxN``` graph, hereafter referred to as the *maze*, composed of 4-connected nodes. Let us randomly select two nodes, on two opposed outskirts of the graph, to be the entrance ```E``` and the goal ```G```. The maze is then generated via the following recursive process:

```sh
Initialisation:
region := maze
orientation := horizontal 
Procedure: generate_maze(region, orientation)
  0. If: size of region is equal to 1 across any dimension, Then: STOP, Else: CONTINUE
  1. region_1, region_2 := Randomly subdivide the graph in two parts along the given orientation
  2. node_1, node_2 := Randomly determine to adjascent nodes located in region_1 and region_2
  3. Connect node_1 and node_2
  4. procedure generate_maze(region_1, NOT orientation)
  5. procedure generate_maze(region_2, NOT orientation)
end
```

By construction the maze is an undirected acyclic graph, therefore the path from ```E``` to ```G``` exists and is unique.

##### Example of a 7x7 maze:

![random_maze_art_03.jpg](../readme_images/art/random_maze_art_03.jpg?raw=true)

##### Example of the four first iterations during the random generation process:

![random_maze_art_04.jpg](../readme_images/art/random_maze_art_04.jpg?raw=true)

## Algorithm to encode the possible moves

When two nodes are connected (see Instruction 3 of ```generate_maze```), one of the two following cases occur:

- Case 1: ```node_1``` and ```node_2``` are horizontally connected
- Case 2: ```node_1``` and ```node_2``` are vertically connected

Depending on the orientation of the connection (namely, first or second case), ```node 1``` is accordingly updated to encode a possible move to ```node 2```, and similarly ```node 2``` is accordingly updated to encode a possible move to ```node 1```. To keep track of connected nodes, I came up with the following encoding, based on subsequent powers of ```2```:

| Code-word | Description | Value |
| --- | --- | --- |
| ```L2R``` | Possible move from left to right | 2^0 |
| ```R2L``` | Possible move from right to left | 2^1 |
| ```U2D``` | Possible move from up to down | 2^2 |
| ```D2U``` | Possible move from down to up | 2^3 |

The advantage of using this encoding is that we can exploit additions of terms in the basis ```{1, 2, 4, 8}``` to uniquely describe, with a single value, a combination that consists of up to four different possible moves, as illustrated hereafter.

##### All sixteen different code-words, derived from adding terms of the basis ```{L2R, R2L, U2D, D2U}```:

![random_maze_art_05.jpg](../readme_images/art/random_maze_art_05.jpg?raw=true)

With this information, we can elaborate the description of the algorithm ```generate_maze``` with further details:

- **Initialization:** All nodes of the maze are zero-initialized (namely, nodes are totally disconnected from one another)

```sh
For each node of the maze:
  node := 0
```

- **Instruction 3 (Connect node_1 and node_2):**

```sh
If: orientation == horizontal
  Then: node_1 += L2R and node_2 += R2L
Else If: orientation == vertical
  Then: node_1 += U2D and node_2 += D2U
```

Ultimately, the maze is effectively built by adding connection between specific nodes (as opposed to the intuitive idea of adding walls). By reading the value of any given node, all the possible moves from this node can be exactly deduced.

##### Example of the code-words for each node in the fully-generated 7x7 maze:

![random_maze_art_06.jpg](../readme_images/art/random_maze_art_06.jpg?raw=true)

## Algorithm to solve the maze

The solution to the path from ```E``` to ```G``` is found via a combination of front propagation and backtracking.

### Front propagation algorithm

This method relies on an encoding where each node belongs to one and only one of the three states ```{a, n, f}```, as described below.

| State | Name | Description | 
| --- | --- | --- |
| ```a``` | Alive region | Nodes that have already been visited |
| ```n``` | Narrow band | Nodes that have not yet been visited, and are immediately reachable from at least one ```a``` node |
| ```f``` | Far away region | Nodes that have not yet been visited, and are not immediately reachable from any ```a``` node |

The pseudo-code of the front propagation is the following:

```sh
Initialisation:
map := NxN array
For each node idx of the map:
  If idx is the coordinates of the entrance E:
    map(idx) := a
  Else If idx is a direct neighbor of E in the maze:
    map(idx) := n
  Else:
    map(idx) := f
goal_has_been_reached := False
Procedure: propagate_front(map, maze, G)
while NOT goal_has_been_reached:
  0. If the goal G is in the narrow band, Then: idx_n := coordinates of G, Else:
  1. idx_n := Randomly select a map node in the narrow band
  2. map(idx_n) := a
  3. For each map node idx_f that is in the far away region AND is a direct neighbor of idx_n in the maze:
    4. map(idx_f) := n
  5. If the goal G is in the alive region, Then: goal_has_been_reached := True, Else: CONTINUE
end
```

##### Example of the fully-propagated 7x7 maze:

![random_maze_art_07.jpg](../readme_images/art/random_maze_art_07.jpg?raw=true)

##### Example of the four first iterations during the front propagation process:

![random_maze_art_08.jpg](../readme_images/art/random_maze_art_08.jpg?raw=true)

### Backtracking algorithm

Once the front has reached the coordinates of the goal ```G``` (more precisely, when ```G``` became alive), a backtracking operation is conducted to extract the path from ```E``` to ```G```.

With this information, we can elaborate the description of the algorithm ```propagate_front``` with further details:

- **Initialization:** Create an ```ascendance``` ledger that keeps track, for each visited node, of the previous node visited during the front propagation, and create an empty ```path``` to store the nodes connecting ```E``` to ```G```.

```sh
ascendance := NxN array
For each node idx of the ascendance ledger:
  If idx is a direct neighbor of E in the maze:
    ascendance(idx) := idx_E
  Else:
    ascendance(idx) := None
path := Empty
```

- **Instruction 3 (Propagation of the narrow band in the far away region from the new alive node):**

```sh
3. For each map node idx_f that is in the far away region AND is a direct neighbor of idx_n in the maze:
  4-bis. ascendance(idx_f) = idx_n
```

- **Instructions 6-11 (Backtracking):**

```sh
6. n := 0
7. path(n) := idx_G
8. While NOT path(n) == idx_E:
  9. n += 1
  10. path(n) := ascendance(path(n-1))
11. Reverse order of path
```

##### Example of the fully-solved maze showing the path from ```E``` to ```G```:

![random_maze_art_09.jpg](../readme_images/art/random_maze_art_09.jpg?raw=true)

##### Path from ```E``` to ```G```:

```sh
0:(0,3) 1:(0,2) 2:(1,2) 3:(2,2) 4:(2,3) 5:(3,3) 6:(3,2) 7:(3,1) 8:(3,0) 9:(4,0) 10:(5,0) 11:(5,1) 12:(6,1)
```

Of note, this front propagation algorithm used here is similar to other approaches such as Dijkstra's algorithm, A* algorithm, fast marching, and dynamic programming. In our case, we use a simple geodesic: the cost of each maze node is 1, the front evolution is determined at random (as opposed to depth-first search, breadth-first search, or cost-first search), and backtracking is defined via exact ascendance (as opposed to via gradient descent).

## Algorithm to display the maze in ascii-art

I was not satisfied with the (currently available) plotting functionalities offered by Rust, and because I hate fun, I decided to create a routine to print the maze (as well as various intermediate steps of the process) in the console in ascii-art.

- A lookup mechanism is used to translate the value of any given node in the adequate combination of four ascii characters
- Printing in the console occurs line-by-line, therefore the maze is printed top-to-bottom
- Two subsequent console lines are requested to display a single horizontal row of the maze
- The symbols for the entrance and the goal are specifically handled in function of the coordinates of ```E``` and ```G```

##### Combination of four ascii characters needed to print a single node:

| Character | Position | Symbol | 
| --- | --- | --- |
| ```W``` | First line, First symbol | Print a wall |
| ```X``` | First line, Second symbol | Print an opening if ```D2U``` move possible, else print a wall |
| ```Y``` | Second line, First symbol | Print an opening if ```R2L``` move possible, else print a wall |
| ```Z``` | Second line, Second symbol | Print the node value |

##### Example of the display for a single node:

![random_maze_art_10.jpg](../readme_images/art/random_maze_art_10.jpg?raw=true)

##### Example of the process to print a ```NxN``` maze in the console

| | | Column 0 | Column 1 | ... | Column N | Closing symbols |
| --- | --- | ---- | --- | --- | --- | --- |
| print this line --> | **Row 0, Line 0** | W_0.0, X_0.0 | W_1.0, X_1.0 | ... | W_N.0, X_N.0 | Wall |
| print this line --> | **Row 0, Line 1** | Y_0.0, Z_0.0 | Y_1.0, Z_1.0 | ... | Y_N.0, Z_N.0 | Wall |
| print this line --> | **Row 1, Line 0** | W_0.1, X_0.1 | W_1.1, X_1.1 | ... | W_N.1, X_N.1 | Wall |
| print this line --> | **Row 1, Line 1** | Y_0.1, Z_0.1 | Y_1.1, Z_1.1 | ... | Y_N.1, Z_N.1 | Wall |
| print this line --> | ... | ... | ... | ... | ... | ... |
| print this line --> | **Row N, Line 0** | W_0.N, X_0.N | W_1.N, X_1.N | ... | W_N.N, X_N.N | Wall |
| print this line --> | **Row N, Line 1** | Y_0.N, Z_0.N | Y_1.N, Z_1.N | ... | Y_N.N, Z_N.N | Wall |
| print this line --> | **Closing symbols** | Wall, Wall | Wall, Wall | ... | Wall, Wall | Wall |
