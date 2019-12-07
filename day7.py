# -*- coding: utf-8 -*-
"""
Created on Sat Dec  7 10:45:15 2019

@author: ttobias
"""
import itertools


def read(str):
    return list(map(lambda x : int(x), str.split(',')))


def get_value(data, index, position_mode):
    if position_mode:
        index = data[index]
    return data[index]


def process_instruction(data, pos, input_values):
    output = None
    instruction = data[pos] % 100
    position_mode_1 = (data[pos] // 100) % 10 == 0
    position_mode_2 = (data[pos] // 1000) % 10 == 0

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
        data[data[pos + 1]] = input_values.pop(0)
        pos += 2

    # output
    elif instruction == 4:
        output = get_value(data, pos + 1, position_mode_1)
        #print(f'output: {output}, last command was: #{last_pos}: {data[pos]}  {data[pos + 1]}')
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
        return -1, None

    return pos, output


def execute(data, input_values):
    pos = 0
    last_output = None
    while pos != -1:
        pos, cur_output = process_instruction(data, pos, input_values)
        if cur_output != None:
            last_output = cur_output
    return last_output


def execute_in_feedback(data, pos, input_values):
    output = None
    while pos != -1 and output is None:
        pos, output = process_instruction(data, pos, input_values)
        #print(f'@{pos} = {output}')
    return pos, output


def execute_amplifier_chain(data, phases):
    output = 0
    for idx in range(len(phases)):
        output = execute(data, [phases[idx], output])
    return output
        

def execute_amplifier_feedback_chain(data, phases):
    last_output = 0
    result = 0
    
    # create memory for each amplifier
    memory = [data.copy() for idx in range(len(phases))]
    positions = [0 for idx in range(len(phases))]
    init = [True for idx in range(len(phases))]
    
        
    while positions[len(phases) - 1] != -1:
        for idx in range(len(phases)):
            if init[idx]:
                init[idx] = False
                if idx == 0:
                    input_values = [phases[idx], 0]
                else:
                    input_values = [phases[idx], last_output]                
            else:
                input_values = [last_output]  
                
            #print(f' > amp{idx} @{positions[idx]}, in:{input_values}')
            positions[idx], last_output = execute_in_feedback(memory[idx], positions[idx], input_values)
            #print(f'amp{idx}: @{positions[idx]}, out:{last_output}')
            
            if last_output is not None and idx == len(phases) - 1:
                result = last_output 

    return result

def find_max_part1(data, phases_count):
    max_output = -1
    for phases in itertools.permutations(range(phases_count)):
        max_output = max(max_output, execute_amplifier_chain(data, phases))
    return max_output


def find_max_part2(data, init_phases):
    max_output = -1
    for phases in itertools.permutations(init_phases):
        max_output = max(max_output, execute_amplifier_feedback_chain(data, phases))
    return max_output



with open('input_day7.txt') as fp:
    data = read(fp.read())

#data = read('3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0')
#data = read('3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0')
#data = read('3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0')


# part 1
#print(find_max_part2(data, range(5, 10)))

# part 2
print(find_max_part2(data, range(5, 10)))   

data = read('3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5 ')
print(execute_amplifier_feedback_chain(data, [9,8,7,6,5]))

data = read('3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10')
#print(execute_amplifier_feedback_chain(data, [9,7,8,5,6]))

print(find_max_part2(data, range(5, 10)))    

