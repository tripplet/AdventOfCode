#!/bin/env python3

import math

sol = []


# 67,7,59,61
a = 17
b = 13
d = 2


for x in range(1000):
  for y in range(1000):
    if (x*a)+d==y*b:
        sol.append((x,y))


idx = 0
for idx in range(len(sol)):

    if idx > 0:
        print(f"> {sol[idx][0]-sol[idx-1][0]} {sol[idx][1]-sol[idx-1][1]} ({(sol[idx][0]*a)-(sol[idx-1][0]*a)})")

    print(f"  {sol[idx][0]} {sol[idx][1]} {sol[idx][0]*a} {sol[idx][1]*b}")

