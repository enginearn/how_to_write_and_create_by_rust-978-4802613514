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
