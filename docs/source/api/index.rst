API リファレンス
================

rustshogiの完全なAPIリファレンスです。

.. toctree::
   :maxdepth: 2

   ../reference/rustshogi

モジュール概要
=============

rustshogiは以下の主要なクラスと列挙型で構成されています：

* :doc:`../reference/rustshogi` - メインモジュール（将棋盤、手、駒などの基本機能）

基本的な型
==========

.. py:class:: Address
   :module: rustshogi

   将棋盤上の座標を表現するクラス。列（column）と行（row）の情報を含みます。

.. py:class:: ColorType
   :module: rustshogi

   先手・後手を表現する列挙型。Black（先手）とWhite（後手）の値を持ちます。

.. py:class:: PieceType
   :module: rustshogi

   将棋の駒の種類を表現する列挙型。King、Gold、Rook、Bishop、Silver、Knight、Lance、Pawn、および成り駒の種類を含みます。

.. py:class:: Piece
   :module: rustshogi

   将棋の駒を表現するクラス。駒の種類（PieceType）と色（ColorType）の情報を含みます。

.. py:class:: Move
   :module: rustshogi

   将棋の手を表現するクラス。移動元、移動先、駒の種類、成りなどの情報を含みます。

.. py:class:: Hand
   :module: rustshogi

   持ち駒を管理するクラス。各プレイヤーの持ち駒の追加・削除・確認機能を提供します。

.. py:class:: Board
   :module: rustshogi

   将棋盤を表現するクラス。局面の状態、合法手の生成、手の実行などの機能を提供します。

.. py:class:: Game
   :module: rustshogi

   ゲーム全体を管理するクラス。対局の進行、勝敗判定、ランダム対局などを担当します。
