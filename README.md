# learn blur

a console project for learning blur methods.

just run `cargo run -- --help` to use it;

```sh
cargo run -- -i "test.png" -o "gaussian.png" -d 0.4 -c 3 gaussian -r 7
cargo run -- -i "test.png" -o "kawase.png" -d 0.4 -c 9 kawase -k 3   
cargo run -- -i "test.png" -o "bokeh.png" bokeh -r 3 -i 150
```
