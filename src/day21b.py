s = set()
a, b, c, d = (65536, 0, 14906355, 0)
while True:
    while True:
        d = a & 255 #8
        c += d #9
        c &= 16777215 #10
        c *= 65899 #11
        c &= 16777215 #12
        if a < 256: #13
            break
        d = a//256
        a = d #26
    print(c) #28 halting condition
    if c in s:
        break
    s.add(c)
    a = c | 65536#6
    c = 14906355  #7
