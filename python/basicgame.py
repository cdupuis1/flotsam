#
# Based on simple example from https://www.youtube.com/watch?v=dz9_-2G6o3o
#
# Requires pygame library
#
import pygame
import sys
import random
import math

# Initialize pygame engine
pygame.init()


radius = random.randint(20, 200)
score = 0
font = pygame.font.Font(None, 30) # 30pt font

# Shows if we are clicking within the circle.
#
# Returns True if we are clicking within the circle, False otherwise
def check_within_circle() -> bool:
    # Note mouse_pos[0] is the x mouse position and mouse_pos[1] is the y position
    mouse_pos = pygame.mouse.get_pos()

    # We square the difference to avoid negative numbers
    radius_dif = math.sqrt(((mouse_pos[0] - circle_pos[0])**2 + (mouse_pos[1] - circle_pos[1])**2))
    if radius_dif < radius:
        return True
    else:
        return False

# Setup screen for our game to 720p
screen = pygame.display.set_mode((1280, 720))
circle_pos = (random.randint(0, 1280), random.randint(0, 720))

# Main game event loop
while True:
    events = pygame.event.get()
    for event in events:
        # We explicitly need to check for a quit event to be able to close the window
        if event.type == pygame.QUIT:
            pygame.quit()
            sys.exit()
        if event.type == pygame.MOUSEBUTTONDOWN:
            # If we are within the circle, generate a new circle
            if check_within_circle() == True:
                circle_pos = (random.randint(0, 1280), random.randint(0, 720))
                score += 1
                radius = random.randint(20, 200)
            

    # Do background fill first as everything after will be drawn on top of this
    screen.fill('blue')

    # Draw the circle
    pygame.draw.circle(screen, 'yellow', circle_pos, radius)

    # Draw the score
    score_surface = font.render(f'SCORE  {score}', True, 'white')
    screen.blit(score_surface, (20, 20))

    # Actually draw the screen
    pygame.display.update()




