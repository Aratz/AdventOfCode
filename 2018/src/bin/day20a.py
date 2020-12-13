from functools import reduce

opp = {
        'W': 'E',
        'E': 'W',
        'N': 'S',
        'S': 'N'
        }

def reduce_path(s):
    return reduce(lambda s, c: c if not s
            else (s + c if opp[c] != s[-1] else s[:-1]), s, '')

def max_path(i, s):
    start = i
    words = []
    current_words = ['']
    while i < len(s):
        if s[i] == '|':
            current_words = [w + s[start:i] for w in current_words]
            words.extend(current_words)
            start = i + 1
            current_words = ['']
        elif s[i] == '(':
            new_i, new_words = max_path(i + 1, s)
            current_words = [w + s[start:i] + nw
                    for w in current_words for nw in new_words]
            start = new_i + 1
            i = new_i
        elif s[i] == ')':
            current_words = [w + s[start:i] for w in current_words]
            words.extend(current_words)
            return (i, words)
        i += 1
    current_words = [w + s[start:i] for w in current_words]
    words.extend(current_words)
    return (i, words)

def max_path_len(i, s):
    start = i
    words = []
    current_words = [0]
    while i < len(s):
        if s[i] == '|':
            current_words = [wl + len(reduce_path(s[start:i])) for wl in current_words]
            words.extend(current_words)
            start = i + 1
            current_words = [0]
        elif s[i] == '(':
            new_i, new_words = max_path_len(i + 1, s)
            current_words = [wl + len(reduce_path(s[start:i])) + new_words
                    for wl in current_words]
            start = new_i + 1
            i = new_i
        elif s[i] == ')':
            current_words = [wl + len(reduce_path(s[start:i])) for wl in current_words]
            words.extend(current_words)
            return (i, max(words))
        i += 1
    current_words = [wl + len(reduce_path(s[start:i])) for wl in current_words]
    words.extend(current_words)
    return (i, max(words))


s = input()[1:-1]
print(max_path_len(0, s)[1])
