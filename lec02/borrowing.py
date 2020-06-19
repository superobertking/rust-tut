import copy

h = 'hello'
h2 = h
assert(h is h2)

h = [1, 2, 3]
h2 = h
assert(h2 is h)
h2[2] = 9
assert(h[2] == 9)

h = [1, 2, 3]
h2 = h[:]
h2[2] = 9
assert(h[2] == 3)

h = [1, 2, 3]
h2 = copy.copy(h)
h2[2] = 9
assert(h[2] == 3)

h = [[1, 2, 3], [4, 5, 6]]
h2 = copy.copy(h)
h2[0][2] = 9
assert(h[0][2] == 9)

h = [[1, 2, 3], [4, 5, 6]]
h2 = copy.deepcopy(h)
h2[0][2] = 9
assert(h[1][2] == 3)
