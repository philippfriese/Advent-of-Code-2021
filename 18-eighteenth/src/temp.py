import numpy as np
import math
from typing import Optional

class Number:
	def __init__(self, s: Optional[str]=None):
		self.n = []
		self.p = []

		if isinstance(s,str):
			opend = 0
			closed = 0
			for c in s:
				if c == '[': opend += 1
				if c == ']': closed += 1
				if c.isdigit():
					self.n.append(int(c))
					self.p.append(opend-closed)

	def __add__(self, other):
		n = self.n + other.n
		p = list(map(lambda x: x+1, self.p + other.p)) 
		num = Number()
		num.n = n
		num.p = p
		return num

	def explode(self):
		explode_idxs = list(filter(lambda t: t[1] > 4, enumerate(self.p)))

		for _idx in range(len(explode_idxs)//2):
			l,r = explode_idxs[_idx*2], explode_idxs[_idx*2+1]

			l_val = self.n[l[0]]
			self.n[l[0]] = 0
			

			if l[0] > 0:
				self.n[l[0]-1] += l_val
			if r[0]+1 <  len(self.n):
				self.n[r[0]+1] += self.n[r[0]]

			r_val = self.n.pop(r[0])
			self.p[r[0]] -= 1
			_ = self.p.pop(l[0])	
			return True
		return False


	def split(self):
		split_idxs = list(filter(lambda t: t[1] > 9, enumerate(self.n)))
		for idx,_ in split_idxs:

			self.n.insert(idx,self.n[idx])
			self.n[idx] = math.floor(self.n[idx]/2)
			self.n[idx+1] = math.ceil(self.n[idx+1]/2)

			self.p.insert(idx,self.p[idx])
			self.p[idx] +=1
			self.p[idx+1] +=1
			return True
		return False


	def __str__(self):
		# return f"{self.n}\n{self.p}\n"
		# return f"{self.n}"
		return f"{''.join(map(str, self.n))}\n{''.join(map(str, self.p))}"


	def reduce(self):
		# print("Reducing")
		changed = True
		c = 0
		while(changed):
			c += 1
			changed = False
			res = self.explode()
			if res:
				# print(f"Explode: {self}")
				changed |= res
				continue

			res2 = self.split()
			if res2:
				# print(f"Split  : {self}")
				changed |= res2
				continue
			if c > 20:
				break



	def magnitude(self):
		c = 0


		for i in range(len(self.n)//2):
			ln,lp = self.n[i*2], self.p[i*2]
			rn,rp = self.n[i*2+1], self.p[i*2+1]


			c += ln*(3**lp) + (rn*2 + rn*(3**(rp-1)))
		return c

if __name__ == "__main__":

	with open("../test","r") as file:
		line = file.readline()
		n = Number(line)
		
		for line in file.readlines():
			n2 = Number(line)
			# print(f"{n} + {n2}")
			n = n+n2
			n.reduce()
			# print(n)
		print(n)
		print(n.magnitude())
	# Number
