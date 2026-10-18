# Problem 20: Factorial Digit Sum

# n! means n x (n - 1) x ... 3 x 2 x 1.

# For example, 10! = 10 x 9 x ... x 3 x 2 x 1 = 3628800, and the sum of the digits
# in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

# Find the sum of the digits in the number 100!.
from math import factorial

def findFactorialSum(n):
    n = str(factorial(n))
    factorialSum = 0
    for char in n:
        factorialSum += int(char)
    return factorialSum

print(findFactorialSum(100))