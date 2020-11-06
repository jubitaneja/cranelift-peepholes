CONTROL=${CONTROL:="main"}
VARIABLE=${VARIABLE:="integrate-peepmatic"}
ITERS=${ITERS:=100}
WASMTIME=${WASMTIME:=~/wasmtime}

set -eux

cd "$(dirname $0)"
BENCHMARK_DIR=$(pwd)

cd "$WASMTIME"

function build_it {
    # cargo build \
    #       --manifest-path ~/wasmtime/cranelift/codegen/Cargo.toml \
    #       --release \
    #       --features 'rebuild-peephole-optimizers' || true
    # if grep -q 'enable-peepmatic' cranelift/codegen/Cargo.toml; then
    #     sed -i -e \
    #         's|default = \["std", "unwind"\]|default = \["std", "unwind", "enable-peepmatic"\]|' \
    #         cranelift/codegen/Cargo.toml
    # fi
    cargo build --release
}

function time_it {
    for (( i = 0; i < $ITERS; i += 1 )); do
        # $(which time) --format "$1,%e,%M" \
        #               ./target/release/wasmtime run \
        #               --optimize \
        #               --disable-cache \
        #               clang.wasm -- \
        #               --version 2>> time.csv
        $(which time) --format "$1,%e,%M" \
                      ./target/release/wasmtime run \
                      --optimize \
                      --disable-cache \
                      "$BENCHMARK_DIR/markdown.wasm" -- \
                      '# Hello, World!' 2>> "$BENCHMARK_DIR/time.csv"
    done
}

function perf_events {
    for (( i = 0; i < $ITERS; i += 1 )); do
        # perf stat -e instructions,cache-misses,branch-misses -x ',' \
        #      ./target/release/wasmtime run \
        #      --optimize \
        #      --disable-cache \
        #      clang.wasm -- \
        #      --version 2>&1 >/dev/null \
        #     | cut -d ',' -f 1,3 \
        #     | xargs -I{} echo "$1,{}" >> perf_events.csv
        perf stat -e instructions,cache-misses,branch-misses -x ',' \
             ./target/release/wasmtime run \
             --optimize \
             --disable-cache \
             "$BENCHMARK_DIR/markdown.wasm" -- \
             '# Hello, World!' 2>&1 >/dev/null \
            | cut -d ',' -f 1,3 \
            | xargs -I{} echo "$1,{}" >> "$BENCHMARK_DIR/perf_events.csv"
    done
}

echo "Branch,ElapsedSeconds,RSS" > "$BENCHMARK_DIR/time.csv"
echo "Branch,Count,Event" > "$BENCHMARK_DIR/perf_events.csv"

## Run for Control

git checkout "$CONTROL"

build_it
time_it "control-$CONTROL"
perf_events "control-$CONTROL"

git reset --hard "$CONTROL"

## Run for Variable

git checkout "$VARIABLE"

build_it
time_it "variable-$VARIABLE"
perf_events "variable-$VARIABLE"

git reset --hard "$VARIABLE"

## Plot the Results

cd "$BENCHMARK_DIR"
./plot.r "control-$CONTROL" "variable-$VARIABLE"
