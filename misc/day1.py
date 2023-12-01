words = 'one two three four five six seven eight nine'.split()
digmap = [(w, i+1) for i, w in enumerate(words)] + [(str(i+1), i+1) for i in range(9)]

with open('input.txt', 'r') as f:
    tot = 0
    for line in f.readlines():
        minind = 10**9
        maxind = -10**9
        mind = -1
        maxd = -1

        for w, v in digmap:
            if w in line:
                idxw = line.index(w)
                if idxw < minind:
                    minind = idxw
                    mind = v
                idxw2 = line.rindex(w)
                if idxw2 > maxind:
                    maxind = idxw2
                    maxd = v

        tot += 10 * mind + maxd
    print(tot)