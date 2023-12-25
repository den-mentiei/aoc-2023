# Advent of Code 2023

Hello, sailor! This year, I have solved it in Rust.

## Routine

Star a day with the following:
```bash
$ ./day
```

This will bootstrap today's solution and download the inputs.
Template is contained in `boilerplate.rs`.

If you skipped a day, you can still do it via:
```bash
$ ./day <day-number>
```

## Setup

To download task inputs, the session cookie is required.
You can get one inspecting the headers of AOC website response,
when you are logged in.

Day script looks for it in `.env` file, in the following form:

```bash
KEY=<long alphanumeric string you have in your aoc cookies>
```

## Notes

### Day 01

Basic parsing, wordy digits could overlap.

It could be SIMD-ified in a pretty straight-forward way to do every
digit lookup at once.

### Day 02

More parsing & fold over the items.

### Day 03

Search for * and parsing around.

### Day 04

Parsing & simulation, basic bit-math.

### Day 05

Intervals mapping/unmapping. Attention to proper intervals
intersection during unmapping.

### Day 06

Closed-form solution for time/distance constraints.

### Day 07

Smart card value-ing & jokers handling modification.

### Day 08

Simulation of multiple loops with LCM to find a single one.

LCM works as every loop starts from 0 and repeats itself.
String identifiers are encoded as numbers for speed.

### Day 09

In-place diff. Added beginning 0 allows to get both values in a single pass.

### Day 10

Pipe walking with shoelace formula and Pick's theorem.

### Day 11

Prefix-sums to skip empty rows/columns tracking the gap length.

### Day 12

Tabulated dynamic programing to calculate every possible substitution.

### Day 13

Bit-packing per row/column and xor + popcnt to get differences.

### Day 14

Pigeon-hole principle says there would be a cycle.

Fast Brent cycle detection - as it gives length immediately, and has
low amount of function evaluation.

### Day 15

Simulation with string identifiers encoding for speed.

### Day 16

BFS.

Bit-packed direction tracking for `seen`.
Bit-math to mirror directions.

### Day 17

Dijkstra with on-the-go graph construction.

Bit-packed direction tracking for `seen`.
Bit-math to get further directions.

### Day 18

Shoelace formula and Pick's theorem.

### Day 19

BFS for interval-constraints.

### Day 20

Input is a NAND-gate soup implementing 4 mod-n counters.

N of every counter could be inferred from the structure itself,
without any live simulation. Flip-flops are frequency dividers, some
of them are connected to the conj nodes, influencing when counter will
be reset.

### Day 21

`f(n)` is a function which gives number of visited spaces after `n` steps.

Based on the following:
- grid is square
- start is in the middle
- empty rows/column around the start

`f(n)` is quadratic aka of the `ax^2 + bx + c` form.
This can be fit with a polynomial to be able to evaluate for any `n`.

Second-order polynomial requires 3 points to fit.
`f(n+0*w)`, `f(n+1*w)`, `f(n+2*w)` and then just calculate the answer
for `f(26501365/w)` which is `f(202300)`.

Nice to notice that `26501365 = 65 + 202300*131`, where 131 is grid
size.

Did both Lagrange and Newotn polynomials, which both give perfect fit.

### Day 22

Height-map to track dependent bricks.

Topo-sort like, indegree based, bricks removal with tracking of
affected ones.

### Day 23

DFS brute-force as longest-path is NP-hard.

As grid is a maze with corridors, it is reduced to a much smaller
graph, making the brute-force viable.

As number of vertices is small, `seen` is a bitmask, which speeds-up
the DFS a lot.

Important observation is that at the junction before the end, the path
to the end *must* be taken, otherwise end could not be reached which
DFS takes a lot of time to discover. Taking it cuts down the search a
lot.

Yet another optimization is to track reachable vertices and continue
the search only if we arrived at a vertice with same reachable set but
by a longer path.

### Day 24

Ray-ray intersection for part 1.

Part 2 gives a system of bilinear equations. Those could be solved via
Z3, which is no fun.

The equation system could be made linear by some linear-algebra tricks
(aka cross-products) giving a 6 equations with 6 unknowns.

That system is trivially solvable with Gaussian elimination. However,
numbers are big and we need an integer solution, meaning that there
might be a lack of precision. Hence, the calculated solution is
brute-force (-1, +1 per axis) adjusted to be exact.

### Day 25

A graph min-cut problem without disguise.

Implemented the randomized Karger algorithm, which is not very fast.
It just cuts the edges to break the graph into components, while
tracking those via disjoint-set.

A faster thing is surely possible.

Updated: removed the Karger as it was too slow. Replaced by a
randomized sampling, which just looks for the shortest path (Dijkstra)
between 2 random vertices multiple times and counts the frequence of
edge usage. It turns out that min-cut edges are *most* used ones.
