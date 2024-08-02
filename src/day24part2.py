from scipy.optimize import fsolve

hailstones = [
	([19, 13, 30], [-2, 1, -2]),
]

def f(x):
	rock_p = x[0:3]
	rock_v = x[3:6]
	t = x[6:]

	y = 0
	for ti, hailstone in enumerate(hailstones):
		for i in range(3):
			ro = rock_p[i] + rock_v[i] * t[ti]
			ho = hailstone[0][i] + hailstone[1][i] * t[ti]
			y += abs(ro - ho)

	print(x, y)
	return y

print("start")
y = fsolve(f, [1.] * (2*3 + len(hailstones)))
print("done")
print(y)
