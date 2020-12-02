# -*- coding: utf-8 -*-
"""
Created on Sun Dec 15 11:06:45 2019

@author: ttobias
"""
import tkinter
import threading
import time

from IntComputer import IntComputer


UP = 1
DOWN = 2
LEFT = 3
RIGHT = 4

go_map_clockwise = {
    (0, UP): RIGHT,
    (0, DOWN): LEFT,
    (0, LEFT): UP,
    (0, RIGHT): DOWN,

    (1, UP): LEFT,
    (1, DOWN): RIGHT,
    (1, LEFT): DOWN,
    (1, RIGHT): UP,
}

go_map_counterclockwise = {
    (0, UP): LEFT,
    (0, DOWN): RIGHT,
    (0, LEFT): DOWN,
    (0, RIGHT): UP,

    (1, UP): RIGHT,
    (1, DOWN): LEFT,
    (1, LEFT): UP,
    (1, RIGHT): DOWN,
}

def get_pos(pos, direction):
    if direction == UP:
         return (pos[0], pos[1] + 1)
    elif direction == DOWN:
         return (pos[0], pos[1] - 1)
    elif direction == LEFT:
         return (pos[0] - 1, pos[1])
    elif direction == RIGHT:
         return (pos[0] + 1, pos[1])

def run_till_output(prog, direction):
    out = None
    while out is None:
        _, out = prog.process_instruction([direction])
    return out

def draw_rect(pos, color):   
    size = 10
    offset = 300
    x = pos[0] * size + offset
    y = pos[1] * size + offset
    x2 = pos[0] * size + size + offset
    y2 = pos[1] * size + size + offset
    w.create_rectangle(x, y, x2, y2, fill=color)

def paint_pos(card, pos, found):    
    if card[pos] < 0:
        draw_rect(pos, "black")
    elif card[pos] == 0:
        draw_rect(pos, "blue")
    elif pos == found:
        draw_rect(pos, "red")
    else:
        draw_rect(pos, "white")

def paint(card, found):
    for pos in card.keys():
        paint_pos(card, pos, found)

prog = IntComputer.from_file('input.txt')



master = tkinter.Tk()
w = tkinter.Canvas(master, width=600, height=600)
w.pack()

def find_shortest_way():
    card = {(0,0): 0}   
    found_clockwise = find_way(card, go_map_clockwise)
    steps_clockwise = card[found_clockwise]
   
    #w.create_rectangle(0, 0, 600, 600, fill="white")
   
    card = {(0,0): 0}
    found_counterclockwise = find_way(card, go_map_counterclockwise)
    steps_counterclockwise = card[found_counterclockwise]
    
    if steps_clockwise < steps_counterclockwise:
        found = found_clockwise
        steps = steps_clockwise
    else:
        found = found_counterclockwise
        steps = steps_clockwise
      
    print(f'Part 1: Found @{found} with {steps} steps')
    paint(card, steps_clockwise)

    for p in card.keys():
        if card[p] > 0:
            card[p] = 0
    
    fill_oxygen_card(card, found_clockwise)

def find_way(card, way):
    found = None
    pos = (0, 0)
    direction = UP
    steps = 0
    
    while pos != (0, 0) or found is None:
        status = run_till_output(prog, direction)
        new_pos = get_pos(pos, direction)        

        if status == 2:
            steps += 1
            found = new_pos
            pos = new_pos
            if new_pos not in card:
                card[new_pos] = steps
            else:
                card[new_pos] = min(steps, card[new_pos])
            steps = card[new_pos]
            status = 1
            #print(f'>>   Found @{found} with {card[found]} steps')
        elif status == 0: # wall
            card[new_pos] = -1
            #paint_pos(card, new_pos, found)
        elif status == 1: # free
            steps += 1
            if new_pos not in card:
                card[new_pos] = steps
            else:
                card[new_pos] = min(steps, card[new_pos])
            steps = card[new_pos]
            pos = new_pos

        direction = way[(status, direction)]
        #paint_pos(card, pos, found)
        #time.sleep(0.02)
    #print(f'Found @{found} with {card[found]} steps')
    return found



def fill_oxygen_card(card, pos):
    next_steps = {pos}
    minutes = 0
    
    while len(next_steps) > 0:
        for cur_pos in next_steps.copy():
            new_steps = fill_oxygen_adjecent(card, cur_pos)
            next_steps.remove(cur_pos)
            next_steps.update(new_steps)
        if len(next_steps) > 0:
            minutes += 1
            #time.sleep(0.05)
    
    print(f'Part 2: Oxygen fill takes {minutes} minutes')
    


def fill_oxygen_adjecent(card, pos):
    next_steps = set()
    fill_oxygen_direction(card, pos, next_steps, UP)
    fill_oxygen_direction(card, pos, next_steps, DOWN)
    fill_oxygen_direction(card, pos, next_steps, LEFT)
    fill_oxygen_direction(card, pos, next_steps, RIGHT)

    return next_steps

def fill_oxygen_direction(card, pos, next_steps, direction):
    test = get_pos(pos, direction)
    if test in card and card[test] == 0:
        draw_rect(test, 'magenta')
        card[test] = 1
        next_steps.add(test)
        

#find_shortest_way()
bg_thread = threading.Thread(name='find_way', target=find_shortest_way)
bg_thread.start()
tkinter.mainloop()