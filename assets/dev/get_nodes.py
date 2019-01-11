from PIL import Image
im = Image.open("./board_nodes.png")
bnw = im.convert(mode="L")
mask = im[1].point(lambda i:i==255 and 255)
