# RULF eval

1. Based on RULF's github version, using all default options(without adding any other options) to generate harness, replay file + init corpus(it uses afl++). Install necessary tools and toolchain components(afl.rs/llvm-tools)
2. Check 2 python scripts, change the running time on run_fuzz.py, modify the correct path in collect_cov.py based on your rust toolchain's location
3. python3 run_fuzz.py ./ && python3 collect_cov.py ./
