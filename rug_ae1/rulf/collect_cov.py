import sys
import os
import subprocess
import multiprocessing as mp
import json


def parse_file(file:str):
    ans = 1
    with open(file, 'r') as fp:
        for l in fp.readlines():
            if '#[cfg(test)]' in l:
                break
            ans += 1
    return ans



def cov_select(dir: str):
    map = {}
    mamp_reg = {}
    for f in os.listdir(dir+'/fuzz/test_files'):
        target = f[:-3]
        subprocess.run('rm -rf cov', shell=True)
        subprocess.run('mkdir cov', shell=True)

        idx = 0
        for ipt in os.listdir(dir+'/fuzz/'+target+'/default/queue'):
            idx+=1
            if ipt.startswith('id'):
                ret = subprocess.run('LLVM_PROFILE_FILE={}/cov/{}.profraw {}/replay/target/debug/{} {}/fuzz/{}/default/queue/{}'.format(dir, idx, dir, target, dir, target, ipt), shell=True, capture_output=True)
                if ret.returncode != 0:
                    continue
        #
        # modify following 2 lines to your system
        #
        fin = subprocess.run('~/snap/rustup/common/rustup/toolchains/nightly-2022-12-10-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata merge -sparse {}/cov/* -o fuzz_cov.profdata'.format(dir), shell=True)
        fin = subprocess.run('~/snap/rustup/common/rustup/toolchains/nightly-2022-12-10-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-cov export {}/replay/target/debug/{} --instr-profile=fuzz_cov.profdata'.format(dir, target), shell=True, capture_output=True)
        assert fin.returncode == 0
        obj = json.loads(fin.stdout.decode('utf-8'))
        for func in obj['data'][0]['functions']:
            assert len(func['filenames']) <= 1
            filename = func['filenames'][0]
            if os.getcwd() in filename and 'src/' in filename:
                #
                # here I exclude the cov on unit test in the file
                #
                if filename not in map:
                    map[filename] = parse_file(filename)
                limit = map[filename]
                for regions in func['regions']:
                    lst = regions[0]
                    led = regions[2]
                    count = regions[4]
                    if led > limit:
                        continue
                    k = filename + '/' +str(lst) +'/' + str(regions[1]) +'/'+str(led) + '/'+ str(regions[3])
                    if k in mamp_reg:
                        mamp_reg[k][1] += count
                    else:
                        mamp_reg[k] = [led - lst + 1, count]
    reg_total = len(mamp_reg)
    reg_count = 0
    line_total = 0
    line_count = 0
    for k, v in mamp_reg.items():
        line_total += v[0]
        if v[1] > 0:
            reg_count += 1
            line_count += v[0]
    print('--------------------------------------', dir)
    print(reg_count, reg_total)
    print('--------------------------------------')




if __name__ == '__main__':
    for f in os.listdir(sys.argv[1]):
        if f.endswith('afl-work') and os.path.exists(sys.argv[1]+'/'+f+'/fuzz'):
            subprocess.run('RUSTFLAGS="-C instrument-coverage" cargo build', shell=True, cwd=sys.argv[1]+'/'+f+'/replay', capture_output=True)
            cov_select(sys.argv[1]+'/'+f)

