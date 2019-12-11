# -*- coding: utf-8 -*-
"""
Created on Wed Dec 11 11:31:22 2019

@author: TTangemann
"""
import numpy as np
from PIL import Image, ImageDraw

from IntComputer import IntComputer


def transform_pos(pos, rotate):
    new_angle = (pos[1] + ((rotate*2) - 1)) % 4

    corr = [pos[0][0], pos[0][1]]

    if new_angle == 0: corr[0] -= 1
    elif new_angle == 1: corr[1] += 1
    elif new_angle == 2: corr[0] += 1
    elif new_angle == 3: corr[1] -= 1

    return ((corr[0], corr[1]), new_angle)


paintprogram = IntComputer.from_file('input.txt')


run = True
input_values = [0]
pos = [(0, 0), 0] # y, x, angle, 0=up, 1=right, 2=down, 3=left

output_nb = 0
hull = {}

hull[pos[0]] = 0 # =0 for part 1, =1 for part 2

while run:
    cur_color = 0
    if pos[0] in hull:
        cur_color = hull[pos[0]]

    run, out = paintprogram.process_instruction([cur_color])

    if out is not None:
        if output_nb == 0:
            hull[pos[0]] = out
            output_nb = 1
        elif output_nb == 1:
            pos = transform_pos(pos, out)
            output_nb = 0

max_y = max(hull.keys(), key=lambda pos: pos[0])[0]
min_y = min(hull.keys(), key=lambda pos: pos[0])[0]
max_x = max(hull.keys(), key=lambda pos: pos[1])[1]
min_x = min(hull.keys(), key=lambda pos: pos[1])[1]

image = np.zeros([abs(max_y - min_y) + 1, abs(max_x - min_x) + 1], dtype=int)

for panel in hull.items():
    image[panel[0][0] - min_y, panel[0][1] - min_x] = panel[1]


png = Image.new('RGB', (abs(max_x - min_x) + 1, abs(max_y - min_y) + 1), color = 'white')
draw = ImageDraw.Draw(png)

for y in range(len(image)):
    for x in range(len(image[0])):
        print('#' if image[y, x] == 1 else ' ', end='')
        if image[y, x] == 1:
            draw.point((x, y), fill=(0,0,0))
    print()

png.save('output.png')