from abc import ABC
import pygame

pygame.init()

win = pygame.display.set_mode((1024, 600))
pygame.display.set_caption('TramPamPam')
# pygame.display.set_icon()


x = 50
y = 520
widht = 60
height = 71
speed = 5
isJump = False
jumpCount = 10

left = False
right = False
aniCount = 0
clock = pygame.time.Clock()
sound = pygame.mixer.music
sound.set_volume(0.2)

lastMove = 'right'
walk_right = [pygame.image.load('img/pygame_right_1.png'),
              pygame.image.load('img/pygame_right_2.png'),
              pygame.image.load('img/pygame_right_3.png'),
              pygame.image.load('img/pygame_right_4.png'),
              pygame.image.load('img/pygame_right_5.png'),
              pygame.image.load('img/pygame_right_6.png'),
              ]

walk_left = [pygame.image.load('img/pygame_left_1.png'),
             pygame.image.load('img/pygame_left_2.png'),
             pygame.image.load('img/pygame_left_3.png'),
             pygame.image.load('img/pygame_left_4.png'),
             pygame.image.load('img/pygame_left_5.png'),
             pygame.image.load('img/pygame_left_6.png'),
             ]
bg = pygame.image.load('img/pygame_bg_3.jpg')
playerStand = pygame.image.load('img/pygame_idle.png')


class Base_Scene(ABC):
    def __init__(
            self,
            back_ground,
            another_pass_hero,
            hero,
            enemies,
            tile_map=None,
            **kwargs,
    ):
        pass


class Base_hero(ABC):

    def __init__(
            self,
            walk_left,
            walk_right,
            playerStand,
            weapon,
            controll=None,
            **kwargs,
    ):
        pass


class Bullet:
    def __init__(self, x, y, radius, color, facing):
        self.x = x
        self.y = y
        self.radius = radius
        self.color = color
        self.facing = facing
        self.vel = 8 * facing

    def draw(self, win):
        pygame.draw.circle(win, self.color, (self.x, self.y), self.radius)


def drow_window():
    # win.fill((0,0,0))
    win.blit(bg, (0, 0))
    global aniCount
    if aniCount + 1 >= 30:
        aniCount = 0
    if left:
        win.blit(walk_left[aniCount // 5], (x, y))
        aniCount += 1
    elif right:
        win.blit(walk_right[aniCount // 5], (x, y))
        aniCount += 1
    else:
        win.blit(playerStand, (x, y))
    for bullet in bullets:
        bullet.draw(win)

    # pygame.draw.rect(win, (0,0,255), (x, y, widht,height))
    pygame.display.update()


bullets = []
run = True
while (run):
    # pygame.time.delay(50)
    clock.tick(30)
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            run = False
    for bullet in bullets:
        if bullet.x < 1024 and bullet.x > 0:
            bullet.x += bullet.vel
        else:
            bullets.pop(bullets.index(bullet))
    keys = pygame.key.get_pressed()
    if keys[pygame.K_f]:  # Bullet
        # sound.load('media/fire1.wav')
        sound.play()
        if lastMove == 'right':
            facing = 1
        else:
            facing = -1

        if len(bullets) < 5:
            bullets.append(
                Bullet(
                    round(x + widht // 2),
                    round(y + height // 2),
                    5,
                    (255, 0, 0),
                    facing,
                )
            )

    if keys[pygame.K_LEFT] and x > 5:
        x -= speed
        left = True
        right = False
    elif keys[pygame.K_RIGHT] and x < 1024 - widht - 5:
        x += speed
        right = True
        left = False
        lastMove = 'right'
    else:
        right = False
        left = False
        aniCount = 0
        lastMove = 'left'

    if not (isJump):


        if keys[pygame.K_UP] and y > 5:
            isJump = True
            sound.load('media/0_jump.wav')
            sound.play()
        # if keys[pygame.K_DOWN]and y<600-height-5:
        #     y+=speed
        if keys[pygame.K_SPACE]:
            isJump = True
    else:
        if jumpCount >= -10:
            if jumpCount < 0:
                y += (jumpCount ** 2) / 2
            else:
                y -= (jumpCount ** 2) / 2
            jumpCount -= 1
        else:
            isJump = False
            jumpCount = 10
    drow_window()

pygame.quit()
