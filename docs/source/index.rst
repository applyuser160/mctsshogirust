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
* 合法手の生成
* 局面評価
* MCTS（モンテカルロ木探索）アルゴリズム
* Pythonバインディング

インストール
===========

.. code-block:: bash

   pip install rustshogi

クイックスタート
==============

.. code-block:: python

   import rustshogi

   # 初期局面を作成
   board = rustshogi.Board()

   # 合法手を取得
   legal_moves = board.get_legal_moves()

   # 手を指す
   board.make_move(legal_moves[0])

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
