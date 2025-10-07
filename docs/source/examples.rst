例
==

このセクションでは、rustshogiを使用した実用的な例を紹介します。

基本的な対局
===========

.. code-block:: python

   import rustshogi
   import random

   def random_game():
       """ランダムな対局を実行"""
       board = rustshogi.Board()

       while not board.is_game_over():
           legal_moves = board.get_legal_moves()
           if not legal_moves:
               break

           # ランダムに手を選択
           move = random.choice(legal_moves)
           board.make_move(move)

           print(f"手数 {board.move_count}: {move}")
           print(board)
           print("-" * 40)

       print(f"ゲーム終了: {board.winner}")
       return board

MCTSアルゴリズムの使用
=====================

.. code-block:: python

   import rustshogi

   def mcts_example():
       """MCTSアルゴリズムを使用した例"""
       board = rustshogi.Board()

       # MCTSで最適な手を探索
       mcts_result = board.mcts_search(
           iterations=1000,
           exploration_constant=1.4
       )

       print(f"最適な手: {mcts_result.best_move}")
       print(f"評価値: {mcts_result.evaluation}")
       print(f"訪問回数: {mcts_result.visit_count}")

       return mcts_result

局面の解析
==========

.. code-block:: python

   import rustshogi

   def analyze_position():
       """局面の詳細な解析"""
       board = rustshogi.Board()

       print("=== 局面解析 ===")
       print(f"手番: {board.turn}")
       print(f"手数: {board.move_count}")
       print(f"局面評価: {board.evaluate()}")

       legal_moves = board.get_legal_moves()
       print(f"合法手数: {len(legal_moves)}")

       # 各合法手の評価
       print("\n=== 合法手の評価 ===")
       for i, move in enumerate(legal_moves[:5]):  # 最初の5手のみ
           board.make_move(move)
           evaluation = board.evaluate()
           board.unmake_move()  # 手を戻す

           print(f"{i+1}. {move}: {evaluation}")

カスタム評価関数
===============

.. code-block:: python

   import rustshogi

   def custom_evaluation(board):
       """カスタム評価関数の例"""
       # 基本的な駒の価値
       piece_values = {
           rustshogi.Piece.PAWN: 1,
           rustshogi.Piece.LANCE: 3,
           rustshogi.Piece.KNIGHT: 3,
           rustshogi.Piece.SILVER: 5,
           rustshogi.Piece.GOLD: 6,
           rustshogi.Piece.BISHOP: 8,
           rustshogi.Piece.ROOK: 10,
           rustshogi.Piece.KING: 100,
       }

       evaluation = 0

       # 盤上の駒を評価
       for square in rustshogi.Square.all():
           piece = board.get_piece(square)
           if piece is not None:
               value = piece_values.get(piece.type, 0)
               if piece.color == rustshogi.Color.BLACK:
                   evaluation += value
               else:
                   evaluation -= value

       return evaluation

   def use_custom_evaluation():
       """カスタム評価関数の使用例"""
       board = rustshogi.Board()

       # カスタム評価関数を使用
       custom_eval = custom_evaluation(board)
       builtin_eval = board.evaluate()

       print(f"カスタム評価: {custom_eval}")
       print(f"組み込み評価: {builtin_eval}")

対局の記録と再生
===============

.. code-block:: python

   import rustshogi

   def record_and_replay():
       """対局の記録と再生"""
       board = rustshogi.Board()
       moves_history = []

       # 対局を記録
       for i in range(10):
           if board.is_game_over():
               break

           legal_moves = board.get_legal_moves()
           if not legal_moves:
               break

           move = legal_moves[0]  # 最初の合法手
           board.make_move(move)
           moves_history.append(move)

       print(f"記録された手数: {len(moves_history)}")

       # 初期局面に戻して再生
       board = rustshogi.Board()
       for i, move in enumerate(moves_history):
           board.make_move(move)
           print(f"再生 {i+1}: {move}")

       return moves_history
