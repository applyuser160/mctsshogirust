クイックスタート
===============

このガイドでは、rustshogiの基本的な使用方法を説明します。

基本的な使用方法
==============

まず、rustshogiをインポートして初期局面を作成します：

.. code-block:: python

   import rustshogi

   # 初期局面を作成
   board = rustshogi.Board()
   print(board)

盤面の表示
----------

盤面の現在の状態を確認できます：

.. code-block:: python

   # 盤面を文字列として表示
   print(board.to_string())

   # 盤面の状態を詳細に表示
   print(f"手番: {board.turn}")
   print(f"手数: {board.move_count}")

合法手の取得
============

現在の局面での合法手を取得：

.. code-block:: python

   # 合法手を取得
   legal_moves = board.get_legal_moves()
   print(f"合法手数: {len(legal_moves)}")

   # 最初の合法手を表示
   if legal_moves:
       print(f"最初の合法手: {legal_moves[0]}")

手を指す
========

合法手を指して局面を進めます：

.. code-block:: python

   # 手を指す
   if legal_moves:
       move = legal_moves[0]
       board.make_move(move)
       print(f"手を指しました: {move}")
       print(board)

手の表現
========

手は以下のような形式で表現されます：

.. code-block:: python

   # 手の詳細情報
   move = legal_moves[0]
   print(f"移動元: {move.from_square}")
   print(f"移動先: {move.to_square}")
   print(f"駒: {move.piece}")
   print(f"成り: {move.promotion}")

局面の評価
==========

局面の評価値を取得：

.. code-block:: python

   # 局面評価（例：先手有利なら正の値）
   evaluation = board.evaluate()
   print(f"局面評価: {evaluation}")

ゲームの終了判定
==============

ゲームが終了したかどうかを確認：

.. code-block:: python

   # ゲーム終了判定
   if board.is_game_over():
       print("ゲーム終了")
       print(f"勝者: {board.winner}")
   else:
       print("ゲーム継続中")

完全な例
--------

以下は簡単な対局の例です：

.. code-block:: python

   import rustshogi

   # 初期局面
   board = rustshogi.Board()

   # 10手まで対局
   for i in range(10):
       if board.is_game_over():
           break

       legal_moves = board.get_legal_moves()
       if not legal_moves:
           break

       # 最初の合法手を指す
       move = legal_moves[0]
       board.make_move(move)

       print(f"手数 {i+1}: {move}")
       print(board)
       print("-" * 40)

   print(f"最終局面評価: {board.evaluate()}")
