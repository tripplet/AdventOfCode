# -*- coding: utf-8 -*-
"""
Created on Tue Dec 17 17:47:15 2019

@author: TTangemann
"""
from itertools import product

from IntComputer import IntComputer

def rotate(y, x):
    if   y ==  1 and x ==  0: return  0, -1
    elif y ==  0 and x == -1: return -1,  0
    elif y == -1 and x ==  0: return  0,  1
    elif y ==  0 and x ==  1: return  1,  0


prog = IntComputer.from_file('input.txt')
prog.execute([])

card = []
line = []
line_count = 0
for char in prog.outputs:
    print(str(chr(char)), end='')
    if char != 10:
        line.append(char)
    else:
        card.append(line)
        line = []

del card[len(card) - 1]

scaffold = ord('#')
part1 = 0
for y, x in product(range(1, len(card) - 1), range(1, len(card[0]) - 1)):
    if card[y][x] == scaffold and \
       card[y+1][x] == scaffold and \
       card[y-1][x] == scaffold and \
       card[y][x+1] == scaffold and \
       card[y][x-1] == scaffold:
          part1 += x * y

print(f'Part1: {part1}')


# Find robot
for y, x in product(range(1, len(card) - 1), range(1, len(card[0]) - 1)):
    v = chr(card[y][x])
    if v == '^' or v == '<' or v == '>' or v == 'v':
        robot = (y, x)

# Find path
path = []
pos_x = robot[1]
pos_y = robot[0]
dir_x = 1
dir_y = 0
r = 0
while True:
    next_pos_x = pos_x + dir_x
    next_pos_y = pos_y + dir_y
    if card[next_pos_y][next_pos_x] == scaffold:
        path[len(path) - 1] +=1
        pos_x = next_pos_x
        pos_y = next_pos_y
        r = 0
    else:
        dir_y, dir_x = rotate_left(dir_y, dir_x)
        r += 1
        if r
        path.append('L')
        path.append(0)
