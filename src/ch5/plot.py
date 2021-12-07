#!/usr/bin/env python3
import numpy as np, sys
from matplotlib import pyplot as plt
if len(sys.argv) < 2: quit()
# read file
x, y = ([], [])
lines = open(sys.argv[1], "r").read().split("\n")
for i, v in enumerate(lines):
    if v != '':
        y.append(float(v))
        x.append(i)
    if i > 1500: break
plt.plot(x, y)
plt.show()

