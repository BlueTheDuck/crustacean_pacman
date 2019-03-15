from math import sqrt, pow


class vec:
    x, y = 0, 0

    def __init__(self, _x, _y):
        self.x = _x
        self.y = _y

    def __add__(self, other):
        if isinstance(other, vec):
            return vec(self.x + other.x,
                       self.y + other.y)
        if isinstance(other, int) or isinstance(other, float):
            return vec(self.x + other,
                       self.y + other)

    def __sub__(self, other):
        if isinstance(other, vec):
            return self.__add__(vec(-other.x, -other.y))
        if isinstance(other, int) or isinstance(other, float):
            return self.__add__(-other)

    def __mul__(self, other):
        if isinstance(other, vec):
            return vec(self.x * other.x,
                       self.y * other.y)
        if isinstance(other, int) or isinstance(other, float):
            return vec(self.x * other,
                       self.y * other)

    def __str__(self):
        return "({};{})".format(self.x, self.y)

    def distance(self, other):
        if not isinstance(other, vec):
            return self
        (dx, dy) = (self.x - other.x, self.y - other.y)
        if dx < 0:
            dx = -dx
        if dy < 0:
            dy = -dy
        return sqrt(pow(dx, 2) + pow(dy, 2))

    def to_tuple(self):
        return (self.x, self.y)


if __name__ == '__main__':
    P1 = vec(0, 0)
    P2 = vec(1, 1)
    assert(P1.x == 0)
    assert(P1.y == 0)
    P1 = P1 + P2  # (1;1)
    assert(P1.x == 1)
    assert(P1.y == 1)
    P1 = P1*2
    assert(P1.x == 2)
    assert(P1.y == 2)
    P1 = P1 * P1
    assert(P1.x == 4)
    assert(P1.y == 4)
    P1 = P1 - 1
    P1 = P1 - P2
    assert(P1.x == 2)
    assert(P1.y == 2)
    P2 += 2
    P2.y += 1
    P1 -= 2
    assert(P1.distance(P2) == 5)
    P1 = vec(0, 0)
    triangles = [(3, 4, 5), (5, 12, 13), (8, 15, 17), (7, 24, 25)]
    for i in range(0, 4):
        tri = triangles[i]
        print("({},{},{})".format(tri[0], tri[1], tri[2]))
        P2 = vec(tri[0], tri[1])
        distance = P1.distance(P2)
        assert(tri[2] == distance)

    print("All tests passed successfully")
