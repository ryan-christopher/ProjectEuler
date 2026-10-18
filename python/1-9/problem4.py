# Problem 4: Largest Palindrome Product

# A palindromic number reads the same both ways. The largest palindrome made from the 
# product of two 2-digit numbers is 9009 = 91 x 99.

# Find the largest palindrome made from the product of two 3-digit numbers.

# function to determine if a number is a palindrome
def isPalindrome(num):
    # convert to string
    num = str(num)
    # calculate middle of string
    midPoint = len(num) // 2
    # compare each character of each end of the string 
    for i in range(midPoint):
        if num[i] != num[(i * -1) -1]:
            return False
    return True

# finds the largest palindromic number made from the 
# products of two numbers that are each digitNum in length
def largestPalindrome(digitNum):
    digit1, digit2 = 0, 0
    # max val that is digitNum number of digits
    maxVal = 1 * (10 ** digitNum) - 1
    # set variable to compare palindrome found to current largest found
    largestPal = 0
    # iterate through all combinations starting from the max val
    for x1 in range(maxVal, 0, -1):
        for x2 in range(maxVal, 0, -1):
            product = x1 * x2
            if (isPalindrome(product)):
                if x1 >= digit1 or x2 >= digit2:
                    if product > largestPal:
                        largestPal = product
                        digit1, digit2 = x1, x2
                # if both values multiplied are less than the current values,
                # we know that max value has been found and can stop
                elif x1 < digit1 and x2 < digit2:
                    break
                
    return largestPal

print(largestPalindrome(3))