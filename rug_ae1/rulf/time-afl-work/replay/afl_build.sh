rm *.o *.bc base.$1.out
cargo clean && RUSTFLAGS="--emit=llvm-bc "  cargo build --release --bin $1
$LLVM_BIN/llvm-link target/release/deps/*.bc -o res.bc
$LLVM_BIN/opt -enable-new-pm=0 -O0 -load /mnt/md0/xiang/rustspot-aflgo/afl-llvm-pass.so res.bc -o tmp.bc > org.txt
rm -rf tmp
mkdir tmp
touch fake.txt
$LLVM_BIN/opt -enable-new-pm=0 -O2 -load /mnt/md0/xiang/rustspot-aflgo/afl-llvm-pass.so -targets $(pwd)/fake.txt -outdir $(pwd)/tmp/ res.bc -o opt.bc
rm -rf bin
mkdir bin && cp res.bc bin
touch tmp/BBtargets.txt
python3.8 -u $AFLGO/scripts/gen_distance_fast.py bin/ tmp/ -p
touch $(pwd)/tmp/distance.cfg.txt
$LLVM_BIN/opt -enable-new-pm=0 -O2 -load /mnt/md0/xiang/rustspot-aflgo/afl-llvm-pass.so -distance $(pwd)/tmp/distance.cfg.txt opt.bc -o inst.bc
$LLVM_BIN/llvm-link inst.bc $AFLGO/llvm_mode/afl-llvm-rt.o.bc -o main.bc
$LLVM_BIN/llc main.bc -o main.o -filetype=obj
clang-10 -v -Bstatic ~/.rustup/toolchains/nightly-2020-07-01-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-31151f98ccdb185a.rlib  -m64 main.o -L ~/.rustup/toolchains/nightly-2020-07-01-x86_64-unknown-linux-gnu/lib/ -lstd-b6aff3703feff874 -lrustc_driver-24ed6b45be4bc6ec -lLLVM-10-rust-1.46.0-nightly -lm -lpthread
mv a.out base.$1.out
