# -*- coding: utf-8 -*-
"""
Created on Tue Dec 17 17:47:15 2019

@author: TTangemann
"""
from itertools import product

from IntComputer import IntComputer

def rotate(v, rot):
    result = complex(v[1], v[0]) * complex(0, rot)
    return (int(result.imag), int(result.real))

def rotate_add(pos, direction, rot):
    result_y, result_x = rotate(direction, rot)
    return (pos[0] + result_y, pos[1] + result_x)

def at(card, pos): 
    if pos[0] >= len(card) or pos[1] >= len(card[0]):
        return '-'
    else:
        return card[pos[0]][pos[1]]

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
    if v == '^':
        direction = (-1, 0)
        robot = (y, x)
    elif v == '<':
        direction = (0, -1)
        robot = (y, x)
    elif v == '>':
        direction = (0, 1)
        robot = (y, x)
    elif v == 'v':
        direction = (1, 0)
        robot = (y, x)

# Find path
path = []
pos = robot

while True:
    next_pos = pos[0] + direction[0], pos[1] + direction[1]
    if at(card, next_pos) == scaffold:
        path[len(path) - 1] += 1
    else:
        next_pos = rotate_add(pos, direction, -1) # left
        if at(card, next_pos) == scaffold:
            direction = rotate(direction, -1)
            path.append('L')
            path.append(0)
            continue
        else:
            next_pos = rotate_add(pos, direction, 1) # right
            if at(card, next_pos) == scaffold:
                direction = rotate(direction, 1)
                path.append('R')
                path.append(0)
                continue
            else:
                break
    
    pos = next_pos

print(','.join(map(str, path)))

# compress path

