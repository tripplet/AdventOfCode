# -*- coding: utf-8 -*-
"""
Created on Fri Dec 13 10:10:37 2019

@author: ttobias
"""
import threading
import time
import tkinter

from IntComputer import IntComputer

def game_loop(delay):
    run = True
    tile_x = 0
    ball_x = 0
    move = 0
    start_game = False
    score = 0

    while run:
        out = []
        while run and len(out) != 3:
            run, output = game.process_instruction([move])
            if output is not None:
                out.append(output)

        if not run:
            print(f'Part 2: Score={score}')
            if delay != 0:
                w.create_text(200, 15, font=('Consolas', 12), text='GAME OVER')
                master.destroy()
            break

        x = out[0]
        y = out[1] + 3
        tile = out[2]

        if x == -1:
            if delay != 0:
                w.create_rectangle(0, 0, 100, 30, fill="white")
                w.create_text(40, 15, font=('Consolas', 12), text=str(tile))
                start_game = True
            score = tile
        elif tile == 0: # empty
            if delay != 0:
                w.create_rectangle(x*10, y*10, x*10 + 10, y*10 + 10, fill="white")
        elif tile == 1: # wall
            if delay != 0:
                w.create_rectangle(x*10, y*10, x*10 + 10, y*10 + 10, fill="blue")
        elif tile == 2: # block tile
            if delay != 0:
                w.create_rectangle(x*10, y*10, x*10 + 10, y*10 + 10, fill="red")
        elif tile == 3: # horizontal paddle tile
            if delay != 0:
                w.create_rectangle(x*10, y*10, x*10 + 10, y*10 + 10, fill="black")
            tile_x = x
        elif tile == 4: # ball
            if delay != 0:
                w.create_oval(x*10, y*10, x*10 + 10, y*10 + 10, fill="green")
            ball_x = x

        if ball_x == tile_x:
            move = 0
        else:
            move = (int((ball_x - tile_x) > 0) * 2) - 1

        if delay != 0 and start_game:
            time.sleep(delay)

# %%

game = IntComputer.from_file('input.txt')
game.execute([])

block_count = 0
for idx in range(2, len(game.outputs), 3):
    if game.outputs[idx] == 2:
        block_count += 1

print(f'Part 1: {block_count} block tiles')


# use delay > 0 to enable visualization
delay = 0 # 0.01
game = IntComputer.from_file('input.txt')
game.outputs = None
game.memory[0] = 2

if delay != 0:
    master = tkinter.Tk()
    w = tkinter.Canvas(master, width=500, height=500)
    w.pack()




if delay != 0:
    bg_thread = threading.Thread(name='game-loop', target=game_loop, args=(delay,))
    bg_thread.start()
    tkinter.mainloop()
else:
    game_loop(delay)
