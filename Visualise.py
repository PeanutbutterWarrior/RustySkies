from PIL import Image
import sys

with open(sys.argv[1]) as file:
    data = [[int(j) for j in i.split()] for i in file.read().splitlines()]

width = max(i[0] for i in data) - min(i[0] for i in data)
height = max(i[1] for i in data) - min(i[1] for i in data)

data_size = max(i[2] for i in data)
data_mean = sum(i[2] for i in data) / len(data)

image = Image.new('L', (width + 1, height + 1))

print(width, height)

for x, y, val in data:
    image.putpixel((x, y), min(255, int(val * (127.5 / data_mean))))

image.save('output/out.png')

