# -*- coding: utf-8 -*-
"""
Created on Wed Dec 11 13:41:50 2019

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
    def from_string(str): return IntComputer(IntComputer.read(str))

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
        output = None

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
        elif opcode == 4:
            output = param[0]
            if self.outputs is not None:
                self.outputs.append(output) # output

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
        elif opcode == 99: return False, None # end
        else:
            raise Exception(f'unsupported opcode {opcode}')

        if not position_updated: self.instruction_pos += param_def[0] + 1
        return True, output

    def execute(self, input_values):
        run = True
        while run: run, _ = self.process_instruction(input_values)
