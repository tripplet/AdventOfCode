# -*- coding: utf-8 -*-
"""
Created on Mon Dec  9 21:21:07 2019

@author: ttobias
"""

class IntComputer:
    # syntax =>  opcode: (number of parameter, output parameter pos)
    opcode_param_count = {
      1:  (3,  2), # add
      2:  (3,  2), # multiply
      3:  (1,  0), # input
      4:  (1, -1), # output
      5:  (2, -1), # jump-if-true
      6:  (2, -1), # jump-if-false
      7:  (3,  2), # less-than
      8:  (3,  2), # equals
      9:  (1, -1), # adjusts-relative-base
      99: (0, -1), # end
    }

    def __init__(self, memory):
        self.memory = memory
        self.instruction_pos = 0
        self.relative_base = 0
        self.outputs = []

    @staticmethod
    def from_string(txt): return IntComputer(IntComputer.read(txt))

    @staticmethod
    def from_file(filename):
        with open(filename) as fp:
            return IntComputer.from_string(fp.read())

    @staticmethod
    def read(str): return list(map(lambda x : int(x), str.split(',')))

    def _ensure_size(self, index):
        if index >= len(self.memory):
            self.memory = self.memory + ([0] * (index - len(self.memory) + 1))

    def __getitem__(self, index):
        self._ensure_size(index)
        return self.memory[index]

    def __setitem__(self, index, value):
        self._ensure_size(index)
        self.memory[index] = value

    def get(self, index, mode, is_output):
        if is_output:
            if mode == 0:
                return self.memory[index]
            elif mode == 2:
                return self.relative_base + self.memory[index]
            else:
                raise Exception(f'unsupported mode {mode}')
        else:
            if mode == 0:
                index = self.memory[index]
            elif mode == 1:
                pass # index is already correct
            elif mode == 2:
                index = self.relative_base + self.memory[index]
            else:
                raise Exception(f'unsupported mode {mode}')
            return self[index]

    def process_instruction(self, input_values):
        # intcode instruction decoding
        opcode = self[self.instruction_pos] % 100
        instruction = self[self.instruction_pos] - opcode
        position_updated = False

        # 0 = position mode, 1 = immediate mode, 2 = relative mode
        mode_param = [-1, -1, -1]
        mode_param[2], instruction = divmod(instruction, 10000)
        mode_param[1], instruction = divmod(instruction, 1000)
        mode_param[0], instruction = divmod(instruction, 100)

        # get the parameters
        param_def = IntComputer.opcode_param_count[opcode]
        param = [0] * param_def[0]

        for idx in range(param_def[0]):
            param[idx] = self.get(self.instruction_pos + idx + 1, mode_param[idx], idx == param_def[1])

        # perform instruction
        if opcode == 1: self[param[2]] = param[0] + param[1] # add
        elif opcode == 2: self[param[2]] = param[0] * param[1] # multiply
        elif opcode == 3: self[param[0]] = input_values.pop(0) # input
        elif opcode == 4: self.outputs.append(param[0]) # output

        elif opcode == 5: # jump-if-true
            if param[0] != 0:
                self.instruction_pos = param[1]
                position_updated = True
        elif opcode == 6: # jump-if-false
            if param[0] == 0:
                self.instruction_pos = param[1]
                position_updated = True
        elif opcode == 7: # less-than
            if param[0] < param[1]:
                self[param[2]] = 1
            else:
                self[param[2]] = 0
        # equals
        elif opcode == 8:
            if param[0] == param[1]:
                self[param[2]] = 1
            else:
                self[param[2]] = 0

        elif opcode == 9: self.relative_base += param[0] # adjusts-relative-base
        elif opcode == 99: return False # end
        else:
            raise Exception(f'unsupported opcode {opcode}')

        if not position_updated: self.instruction_pos += param_def[0] + 1
        return True

    def execute(self, input_values):
        run = True
        while run: run = self.process_instruction(input_values)


test_prog1 = IntComputer.from_string('109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99')
test_prog2 = IntComputer.from_string('1102,34915192,34915192,7,4,7,99,0')
test_prog3 = IntComputer.from_string('104,1125899906842624,99')


test_prog1.execute([])
print(f'Test 1: {test_prog1.outputs}')

test_prog2.execute([])
assert test_prog2.outputs[0] == 1219070632396864
print(f'Test 2: {test_prog2.outputs}')

test_prog3.execute([])
assert test_prog3.outputs[0] == 1125899906842624
print(f'Test 3: {test_prog3.outputs}')

boost_prog_part1 = IntComputer.from_file('input.txt')
boost_prog_part1.execute([1])
assert boost_prog_part1.outputs[0] == 2752191671
print(f'Part 1: {boost_prog_part1.outputs}')

boost_prog_part2 = IntComputer.from_file('input.txt')
boost_prog_part2.execute([2])
assert boost_prog_part2.outputs[0] == 87571
print(f'Part 2: {boost_prog_part2.outputs}')
