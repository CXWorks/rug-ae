from openai import OpenAI
import subprocess
import os
import sys
import json
import tiktoken
import copy
import time
import multiprocessing


def load_analysis(f: str):
    ans = None
    with open(f, 'r') as fp:
        ans = json.load(fp)
    return ans


def handle_gpt_output(code:str):
    ans = []
    in_it = False
    if '```' not in code:
        return code
    for l in code.splitlines():
        if not in_it:
            if '```rust' in l or '```Rust' in l or '```RUST' in l:
                in_it = True
        else:
            if '```' in l:
                in_it = False
            else:
                ans.append(l)
    return "\n".join(ans)


def run_each_target(args):
    data=args[0]
    fd=args[1]
    crate=args[2]
    path = args[3]
    targets = data['targets']
    dependencies = data['dependencies']
    srcs = data['srcs']
    struct_to_trait = data['struct_to_trait']
    trait_to_struct = data['trait_to_struct']
    self_to_fn = data['self_to_fn']
    ans = {}
    count = 0
    ok = 0
    succeed = 0
    repair = 0
    repair_succeed = 0
    exceed_16 = 0
    exceed_128 = 0
    uid = 0
    ans = ''
    if not os.path.exists(crate+'.gpt.json'):
        return (0,0)
    ans = {}
    with open(crate+'.gpt.json', 'r') as fp:
        ans = json.load(fp)
    res = {}
    with open('{}_base_collect.log'.format(crate), 'w') as sys.stdout:
        for target, meta in targets.items():
            filename = meta[1][meta[1].find('"')+1:meta[1].rfind('"')]
            if filename.startswith("/home") or target.endswith('>::fmt'):
                continue
            if target not in ans:
                print(target,'exceed')
                continue
            count += 1
            program = ans[target]
            program = handle_gpt_output(program)
            ls = program.splitlines(keepends=True)
            st = 0
            for i in range(len(ls)):
                l = ls[i]
                if 'mod tests' in l:
                    st = i+1
                if '{}::'.format(crate) in l:
                    idx = ''.find('{}::'.format(crate))
                    if idx > 0 and l[idx-1] in 'qazwsxedcrfvtgbyhnujmikolp:':
                        continue
                    ls[i] = l.replace('{}::'.format(crate), 'crate::')
                if 'use super::*' in l:
                    ls[i] = l+'\nuse crate::*;\n'
                elif 'use super::' in l:
                    ls[i] = l.replace('use super::', 'use crate::')
            code = ''.join(ls[st:])
            print('='*40)
            print(code)
            with open(filename, 'r') as ffp:
                ls = ffp.read()
                print(code in ls)
                res[target] = code in ls
                if code in ls:
                    ok += 1
        print(fd, crate, ok, count)
    sys.stdout = sys.__stdout__
    with open('{}_base_res.json'.format(crate), 'w') as fp:
        json.dump(res, fp)
    return (ok, count)





def run_single(fd):
    fin = subprocess.run("python3.10 -u main.py {}".format(fd), shell=True)
    if fin.returncode != 0:
        print(fd, fin.returncode)


if __name__ == '__main__':
    args = []
    if len(sys.argv) < 2:
        for fd in os.listdir('.'):
            if not os.path.isdir(fd):
                continue

            args.append(fd)
            # run_each_target((load_analysis(fd+'/'+crate+'.json'), fd, crate))
        print(args)
        with multiprocessing.Pool(18) as p:
            p.map(run_single, args)
    else:
        fd = sys.argv[1]
        os.chdir(fd)
        fin = subprocess.run('cargo ws list -l', shell=True, capture_output=True)
        ok = 0
        count = 0
        for l in fin.stdout.decode('utf-8').splitlines():
            ls = l.split(' ')
            crate = ls[0].strip()
            path = ls[-1]
            if not os.path.exists(crate+'.json'):
                subprocess.run('cargo clean && CHAT_UNIT=1 cargorunner rudra', shell=True, capture_output=True, cwd=path)
                subprocess.run('mv preprocess.json {}.json'.format(crate), shell=True, capture_output=True)
            if not os.path.exists(crate+'.json'):
                print('missing', crate, path)
                continue
            data = load_analysis(crate+'.json')
            a, b = run_each_target((data, fd, crate, path))
            ok += a
            count += b
        print(fd, ok, count)
