import parse
from PIL import Image

points = []

while True:
    try:
        line = input()
    except:
        break

    point = parse.parse("position=<{:d}, {:d}> velocity=<{:d}, {:d}>", line).fixed
    points.append(point)




start = 10900

for s in range(start, start + 50):
    if s == start:
        min_x = min(points, key=lambda e: e[0]+ start*e[2])
        max_x = max(points, key=lambda e: e[0]+ start*e[2])
        min_y = min(points, key=lambda e: e[1]+ start*e[3])
        max_y = max(points, key=lambda e: e[1]+ start*e[3])

        min_x = min_x[0] + start*min_x[2]
        max_x = max_x[0] + start*max_x[2]
        min_y = min_y[0] + start*min_y[2]
        max_y = max_y[0] + start*max_y[2]
        print(min_x, max_x)

    sky = Image.new("RGB", (1024, 1024), (0, 0, 0))
    pixels = sky.load();

    for point in points:
        pixels[
                512*((point[0] + s*point[2]) - min_x)//(max_x - min_x + 1) + 256,
                512*((point[1] + s*point[3]) - min_y)//(max_y - min_y + 1) + 256,
                ] = (255, 255, 255)

    sky.save('day10/sky-{}.jpg'.format(s))
