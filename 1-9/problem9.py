# Problem 9: Special Pythagorean Triplet

# A Pythagorean triplet is a set of three natural numbers, a < b < c, for which a^2 + b^2 = c^2.
# For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.

# There exists exactly one Pythagorean triplet for which a + b + c = 1000. Find the product abc.

# return true if the triplet is a Pythagorean triplet
def isTriplet(a, b, c):
    if a < b and b < c:
        if (a**2 + b**2) == c**2:
            return True
    return False

# go through all possibilities of a < b < c and a+b+c = 1000
def findTriplets(sumVal):
    a, b= 1, 2
    c = sumVal - a - b
    while a < b:
        while b < c:
            # return once the triplet is found
            if isTriplet(a, b, c):
                return a, b, c
            b += 1
            c -= 1
        a += 1
        b = a + 1
        c = sumVal - a - b


a, b, c = findTriplets(1000)
print("a: ", a, "b: ", b, "c: ", c)
print("Product: ", a*b*c)