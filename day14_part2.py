# Part two of day 2, for each generation print the result of the robot
# and then feed the output to grep with command `python3 day14.py | grep -E '^.+#######.+$'`
# which finds the lines with possible line of the tree and then manually check the output

WIDTH = 101
HEIGHT = 103


class Robot:
    def __init__(self, vx, vy, px, py):
        self.vx = vx
        self.vy = vy
        self.px = px
        self.py = py


def pretty_print(positions, robots, generation):
    for i in range(len(positions)):
        for j in range(len(positions[i])):
            positions[i][j] = '.'

    for robot in robots:
        positions[robot.py][robot.px] = "#"

    for row in positions:
        print("".join(row), generation)


file_contents = open("day14.in", "r")
positions = [["." for _ in range(WIDTH)] for _ in range(HEIGHT)]
robots = []

for line in file_contents:
    p, v = line.split(" ")

    px, py = p.split("=")[1].split(",")
    vx, vy = v.split("=")[1].split(",")

    robots.append(Robot(int(vx), int(vy), int(px), int(py)))

for i in range(10000):
    pretty_print(positions, robots, i)

    for robot in robots:
        robot.px = (robot.px + robot.vx) % WIDTH
        robot.py = (robot.py + robot.vy) % HEIGHT
