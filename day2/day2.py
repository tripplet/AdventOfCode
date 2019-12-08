# -*- coding: utf-8 -*-
"""
Created on Mon Dec  2 11:14:25 2019

@author: ttobias
"""

def read(str):
    return list(map(lambda x : int(x), str.split(',')))


def process_instruction(input, pos):
    if input[pos] == 1:
        input[input[pos + 3]] = input[input[pos + 1]] + input[input[pos + 2]]
    elif input[pos] == 2:
        input[input[pos + 3]] = input[input[pos + 1]] * input[input[pos + 2]]
    elif input[pos] == 99:
        return -1
    return pos + 4


def execute(data):
    pos = 0
    while pos != -1:
        pos = process_instruction(data, pos)


def calc(data, noun, verb):
    data_cur = data.copy()
    data_cur[1] = noun
    data_cur[2] = verb

    execute(data_cur)
    return(data_cur[0])


def find_target():
    target = 19690720
    with open('input.txt') as fp:
        file_content = read(fp.read())
    data = read(file_content)

    for noun in range(0,99):
        for verb in range(0,99):
            if calc(data, noun, verb) == target:
                print(noun, verb)
                print(100 * noun + verb)


find_target()