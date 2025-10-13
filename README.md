# RustShogi

Rustで実装された将棋ライブラリ

## 特徴

- 高速なRust実装
- Pythonバインディング対応
- メモリ効率的なデータ構造
- 完全な将棋ルール実装

## インストール

```bash
pip install rustshogi
```

## 基本的な使い方

```python
import rustshogi

# 新しいゲームを作成
game = rustshogi.Game()

# 盤面の状態を表示
print(game.board)

# 可能な手を検索
moves = game.board.search_moves(rustshogi.ColorType.Black)
print(f"可能な手の数: {len(moves)}")

# 手を実行
if moves:
    game.execute_move(moves[0])
    print("手を実行しました")
    print(game.board)

# ゲーム終了判定
is_finished, winner = game.is_finished()
if is_finished:
    print(f"ゲーム終了: 勝者 = {winner}")
```

### アドレスの操作

```python
# アドレスを作成
address = rustshogi.Address(3, 4)  # 3列4行
print(f"アドレス: {address}")

# 文字列からアドレスを作成
address = rustshogi.Address.from_string("3d")
print(f"アドレス: {address}")

# インデックスに変換
index = address.to_index()
print(f"インデックス: {index}")
```

### 駒の操作

```python
# 駒を作成
piece = rustshogi.Piece(rustshogi.ColorType.Black, rustshogi.PieceType.King)
print(f"駒: {piece}")

# 文字から駒を作成
piece = rustshogi.Piece.from_char('K')  # 黒の王
print(f"駒: {piece}")

piece = rustshogi.Piece.from_char('p')  # 白の歩
print(f"駒: {piece}")
```

### 手の操作

```python
# 通常の手を作成
from_addr = rustshogi.Address(3, 3)
to_addr = rustshogi.Address(3, 4)
move = rustshogi.Move(from_address=from_addr, to_address=to_addr, promote=False)
print(f"手: {move}")

# ドロップ手を作成
piece = rustshogi.Piece.from_char('p')
to_addr = rustshogi.Address(3, 4)
drop_move = rustshogi.Move(piece=piece, to_address=to_addr)
print(f"ドロップ手: {drop_move}")

# CSA形式から手を作成
csa_move = rustshogi.Move(csa="3c3d")
print(f"CSA手: {csa_move}")
```

### 盤面の操作

```python
# 新しい盤面を作成
board = rustshogi.Board()

# 初期配置を設定
board.startpos()
print("初期配置:")
print(board)

# 駒を配置
piece = rustshogi.Piece(rustshogi.ColorType.Black, rustshogi.PieceType.King)
address = rustshogi.Address(5, 1)
board.deploy(address, piece.piece_type, piece.owner)

# 特定の位置の駒を取得
piece_at_pos = board.get_piece(address)
print(f"位置{address}の駒: {piece_at_pos}")

# 可能な手を検索
moves = board.search_moves(rustshogi.ColorType.Black)
print(f"黒の可能な手: {len(moves)}手")

# 手を実行
if moves:
    board.execute_move(moves[0])
    print("手を実行後の盤面:")
    print(board)
```

### ゲームの進行

```python
# ゲームを作成
game = rustshogi.Game()

# ランダムプレイ
result = game.random_play()
print(f"ランダムプレイ結果: 勝者 = {result.winner}")

# 手動でゲームを進行
game = rustshogi.Game()
while not game.is_finished()[0]:
    moves = game.board.search_moves(game.turn)
    if moves:
        # 最初の手を選択
        game.execute_move(moves[0])
    else:
        break

is_finished, winner = game.is_finished()
print(f"ゲーム終了: 勝者 = {winner}")
```

## ドキュメント

https://applyuser160.github.io/mctsshogirust/

## データ構造

### ColorType

```python
rustshogi.ColorType.Black    # 先手
rustshogi.ColorType.White    # 後手
rustshogi.ColorType.None     # なし
```

### PieceType

```python
rustshogi.PieceType.King      # 王
rustshogi.PieceType.Gold      # 金
rustshogi.PieceType.Rook      # 飛車
rustshogi.PieceType.Bichop    # 角
rustshogi.PieceType.Silver    # 銀
rustshogi.PieceType.Knight    # 桂
rustshogi.PieceType.Lance     # 香
rustshogi.PieceType.Pawn      # 歩
# 成駒
rustshogi.PieceType.Dragon    # 龍
rustshogi.PieceType.Horse     # 馬
rustshogi.PieceType.ProSilver # 成銀
rustshogi.PieceType.ProKnight # 成桂
rustshogi.PieceType.ProLance  # 成香
rustshogi.PieceType.ProPawn   # と
```

## パフォーマンス

このライブラリは以下の最適化により高速な処理を実現しています：

- ビットボードによる効率的な盤面表現
- メモリ効率的なデータ構造（u16によるMove表現など）
- SIMD命令の活用
- ゼロコスト抽象化

## ライセンス

このプロジェクトはMITライセンスの下で公開されています。

## 貢献

プルリクエストやイシューの報告を歓迎します。開発に参加したい場合は、まずイシューを作成してご連絡ください。
