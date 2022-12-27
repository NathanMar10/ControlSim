import draw_tools
import input_tools
import PIL
import PIL.Image
import PIL.ImageDraw

import pygame


def main():

    pygame.init()
    clock = pygame.time.Clock()

    SCREEN_SIZE = (800, 800)
    SCREEN_NAME = "I LOVE DR. SAMI AINANE"

    root = pygame.display.set_mode(SCREEN_SIZE)
    pygame.display.set_caption(SCREEN_NAME)


    exit = False

    while not exit: 
        inFile = open("Outputs/output.txt", 'r', 1)
        for line in inFile:
            root.fill((0, 0, 0))
            clock.tick(10)
            input_tools.read_frame(line, root)
            pygame.display.update()
        inFile.close()
        
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                exit = True
 



if __name__ == "__main__":
    main()