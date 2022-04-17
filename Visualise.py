from PIL import Image
import pygame
import sys

with open(sys.argv[1]) as file:
    data = [[int(j) for j in i.split()] for i in file.read().splitlines()]

width = max(i[0] for i in data) - min(i[0] for i in data)
height = max(i[1] for i in data) - min(i[1] for i in data)

data_size = max(i[2] for i in data)
data_mean = sum(i[2] for i in data) / len(data)
threshold = data_mean * 2


pygame.init()
screen = pygame.display.set_mode((width, height))

while True:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            pygame.quit()
            quit()
        elif event.type == pygame.MOUSEWHEEL:
            threshold += data_mean / 20 * event.y
    for x, y, val in data:
        pygame.draw.rect(screen, (max(0, min(255, int(val / threshold * 255))),) * 3, (x, y, 1, 1))
    pygame.display.flip()
