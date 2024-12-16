# Problem 17: Number Letter Counts

# If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there 
# are 3 + 3 + 5 + 4 + 4 = 19 letters used in total. 

# If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words,
# how many letters would be used?

spellings = {
    "1": "one", "2": "two", "3": "three", "4": "four", "5": "five",
    "6": "six", "7": "seven", "8": "eight", "9": "nine", 
    
    "10": "ten", "11": "eleven", "12": "twelve", "13": "thirteen", 
    "14": "fourteen", "15": "fifteen", "16": "sixteen", "17": "seventeen", 
    "18": "eighteen", "19": "nineteen", 
    
    "20": "twenty", "30": "thirty", "40": "forty", "50": "fifty", "60": "sixty", 
    "70": "seventy", "80": "eighty", "90": "ninety",
}

def letterCount(targetVal):
    totalCount = 0
    x = 1
    while x <= targetVal:
        hundreds = ""
        tens = ""
        ones = ""
        currVal = str(x)
        if x < 10:
            ones = spellings[currVal[0]]
        elif x < 20:
            tens = spellings[currVal]
        elif x < 100:
            tens = spellings[currVal[0]+"0"]
            if currVal[1] != "0":
                ones = spellings[currVal[1]]
            else:
                ones = ""
        elif x < 1000:
            hundreds = spellings[currVal[0]] + "hundred"
            if currVal[1] != "0" and currVal[2] != "0":
                if currVal[-2] + currVal[-1] in spellings:
                    tens = "and" + spellings[currVal[-2] + currVal[-1]]
                    ones = ""
                else:
                    tens = "and" + spellings[currVal[1]+"0"]
                    ones = spellings[currVal[2]]
            elif currVal[1] == "0" and currVal[2] != "0":
                tens = ""
                ones = "and" + spellings[currVal[2]]
            elif currVal[1] != "0" and currVal[2] == "0": 
                if currVal[-2] + currVal[-1] in spellings:
                    tens = "and" + spellings[currVal[-2] + currVal[-1]]
                    ones = ""
                else:
                    tens = "and" + spellings[currVal[1]]
                    ones = ""
            elif currVal[1] == "0" and currVal[2] == "0":
                tens = ""
                ones = ""
        else:
            hundreds = "onethousand"
            tens = ""
            ones = ""
        totalCount += len(hundreds + tens + ones)
        x += 1
    return totalCount

print(letterCount(1000))