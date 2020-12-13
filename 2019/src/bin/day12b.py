import parse
while True:
    try:
        print(*parse.parse("<x={:d}, y={:d}, z={:d}>", input()).fixed)
    except EOFError:
        break
