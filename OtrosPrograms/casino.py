#Encuentra el valor de π
import random

n = 1000000
r = 1.0
a, b = (0.0, 0.0)
x_neg, x_pos = a - r, a + r
y_neg, y_pos = b - r, b + r

count = 0.0
for i in range(0, n):

    x = random.uniform(x_neg, x_pos)
    y = random.uniform(y_neg, y_pos)
    
    if x*x + y*y <= 1.0:
        count += 1

result = (count / float(n)) * 4
print ("result is ",result)