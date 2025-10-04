Snapcraft ドキュメント
https://snapcraft.io/docs

Snapcraft コマンド説明
1. Snap パッケージを作成
snapcraft pack


現在のディレクトリにある snap 定義をもとに .snap パッケージを生成する。

snapcraft を用いてビルド済みの成果物をパッケージ化するコマンド。

2. Snap をインストール
sudo snap install test-gst-libcamera_0.1.0_arm64.snap --dangerous --devmode


ローカルにある .snap ファイルをインストールする。

--dangerous は署名がないパッケージでもインストールを許可するオプション。

--devmode は制限なしで実行できる開発モードでのインストールを意味する。

3. インターフェース接続
sudo snap connect test-gst-libcamera:camera


Snap がカメラデバイスへアクセスできるようにインターフェースを接続する。

test-gst-libcamera Snap に対し、ホストの camera インターフェースを明示的に許可する操作。

4. Snap アプリを実行
sudo snap run test-gst-libcamera


インストール済みの test-gst-libcamera アプリを実行する。

snap run は Snap アプリを起動する基本的なコマンド。

5. 接続状況を確認
snap connections netfleece


netfleece Snap がどのインターフェースに接続されているかを確認する。

インターフェースの接続可否を一覧表示する。

説明
https://snapcraft.io/docs/interface-management