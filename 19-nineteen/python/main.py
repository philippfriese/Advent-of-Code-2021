import itertools
import numpy as np
import scipy.spatial.distance
import datetime

import matplotlib

matplotlib.use('TkAgg')


# Note
# I was not capable enough to solve day 19 on my own.
# This solution below is essentially the solution from https://github.com/kupuguy aka u/kupuguy
# However, I tried to make it a bit faster

class Scanner:
    def __init__(self, beacons: list[np.array]):
        self.beacons = beacons
        self.mat = scipy.spatial.distance.cdist(self.beacons, self.beacons)
        # calculate distances between all beacon positions
        self.distances = set(itertools.chain(*self.mat))
        # gather all distances in a set

        self.position = np.array([0, 0, 0])

    def overlap(self, other) -> int:
        return len(self.distances & other.distances)

    def align(self, other):
        # determine all candidates, i.e. beacon pairs from self and other
        # and descending sort by the number of shared inter-beacon distances
        candidates = sorted(
            [(_sb, _ob, len(set(self.mat[si, :]) & set(other.mat[oi, :])))
             for (si, _sb) in enumerate(self.beacons)
             for (oi, _ob) in enumerate(other.beacons)],
            key=lambda t: t[2],
            reverse=True
        )

        # for all two beacons (pairwise iterates with a sliding-window)
        # try to determine the matrix from lhs -> rhs
        #  this will fail if get_matrix returns None aka if there is now way to transform lhs->rhs
        #  this happens if lhs and rhs do not share the absolute value of all three coordinates
        # if a matrix is found:
        # - determine the scanner position relative to other via the distance vector between a rotated self beacon
        #   and an 'other' beacon
        # - rotate all self beacons by matrix and add the position offset
        for lhs, rhs in itertools.pairwise(candidates):
            if isinstance(matrix := get_matrix(lhs[1] - rhs[1], lhs[0] - rhs[0]), np.matrix):
                self.position = candidates[0][1] - np.array(candidates[0][0] * matrix)[0]
                self.beacons = list(map(lambda x:
                                        np.array(x * matrix)[0] + self.position,
                                        self.beacons))
                return


def get_matrix(lhs, rhs):
    # get the rotation matrix by checking if all three coordinates from lhs can be permuted & sign-swapped from rhs
    x = [
        (abs(lhs[0]) == abs(rhs[0])) * ((np.sign(lhs[0]) == np.sign(rhs[0])) * 2 - 1),
        (abs(lhs[1]) == abs(rhs[0])) * ((np.sign(lhs[1]) == np.sign(rhs[0])) * 2 - 1),
        (abs(lhs[2]) == abs(rhs[0])) * ((np.sign(lhs[2]) == np.sign(rhs[0])) * 2 - 1),
    ]
    y = [
        (abs(lhs[0]) == abs(rhs[1])) * ((np.sign(lhs[0]) == np.sign(rhs[1])) * 2 - 1),
        (abs(lhs[1]) == abs(rhs[1])) * ((np.sign(lhs[1]) == np.sign(rhs[1])) * 2 - 1),
        (abs(lhs[2]) == abs(rhs[1])) * ((np.sign(lhs[2]) == np.sign(rhs[1])) * 2 - 1),
    ]
    z = [
        (abs(lhs[0]) == abs(rhs[2])) * ((np.sign(lhs[0]) == np.sign(rhs[2])) * 2 - 1),
        (abs(lhs[1]) == abs(rhs[2])) * ((np.sign(lhs[1]) == np.sign(rhs[2])) * 2 - 1),
        (abs(lhs[2]) == abs(rhs[2])) * ((np.sign(lhs[2]) == np.sign(rhs[2])) * 2 - 1),
    ]
    # matrix is only valid if all three coordinates have a corresponding rotation entry
    return np.matrix([x, y, z]) if (sum(map(abs, x + y + z)) == 3) else None


def main(lines):
    data: list[Scanner] = []
    datum = []
    for line in lines:
        if len(line.strip()) == 0: continue
        if line.startswith("---"):
            if len(datum) > 0:
                data.append(Scanner(datum))
                datum = []
        else:
            coords = line.split(",")
            datum.append(np.array([int(coords[0].strip()),
                                   int(coords[1].strip()),
                                   int(coords[2].strip())]))
    data.append(Scanner(datum))

    aligned_scanners = [data.pop(0)]

    while data:
        unknown, aligned = max([(unknown, aligned)
                                for unknown in data for aligned in aligned_scanners],
                               key=lambda t: t[0].overlap(t[1]))

        unknown.align(aligned)
        aligned_scanners.append(unknown)
        data.remove(unknown)

    beacons = set()
    for scanner in aligned_scanners:
        beacons |= set(map(tuple, scanner.beacons))

    return len(beacons)
    # print(max([scipy.spatial.distance.cityblock(lhs.position,rhs.position)
    #            for lhs, rhs in itertools.product(aligned_scanners, aligned_scanners)]))
    # viz(aligned_scanners)


if __name__ == '__main__':
    with open("../data", "r") as infile:
        lines = infile.readlines()

    main(lines)
