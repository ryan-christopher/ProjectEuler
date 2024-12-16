# Problem 7: 10,001st Prime

# By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that
# the 6th prime is 13.

# What is the 10,001st prime number?

import math

# function to determine if number is prime or not
def isPrime(num):
    if num % 2 == 0:
        return False
    val = 3
    maxVal = int(math.sqrt(num) + 1)
    while val <= maxVal:
        divVal = num / val
        if divVal.is_integer():
            return False
        val += 2
    return True

# function to find the nth prime number
def findNthPrime(n):
    # keep track of the current number prime found
    currPrime = 1
    # set x to 1, then increment x before checking to start at
    # 2, the first prime number
    x = 1
    # check until currPrime matches currPrime, meaning x is the nth
    # prime number
    while currPrime < n:
        x += 1
        if isPrime(x):
            currPrime += 1
    return x

print(findNthPrime(10001))