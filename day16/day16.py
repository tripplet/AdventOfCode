# -*- coding: utf-8 -*-
"""
Created on Mon Dec 16 07:46:41 2019

@author: ttobias
"""
import math
import numpy as np


def read(txt):
    return np.array([int(char) for char in txt])


def read_fromfile(filename):
    with open(filename) as fp:
        return read(fp.read())


def fft(transmission, phases, count):
    for idx in range(count):
        print(idx)
        tr_next = transmission.copy()
        for x in range(len(transmission)):
            a = np.repeat(phases, x + 1)
            b = np.tile(a, math.ceil(len(transmission) / len(phases)) + 1)[1:len(transmission) + 1]
            tr_next[x] = np.abs(np.sum(transmission * b)) % 10
        transmission = tr_next
    return transmission


def fft_offset(transmission, count, offset):
    for cnt in range(count):
        for idx in reversed(range(offset, len(transmission) - 1)):
            transmission[idx] = (transmission[idx] + transmission[idx+1]) % 10

    return transmission[offset:offset+8]

phases = [0, 1, 0, -1]
assert np.array_equal(fft(read('12345678'), phases, 4)[0:8], read('01029498'))

#part1 = fft(read_fromfile('input.txt'), phases, 100)[0:8]
#print(f'Part 1: {part1}')

part2 = np.tile(read('03036732577212944063491565474664'), 10000)
part2_result = fft_offset(part2, 100, 303673)
print(part2_result)

part2 = np.tile(read_fromfile('input.txt'), 10000)
part2_result = fft_offset(part2, 100, 5976697)
print(part2_result)
