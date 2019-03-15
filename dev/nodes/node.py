from vector import vec
from math import inf


class node:
    pos = vec(-1, -1)
    connections = [(-1, inf)]*4  # ID,Distance (Up,Right,Down,Left)

    def __init__(self, _pos):
        if not isinstance(_pos, vec):
            _pos = vec(_pos[0], _pos[1])
        self.pos = _pos

    def __str__(self):
        return "{} [{};{};{};{}]".format(self.pos.__str__(), self.connections[0], self.connections[1], self.connections[2], self.connections[3])

    def distance(self, other):
        return self.pos.distance(other.pos)

    def set_connection(self, side, id, distance):
        sides = ["up", "right", "down", "left"]
        if side < 0 or side > 3:
            raise IndexError("Side was {} ( [0;4] expected ) ".format(side))
        print("Setting connection for side {} with {} separated by {} units".format(
            sides[side], id, distance))
        self.connections[side] = (id, abs(distance))
