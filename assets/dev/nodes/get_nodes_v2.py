from PIL import Image, ImageDraw, ImageColor
import csv
from tqdm import tqdm
from math import floor
from sys import exit
"""
Pillow: https://pillow.readthedocs.io/en/stable/reference/ImageDraw.html?highlight=draw_arc
csv: https://docs.python.org/2/library/csv.html
"""


class node:
    pos = (0, 0)
    connections = []

    def __init__(self, _pos):
        pos = _pos


def color_to_html(color):
    return '#{:06X}'.format((color[0] << 16) + (color[1] << 8) + color[2])


def html_to_color(html):
    n = int(html.replace("#", ""), 16)
    return (n >> 16 & 0xFF, n >> 8 & 0xFF, n & 0xFF)


def as_index(x, y):
    if x < 0 or x > width:
        raise IndexError("Out of range")
    return y * width + x


def as_coord(i):
    return (i % width, floor(i/width))


def is_bounding_pixel(origin, pixel):
    return (abs(pixel[0] - origin[0]) <= brush_size) and (abs(pixel[1] - origin[1]) <= brush_size)


def check_linear_path(start, end, step):
    pos = [0, 0]
    subi = 1
    if start[0] == end[0]:
        pos[0] = start[0]
        subi = 1
    else:
        pos[1] = start[1]
        subi = 0
    for i in range(start[subi] + step*8, end[subi], step):
        pos[subi] = i
        pixel_color = im.getpixel((pos[0], pos[1]))
        if pixel_color[2] > 0x0F and color_to_html(pixel_color) != colors["node"]:
            print("({start[0]};{start[1]}) failed on ({pos[0]};{pos[1]})".format(
                start=start, pos=pos))
            return False
        if color_to_html(pixel_color) == colors["node"]:
            return True
    return True


colors = {}
colors["red"] = "#FF0000"
colors["blue"] = "#1E1EE4"
colors["green"] = "#00FF00"
colors["yellow"] = "#FFFF00"
colors["black"] = "#000000"
colors["node"] = colors["yellow"]
colors["wall"] = colors["blue"]

im = Image.open("./board_nodes.png")
draw = ImageDraw.Draw(im)
width = im.width
height = im.height

brush_size = 8
