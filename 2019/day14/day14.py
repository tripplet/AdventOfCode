# -*- coding: utf-8 -*-
"""
Created on Sat Dec 14 14:47:34 2019

@author: ttobias
"""
import math

def formula_from_file(filename):
    with open(filename) as fp:
        return parse(fp.read())

def parse(txt):
    formula = {}
    for line in txt.splitlines():
        cur = line.split(' => ')
        parts = []
        for chem in cur[0].split(', '):
            inputs = chem.split(' ')
            parts.append((inputs[1], int(inputs[0])))

        output = cur[1].split(' ')
        formula[output[1]] = (int(output[0]), parts)
    return formula


def resolve(need, rest, formula):
    keys = list(need.keys())
    for out in keys:
        if out == 'ORE' or need[out] == 0:
            continue

        cur_formula = formula[out]
        need_out = need[out]

        if out in rest and rest[out] > 0:
            need_out -= rest[out]

        nb_needed = math.ceil(need_out / cur_formula[0])
        for chem in cur_formula[1]:
            if chem[0] not in need:
                need[chem[0]] = 0
            need[chem[0]] += chem[1] * nb_needed

        if out not in rest:
            rest[out] = 0
        rest[out] += nb_needed * cur_formula[0] - need[out]
        need[out] = 0
    return need, rest


def part1(formula, need):
    rest = {}
    finished = False
    while not finished:
        need, rest = resolve(need, rest, formula)
        finished = True
        for chem in need.keys():
            if chem != 'ORE' and need[chem] != 0:
                finished = False
                break
    return need["ORE"]


def part2(formula):
    ore_needed = part1(formula, {'FUEL':1})
    incr = 1
    trillion = 1000000000000
    possible = trillion // ore_needed

    while True:
        ore_needed = part1(formula, {'FUEL': possible})
        ore_needed_plus = part1(formula, {'FUEL': possible + 1})

        if ore_needed <= trillion < ore_needed_plus:
            break
        elif ore_needed < trillion:
            incr *= 2
            possible += incr
        elif ore_needed > trillion:
            possible -= incr - 1
            incr = 1

    return possible



assert part1(formula_from_file('test1.txt'), {'FUEL':1}) == 31
assert part1(formula_from_file('test2.txt'), {'FUEL':1}) == 165
assert part1(formula_from_file('test3.txt'), {'FUEL':1}) == 13312
assert part1(formula_from_file('test4.txt'), {'FUEL':1}) == 180697
assert part1(formula_from_file('test5.txt'), {'FUEL':1}) == 2210736

ore_part1 = part1(formula_from_file('input.txt'), {'FUEL':1})
print(f'Part 1: {ore_part1} ORE needed for 1 FUEL')

assert part2(formula_from_file('test5.txt')) == 460664
assert part2(formula_from_file('test4.txt')) == 5586022
assert part2(formula_from_file('test3.txt')) == 82892753

fuel_possible = part2(formula_from_file('input.txt'))
print(f'Part 2: {fuel_possible} FUEL can be procuded with 1 trillion ORE')
