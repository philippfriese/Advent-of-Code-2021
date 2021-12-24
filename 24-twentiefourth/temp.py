import math
runs = [
( 1,  15, 13),
( 1,  10, 16),
( 1,  12,  2),
( 1,  10,  8),
( 1,  14, 11),
(26, -11,  6),
( 1,  10, 12),
(26, -16,  2),
(26,  -9,  2),
( 1,  11, 15),
(26,  -8,  1),
(26,  -8, 10),
(26, -10, 14),
(26, -9, 10),
]


def single_run(inp, run, registers):
	registers['w'] = inp
	registers['x'] = 0
	registers['x'] = registers['x'] + registers['z']
	registers['x'] = registers['x'] % 26

	registers['z'] = int(math.floor(registers['z'] / run[0]))
	registers['x'] = registers['x'] + run[1]
	registers['x'] = registers['x'] == registers['w']

	registers['x'] = registers['x'] == 0
	registers['y'] = 0
	registers['y'] = registers['y'] + 25

	registers['y'] = registers['y'] * registers['x']
	registers['y'] = registers['y'] + 1
	registers['z'] = registers['z'] * registers['y']

	registers['y'] = 0
	registers['y'] = registers['y'] + registers['w']
	registers['y'] = registers['y'] + run[2]

	registers['y'] = registers['y'] * registers['x']
	registers['z'] = registers['z'] + registers['y']

	return registers


def single_run_optimised(inp, run, registers):
	registers['w'] = inp

	registers['x'] = registers['z'] % 26

	registers['z'] = int(registers['z'] / run[0])

	registers['x'] = registers['x'] + run[1]
	registers['x'] = registers['x'] == registers['w']
	registers['x'] = registers['x'] == 0

	registers['y'] = 25  * registers['x'] + 1

	registers['z'] = registers['z'] * registers['y']

	registers['y'] = registers['w'] + run[2]

	registers['y'] = registers['y'] * registers['x']
	registers['z'] = registers['z'] + registers['y']

	return registers


if __name__ == "__main__":
	
	program = []
	with open("data", "r") as infile:
		block = []
		for line in infile.readlines():
			if line.startswith("inp"):
				if len(block) > 0:
					program.append(block)
					block = []
			
			cmd = list(line.strip().split(" "))
			block.append(cmd)
		program.append(block)

	for b in program:
		print(list(map(lambda x: list(map(lambda y: f"{y:>3}", x)), b)))
	exit()

	inp = [1,2,3,4,5,6,7,8,9,1,2,3,4,5]
	registers = {'x': 0, 'y': 0, 'z': 0, 'w': 0}
	for run, i in zip(runs, inp):
		registers = single_run_optimised(i,run,registers)

	print(registers)