# Problem 16: Power Digit Sum

# 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

# What is the sum of the digits of the number 2^1000?

def powerSum(x):
    # get power number
    powerNum = 2 ** x
    # store sum
    total = 0
    # cast to string and iterate
    powerNum = str(powerNum)
    for digit in range(len(powerNum)):
        # add each value to total
        total += int(powerNum[digit])
    return total

print(powerSum(1000))