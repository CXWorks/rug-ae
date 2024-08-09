from openai import OpenAI
import subprocess
import os
import sys
import json
import tiktoken
import copy
import time
import multiprocessing


msgs=[
    {"role": "system", "content": "You are an expert in Rust. I need your help to develop unit tests for the given function in the crate."
                                  "I will give you the information about the target function and relevant definitions. Please only output the unit test(Rust code) for the target"
                                  "function without any explainations and be strict about compiler checks and import paths. Please think it step by step."},

]


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


prompt_target= """The target function is `{}` in `{}` crate's `{}` file, its definition path is `{}` and source code is like below:
```rust
{}
```

"""

prompt_struct = """ The relevant definition, and method of `{}` are shown below:
```rust
{}
```
"""

prompt_impls = """The `{}` impls `{}` traits.
"""

prompt_rimpls = """The `{}` trait has `{}` that implements it.
"""


def run_each_target(args):
    data=args[0]
    fd=args[1]
    crate=args[2]
    path = args[3]
    enc = tiktoken.encoding_for_model("gpt-4")
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
    client = OpenAI(api_key='')
    with open('{}_inject.log'.format(crate), 'w') as sys.stdout:
        for target, meta in targets.items():
            prompt_length = len(enc.encode(msgs[0]['content']))
            func_name = meta[0]
            filename = meta[1][meta[1].find('"')+1:meta[1].rfind('"')]
            if filename.startswith("/home") or target.endswith('>::fmt'):
                continue
            deps = dependencies[target]
            func_src = srcs[target][0]
            output = ''
            output_less = ''
            pr_target = prompt_target.format(func_name, crate, filename,target, func_src)
            output += pr_target
            for dep in deps:
                if dep in struct_to_trait:
                    output += prompt_impls.format(dep, ','.join(struct_to_trait[dep]))
                if dep in trait_to_struct:
                    output += prompt_rimpls.format(dep, ','.join(trait_to_struct[dep]))
            output_less = copy.deepcopy(output)
            output_min = copy.deepcopy(output)
            for dep in deps:
                code = ''
                if dep in srcs:
                    code += srcs[dep][0]
                    if len(code) > 0:
                        output_min += prompt_struct.format(dep, code)
                if dep in self_to_fn:
                    if dep not in func_src and len(code) > 0:
                        output_less += prompt_struct.format(dep, code)
                    for c in self_to_fn[dep]:
                        if c not in 'CloneCopyDebug':
                            code += c+'\n'
                    if dep in func_src and len(code) > 0:
                        output_less += prompt_struct.format(dep, code)
                if len(code) > 0:
                    output += prompt_struct.format(dep, code)
            messages = copy.deepcopy(msgs)
            final_prompt = ''
            count += 1
            if prompt_length + len(enc.encode(output)) <= 128000:
                final_prompt = output
            else:
                if prompt_length + len(enc.encode(output)) <= 32750:
                    exceed_16 += 1
                if prompt_length + len(enc.encode(output)) <= 128000:
                    exceed_128 += 1
                continue
            ok += 1
            messages.append({"role": "user", "content":final_prompt})
            finished = False
            uid += 1
            while not finished:
                try:
                    response = client.chat.completions.create(
                        model="gpt-4-1106-preview",
                        presence_penalty=-1,
                        messages = messages,
                    )
                    print(response)
                    print(prompt_length)
                    ans[target] = response.choices[0].message.content
                except Exception as e:
                    print('err', e)
                    if "This model's maximum context length is " in str(e):
                        print('missing', target)
                        break
                    if "Connection err" in str(e):
                        client = OpenAI(api_key='')
                    time.sleep(15)
                # start verify phase
                program = handle_gpt_output(ans[target])

                ls = program.splitlines(keepends=True)
                for i in range(len(ls)):
                    l = ls[i]
                    if 'mod tests' in l:
                        ls[i] = l.replace('mod tests', 'mod tests_llm_16_{}'.format(uid))
                    if '{}::'.format(crate) in l:
                        idx = ''.find('{}::'.format(crate))
                        if idx > 0 and l[idx-1] in 'qazwsxedcrfvtgbyhnujmikolp:':
                            continue
                        ls[i] = l.replace('{}::'.format(crate), 'crate::')
                    if 'use super::*' in l:
                        ls[i] = l+'\nuse crate::*;\n'
                    elif 'use super::' in l:
                        ls[i] = l.replace('use super::', 'use crate::')
                if not os.path.exists(filename):
                    print('file not found', target, fd, filename)
                    continue
                with open(filename, 'r+') as fp:
                    origins = fp.readlines()
                    mutate = copy.deepcopy(origins)
                    mutate.extend(ls)
                    fp.truncate(0)
                    fp.seek(0)
                    fp.writelines(mutate)
                    fp.flush()
                    ret = subprocess.run('RUSTFLAGS="-Awarnings" cargo test --no-run', shell=True, cwd=path, capture_output=True)
                    if ret.returncode == 0:
                        print('inject succeed', fd, crate, target)
                        finished = True
                        succeed += 1
                        if len(messages) > 3:
                            repair_succeed += 1
                    else:
                        fp.truncate(0)
                        fp.seek(0)
                        fp.writelines(origins)
                        fp.flush()
                        print('='*40)
                        print('inject err', target, filename, func_name)
                        print(''.join(ls))
                        print(ret.stderr.decode('utf-8'))
                        if len(messages) > 3:
                            finished = True
                            print('repair err',target, filename, func_name)
                            break
                        repair += 1
                        le = len(origins)
                        messages.append({"role": "assistant", "content":ans[target]})
                        messages.append({"role": "user", "content":'I put your code as below starting from line `{}` in `{}`, please revise based on the compiler'
                                                                   'error message and try to resolve compilation errors.\n```rust\n{}\n```\n```error\n{}\n```'.format(le, filename, "".join(ls), ret.stderr.decode('utf-8'))})
        print(succeed, repair, repair_succeed, ok, count)
    sys.stdout = sys.__stdout__
    with open(crate+'.gpt.json','w') as fp:
        json.dump(ans, fp)



def run_single(fd):
    subprocess.run("python3.10 -u main.py {} > {}/uni_test.log 2>&1".format(fd, fd), shell=True)


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
            run_each_target((data, fd, crate, path))
