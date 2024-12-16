# Problem 10: Summation of Primes

# The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

# Find the sum of all the primes below two million.

# function to determine if number is prime or not
import math

def isPrime(num):
    if num == 2:
        return True
    elif num % 2 == 0:
        return False
    val = 3
    maxVal = int(math.sqrt(num) + 1)
    while val <= maxVal:
        divVal = num / val
        if divVal.is_integer():
            return False
        val += 2
    return True

# finds the sum of all the primes below the max val given as input
def findSumOfPrimes(maxVal):
    # start at 2
    i = 2
    sumOfPrimes = 0
    # for every value i, check if it is prime
    while i < maxVal:
        if isPrime(i):
            # if it is prime, add to sum variable
            sumOfPrimes += i
        i += 1
    return sumOfPrimes

print(findSumOfPrimes(2000000))