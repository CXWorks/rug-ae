import subprocess
import json
import os
import sys
import random
import time
import multiprocessing
import toml

def recur_scan(folder:str, ans: list):
    for f in os.listdir(folder):
        path = folder +'/'+f
        if f.endswith('.rs'):
            ans.append(path)
        if os.path.isdir(path):
            recur_scan(path, ans)


def read_targets():
    subprocess.run('cargo clean && rm *.profraw', shell=True, capture_output=True, timeout=3600)
    fin = subprocess.run('RUSTFLAGS="-C instrument-coverage" cargo test rusty_tests --no-fail-fast', shell=True, capture_output=True, timeout=3600)
    if fin.returncode != 0:
        print('error in running')
    bins = []
    for l in fin.stderr.decode('utf-8').splitlines(keepends=False):
        if 'Running ' in l and '(' in l and ')' in l:
            binary = l[l.find('(')+1: l.find(')')]
            if os.path.exists(binary):
                bins.append(binary)
    return bins


def parse_file(file:str):
    ans = 1
    with open(file, 'r') as fp:
        ls = fp.readlines()
        for i in range(len(ls)):
            l = ls[i]
            if 'fuzzdriver_' in l or '#[cfg(test)]' in l:
                if ls[i+1] .startswith('mod'):
                    break
            ans += 1
    return ans


def collect_result(binaries: list):
    fin = subprocess.run('~/snap/rustup/common/rustup/toolchains/nightly-2022-12-10-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata merge -sparse *.profraw -o fuzz_cov.profdata', shell=True)
    assert fin.returncode == 0
    map = {}
    reg_count = 0
    mamp_reg = {}
    cmd = "~/snap/rustup/common/rustup/toolchains/nightly-2022-12-10-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-cov export {} --instr-profile=fuzz_cov.profdata --ignore-filename-regex='/home/xiang/.cargo/registry' --ignore-filename-regex='rustc/*' --ignore-filename-regex='tests/*' "
    fin = subprocess.run(cmd.format(binaries[0]), shell=True, capture_output=True)
    assert fin.returncode == 0
    obj = json.loads(fin.stdout.decode('utf-8'))
    limit = 1000000
    for func in obj['data'][0]['functions']:
        assert len(func['filenames']) <= 1
        filename = func['filenames'][0]
        if os.getcwd() in filename and not filename.endswith('rusty_monitor.rs'):
            if filename not in map:
                map[filename] = parse_file(filename)
            limit = map[filename]
            for regions in func['regions']:
                lst = regions[0]
                led = regions[2]
                count = regions[4]
                if led > limit:
                    break
                k = filename + '/' +str(lst) +'/' + str(regions[1]) +'/'+str(led) + '/'+ str(regions[3])
                if k not in mamp_reg:
                    mamp_reg [k] = 0
                mamp_reg[k] = max(count, mamp_reg[k])
    reg_total = len(mamp_reg)
    for k,v in mamp_reg.items():
        if v > 0:
            reg_count += 1
    with open('cov_map.json', 'w') as fp:
        json.dump(mamp_reg, fp)
    time.sleep(random.randint(1, 90))
    print('-'*20, os.getcwd(),'-')
    print(reg_count, reg_total, map)
    print('-'*20)


def run_single(fd:str):
    cwd = os.path.abspath(os.getcwd())
    os.chdir(fd)
    tars = []
    recur_scan('src/', tars)
    for src in tars:
        if src.endswith('.rs'):
            print(src)
            with open(src, 'r+') as fp:
                ls = fp.readlines()
                for i in range(len(ls)):
                    l = ls[i]
                    if l.startswith('#[no_coverage]') or l.startswith('#[should_panic]') or l.startswith('#[timeout(3000)]')\
                            or ('panic!("From RustyUnit with love");' in l and '//' not in l)\
                            or ('rusty_monitor::set_test_id(' in l and '//' not in l):
                        ls[i] = '//'+l
                fp.truncate(0)
                fp.seek(0)
                fp.writelines(ls)
                fp.flush()
    info = toml.load(fd + '/Cargo.toml')
    if 'profile' in info:
        del info['profile']
    with open(fd+'/Cargo.toml', 'w') as fp:
        toml.dump(info, fp)
    bins = read_targets()
    collect_result(bins)
    os.chdir(cwd)


if __name__ == '__main__':
    os.chdir(os.getcwd() + '/' + sys.argv[1])
    wl = []
    run_single('/home/xiang/workspace/rustyunit/time/rusty-unit/seeded-dynamosa/rusty-unit-18')
    # for fd in os.listdir('.'):
    #     for f in os.listdir(fd+'/rusty-unit/seeded-dynamosa/'):
    #         if os.path.isdir(fd+'/rusty-unit/seeded-dynamosa/' + f):
    #             wl.append(fd+'/rusty-unit/seeded-dynamosa/' + f)
    # with multiprocessing.Pool(48) as p:
    #     p.map(run_single, wl)
