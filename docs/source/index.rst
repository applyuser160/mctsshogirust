rustshogi ドキュメント
========================

rustshogiは、Rustで実装された高性能な将棋ライブラリです。
Pythonバインディングを通じて、Pythonアプリケーションから利用できます。

.. toctree::
   :maxdepth: 2
   :caption: 目次:

   installation
   quickstart
   api/index
   examples
   changelog

機能
----

* 高速な将棋盤の表現と操作
* 合法手の生成と検索
* 駒の配置と移動
* 持ち駒の管理
* ゲーム状態の管理
* Pythonバインディング

インストール
===========

.. code-block:: bash

   pip install rustshogi

クイックスタート
==============

.. code-block:: python

   from rustshogi import Board, ColorType, Move, Address

   # 初期局面を作成
   board = Board("startpos")

   # 合法手を検索
   legal_moves = board.search_moves(ColorType.Black)

   # 手を実行
   if legal_moves:
       board.execute_move(legal_moves[0])

詳細な使用方法については、:doc:`quickstart` を参照してください。

API リファレンス
===============

完全なAPIリファレンスは :doc:`reference/rustshogi` で確認できます。

.. toctree::
   :maxdepth: 1
   :caption: API リファレンス:

   reference/rustshogi

インデックスとテーブル
====================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`
