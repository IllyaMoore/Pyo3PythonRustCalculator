import rust_calculator
import numpy as np


data = np.loadtxt('python_test/data.txt', dtype={'names': 
                                     ('name', 'value1', 
                                      'value2'), 'formats': 
                                      ('U10', 'f4', 'f4')})

print("Data from text file:")
print(data)

for row in data:
    name, value1, value2 = row
    print(f"Performing calculations for {name}:")
    print(f"Addition: {rust_calculator.add(value1, value2)}")
    print(f"Multiplication: {rust_calculator.multiply(value1, value2)}")
    print(f"Power: {rust_calculator.power(value1, value2)}")
    print("-" * 30)