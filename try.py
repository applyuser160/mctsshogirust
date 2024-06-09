import rustshogi
from datetime import datetime

s = datetime.now()

l = rustshogi.goresult(
    "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL",
    "b",
    "-",
    "1",
    100
)

e = datetime.now()

print(e - s)

# print(dir(l))
print(l.result)
# print(l.next_moves)