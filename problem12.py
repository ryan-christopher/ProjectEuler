# Problem 12: Highly Divisible Triangular Number

# The sequence of triangle numbers is generated by adding the natural numbers. So the 
# 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms would be:
# 1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...

# Let us list the factors of the first seven triangle numbers:
# 1: 1
# 3: 1, 3
# 6: 1, 2, 3, 6
# 10: 1, 2, 5, 10
# 15: 1, 3, 5, 15
# 21: 1, 3, 7, 21
# 28: 1, 2, 4, 7, 14, 28

# We can see that 28 is the first triangle number to have over five divisors. What is the value
# of the first triangle number to have over five hundred divisors?
import math

def findDivisors(minDivisors):
    x = 1
    divisors = 0
    while divisors < minDivisors:
        y = 1
        triangleSum = 0
        divisors = 0
        while y <= x:
            triangleSum += y
            y += 1
        divVal = 1
        midPoint = int(math.sqrt(triangleSum)) + 1
        while divVal <= midPoint:
            if (triangleSum/divVal).is_integer():
                divisors += 2
            divVal += 1
        if divisors < minDivisors:
            x += 1
    return "Triangle Number: ", x, "Triangle Sum: ", triangleSum, "Num of divisors: ", divisors

        
print(findDivisors(500))

