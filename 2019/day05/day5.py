# -*- coding: utf-8 -*-
"""
Created on Mon Dec  2 11:14:25 2019

@author: ttangemann
"""

last_pos = -1

def read(str):
    return list(map(lambda x : int(x), str.split(',')))

def get_value(data, index, position_mode):
    if position_mode:
        index = data[index]
    return data[index]


def process_instruction(data, pos):
    global last_pos

    instruction = data[pos] % 100
    position_mode_1 = (data[pos] // 100) % 10 == 0
    position_mode_2 = (data[pos] // 1000) % 10 == 0
    #position_mode_3 = data[pos] // 10000 == 0

    # add
    if instruction == 1:
        data[data[pos + 3]] = get_value(data, pos + 1, position_mode_1) + get_value(data, pos + 2, position_mode_2)
        pos += 4

    # multiplay
    elif instruction == 2:
        data[data[pos + 3]] = get_value(data, pos + 1, position_mode_1) * get_value(data, pos + 2, position_mode_2)
        pos += 4

    # input
    elif instruction == 3:
        print(f'using input: {input_value}')
        data[data[pos + 1]] = input_value
        pos += 2

    # output
    elif instruction == 4:
        output = get_value(data, pos + 1, position_mode_1)
        print(f'output: {output}, last command was: #{last_pos}: {data[pos]}  {data[pos + 1]}')
        pos += 2

    #  jump-if-true
    elif instruction == 5:
        if get_value(data, pos + 1, position_mode_1) != 0:
            pos = get_value(data, pos + 2, position_mode_2)
        else:
            pos += 3

    #  jump-if-false
    elif instruction == 6:
        if get_value(data, pos + 1, position_mode_1) == 0:
            pos = get_value(data, pos + 2, position_mode_2)
        else:
            pos += 3

    #  less than
    elif instruction == 7:
        if get_value(data, pos + 1, position_mode_1) < get_value(data, pos + 2, position_mode_2):
            data[data[pos + 3]] = 1
        else:
            data[data[pos + 3]] = 0
        pos += 4

    #  equals
    elif instruction == 8:
        if get_value(data, pos + 1, position_mode_1) == get_value(data, pos + 2, position_mode_2):
            data[data[pos + 3]] = 1
        else:
            data[data[pos + 3]] = 0
        pos += 4

    elif data[pos] == 99:
        return -1

    last_pos = pos
    return pos


def execute(data):
    pos = 0
    while pos != -1:
        pos = process_instruction(data, pos)



with open('input_day5.txt') as fp:
    data = read(fp.read())

#data = read('''3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
#1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
#999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99''')

input_value = 5
execute(data)