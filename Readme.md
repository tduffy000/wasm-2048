# 2048 WebAssembly

<p align="center">
  <a target="_blank" href="https://2048.dev.family">
    <img src="https://github.com/dev-family/wasm-2048/blob/master/images/result.png">
  </a>
</p>

<p align="center">
  <a href="https://2048.dev.family">Live Demo</a>
</p>

A search-based "AI" player of [the famous 2048 game](https://github.com/gabrielecirulli/2048) forked from the [dev.family](https://dev.family/) implementation using Rust ([Yew](https://yew.rs/)) and compiled to WASM. Currently, we make use of several heuristics, e.g. monotonicity and smoothness, inside of an expectimax tree search. 

# Running

The simplest way to run is via docker:

```
docker build -t wasm-2048 .
docker run -it --rm -p 8080:8000 wasm-2048
```

Then open http://127.0.0.1:8080 and watch the computer play itself. Hopefully it makes it to 2048!

<hr />

<p align="center">
  <a target="_blank" href="https://dev.family?utm_source=n7&utm_medium=github&utm_campaign=2048">
    <img src="https://github.com/dev-family/wasm-2048/blob/master/images/dev-family.png">
  </a>
</p>
