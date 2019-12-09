# -*- coding: utf-8 -*-
"""
Created on Mon Dec  9 21:21:07 2019

@author: ttobias
"""

class IntCodeProgram:
    # syntax =>  opcode: (number of parameter, output parameter pos) 
    opcode_param_count = {
      1:  (3, 2), # add
      2:  (3, 2), # multiply
      3:  (1, 0), # input
      4:  (1, -1), # output
      5:  (2, -1), # jump-if-true
      6:  (2, -1), # jump-if-false
      7:  (3, 2), # less-than
      8:  (3, 2), # equals
      9:  (1, -1), # adjusts-relative-base
      99: (0, -1), # end
    }

    def __init__(self, memory):
        self.memory = memory
        self.instruction_pos = 0
        self.relative_base = 0
        self.outputs = []



def read(str):
    return list(map(lambda x : int(x), str.split(',')))


def get_value(state, index, mode, is_output):
    if is_output:
        if mode == 0:
            return state.memory[index]
        elif mode == 2:
            return state.relative_base + state.memory[index] 
        else:
            raise Exception(f'unsupported mode {mode}')   
    else:
        if mode == 0:
            index = state.memory[index]
        elif mode == 1:
            pass # index is already correct
        elif mode == 2:
            index = state.relative_base + state.memory[index]
        else:
            raise Exception(f'unsupported mode {mode}')
    
        if index >= len(state.memory):
            state.memory = state.memory + ([0] * (index - len(state.memory) + 1))
    
        return state.memory[index]


def set_value(state, index, value):
    if index >= len(state.memory):
        state.memory = state.memory + ([0] * (index - len(state.memory) + 1))

    state.memory[index] = value



def process_instruction(state, input_values):
    # intcode instruction decoding
    opcode = state.memory[state.instruction_pos] % 100
    instruction = state.memory[state.instruction_pos] - opcode
    position_updated = False

    # 0 = position mode, 1 = immediate mode, 2 = relative mode
    mode_param = [-1, -1, -1]
    mode_param[2], instruction = divmod(instruction, 10000)
    mode_param[1], instruction = divmod(instruction, 1000)
    mode_param[0], instruction = divmod(instruction, 100)

    # get the parameters
    param_def = IntCodeProgram.opcode_param_count[opcode]
    param = [0] * param_def[0]

    for idx in range(param_def[0]):
        param[idx] = get_value(state, state.instruction_pos + idx + 1, mode_param[idx], idx == param_def[1])

    # perform instruction
    # add
    if opcode == 1:
        set_value(state, param[2], param[0] + param[1])

    # multiply
    elif opcode == 2:
        set_value(state, param[2], param[0] * param[1])

    # input
    elif opcode == 3:
        set_value(state, param[0], input_values.pop(0))

    # output
    elif opcode == 4:
        state.outputs.append(param[0])

    # jump-if-true
    elif opcode == 5:
        if param[0] != 0:
            state.instruction_pos = param[1]
            position_updated = True

    # jump-if-false
    elif opcode == 6:
        if param[0] == 0:
            state.instruction_pos = param[1]
            position_updated = True

    # less-than
    elif opcode == 7:
        if param[0] < param[1]:
            set_value(state, param[2], 1)
        else:
            set_value(state, param[2], 0)

    # equals
    elif opcode == 8:
        if param[0] == param[1]:
            set_value(state, param[2], 1)
        else:
            set_value(state, param[2], 0)

    # adjusts-relative-base
    elif opcode == 9:
        state.relative_base += param[0]

    # end
    elif opcode == 99:
        return False

    else:
        raise Exception(f'unsupported opcode {opcode}')

    if not position_updated:
        state.instruction_pos += param_def[0] + 1
    return True


def execute(computer_state, input_values):
    run = True
    while run:
        run = process_instruction(computer_state, input_values)


with open('input.txt') as fp:
    data = read(fp.read())


test_prog1 = IntCodeProgram(read('109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99'))
test_prog2 = IntCodeProgram(read('1102,34915192,34915192,7,4,7,99,0'))
test_prog3 = IntCodeProgram(read('104,1125899906842624,99'))


execute(test_prog1, [])
print(f'Test 1: {test_prog1.outputs}')

execute(test_prog2, [])
print(f'Test 2: {test_prog2.outputs}')

execute(test_prog3, [])
print(f'Test 3: {test_prog3.outputs}')

boost_prog_part1 = IntCodeProgram(data)
execute(boost_prog_part1, [1])
print(f'Part 1: {boost_prog_part1.outputs}')

boost_prog_part2 = IntCodeProgram(data)
execute(boost_prog_part2, [2])
print(f'Part 2: {boost_prog_part2.outputs}')
