# README

![random_maze_art_01.jpg](../readme_images/random_maze_art_01.jpg?raw=true)

## Yes, but why?

This is a toy project for me to learn a language I did not know yet (Rust), applied to an algorithm I was already familiar with (front propagation).

![random_maze_art_02.jpg](../readme_images/random_maze_art_02.jpg?raw=true)

## Parameters

```sh
DIM                     := 7        // Size of the maze
MAZE_ORIENTATION        := "random" // Maze orientation ("horizontal", "vertical", "random")
SHOW_GENERATION_PROCESS := false    // Intermediate display of the maze generation steps
SHOW_SOLVING_PROCESS    := false    // Intermediate display of the front propagation steps
USE_FANCY_ASCII         := true     // Print the maze in the console in ascii-art (either fancy, or pure)
USE_RANDOM_SEED         := false    // Random seed for reproducibility
SEED                    := 893      // Japanese speakers will know what this number stands for ;)
```

## Algorithm to generate the maze

Let us consider a ```NxN``` graph, hereafter referred to as the *maze*, composed of 4-connected nodes. Let us randomly select two nodes, on two opposed outskirts of the graph, to be the entrance ```E``` and the goal ```G```. The maze is then generated via the following recursive process:

```sh
Initialization:
0. region := maze
1. orientation := maze_orientation
```

```sh
Procedure: generate_maze(region, orientation)
0. If: size of region is equal to 1 across any dimension, Then: STOP, Else: CONTINUE
1.    region_1, region_2 := Randomly subdivide the graph in two parts along the given orientation
2.    node_1, node_2 := Randomly determine two adjacent nodes located in region_1 and region_2
3.    Connect node_1 and node_2
4.    procedure generate_maze(region_1, NOT orientation)
5.    procedure generate_maze(region_2, NOT orientation)
6. EndIf
```

By construction the maze is an undirected acyclic graph, therefore the path from ```E``` to ```G``` exists and is unique, as visible in the example below.

##### Example of a 7x7 maze:

![random_maze_art_03.jpg](../readme_images/random_maze_art_03.jpg?raw=true)

##### Example of the first four iterations during the random generation process:

![random_maze_art_04.jpg](../readme_images/random_maze_art_04.jpg?raw=true)

## Algorithm to encode the possible moves

When two nodes are connected (see Instruction 3 of ```generate_maze```), one of the two following cases occur:

- Case 1: ```node_1``` and ```node_2``` are horizontally connected
- Case 2: ```node_1``` and ```node_2``` are vertically connected

Depending on the orientation of the connection (namely, first or second case), ```node 1``` is accordingly updated to encode a possible move to ```node 2```, and similarly ```node 2``` is accordingly updated to encode a possible move to ```node 1```. To keep track of connected nodes, I came up with the following encoding, based on subsequent powers of 2:

| Code-word | Description | Value |
| --- | --- | --- |
| ```L2R``` | Possible move from left to right | 2^0 |
| ```R2L``` | Possible move from right to left | 2^1 |
| ```U2D``` | Possible move from up to down | 2^2 |
| ```D2U``` | Possible move from down to up | 2^3 |

The advantage of using this encoding is that we can exploit additions of terms in the basis ```{1, 2, 4, 8}``` to uniquely describe, with a single value, a combination that consists of up to four different possible moves, as illustrated hereafter.

##### All sixteen different code-words (in hexadecimal), derived from adding terms of the basis ```{L2R, R2L, U2D, D2U}```:

![random_maze_art_05.jpg](../readme_images/random_maze_art_05.jpg?raw=true)

With this information, we can elaborate the description of the algorithm ```generate_maze``` with further details:

- **Initialization:** Add ```Instructions 0.1-0.3``` to explicitly zero-initialize all nodes (namely, nodes are totally disconnected from one another)

```sh
0.0. region := maze
0.1. For each node of the maze:
0.2.   node := 0
0.3. EndFor
```

- **Procedure**: Add ```Instruction 3.0-3.4``` to add the code-word corresponding the the possible move (namely, adequately connect ```node_1``` and ```node_2```)

```sh
3.0 If: orientation == horizontal
3.1   Then: node_1 += L2R and node_2 += R2L
3.2 Else If: orientation == vertical
3.3   Then: node_1 += U2D and node_2 += D2U
3.4 EndIf
```

Ultimately, the maze is effectively built by adding connection between specific nodes (as opposed to the intuitive idea of adding walls). By reading the value of any given node, all the possible moves from this node can be exactly deduced, as depicted in the Figure below.

##### Example of the code-words (in hexadecimal) for each node in the fully-generated 7x7 maze:

![random_maze_art_06.jpg](../readme_images/random_maze_art_06.jpg?raw=true)

## Algorithm to solve the maze

The solution to finding the path from ```E``` to ```G``` is determined via a combination of front propagation and backtracking.

### Front propagation algorithm

This method relies on an iterative and monotonic encoding process where each node belongs to one and only one of the three states ```{a, n, f}```, as described below.

| State | Name | Description | 
| --- | --- | --- |
| ```a``` | Alive region | Nodes that have already been visited |
| ```n``` | Narrow band | Nodes that have not yet been visited, and are immediately reachable from at least one ```a``` node |
| ```f``` | Far away region | Nodes that have not yet been visited, and are not immediately reachable from any ```a``` node |

The pseudo-code of the front propagation algorithm is the following:

```sh
Initialization:
00. map := NxN array
01. For each node idx of the map:
02.   If idx is the coordinates of the entrance E:
03.     Then: map(idx) := a
04.   Else If idx is a direct neighbor of E in the maze:
05.     Then: map(idx) := n
06.   Else:
07.     Then: map(idx) := f
08.   EndIf
09. EndFor
10. goal_has_been_reached := False
```

```sh
Procedure: propagate_front(map, maze, G)
00. While NOT goal_has_been_reached:
01.   If the goal G is in the narrow band, Then: idx_n := coordinates of G, Else:
02.     idx_n := Randomly select a map node in the narrow band
03.     map(idx_n) := a
04.     For each map node idx_f that is in the far away region AND is a direct neighbor of idx_n in the maze:
05.       map(idx_f) := n
06.     EndFor
07.   EndIf
08.   If the goal G is in the alive region, Then: goal_has_been_reached := True, Else: CONTINUE
09.   EndIf
10. EndWhile
```

##### Example of the fully-propagated 7x7 maze:

![random_maze_art_07.jpg](../readme_images/random_maze_art_07.jpg?raw=true)

##### Example of the first four iterations during the front propagation process:

![random_maze_art_08.jpg](../readme_images/random_maze_art_08.jpg?raw=true)

### Backtracking algorithm

Once the front has reached the coordinates of the goal ```G``` (more precisely, when ```G``` became alive), a backtracking operation is conducted to extract the path from ```E``` to ```G```.

With this information, we can elaborate the description of the algorithm ```propagate_front``` with further details:

- **Initialization:** Add ```Instructions 11-17``` to create an ```ascendance``` ledger that keeps track, for each visited node, of the previous node visited during the front propagation.

```sh
11. ascendance := NxN array
12. For each node idx of the ascendance ledger:
13.   If idx is a direct neighbor of E in the maze:
14.     Then: ascendance(idx) := idx_E
15.   Else:
16.     Then: ascendance(idx) := None
17.   EndIf

```

- **Procedure:** Add ```Instruction 05.1``` to update the ascendance ledger:

```sh
04.0. For each map node idx_f that is in the far away region AND is a direct neighbor of idx_n in the maze:
05.0.   map(idx_f) := n
05.1.   ascendance(idx_f) = idx_n
06.0. EndFor
```

We then use backtracking to extract the path that consists of the set of ordered nodes connecting ```E``` to ```G```. The pseudo-code of the backtracking algorithm is the following:

```sh
Initialization:
0. path := Empty
1. n := 0
2. path(n) := idx_G
```

```sh
Procedure: backtracking(ascendance, idx_E)
1. While NOT path(n) == idx_E:
2.   n += 1
3.   path(n) := ascendance(path(n-1))
4. EndWhile
5. Reverse order of path
```

##### Example of the fully-solved maze showing the path from ```E``` to ```G```:

![random_maze_art_09.jpg](../readme_images/random_maze_art_09.jpg?raw=true)

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

| Symbol ID | Position line | Position symbol | Printed character | 
| --- | --- | --- | --- |
| ```W``` | First line | First symbol | Print a wall (```#```) |
| ```X``` | First line | Second symbol | Print an opening if ```D2U``` move possible (``` ```) , else print a wall  (```#```) |
| ```Y``` | Second line | First symbol | Print an opening if ```R2L``` move possible (``` ```) , else print a wall  (```#```) |
| ```Z``` | Second line | Second symbol | Print the node value, e.g., ```.``` (generic node),  ```E``` / ```G``` (entrance / goal), ```1``` - ```F``` (possible moves),  ```a```/```n```/```f``` (alive region / narrow band / far away region),  ```>``` / ```<``` / ```^``` / ```v``` (path) |

##### Example of the display for a single node:

![random_maze_art_10.jpg](../readme_images/random_maze_art_10.jpg?raw=true)

##### Example of the process to print a ```NxN``` maze in the console

| | | Column 0 | Column 1 | ... | Column N | Closing symbol |
| --- | --- | ---- | --- | --- | --- | --- |
| *print this line:* | **Row 0, Line 0** | W(0,0), X(0,0) | W(1,0), X(1,0) | ... | W(N,0), X(N,0) | Wall |
| *print this line:* | **Row 0, Line 1** | Y(0,0), Z(0,0) | Y(1,0), Z(1,0) | ... | Y(N,0), Z(N,0) | Wall |
| *print this line:* | **Row 1, Line 0** | W(0,1), X(0,1) | W(1,1), X(1,1) | ... | W(N,1), X(N,1) | Wall |
| *print this line:* | **Row 1, Line 1** | Y(0,1), Z(0,1) | Y(1,1), Z(1,1) | ... | Y(N,1), Z(N,1) | Wall |
| ... | ... | ... | ... | ... | ... | ... |
| *print this line:* | **Row N, Line 0** | W(0,N), X(0,N) | W(1,N), X(1,N) | ... | W(N,N), X(N,N) | Wall |
| *print this line:* | **Row N, Line 1** | Y(0,N), Z(0,N) | Y(1,N), Z(1,N) | ... | Y(N,N), Z(N,N) | Wall |
| *print this line:* | **Closing symbols** | Wall, Wall | Wall, Wall | ... | Wall, Wall | Wall |

## Some random 7x7 mazes

![random_maze_art_11.jpg](../readme_images/random_maze_art_11.jpg?raw=true)
