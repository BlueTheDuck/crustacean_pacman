from PIL import Image, ImageDraw, ImageColor
import csv
from tqdm import tqdm
from math import floor, inf
from sys import exit
from vector import vec
from node import node
"""
Pillow: https://pillow.readthedocs.io/en/stable/reference/ImageDraw.html?highlight=draw_arc
csv: https://docs.python.org/2/library/csv.html
"""


def save():
    with open("nodes.csv", "w") as file:
        writer = csv.DictWriter(
            file, fieldnames=["id", "x", "y", "up", "right", "down", "left"])
        writer.writeheader()
        print("Saving {} nodes".format(nodes.__len__()))
        for i in tqdm(range(0, nodes.__len__()), desc="Writing file"):
            n = nodes[i]
            print("Writing node {}".format(n))
            draw.point((x, y), 1)
            writer.writerow({
                "id": i,
                "x": n.pos.x,
                "y": n.pos.y,
                "up": n.connections[0][0],
                "right": n.connections[1][0],
                "down": n.connections[2][0],
                "left": n.connections[3][0]
            })


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


def in_range(n1, n2):
    print("In range {} / {}".format(n1, n2))
    return (n1 + brush_size / 2) > n2 and (n1 - brush_size / 2) < n2


colors = {}
colors["red"] = "#FF0000"
colors["blue"] = "#1E1EE4"
colors["green"] = "#00FF00"
colors["yellow"] = "#FFFF00"
colors["black"] = "#000000"
colors["node"] = colors["yellow"]
colors["wall"] = colors["blue"]
sides = ["up", "right", "down", "left"]

im = Image.open("./dev/nodes/board_nodes.png")
""" draw = ImageDraw.Draw(im) """
width = im.width
height = im.height
pixelcount = width * height

bitmap = [False]*(pixelcount)
nodes = [node((-1, -1))] * 90

im2 = Image.new("RGB", (width, height))
draw = ImageDraw.Draw(im2)

brush_size = 20

print("{w}x{h}".format(w=width, h=height))

for i in tqdm(range(0, pixelcount), desc="Converting to boolean map"):
    (x, y) = as_coord(i)
    pixel = im.getpixel((x, y))
    bitmap[i] = (color_to_html(pixel) == colors["node"])
    if bitmap[as_index(x, y)]:
        draw.point((x, y), html_to_color(colors["red"]))

for j in tqdm(range(0, pixelcount), desc="Analyzing and deduplicating bitmap"):
    i = pixelcount - j
    (x, y) = as_coord(i)
    if x > 0 and bitmap[as_index(x-1, y)] == True:
        bitmap[as_index(x, y)] = False
        draw.point((x, y), html_to_color(colors["black"]))
    if y > 0 and bitmap[as_index(x, y-1)] == True:
        bitmap[as_index(x, y)] = False
        draw.point((x, y), html_to_color(colors["black"]))

last_node = 0
for i in tqdm(range(0, pixelcount), desc="Discovering nodes"):
    if bitmap[i] == True:
        """ nodes.append(node(as_coord(i))) """
        nodes[last_node] = node(as_coord(i))
        draw.point(as_coord(i), html_to_color(colors["yellow"]))
        draw.text(as_coord(i), "{}".format(last_node))
        last_node += 1
        # nodes[nodes.__len__()-1].pos.__str__()

for i in tqdm(range(0, nodes.__len__()), desc="Finding neighbourghs"):
    print("Testing node {}".format(i))
    for j in range(i+1, nodes.__len__()):
        print("                               ")
        print("Testing {}-{}".format(i, j))
        side = -1
        print(nodes[i].pos.x, nodes[j].pos.x)
        print(nodes[i].pos.y, nodes[j].pos.y)
        if in_range(nodes[i].pos.x, nodes[j].pos.x):
            if nodes[i].pos.y > nodes[j].pos.y:
                side = 0  # Other up
            if nodes[i].pos.y < nodes[j].pos.y:
                side = 2  # Other down
        elif in_range(nodes[i].pos.y, nodes[j].pos.y):
            if nodes[i].pos.x < nodes[j].pos.x:
                side = 1  # Other right
            if nodes[i].pos.x > nodes[j].pos.x:
                side = 3  # Other left

        if side != -1:
            """ and nodes[i].connections[side][1] == inf """
            distance = nodes[i].distance(nodes[j])
            nodes[i].set_connection(side, j, distance)
            draw.line([nodes[i].pos.to_tuple(), nodes[j].pos.to_tuple()],
                      fill=html_to_color(colors["green"]))

directions = [
    vec(0, -1),
    vec(1,  0),
    vec(0,  1),
    vec(-1,  0),
]
for i in tqdm(range(0, nodes.__len__()), desc="Generating connections graph"):
    for s in tqdm(range(0, 4), desc="Doing sides"):
        (nc, dist) = nodes[i].connections[s]
        end = directions[s] * dist + nodes[i].pos
        print("{}".format(end))
        j = nodes[i].connections[s][0]
        if j != -1:
            draw.line([nodes[i].pos.to_tuple(), nodes[j].pos.to_tuple()],
                      fill=html_to_color(colors["yellow"]))
        """ draw.line([nodes[i].pos.to_tuple(), end.to_tuple()]) """
"""         end = directions[s] * dist + nodes[i].pos
        draw.line([nodes[i].pos.to_tuple(), end.to_tuple()],
                  html_to_color(colors["red"])) """
"""         draw.line([nodes[i].pos.to_tuple(), nodes[nc].pos.to_tuple()],
                  html_to_color(colors["green"])) """


im2.show()
save()
