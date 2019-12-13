# -*- coding: utf-8 -*-
"""
Created on Fri Dec  6 19:39:37 2019

@author: ttobias
"""

class Obj:
    def __init__(self, name, center):
        self.name = name
        self.center = center
        self.satellites = dict()
        self.orbit_count = 0

    def __repr__(self):
        return f'{self.satellites}'


def parse(text):
    orbits = dict()
    for line in text.splitlines():
        center_str, satellit_str = line.split(')')

        if center_str in orbits:
            center = orbits[center_str]
        else:
            center = Obj(center_str, None)
            orbits[center_str] = center

        if satellit_str in orbits:
            satellit = orbits[satellit_str]
            satellit.center = center
        else:
            satellit = Obj(satellit_str, center)
            orbits[satellit_str] = satellit

        center.satellites[satellit_str] = satellit


    return orbits


def count_indirect(obj):
    if obj.center is None:
        return 0
    elif obj.orbit_count != 0:
        return obj.orbit_count
    else:
        return 1 + count_indirect(obj.center)


def count_all_indirect(orbits):
    result = 0
    for obj in orbits.values():
        obj.orbit_count = count_indirect(obj)
        result += obj.orbit_count

    return result



def find_santa(graph, you, santa, ignore_sat, go_up = True):
    # search satellites first
    for sat in you.satellites.values():
        if sat == santa:
            return 1
        elif sat == ignore_sat:
            continue
        else:
            # search sub satellites
            sub = find_santa(graph, sat, santa, None, False)
            if sub is None:
                continue
            else:
                return 1 + sub

    # go one "up"
    if go_up:
        return 1 + find_santa(graph, you.center, santa, you)
    else:
        return None


with open('input_day6.txt') as fp:
    text = fp.read()

text2='''B)C
C)D
D)E
E)F
B)G
G)H
D)I
COM)B
E)J
J)K
K)L'''

text3 = '''COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN'''

graph = parse(text)
#for x in graph.values(): print(f'{x.name}: {x.orbit_count}')

print(count_all_indirect(graph))

you = graph['YOU'].center
santa = graph['SAN'].center

print(find_santa(graph, you, santa, you))
