# -*- coding: utf-8 -*-
"""
Created on Tue Dec 10 19:53:25 2019

@author: ttobias
"""
import math
import pdb
import numpy as np


def parse(txt):
    char_to_int = lambda char: 0 if char == '.' else 1
    return np.array(list(map(lambda line: [char_to_int(char) for char in line], txt.splitlines())))


def get_map(filename):
    with open(filename) as fp:
        return parse(fp.read())

def count_visible(asteroidmap, pos):
    count = 0
    localmap = asteroidmap.copy()
    localmap[pos[0], pos[1]] = -1
    size_x = len(localmap[0])
    size_y = len(localmap)

    for ay in range(size_y):
        for ax in range(size_x):
            if localmap[ay, ax] <= 0:
                continue

            # possible asteroid detected
            # calculate vector
            vector = np.array([ay, ax]) - pos
            div = math.gcd(vector[0], vector[1])
            base_vector = vector // div

            if div == 1 and localmap[ay, ax] == 1:
                # in direct line of sight
                count += 1
                localmap[ay, ax] = 2
            else:
                # might be obstructed
                check = pos + base_vector

                new = None
                count_new = True
                while check[0] >= 0 and check[0] < size_y and \
                      check[1] >= 0 and check[1] < size_x:
                    value = localmap[check[0], check[1]]
                    if new is not None:
                        if value == 2:
                            localmap[check[0], check[1]] = 1
                            count_new = False
                    else:
                        if value == 1:
                            new = check
                            localmap[check[0], check[1]] = 2
                        elif value == 2:
                            break
                    check += base_vector
                if count_new and new is not None:
                    count += 1
    return count, localmap

def pos_to_angle(y, x):
    '''map x,y pos to angle where 0 rad is up going clockwise'''
    return ((math.pi/2 - math.atan2(-y, x)) + 2*math.pi) % (2*math.pi)

def check_map(asteroidmap):
    max_count = 0
    result = asteroidmap.copy()

    for y in range(len(asteroidmap)):
        for x in range(len(asteroidmap[0])):
            if asteroidmap[y, x] == 0:
                continue
            count, direct_sight_map = count_visible(asteroidmap, np.array([y, x]))
            result[y, x] = count
            if count > max_count:
                max_count = count
                max_sight_map = direct_sight_map
                best = (x, y)
    return max_sight_map, max_count, best


def find_plant_destroy(asteroidmap, target):
    max_sight_map, _, laser_pos = check_map(asteroidmap)

    angles = []
    lx = laser_pos[0]
    ly = laser_pos[1]
    loop = 1

    while True:
        for y in range(len(max_sight_map)):
            for x in range(len(max_sight_map[0])):
                if max_sight_map[y, x] == 2:
                    angles.append((pos_to_angle(y - ly, x - lx) + 2*math.pi*loop, x, y))
                    max_sight_map[y, x] = 0 # destory asteroid
        angles.sort(key=lambda pos: pos[0])

        if len(angles) < target:
            max_sight_map, _ = count_visible(asteroidmap, np.array([ly, lx]))
            loop += 1
        else:
            break

    print(angles[target - 1], angles[target - 1][1]* 100 + angles[target - 1][2])




def test(filename, pos, count):
    asteroidmap = get_map(filename)
    try:
        _, max_count, result_pos = check_map(asteroidmap)
        if max_count != count or pos != result_pos:
            raise Exception("Invalid result")
    except:
        pdb.post_mortem()

if True:
    # checks
    test('test0.txt', (3, 4), 8)
    test('test1.txt', (5, 8), 33)
    test('test2.txt', (1, 2), 35)
    test('test3.txt', (6, 3), 41)
    test('test4.txt', (11, 13), 210)
    test('input.txt', (23, 19), 278)
    print('Checks passed')

# Part 2
find_plant_destroy(get_map('input.txt'), 200)
