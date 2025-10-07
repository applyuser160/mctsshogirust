インストール
============

rustshogiはPythonパッケージとして配布されています。

要件
----

* Python 3.8以上
* Windows、macOS、Linux（x86_64）

pipでのインストール
==================

.. code-block:: bash

   pip install rustshogi

開発版のインストール
==================

最新の開発版をインストールする場合：

.. code-block:: bash

   pip install git+https://github.com/yourusername/rustshogi.git

ソースからのビルド
================

Rustツールチェーンが必要です：

.. code-block:: bash

   # Rustのインストール
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # プロジェクトのクローン
   git clone https://github.com/yourusername/rustshogi.git
   cd rustshogi

   # ビルドとインストール
   pip install -e .

インストールの確認
================

インストールが成功したか確認：

.. code-block:: python

   import rustshogi
   print(rustshogi.__version__)
