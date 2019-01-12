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


brush_size = 8
colored = []

im = Image.open("./board_nodes.png")
draw = ImageDraw.Draw(im)

for y in range(0, im.height):
    for x in range(0, im.width):
        pixel = im.getpixel((x, y))
        if (color_to_html(pixel)) == "#FFFF00":
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

with open("../nodes.csv", "w") as file:
    writer = csv.DictWriter(file,fieldnames=["x","y","up","right","down","left"])
    writer.writeheader()
    for p in colored:
        writer.writerow({
            "x": p[0]+(brush_size/2),
            "y": p[1]+(brush_size/2),
            "up": True,
            "right": True,
            "down": True,
            "left": True
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