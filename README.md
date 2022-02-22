# README

## Can you solve it?

##### Maze (7×7)

``◆``: Entrance

``♥``: Goal

```sh
■■■■■■■■■■■■■◆■
■○ ○ ○ ○ ○ ○ ○■
■ ■■■■■■■■■ ■■■
■○ ○■○ ○ ○ ○ ○■
■■■■■■■■■ ■■■■■
■○ ○ ○ ○ ○ ○■○■
■ ■■■■■■■■■■■ ■
■○ ○■○ ○ ○■○■○■
■ ■■■ ■■■ ■ ■ ■
■○ ○ ○ ○■○ ○ ○■
■ ■ ■■■ ■■■ ■ ■
■○■○■○ ○ ○■○■○■
■■■■■ ■■■■■ ■ ■
■○ ○ ○ ○ ○■○■○■
■■■■■■■■■■■■■♥■
```

##### Hexadecimal code-words to encode the possible moves

```sh
■■■■■■■■■■■■■◆■
■5 3 3 3 3 7 2■
■ ■■■■■■■■■ ■■■
■9 2■1 3 7 B 2■
■■■■■■■■■ ■■■■■
■5 3 3 3 B 2■4■
■ ■■■■■■■■■■■ ■
■D 2■5 3 6■4■C■
■ ■■■ ■■■ ■ ■ ■
■D 7 B 6■9 F E■
■ ■ ■■■ ■■■ ■ ■
■8■8■5 B 2■C■C■
■■■■■ ■■■■■ ■ ■
■1 3 B 3 2■8■8■
■■■■■■■■■■■■■♥■
```

##### Maze solved in 42 iteration (85% of the nodes have been visited)

``a``: Alive region

``n``: Narrow band

``f``: Far away region

```sh
■■■■■■■■■■■■■◆■
■a a a a a a a■
■ ■■■■■■■■■ ■■■
■a a■a a a a a■
■■■■■■■■■ ■■■■■
■a a a a a a■f■
■ ■■■■■■■■■■■ ■
■a a■a a a■n■n■
■ ■■■ ■■■ ■ ■ ■
■a a a a■a a a■
■ ■ ■■■ ■■■ ■ ■
■a■a■a a a■a■a■
■■■■■ ■■■■■ ■ ■
■f n a a n■n■a■
■■■■■■■■■■■■■♥■
```

##### Path

```sh
■■■■■■■■■■■■■◆■
■○ ○ ○ ○ ○ ↓ ←■
■ ■■■■■■■■■ ■■■
■○ ○■○ ○ ↓ ← ○■
■■■■■■■■■ ■■■■■
■↓ ← ← ← ← ○■○■
■ ■■■■■■■■■■■ ■
■↓ ○■→ → ↓■○■○■
■ ■■■ ■■■ ■ ■ ■
■→ → ↑ ○■→ → ↓■
■ ■ ■■■ ■■■ ■ ■
■○■○■○ ○ ○■○■↓■
■■■■■ ■■■■■ ■ ■
■○ ○ ○ ○ ○■○■↓■
■■■■■■■■■■■■■♥■
```

## Roadmap

- [DONE] Implement algorithm for random maze generation via recursive splitting
- [DONE] Implement routine to display the maze in ASCII-art in the console
- [DONE] Implement algorithm to solve the maze via front propagation
- [TODO] Tame the last TODOs in the code
- [TODO] Write this README
