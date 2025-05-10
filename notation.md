# Tiles

The 5 tiles types are represented with the following letters

- Wheat: **W**
- Ore: **O**
- Brick: **B**
- Sheep: **S**
- Lumber: **L**
- Desert: **D**

# Coordinates\

## Tiles

This notation uses a cube coordinate system (x, y, z), such that: `x + y + z = 0`
- The coordinates simply with x and y since  `z = -x - y`

*z does not need to be specified in the coordinates, see above

The coordinates are not like Cartesian coordinates where x is the horizontal axis and y is the vertical axis. Rather, in our case, y is the horizontal row of tiles, and x is the diagonal row of tiles (from top left to bottom right). z is *technically* the row of hexes from top right to bottom left, but as mentioned before, we are not specifying z.

# Vertex

A vertex is the point between 3 Tiles. To understand how the vertex coordinate is calculated, lets consider the 2 shapes can have.

1. A single tile above 2 tiles

```
   / \
  | A |
 / \ / \
| B | C | 
 \ / \ / 
```

The coordinate of the vertex between ABC is (Cx, By, Az). Notably, the sum of this will always be +1.

2. 2 tiles above a single tile

```
 / \ / \
| B | C | 
 \ / \ / 
  | A |
   \ /
```

The coordinate of the vertex between ABC is (Ax, By, Cz) in this configuration. Notably, the sum of this will always be -1. When we refer to this shape, we will address is as the "prime" shape denoted by `'`.

The problem is we don't necessarily know which Tiles are A, B or C when we provide coordinates. So first we need a way to assign A, B and C. The easiest way to do this is to first identify which two tiles share the same Y axis values, then determine if the remaining Y axis value is larger or smaller. If it's larger, you have a regular shape - otherwise it's the prime shape. You've now identified the A Tile, as well as its primeness - you must determine B and C which is done by comparing Ax on one of the other coordinates

- If the shape is not prime and you find Ax matches, then you've indentified C. Otherwise you've identified B
- If shape is prime and you find Ax matches, then you've identified B. Otherwise you've identified C

This coordinate system is complicated, but results in a unique coordinate for all the egdes with a process to identify the adjacent tiles by reversing this process. To check if the shape of the tiles are prime, use the equation x + y + z. If the result is -1, it is prime. This can also be used to verify if an edge coordinate is valid, because results other than 1 or -1 are invalid.

# Edges

Edges are defined by a direction vector relative to the center of the board. There are 3 type of edges. North, South - East, and South West. Each edge type has its own coordinate system, which we'll define based on the respective axis is is perpendicular to X, Y, Z (uppercase to distinguish from the hex coordinates).

Lets consider an X axis edge (all edges who point South West to North East). It is super simple to get the coordinate of this edge when you know the coordinate of the tile. Since we are on the X axis, the coordinates will be the (y, z) position of the tile. Note that the Edge identified by a tile's coordinates is the one closest to the center of the board. This means to capture and Edge on the extremes of a board, you'll need to imagine the board's radius as being r + 1 to get all valid locations of the edges (AKA 3 for standard Catan)

So when we refer to an edge, we will define the coordinate by its axis like so:

X axis: Xy,z
Y axis: Yz,x
Z axis: Zx,y

For example: `X-3,3`

Lets consider the edge between 2 vertices A and A'.

All edges have 2 vertices, and it is guaranteed one of the two vertices is prime denoted A'. We can assign a coordinate to the edge depending on which axis the edge is perpendicular to.

```
X(A'y, Az)
Y(A'z, Ax)
Z(A'x, Ay)
```


# Notation

## Ignored characters

Whitespace or the pipe (`|`) are ignored. This allows you to structure the notation in a way that may be more user friendly.

## Game start

In standard Catan, there are 19 tiles on the board, 4 Sheep, 4 Wheat, 4 Lumber, 3 Brick, 3 Ore and 1 Desert. While the notation could support variations of the resource types, a canonical game should have exactly those tiles defined.

## Algorithm for getting board positions

The order that the tiles should be specified is in the order that they appear, top to bottom and left to right, as if you were reading paragraphs in English.

The first position is (0, -2), the next is (1, -2), then (2, -2). The next row starts at (-1, -1), then (0, -1), and so on.

Here's some pseudocode on how you can generate the set of tile locations in order

```
r = 2 -- Radius of the board, standard is 2
for y = -r to r
  for x in -r to r
    if abs(-x - y) <= r -- Only valid z coordinates
        print(x, y) -- Resulting x, y coordinates
    end
  end
end
```

## Number of players
You may play 2 - 4 players. This is denoted by the number 2, 3 or 4.

## Tiles

Tiles are denoted by their Tile character, followed by the value of the tile (2-12). The desert tile does not specify a value. Note: in strict Catan, 2 of the same tile values may not be adjacent. There may only be the following values:

- 2 x 1
- [3 ... 6] x 2
- 7 x 0
- [8 ... 11] x 2
- 12 x 2

Example: `W2 O3 B4 | S12 D L9 O4 | ...`

## Ports

There are also 9 ports, which are spaced around the board separated by an empty water tile. Starting from the port that touches the first tile (0, -2), we go clockwise around the board to fill in the 9 ports.

The 2:1 ports are denoted as their Tile characters (all tile specific ports implicitly have 2:1). The 3:1 ports are denoted by a T (for Three, avoiding numbers to avoid confusing with the coordinate positions).

Example: `W T O B T T L S T`

## Putting it together

This is all that is required to build the game start. The sections are delimited by a forward slash.

`<num players>/<tiles>/<ports>`

Example: A 2 player game before anybody has places anything

```
2
/
   W2  O3  B10
  S8  D   L5  O6
B3 S4  W10 O11 W9 
  B12 S6  L4  B5 
   S9 L11 W8
/
W T O T T L B S T
```

Reminder that whitespace and pipe (`|`) characters are ignored. This can be written as:

```
2/W2O3B10S8DL5O6B3S4W10O11W9B12S6L4B5S9L11W8/WTSOTTLBST
```

This would be the minimum notation for a Catan game. The Catan notation also supports the actions taken by users, but is not required the display the current position of the game.

## Initial placements

Players are indexed by 1, 2, 3, 4. Color does not matter in this notation since it is a cosmetic decision. The first placements happen in ascending player order. The second placements are in reverse player order. This means we don't need to be explicit about what players are placing.

The first player places a settlement, then a road to an edge next to that position.