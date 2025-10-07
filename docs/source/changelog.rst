変更履歴
========

このドキュメントでは、rustshogiの各バージョンでの変更点を記録しています。

バージョン 0.1.0 (2025-01-XX)
=============================

初回リリース

新機能
~~~~~~

* 基本的な将棋盤の表現と操作
* 合法手の生成
* 手の実行と取り消し
* 局面評価機能
* MCTS（モンテカルロ木探索）アルゴリズム
* Pythonバインディング
* 基本的なゲーム管理機能

API
~~~

* :py:class:`Board` - 将棋盤の表現
* :py:class:`Move` - 手の表現
* :py:class:`Piece` - 駒の表現
* :py:class:`Color` - 先手・後手の表現
* :py:class:`Hand` - 持ち駒の管理
* :py:class:`Game` - ゲーム管理

新機能
~~~~~~

* 基本的な将棋盤の表現と操作（Boardクラス）
* 合法手の生成と検索（search_movesメソッド）
* 手の実行（execute_moveメソッド）
* 駒の配置（deployメソッド）
* 持ち駒の管理（Handクラス）
* ゲーム状態の管理（Gameクラス）
* 座標の表現（Addressクラス）
* 駒の種類と色の表現（PieceType、ColorType列挙型）
* Pythonバインディング

API
~~~

* :py:class:`Address` - 座標の表現
* :py:class:`ColorType` - 先手・後手の表現
* :py:class:`PieceType` - 駒の種類の表現
* :py:class:`Piece` - 駒の表現
* :py:class:`Move` - 手の表現
* :py:class:`Hand` - 持ち駒の管理
* :py:class:`Board` - 将棋盤の表現
* :py:class:`Game` - ゲーム管理
