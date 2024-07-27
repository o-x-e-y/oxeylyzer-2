rain1 = [
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

rain2 = [
    "ff0000",
    "ff7f00",
    "ffff00",
    "7fff00",
    "00ff00",
    "00ff7f",
    "00ffff",
    "007fff",
    "0000ff",
    "7f00ff",
]

microsoft = [
    "fff100",
	"ff8c00",
	"e81123",
	"ec008c",
	"68217a",
	"00188f",
	"00bcf2",
	"00b294",
	"009e49",
	"bad80a",
]

gruv = [
    "1d2021",
    "a89984",
    "458588",
    "cc241d",
    "fabd2f",
    "d79921",
    "98971a",
    "d3869b",
    "689d6a",
    "b16286",
]

pastel = [
    "9e0142",
    "d53e4f",
    "f46d43",
    "fdae61",
    "fee08b",
    "e6f598",
    "abdda4",
    "66c2a5",
    "3288bd",
    "5e4fa2",
]

pastel5 = [
    "#f09898",
	"#fff2cc",
	"#d9ead3",
	"#a0dbe6",
	"#d9d2e9",
]

def h(num: int) -> str:
    return f"{num:0{2}x}"


def n(num: int) -> int:
    return min(int((num * 1.14)), 255)


def p(colors: list[str]):
    for c in colors:
        c = c.removeprefix("#")
        r, g, b = c[0:2], c[2:4], c[4:6]
        r, g, b = int(r, 16), int(g, 16), int(b, 16)

        r, g, b = n(r), n(g), n(b)

        c = h(r) + h(g) + h(b)

        print(f"\"#{c}\",")


def fmt(colors: list[str]):
    for c in colors:
        c = c.removeprefix("#")

        print(f"\"#{c.lower()}\",")


def with_reverse(colors: list[str]):
    for c in colors:
        fmt(c)

    for c in reversed(colors):
        fmt(c)


if __name__ == "__main__":
    p(pastel)