import rustshogi

l = rustshogi.goresult(
    "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL",
    "b",
    "-",
    "1",
    10
)

print(dir(l))
print(l.result)
print(l.next_moves)