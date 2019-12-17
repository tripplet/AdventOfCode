# -*- coding: utf-8 -*-
"""
Created on Tue Dec 17 17:47:15 2019

@author: TTangemann
"""
from itertools import product

from IntComputer import IntComputer

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
crossings = 0
for y, x in product(range(1, len(card) - 1), range(1, len(card[0]) - 1)):
    if card[y][x] == scaffold and \
       card[y+1][x] == scaffold and \
       card[y-1][x] == scaffold and \
       card[y][x+1] == scaffold and \
       card[y][x-1] == scaffold:
          crossings += x * y