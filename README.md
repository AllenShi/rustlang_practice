# rustlang_practice
demonstrate for typical rust-lang features

## Quick Start

1. Create workspace manually
~~~
mkdir rustlang_practice
cd rustlang_practice
~~~

Cargo.toml
~~~
[workspace] 
members = [   "rust_exp_lib",   "rust_exp" ]
~~~

2. Create lib project
~~~
cargo new --lib rust_exp_lib
~~~

3. Create binary project
~~~
cargo new rust_exp
~~~

4. Add lib dependency into binary project
~~~~
[dependencies] 
rust_exp_lib = { path = "../rust_exp_lib" }
~~~~

5. Use lib in binary project
~~~
use rust_exp_lib::utils::*
~~~

## Reference
- [Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
