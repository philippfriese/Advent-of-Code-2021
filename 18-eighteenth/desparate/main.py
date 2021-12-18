import numpy as np
import math
import itertools
from typing import Optional, Union
import json
import re

class Number:
    def __init__(self, s:str):
        self.s: str = s

        self._split_pattern = re.compile(r"\d{2}")
        self._number_pattern = re.compile(r"\[(\d+),(\d+)\]")
    def __add__(self, other):
        return Number(f"[{self.s.strip()},{other.s.strip()}]")

    def __str__(self):
        return f"{self.s}"

    def magnitude(self) -> int:

        found = True
        s = self.s
        while found:
            found = False
            for match in re.finditer(self._number_pattern, s):
                found = True
                left = int(match.group(1))
                right = int(match.group(2))

                lhs = s[:match.start()]
                rhs = s[match.end():]
                mhs = left*3+right*2
                s = lhs + f"{mhs}" + rhs
                break
        return int(s)

    def split(self) -> bool:
        #print("Split")
        offset = 0
        changed = False
        for match in re.finditer(self._split_pattern, self.s):
            #print(f"Splitting {match.group(0)}")
            d = int(str(match.group(0)))
            inner = f"[{math.floor(d/2.)},{math.ceil(d/2.)}]"
            self.s = f"{self.s[0:match.start()+offset]}{inner}{self.s[match.end()+offset:]}"
            offset += len(inner)-len(match.group(0))
            changed = True
            break
        return changed


    def explode(self) -> bool:
        #print("Explode")
        delta = []
        changed = False
        opend = 0
        closed = 0
        for i, c in enumerate(self.s):
            if c == '[': opend += 1
            if c == ']': closed += 1
            delta.append(opend - closed)


        numbers = re.finditer(self._number_pattern, self.s)

        offset = 0
        for i in numbers:
            if delta[i.start()] > 4:
                #print(f"Exploding {i.group(0)}")
                changed = True
                match_len = (i.span()[1]-i.span()[0])
                offset -= match_len-1

                (l_pos, l_value) = i.start(), int(i.group(1))
                (r_pos, r_value) = i.end(), int(i.group(2))

                self.s = self.s[0:l_pos] + f"0" + self.s[r_pos:]
                #print(f"removed explodepair")
                #print()
                #print(self.s)
                for c_i in reversed(range(l_pos-1)):
                    l_idx = c_i
                    c = self.s[c_i]
                    if c.isdigit() and self.s[c_i-1].isdigit():
                        c = self.s[c_i-1:c_i+1]
                        l_idx = c_i-1

                    if c.isdigit():
                        mhs = f"{int(c)+int(l_value)}"
                        lhs = self.s[0:l_idx]
                        rhs = self.s[l_idx+len(c):]
                        self.s = lhs + mhs + rhs
                        offset += len(mhs)
                        break
                #print(f" add lhs")
                #print(self.s)
                #print()
                for c_i in range(len(self.s) - (r_pos-1)):
                    r_idx = r_pos+c_i+offset

                    c = self.s[r_idx]
                    if c.isdigit() and self.s[r_idx + 1].isdigit():
                        c = self.s[r_idx:r_idx + 2]
                    if c.isdigit():
                        mhs = f"{int(c)+int(r_value)}"
                        lhs = self.s[0:r_idx]
                        rhs = self.s[r_idx + len(c):]
                        self.s = lhs + mhs + rhs
                        offset += len(rhs)
                        break

                #print(f" add rhs")
                #print(self.s)
                #print()
                break

        return changed

    def reduce(self):
        changed = True
        #print(self.s)
        while changed:
            changed = False
            if self.explode():
                changed = True
                #print(self.s)
                continue
            if self.split():
                changed = True
                #print(self.s)
                continue




if __name__ == "__main__":
    with open("../data", "r") as file:
        n1 = Number(file.readline().strip())
        numbers = []
        for line in file.readlines():
            numbers.append(Number(line.strip()))

        max_mag = 0
        for i in range(len(numbers)):
            for j in range(len(numbers)):
                if i == j: continue
                n = numbers[i] + numbers[j]
                n.reduce()
                max_mag = max(max_mag, n.magnitude())

        print(max_mag)