# Problem 5: Smallest Multiple

# 2520 is the smallest number that can be divided by each of the numbers from
# 1 to 10 without any remainder.

# What is the smallest positive number that is evenly divisible by all of the
# numbers from 1 to 20?

# divByAll takes num as input and finds the smallest positive number that is 
# evenly divisible by all numbers from 1 to num
def divByAll(num):
    # set value equal to max number to be divided by
    testVal = num
    # begin loop to check for each value of testVal, increment by num each loop
    while True:
        # for every value of testVal, go through all numbers between 1 and num
        for i in range(1, num + 1):
            # if any value between 1 and num is not evenly divided, break and continue search
            if (testVal / i).is_integer() == False:
                break
            # if i is now the max number to be divided by and the result is even, we have 
            # found the first number that can be divided by all numbers between 1 and num
            elif (i == num) and (testVal / i).is_integer():
                return testVal
        testVal += num

print(divByAll(20))