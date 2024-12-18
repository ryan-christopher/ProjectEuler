# Problem 6: Sum Square Difference

# The sum of the squares of the first ten natural numbers is:
# 1^2 + 2^2 + ... + 10^2 = 385

# The square of the sum of the first ten natural numbers is:
# (1 + 2 + ... + 10)^2 == 55^2 = 3025

# Hence the difference between the sum of the squares of the first ten natural 
# numbers and the square of the sum is 3025 - 385 = 2640.

# Find the difference between the sum of the squares of the first one hundred
# natural numbers and the square of the sum.

def sumSquareDiff(num):
    # find sum of squares of 1 to num:
    sumSquare = 0
    for x in range(1, num + 1):
        sumSquare += x ** 2
    # find square of sums of 1 to num:
    squareSum = 0
    for y in range(1, num + 1):
        squareSum += y
    squareSum = squareSum ** 2

    return squareSum - sumSquare

print(sumSquareDiff(100))