# -*- coding: utf-8 -*-
"""
Created on Tue Dec 10 19:53:25 2019

@author: ttobias
"""
import math
import numpy as np
import pdb


def char_to_int(char):
    return 0 if char == '.' else 1


def parse(str):
    return np.array(list(map(lambda line: [char_to_int(char) for char in line], str.splitlines())))


def count_visible(asteroidmap, pos):
    count = 0
    localmap = asteroidmap.copy()

    localmap[pos[0], pos[1]] = -1

    for ay in range(len(localmap)):
        for ax in range(len(localmap[0])):
            check_pos = np.array([ay, ax])

            if localmap[ay,ax] == 0 or (check_pos[0] == pos[0] and check_pos[1] == pos[1]):
                continue

            vector = np.array([ay, ax]) - pos
            div = math.gcd(vector[0], vector[1])
            base_vector = vector // div

            if div == 1:
                count += 1
                localmap[ay,ax] = 2
                #print(localmap)
            else:
                if base_vector[0] != 0:
                    nb_segements = vector[0] // base_vector[0]
                else:
                    nb_segements = vector[1] // base_vector[1]

                found = False
                for mult in range(1, nb_segements):
                    possible = pos + (mult*base_vector)
                    value = localmap[possible[0], possible[1]]
                    if value >= 1:
                        localmap[possible[0], possible[1]] = 3
                        #print(localmap)
                        found = True
                if not found:
                    count += 1
                    localmap[ay,ax] = 4
                    #print(localmap)
    #print(localmap)
    return count, localmap

def check_map(asteroidmap):
    max_count = 0

    result = asteroidmap.copy()


    for y in range(len(asteroidmap)):
        for x in range(len(asteroidmap[0])):
            if asteroidmap[y,x] == 0:
                continue
            count, location_map = count_visible(asteroidmap, np.array([y, x]))
            #print(x, y, count)
            result[y, x] = count
            if count > max_count:
                max_count = count
                max_location_map = location_map
                best = (x, y)

    print()
    #print(result)
    print(max_count, best)
    print(max_location_map)
    return max_location_map, (y, x)

with open('input.txt') as fp:
    asteroidmap = parse(fp.read())


destroy_map = None

try:
    destroy_map, _ = check_map(asteroidmap)
except:
    pdb.post_mortem()

