"""
Input_Tools.py:

Contains the tools to read the information given to the Python script from the Rust program.
For the time being, this really just be reading a single line of the input file and then grabbing the numbers 
    out of it for the positions. Eventually, this will be a much more complicated situation that like reads JSON
Right now, I'm most interested in testing the Rust component, and the Python is just a way to visualize wtf is going on!

Note to Future Reader: If the documentation in this project is subpar, direct your compaints to thenatemartin@gmail.com. 
    It is literally my goal to not suck at documentation in this project, you hear?

"""
import re
import PIL
import pygame
from pygame import gfxdraw

def read_frame(line, root):
    regex = "\((\d+\.\d+), (\d+\.\d+)\), \((\d+\.\d+), (\d+\.\d+)\), \((\d+\.\d+), (\d+\.\d+)\)"
    out = re.search(regex, line)
    doubles = []
    for entry in out.groups():
        doubles.append(int(float(entry)))

    gfxdraw.line(root, doubles[0], doubles[1], doubles[2], doubles[3], (255, 255, 255))
    gfxdraw.line(root, doubles[2], doubles[3], doubles[4], doubles[5], (255, 255, 255))

    gfxdraw.aacircle(root, doubles[0], doubles[1], 15, (255, 0, 0))
    gfxdraw.filled_circle(root, doubles[0], doubles[1], 15, (255, 0, 0))
    gfxdraw.aacircle(root, doubles[2], doubles[3], 15, (0, 255, 0))
    gfxdraw.filled_circle(root, doubles[2], doubles[3], 15, (0, 255, 0))
    gfxdraw.aacircle(root, doubles[4], doubles[5], 15, (0, 0, 255))
    gfxdraw.filled_circle(root, doubles[4], doubles[5], 15, (0, 0, 255)) 