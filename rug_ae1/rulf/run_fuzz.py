import sys
import os
import subprocess
import multiprocessing as mp


def run_fuzzer(tp:(str, str)):
    #
    # change the running time limitation, 86400 seconds = 24h
    #
    timeout_sec = 86400
    template = "AFL_NO_AFFINITY=1 AFL_EXIT_WHEN_DONE=1 cargo afl fuzz -i ../afl_init/ -o ./{}/ -V {} target/debug/{}"
    try:
        ret = subprocess.run(template.format(tp[1], timeout_sec, tp[1]), shell=True, capture_output=True, cwd=tp[0] timeout=timeout_sec)
    except Exception:
        pass
    print(template.format(tp[1], timeout_sec tp[1]))

def launch_fuzz(dir:str):
    targets = []
    ret = subprocess.run("cargo afl build", shell=True, capture_output=True, cwd=dir)
    assert ret.returncode == 0
    for f in os.listdir(dir+'/test_files'):
        targets.append((dir, f[:-3]))
    return targets


if __name__ == '__main__':
    args = []
    for f in os.listdir(sys.argv[1]):
        if f.endswith('afl-work') and os.path.exists(sys.argv[1]+'/'+f+'/fuzz'):
            args.extend(launch_fuzz(sys.argv[1]+'/'+f+'/fuzz'))
    print(args)
    print(len(args))
    #
    # change the num of processes in parallel
    #
    with mp.Pool(48) as pool:
        pool.map(run_fuzzer, args)
    print("done")
