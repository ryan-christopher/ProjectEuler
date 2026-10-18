# Problem 14: Longest Collatz Sequence

# The following iterative sequence is defined for the set of positive integers:
#   n --> n/2 (n is even)
#   n --> 3n + 1 (n is odd)

# Using the rule above and starting with 13, we generate the following sequence:
# 13 -> 40 -> 20 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1

# It can be seen that this sequence (starting at 13 and finishing at 1) contains 10
# items. Although it has not been proved (Collatz Problem), it is thought that all 
# starting numbers finish at 1.

# Which starting number, under one million, produces the longest chain?

def collatz(n):
    # if n is accepted, create hailstone array
    hailstone = []

    # until n is 1, check if n is even or odd and
    # perform the corresponding calculation on n
    while n != 1:
        hailstone.append(n)
        if (n % 2) == 0:
            n = n // 2
        else:
            n = (3*n) + 1

    # once n is 1, append to array
    hailstone.append(n)
    # return hailstone array
    return hailstone


def findLongestChain(maxStartVal):
    # start at 1
    currentVal = 1
    longestChainVal = None
    longestChain = []
    # go until maxStartVal
    while currentVal < maxStartVal:
        currentHailstone = collatz(currentVal)
        # replace longest variables if new longest is found
        if len(currentHailstone) > len(longestChain):
            longestChainVal = currentVal
            longestChain = currentHailstone
        currentVal += 1
    return longestChainVal, len(longestChain)

print(findLongestChain(1000000))