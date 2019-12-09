# -*- coding: utf-8 -*-
"""
Created on Wed Dec  4 10:23:09 2019

@author: ttobias
"""


def any_two_adjacent_digits_match(digits):
    for idx in range(len(digits) - 1):
        if digits[idx] == digits[idx+1]:
            return True
    return False


def exactly_two_adjacent_digits_match(digits):
    for idx in range(len(digits) - 1):
        nb_after = digits[idx + 2] if idx + 2 < len(digits) else None
        nb_before = digits[idx - 1] if idx - 1 >= 0 else None

        if digits[idx] == digits[idx+1] and \
           (digits[idx] != nb_before or nb_before is None) and \
           (digits[idx] != nb_after or nb_after is None):
            return True
    return False


def digits_never_decrease(digits):
    for idx in range(len(digits) - 1):
        if digits[idx + 1] < digits[idx]:
            return False
    return True


def find_code_count(start, end):
    count = 0
    for nb in range(start, end + 1):
        digits = list([int(digit) for digit in str(nb)])
        if any_two_adjacent_digits_match(digits) and \
           digits_never_decrease(digits):
           count += 1
    return count


def find_code_count_part2(start, end):
    count = 0
    for nb in range(start, end + 1):
        digits = [int(digit) for digit in str(nb)]
        if exactly_two_adjacent_digits_match(digits) and \
           digits_never_decrease(digits):
           count += 1
    return count


print(find_code_count(128392,643281))
print(find_code_count_part2(128392,643281))
