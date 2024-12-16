# Problem 3: Largest Prime Factor

# The prime factors of 13195 are 5, 7, 13, and 29.

# What is the largest prime factor of the number 600851475143.
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

# start at the maximum possible prime number, then work 
# down until the first prime is found
def largestPrime(num):
    val = int(math.sqrt(num) + 1)
    while val > 0:
        if (num / val).is_integer():
            if isPrime(val):
                return val
        val -= 1
    return None

#print(largestPrime(13195))
print(largestPrime(600851475143))