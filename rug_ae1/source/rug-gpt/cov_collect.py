import subprocess
import os
import copy
import json


def read_targets():
    ret = subprocess.run('cargo clean && RUSTFLAGS="-C instrument-coverage" cargo test -- --list', shell=True, capture_output=True)
    bins = []
    if ret.returncode != 0:
        raise Exception("err read list")
    else:
        stderr = ret.stderr.decode('utf-8')
        for l in stderr.splitlines():
            if 'Running unittests' in l:
                st = l.find('(')
                ed = l.find(')')
                binary = l[st+1:ed]
                bins.append(binary)
    ans = {}
    for l in ret.stdout.decode('utf-8').splitlines():
        if 'fuzzdriver' in l and 'fuzz_replay' in l:
            l = l.split(': ')[0]
            k = copy.deepcopy(l)
            v = ''

            for c in l.split('::'):
                if 'replay_' in c:
                    v = c
            ans[k] = v
    subprocess.run("rm *.profraw", shell=True)
    return ans, bins


def execute_cov(ans:dict, corpus: list, bins: list):
    if os.path.exists('cov'):
        subprocess.run('rm -rf cov', shell=True, capture_output=True)
    os.mkdir('cov')
    HOME = os.getcwd() + '/'
    counter = 0
    for k, v in ans.items():

        cor_path = ''
        for c in corpus:
            if v.replace('fuzz_replay', 'unit_test') + '/' in c:
                cor_path = c
                break
        if len(cor_path) == 0:
            print(k, v)
            continue
        for binary in bins:
            counter+=1
            print('*' * 20)
            print('FUZZ_CORPUS={} LLVM_PROFILE_FILE={}/cov/{}.profraw {} --test --exact {}'.format(HOME +cor_path, HOME, v + str(counter), binary, k))
            subprocess.run('FUZZ_CORPUS={} LLVM_PROFILE_FILE={}/cov/{}.profraw timeout 2m {} --test --exact {}'.format(HOME +cor_path, HOME, v+str(counter), binary, k), shell=True, capture_output=True)


def parse_file(file:str):
    ans = 1
    with open(file, 'r') as fp:
        for l in fp.readlines():
            if 'fuzzdriver_' in l or '#[cfg(test)]' in l:
                break
            ans += 1
    return ans


def collect_result(bins: list):
    fin = subprocess.run('~/snap/rustup/common/rustup/toolchains/nightly-2022-12-10-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata merge -sparse cov/* -o fuzz_cov.profdata', shell=True)
    assert fin.returncode == 0
    cmd = '~/snap/rustup/common/rustup/toolchains/nightly-2022-12-10-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-cov export --instr-profile=fuzz_cov.profdata '
    for binary in bins:
        cmd += "--object {} ".format(binary)
    fin = subprocess.run(cmd, shell=True, capture_output=True)
    assert fin.returncode == 0
    obj = json.loads(fin.stdout.decode('utf-8'))
    with open('cov.json', 'w') as fp:
        json.dump(obj, fp)
    map = {}
    reg_total = 0
    reg_count = 0
    line_total = 0
    line_count = 0
    func_total = 0
    func_count = 0
    mamp_reg = {}
    for func in obj['data'][0]['functions']:
        assert len(func['filenames']) <= 1
        filename = func['filenames'][0]
        is_valid_func = True
        has_hit  = False
        if os.getcwd() in filename:
            # print(filename)
            if filename not in map:
                map[filename] = parse_file(filename)
            limit = map[filename]
            for regions in func['regions']:
                lst = regions[0]
                led = regions[2]
                count = regions[4]
                if led > limit:
                    is_valid_func = False
                    break
                k = filename + '/' +str(lst) +'/' + str(regions[1]) +'/'+str(led) + '/'+ str(regions[3])
                if k in mamp_reg:
                    continue
                mamp_reg[k] = 1
                reg_total += 1
                line_total += led - lst + 1
                if count > 0:
                    has_hit = True
                    reg_count += 1
                    line_count += led - lst + 1
            if is_valid_func:
                func_total += 1
                if has_hit:
                    func_count += 1
    print(map)
    print(reg_count, reg_total, line_count, line_total, func_count, func_total)


if __name__ == '__main__':
    ans, binary = read_targets()
    corpus = build_corpus_map()
    execute_cov(ans, corpus, binary)
    collect_result(binary)