from PIL import Image, ImageDraw, ImageColor
import csv
from tqdm import tqdm
from math import floor
from sys import exit
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
        if pixel_color[2] > 0x0F and color_to_html(pixel_color) != node:
            print("({start[0]};{start[1]}) failed on ({pos[0]};{pos[1]})".format(
                start=start, pos=pos))
            return False
        if color_to_html(pixel_color) == node:
            return True
    return True


wall = "#1E1EE4"
node = "#FFFF00"
brush_size = 8
colored = []
colored_dirs = []
colored_cons = []
im = Image.open("./board_nodes.png")
draw = ImageDraw.Draw(im)
width = im.width
height = im.height
pixelcount = width * height

for i in tqdm(range(0, pixelcount), desc="Finding pixels"):
    (x, y) = as_coord(i)
    pixel = im.getpixel((x, y))
    if (color_to_html(pixel)) == node:
        skip = False
        for c in colored:
            if is_bounding_pixel(c, (x, y)):
                skip = True
                break
        if skip:
            continue
        colored.append((int(x+(brush_size/2)), int(y+(brush_size/2))))

print("List {}\nLen: {}".format(colored, colored.__len__()))

for p in tqdm(colored, desc="Checking whether a node has a connection"):
    print("Testing node ({p[0]};{p[1]})".format(p=p))
    possible = [
        check_linear_path(p, [p[0], 0], -1),
        check_linear_path(p, [im.width, p[1]], 1),
        check_linear_path(p, [p[0], im.height], 1),
        check_linear_path(p, [0, p[1]], -1)]
    colored_dirs.append(possible)

for i in tqdm(range(0, colored.__len__())):
    possible = colored_dirs[i]
    pixel_act = colored[i]
    colored_cons.append([-1, -1, -1, -1])
    for j in tqdm(range(0, colored.__len__())):
        pixel_testing = colored[j]
        if pixel_testing[0] == pixel_act[0] and pixel_testing[1] > pixel_act[1]:
            print("({p1[0]};{p1[1]}) <==down==> ({p2[0]};{p2[1]})".format(
                p1=pixel_act, p2=pixel_testing))

exit(-1)

with open("../nodes.csv", "w") as file:
    writer = csv.DictWriter(
        file, fieldnames=["x", "y", "up", "right", "down", "left"])
    writer.writeheader()
    for i in range(0, colored.__len__()):
        c = colored[i]
        cd = colored_dirs[i]
        writer.writerow({
            "x": c[0],
            "y": c[1],
            "up": cd[0],
            "right": cd[1],
            "down": cd[2],
            "left": cd[3]
        })


"""for p in colored:
    draw.arc([
        p[0],
        p[1],
        p[0] + brush_size,
        p[1] + brush_size,
    ],
        0,
        360)
im.show()
"""
