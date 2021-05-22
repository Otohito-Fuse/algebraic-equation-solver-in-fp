# algebraic equation solver in F_p
有限素体 F_p 上での方程式の解を求める。

## 使い方

### 標数pの設定
```src/main.rs```の最初の行の右辺を書き換える。初期状態では```59```になっている。
もし素数でない値を書いた場合はプログラムの実行時に教えてくれる（方程式は```Z/PZ```上で解いてくれる）。

### プログラムの実行
Rust をインストールしていない場合はインストールする。```cargo run```を打つ。

### 方程式の入力
多項式の次数と、各係数を入力していく。

### 計算
入力された多項式の根を全探索して表示する。