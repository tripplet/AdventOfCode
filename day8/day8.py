"""
Created on Sun Dec  8 14:53:42 2019

@author: ttobias
"""
import numpy as np


def read(str):
    return list(map(lambda char : int(char), [*str]))


def get_layers(pixels, rows, cols):
    return np.reshape(pixels, [int(len(pixels) / (rows * cols)), rows * cols])


def count_digits(layer, digit):
    return np.sum(list(map(lambda pixel: 1 if pixel == digit else 0, layer)))
    

def combine_pixel(top, bottom): 
    return top if top == 0 or top == 1 else bottom


def combine_layer(layer_top, layer_bottom):
    a = [0] * len(layer_top)
    for idx in range(len(layer_top)):
        a[idx] = combine_pixel(layer_top[idx], layer_bottom[idx])
    return a


def get_part1(layers):
    count_digits(layers[0], 0)
    
    counts = list(map(lambda arr: count_digits(arr, 0), layers))    
    max0layer = np.where(counts == np.amin(counts))[0][0]
    
    part1 = count_digits(layers[max0layer], 1) * count_digits(layers[max0layer], 2)
    print(part1)
    

def get_part2(layers, rows, cols):
    cur = layers[len(layers) - 1]
    for x in range(len(layers) - 2, -1, -1):
        cur = combine_layer(layers[x], cur)
    
    # Draw combined picture
    pic = ''
    for x in range(len(cur)):
        pic += ' ' if cur[x] == 0 else '#'
        if (x+1)%cols == 0:
            pic += '\r\n'
    print(pic)

        
    
with open('input.txt') as fp:
    data = read(fp.read())


rows = 6
cols = 25
    
layers = get_layers(data, rows, cols)

get_part1(layers)
get_part2(layers, rows, cols)