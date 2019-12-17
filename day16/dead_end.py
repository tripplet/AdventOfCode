# -*- coding: utf-8 -*-
"""
Created on Mon Dec 16 17:48:05 2019

@author: TTangemann
"""

# -*- coding: utf-8 -*-
"""
Created on Mon Dec 16 07:46:41 2019

@author: ttobias
"""
import math
import numpy as np
import time

def read(txt):
    return np.array([int(char) for char in txt])

def read_fromfile(filename):
    with open(filename) as fp:
        return read(fp.read())

def fft(transmission, phases, count):
    print()
    for idx in range(count):
        print(idx)
        tr_next = transmission.copy()


        for x in range(len(transmission)):
            ss = 0

            #a = np.repeat(phases, x + 1)
            #b = np.tile(a, math.ceil(len(transmission) / len(phases)) + 1)[1:len(transmission) + 1]

            for part in range(0, math.ceil(len(transmission) / len(phases)) + 1):
                start = part * (len(phases)*(x+1))
                aaa = transmission[start + 1*(x + 1) - 1:start + 2*(x + 1) - 1]
                bbb = transmission[start + 3*(x + 1) - 1:start + 4*(x + 1) - 1]

                ss += np.sum(aaa)
                ss -= np.sum(bbb)

            tr_next[x] = np.abs(ss) % 10
        transmission = tr_next
    return transmission

phases = [0, 1, 0, -1]

assert np.array_equal(fft(read('12345678'), phases, 4)[0:8], read('01029498'))
print('Test1 done')

assert np.array_equal(fft(read('80871224585914546619083218645595'), phases, 100)[0:8], [2, 4,1,7,6,1,7,6])
print('Test2 done')


assert np.array_equal(fft(np.repeat(read('03036732577212944063491565474664'), 10000), phases, 100)[0:8], [2,4,1,7,6,1,7,6])
print('Test3 done')


start = time.time()
assert np.array_equal(fft(read_fromfile('input.txt'), phases, 100)[0:8], [2,7,8,3,1,6,6,5])
print(time.time() - start, 'seconds')

# 0= 0:1 -  3: 3    4: 4 -  6: 6
# 1= 1:2 -  5: 6    9:10 - 13:14
# 2= 2:4 -  8:10   14:16 - 20:22
# 3= 3:6 - 11:14   19:22 - 27:30
# 4= 4:8 - 14:18   24:


