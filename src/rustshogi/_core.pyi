from enum import Enum

class Address:
    column: int
    row: int

    def __init__(self, column: int, row: int) -> None: ...
    def __ptr__(self) -> str: ...

class ColorType(Enum):
    Black = 0
    White = 1
    ColorNumber = 2

    def __init__(self, value: int) -> None: ...
    def __ptr__(self) -> str: ...

class PieceType(Enum):
    King = 1
    Gold = 2
    Rook = 3
    Bichop = 4
    Silver = 5
    Knight = 6
    Lance = 7
    Pawn = 8
    Dragon = 9
    Horse = 10
    ProSilver = 11
    ProKnight = 12
    ProLance = 13
    ProPawn = 14

    def __init__(self, value: int) -> None: ...
    def __ptr__(self) -> str: ...

class Piece:
    owner: ColorType
    piece_type: PieceType

    def __init__(self, owner: ColorType, piece_type: PieceType) -> None: ...
    def __ptr__(self) -> str: ...

class Move:
    def __init__(self, csa: str) -> None: ...
    def __ptr__(self) -> str: ...

class Hand:
    pieces: list[Piece]
    counts: list[int]

    def __init__(self, pieces: list[Piece], counts: list[int]) -> None: ...
    def __ptr__(self) -> str: ...

class Board:
    def __init__(self) -> None: ...
    def __ptr__(self) -> str: ...
