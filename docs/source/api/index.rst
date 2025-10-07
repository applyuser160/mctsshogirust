API リファレンス
================

rustshogiの完全なAPIリファレンスです。

.. toctree::
   :maxdepth: 2

   ../reference/rustshogi

モジュール概要
=============

rustshogiは以下の主要なモジュールで構成されています：

* :doc:`../reference/rustshogi` - メインモジュール（将棋盤、手、駒などの基本機能）

基本的な型
==========

.. py:class:: Board
   :module: rustshogi

   将棋盤を表現するクラス。局面の状態、合法手の生成、手の実行などの機能を提供します。

.. py:class:: Move
   :module: rustshogi

   将棋の手を表現するクラス。移動元、移動先、駒の種類、成りなどの情報を含みます。

.. py:class:: Piece
   :module: rustshogi

   将棋の駒を表現するクラス。駒の種類と色（先手・後手）の情報を含みます。

.. py:class:: Color
   :module: rustshogi

   先手・後手を表現する列挙型。

.. py:class:: Hand
   :module: rustshogi

   持ち駒を管理するクラス。

.. py:class:: Game
   :module: rustshogi

   ゲーム全体を管理するクラス。対局の進行、勝敗判定などを担当します。

例外
----

.. py:exception:: InvalidMoveError
   :module: rustshogi

   無効な手が指された場合に発生する例外。

.. py:exception:: GameOverError
   :module: rustshogi

   ゲームが終了した後に手が指された場合に発生する例外。
