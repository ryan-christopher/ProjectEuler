# Problem 19: Counting Sundays

# You are given the following information, but you may prefer to do some research
# for yourself.

# -1 Jan 1900 was a Monday.
# -Thirty days has September, April, June, and November.
# -All the rest have thirty-one.
# -February has twenty-eight, and on leap years twenty-nine.
# -A leap year occurs on any year evenly divisible by 4, but not on a century 
#  unless it is divisible by 400.

# How many Sundays fell on the first of the month during the twentieth century
# (1 Jan 1901 to 31 Dec 2000)?

# use modulo to keep track of month and day
# January = 0, February = 1, March = 2, ... December = 11
# Monday = 0, Tuesday = 1, Wednesday = 2, ... Sunday = 6
month, dayOfTheWeek = 0, 0

# store months in dictionary
months = {
    "0": 31, "1": 28, "2": 31, "3": 30, "4": 31, "5": 30, 
    "6": 31, "7": 31, "8": 30, "9": 31, "10": 30, "11": 31,  
}

sundayCount = 0
year = 1900

while year < 2001:
    while month <= 11:
        day = 1
        if month == 1 and year % 4 == 0:
            if year % 100 == 0 and year % 400 == 0:
                daysInMonth = 29
            elif year % 100 == 0 and year % 400 != 0:
                daysInMonth = 28
            else:
                daysInMonth = 29
        else:
            daysInMonth = months[str(month)]
        while day <= months[str(month)]:
            if dayOfTheWeek == 6 and day == 1:
                if year > 1900:
                    sundayCount += 1
            day += 1
            dayOfTheWeek = (dayOfTheWeek + 1) % 7
        month = month + 1
    month = 0
    year += 1

print(sundayCount)