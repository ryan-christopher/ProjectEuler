# Problem 15: Lattice Paths

# Starting in the top left corner of a 2 x 2 grid, and only being able to move to the right
# and down, there are exactly 6 routes to the bottom corner.

# How many such routes are there through a 20 x 20 grid?


def findLatticePaths(gridSize):
    return ((gridSize ** (gridSize))) - gridSize * 2

print(findLatticePaths(2))