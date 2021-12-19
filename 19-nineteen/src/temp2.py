import numpy as np
import math
from typing import Optional, Union
import json



class Number:
	def __init__(self, s: Union[str,list]):

		if isinstance(s,str):
			self.values = json.loads(s)
		else:
			self.values = s

	def __add__(self, other):
		n = self.n + other.n
		return Number(self.values + other.values)

	def split(self):
		self.values = split(self.values)
	# def explode(self):
	# 	explode_idxs = list(filter(lambda t: t[1] > 4, enumerate(self.p)))

	# 	for _idx in range(len(explode_idxs)//2):
	# 		l,r = explode_idxs[_idx*2], explode_idxs[_idx*2+1]

	# 		l_val = self.n[l[0]]
	# 		self.n[l[0]] = 0
			

	# 		if l[0] > 0:
	# 			self.n[l[0]-1] += l_val
	# 		if r[0]+1 <  len(self.n):
	# 			self.n[r[0]+1] += self.n[r[0]]

	# 		r_val = self.n.pop(r[0])
	# 		self.p[r[0]] -= 1
	# 		_ = self.p.pop(l[0])	
	# 		return True
	# 	return False

	def __str__(self):
		# return f"{self.n}\n{self.p}\n"
		# return f"{self.n}"
		return f"{self.values}"

def split(number: Union[list,int]) -> Union[list,int]:
	if isinstance(number, int): return number
	l,r = number
	if isinstance(l,int) and l > 9:
		return [[math.floor(l/2), math.ceil(l/2)], r]
	if isinstance(r,int) and r > 9:
		return [l,[math.floor(r/2), math.ceil(r/2)]]
	return [split(l),split(r)]


def explode(number: Union[list,int], depth=0) -> Union[list,int]:
	if isinstance(number, int): return number
	l,r = number

	if depth >= 4:
		if l isinstance(list): # unpack l
			# [[1,2],3] -> [0,4]
			# [1,[1,[1,[1,2]]],[1,2]] -> []
			return [0,]

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




	# def reduce(self):
	# 	# print("Reducing")
	# 	changed = True
	# 	c = 0
	# 	while(changed):
	# 		c += 1
	# 		changed = False
	# 		res = self.explode()
	# 		if res:
	# 			# print(f"Explode: {self}")
	# 			changed |= res
	# 			continue

	# 		res2 = self.split()
	# 		if res2:
	# 			# print(f"Split  : {self}")
	# 			changed |= res2
	# 			continue
	# 		if c > 20:
	# 			break



	# def magnitude(self):
	# 	c = 0


	# 	for i in range(len(self.n)//2):
	# 		ln,lp = self.n[i*2], self.p[i*2]
	# 		rn,rp = self.n[i*2+1], self.p[i*2+1]


	# 		c += ln*(3**lp) + (rn*2 + rn*(3**(rp-1)))
	# 	return c

if __name__ == "__main__":

	with open("../test","r") as file:
		line = file.readline()
		n = Number(line)
		print(n)
		n.split()
		print(n)
		
		# for line in file.readlines():
		# 	n2 = Number(line)
		# 	# print(f"{n} + {n2}")
		# 	n = n+n2
		# 	n.reduce()
		# 	# print(n)
		# print(n)
		# print(n.magnitude())
	# Number
