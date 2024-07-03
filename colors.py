colors = [
    "CC2F00",
    "DB6600",
    "E39E00",
    "76B80D",
    "007668",
    "006486",
    "007CB5",
    "465AB2",
    "6D47B1",
    "873B9C",
]

def h(num: int) -> str:
    return f"{num:0{2}x}"


def n(num: int) -> int:
    return (num // 6) + 11


for c in colors:
    r, g, b = c[0:2], c[2:4], c[4:6]
    r, g, b = int(r, 16), int(g, 16), int(b, 16)

    r, g, b = n(r), n(g), n(b)

    c = h(r) + h(g) + h(b)

    print(f"#{c}")
