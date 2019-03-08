from PIL import Image, ImageDraw, ImageColor, ImageMode
from math import floor
from tqdm import tqdm
from sys import exit
import csv

"""
Pillow: https://pillow.readthedocs.io/en/stable/reference/ImageDraw.html?highlight=draw_arc
csv: https://docs.python.org/2/library/csv.html
"""


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


colors = {}
colors["red"] = "#FF0000"
colors["blue"] = "#1E1EE4"
colors["green"] = "#00FF00"
colors["yellow"] = "#FFFF00"
colors["black"] = "#000000"
colors["node"] = colors["red"]
im = Image.open("./board+dots.png")
width = im.width
height = im.height
pixelcount = width * height
bitmap = [False]*(pixelcount)
draw = ImageDraw.Draw(im)

im2 = Image.new("RGB", (width, height))

print("{w}x{h}".format(w=width, h=height))
for i in tqdm(range(0, pixelcount), desc="Checking program status"):
    (x, y) = as_coord(i)
    j = as_index(x, y)
    try:
        assert(j == i)
        assert(x >= 0 and x < width and y >= 0 and y < height)
    except AssertionError:
        print("\n\t({x: 04},{y: 04}) = as_coord({i: 8})\nas_index({x: 04},{y: 04}) =        {j: 10}\n0 <= {x: 4} < {w}\n0 <= {y: 4} < {h}".format(
            x=x, y=y, j=j, i=i, w=width, h=height))
        exit(-1)


""" Preprocess: RGB > Boolean """
for i in tqdm(range(0, pixelcount), desc="Converting to bitmap"):
    (x, y) = as_coord(i)
    pixel = im.getpixel((x, y))
    bitmap[i] = (color_to_html(pixel) == colors["node"])
    if bitmap[i]:
        im2.putpixel((x, y), html_to_color(colors["node"]))

""" Analyze bitmap: True+True+True > True+False+False """
for j in tqdm(range(0, pixelcount), desc="Analyzing bitmap"):
    i = pixelcount - j
    (x, y) = as_coord(i)
    if x > 0 and bitmap[as_index(x-1, y)] == True:
        bitmap[as_index(x, y)] = False
        im2.putpixel((x, y), html_to_color(colors["black"]))
    if y > 0 and bitmap[as_index(x, y-1)] == True:
        bitmap[as_index(x, y)] = False
        im2.putpixel((x, y), html_to_color(colors["black"]))

""" Fix bitmap positions: True+False+False > False+True+False """
""" for j in tqdm(range(0, pixelcount), desc="Fixing positions"):
    i = pixelcount - j
    (x, y) = as_coord(i)
    if bitmap[as_index(x, y)] == True:
        bitmap[as_index(x, y)] = False
        bitmap[as_index(x+3, y+3)] = True """
""" im2.putpixel((x, y), (0xFF, 0xFF, 0x0))
    try:
        im2.putpixel((x+3, y+3), (0x00, 0xFF, 0x0))
    except IndexError:
        None """
im2.show()
with open("../dots.csv", "w") as file:
    writer = csv.DictWriter(
        file, fieldnames=["x", "y", "score"])
    writer.writeheader()
    for i in tqdm(range(0, pixelcount), desc="Writing file"):
        if bitmap[i] == True:
            (x, y) = as_coord(i)
            im2.putpixel((x, y), 1)
            writer.writerow({
                "x": x,
                "y": y,
                "score": 10,
            })
