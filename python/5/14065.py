number = float(input())  # mi/gl
number = number / 3.785411784  # mi/L
number = number * 1609.344  # m/L
number = number / 1000  # km/L
number = 100 / number  # L/(100km)
print(number)
