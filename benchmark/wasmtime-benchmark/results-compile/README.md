# `wasmtime-benchmark`

At the time of writing, this is (intentionally) more of a benchmark of Cranelift
than Wasmtime. Also, this isn't for comparing Wasmtime/Cranelift to other Wasm
engines/compilers, just for checking proposed changes vs the main branch.

## Run the Benchmark

```
$ CONTROL="master" VARIABLE="test-branch" ITERS=100 ./run.sh
```
