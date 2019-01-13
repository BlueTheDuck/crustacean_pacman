from PIL import Image, ImageDraw, ImageColor
import csv
"""
Pillow: https://pillow.readthedocs.io/en/stable/reference/ImageDraw.html?highlight=draw_arc
csv: https://docs.python.org/2/library/csv.html
"""


def color_to_html(color):
    return '#{:06X}'.format((color[0] << 16) + (color[1] << 8) + color[2])


def is_bounding_pixel(origin, pixel):
    return (abs(pixel[0] - origin[0]) <= brush_size) and (abs(pixel[1] - origin[1]) <= brush_size)

def check_linear_path(start,end,step):
    pos = [0,0]
    subi = 1
    if start[0]==end[0]:
        pos[0] = start[0]
        subi = 1
    else:
        pos[1] = start[1]
        subi = 0
    #print("Testing range: {start[0]};{start[1]} - {end[0]};{end[1]}".format(start=start,end=end))
    #print("Constant is {}".format(start[not subi]))
    for i in range(start[subi] + step*8, end[subi], step):
        pos[subi] = i
        #print("Checking {}".format(pos))
        if color_to_html(im.getpixel( (pos[0],pos[1]) )) == node:
            return True
    return False

wall = "#1E1EE4"
node = "#FFFF00"
brush_size = 8
colored = []
colored_dirs = []

im = Image.open("./board_nodes.png")
draw = ImageDraw.Draw(im)

for y in range(0, im.height):
    for x in range(0, im.width):
        pixel = im.getpixel((x, y))
        if (color_to_html(pixel)) == node:
            skip = False
            for c in colored:
                if is_bounding_pixel(c, (x, y)):
                    skip = True
                    break
            if skip:
                continue
            print("Appending to list")
            colored.append([x, y])

print("List {}\nLen: {}".format(colored, colored.__len__()))

for p in colored:
    possible = [
        check_linear_path(p,[p[0],0],-1),
        check_linear_path(p,[im.width,p[1]],1),
        check_linear_path(p,[p[0],im.height],1),
        check_linear_path(p,[0,p[1]],-1)]
    colored_dirs.append(possible)


with open("../nodes.csv", "w") as file:
    writer = csv.DictWriter(
        file, fieldnames=["x", "y", "up", "right", "down", "left"])
    writer.writeheader()
    for i in range(0,colored.__len__()):
        c = colored[i]
        cd = colored_dirs[i]
        writer.writerow({
            "x": c[0]+(brush_size/2),
            "y": c[1]+(brush_size/2),
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
