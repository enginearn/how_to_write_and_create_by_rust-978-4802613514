<details>
<summary>RUST_BACKTRACE=1を付けて実行</summary>

```
RUST_BACKTRACE=1 ./caesar_enc.exe
Please enter a text to encrypt:
HellO WOrLd
HellO
thread 'main' panicked at 'attempt to add with overflow', caesar_enc.rs:7:26
stack backtrace:
   0: std::panicking::begin_panic_handler
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library\std\src\panicking.rs:584
   1: core::panicking::panic_fmt
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library\core\src\panicking.rs:142
   2: core::panicking::panic
             at /rustc/4b91a6ea7258a947e59c6522cd5898e7c0a6a88f/library\core\src\panicking.rs:48
   3: alloc::boxed::Box<T>::from_raw
   4: alloc::boxed::Box<T>::from_raw
   5: alloc::boxed::Box<T>::from_raw
   6: core::ops::function::FnOnce::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

</details>

---

## Reference

<details>

[How to Install Python pip on Ubuntu 22.04](https://linuxhint.com/install-python-pip-ubuntu-22-04/)

[pip/pip3のパッケージインストール時にWARNINGが出るときの対応 – EXCEEDSYSTEM](https://www.exceedsystem.net/2020/09/29/1321/)

[panic!で回復不能なエラー - The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/ch09-01-unrecoverable-errors-with-panic.html)

[【Python】round()を使って小数点の四捨五入を行う！｜フライテック](https://flytech.work/blog/21651/)

[Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=dafaeef0233d4424fdaf3626595e40f2)

[Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=34e0e3608accdca4072163d2b4063d89)

[Rust 文字列を指定した区切り文字で区切って逆順にする | mebee](https://mebee.info/2022/08/06/post-76007/)

[Rustで「文字が特定の文字集合に含まれるか」を判定するのはどれが速いか | κeenのHappy Hacκing Blog](https://keens.github.io/blog/2019/10/06/rustde_mojigatokuteinomojishuugounifukumareruka_wohanteisurunohadoregahayaika/)

[convert i32 to a string rust Code Example](https://www.codegrepper.com/code-examples/rust/convert+i32+to+a+string+rust)

[Carbon Language が発表されたので実際に動かしてみた](https://zenn.dev/blendthink/articles/d1a3b397bdae82)

[C++ compiler support - cppreference.com](https://en.cppreference.com/w/cpp/compiler_support)

[Rustでの整数オーバーフローまとめ - Qiita](https://qiita.com/garkimasera/items/c5e06de1a7c66aa7652a)

[enginearn/Rust_the_Book](https://github.com/enginearn/Rust_the_Book)

[トレンド - Qiita](https://qiita.com/trend)

[記事を探す | Zenn](https://zenn.dev/articles/explore?order=alltime)

[Pythonでリストの要素をシャッフル（random.shuffle, sample） | note.nkmk.me](https://note.nkmk.me/python-random-shuffle/)

[Pythonを使ってランダムな文字列を生成 - Qiita](https://qiita.com/Scstechr/items/c3b2eb291f7c5b81902a)

[無料 英和辞書データ ダウンロード - ブラウザで使えるWeb便利ツール](https://kujirahand.com/web-tools/EJDictFreeDL.php)

</details>
