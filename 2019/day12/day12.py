# -*- coding: utf-8 -*-
"""
Created on Thu Dec 12 10:27:08 2019

@author: ttobias
"""
import copy
import math

import numpy as np


class Moon:
    def __init__(self, x, y, z):
        self.pos = np.array([x, y, z])
        self.vol = np.array([0, 0, 0])

    def gravity(self, other): self.vol -= np.sign(self.pos - other.pos)
    def velocity(self): self.pos += self.vol
    def energy(self): return np.sum(np.abs(self.pos)) * np.sum(np.abs(self.vol))

    def __repr__(self):
        return 'pos=' + str(self.pos) + ' vol=' + str(self.vol)

    def __eq__(self, other):
        return np.array_equal(self.pos, other.pos) and np.array_equal(self.vol, other.vol)


def lcm(a, b):
    return abs(a*b) // math.gcd(a, b)


def update_moons(moons):
    for m1 in range(len(moons)):
        for m2 in range(len(moons)):
            moons[m1].gravity(moons[m2])#

    for m1 in range(len(moons)):
        moons[m1].velocity()



moons = [Moon(1, 4, 4), Moon(-4, -1, 19), Moon(-15, -14, 12), Moon(-17, 1, 10)]
#moons = [Moon(-8, -10, 0), Moon(5, 5,  10), Moon(2, -7, 3), Moon(9, -8, -3)]
#moons = [Moon(-1, 0, 2), Moon(2, -10, -7), Moon(4, -8, 8), Moon(3, 5, -1)]

moons_start = copy.deepcopy(moons)

#start = time()
#for _ in range(2772):
#    update_moons(moons)
#end = time()

#print(end - start)

count = 0
cycle_length = [0, 0, 0] # x, y, z

while True:
    update_moons(moons)
    count += 1

    for coor in range(3):
        if cycle_length[coor] == 0:
            found = True
            for idx in range(len(moons)):
                if moons[idx].pos[coor] != moons_start[idx].pos[coor] or \
                   moons[idx].vol[coor] != moons_start[idx].vol[coor]:
                    found = False
                    break
            if found:
                cycle_length[coor] = count
    if all([c != 0 for c in cycle_length]):
        break

for m in moons:
    print(m)


total = 0
for m in moons:
    total += m.energy()
print(total)

for m in moons:
    print(m)
