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
    enc = tiktoken.encoding_for_model("gpt-3.5-turbo")
    targets = data['targets']
    dependencies = data['dependencies']
    srcs = data['srcs']
    struct_to_trait = data['struct_to_trait']
    trait_to_struct = data['trait_to_struct']
    self_to_fn = data['self_to_fn']
    ans = {}
    count = 0
    ok = 0
    exceed_16 = 0
    exceed_128 = 0
    client = OpenAI(api_key='')
    with open('{}/{}_inject.log'.format(fd, crate), 'w') as fp:
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
        # print(fd, crate, exceed_16, exceed_128, count)
            if prompt_length + len(enc.encode(output)) <= 16350:
                final_prompt = output
            else:
                if prompt_length + len(enc.encode(output)) <= 32750:
                    exceed_16 += 1
                if prompt_length + len(enc.encode(output)) <= 128000:
                    exceed_128 += 1
                final_prompt = enc.decode(enc.encode(output)[:16350 - prompt_length])
            ok += 1
            messages.append({"role": "user", "content":final_prompt})
            finished = False
            while not finished:
                try:
                    response = client.chat.completions.create(
                        model="gpt-3.5-turbo-16k",
                        presence_penalty=-1,
                        messages = messages,
                    )
                    print(response)
                    print(prompt_length)
                    ans[target] = response.choices[0].message.content
                    finished = True
                    time.sleep(1)
                except Exception as e:
                    print('err', e)
                    if "This model's maximum context length is " in str(e):
                        print('missing', target)
                        break
                    if "Connection err" in str(e):
                        client = OpenAI(api_key='')
                    time.sleep(15)
    with open(fd+'/'+crate+'.gpt.json','w') as fp:
        json.dump(ans, fp)


def run_single(args):
    fd = args[0]
    crate = args[1]
    # print("python3.10 -u main.py {} {} > {}_{}.log 2>&1".format(fd, crate, fd, crate))
    subprocess.run("python3.10 -u main.py {} {} > {}_{}.log 2>&1".format(fd, crate, fd, crate), shell=True)


if __name__ == '__main__':
    args = []
    if len(sys.argv) < 3:
        for fd in os.listdir('.'):
            if not os.path.isdir(fd):
                continue
            fin = subprocess.run('cargo ws list -l', shell=True, cwd=fd, capture_output=True)
            for l in fin.stdout.decode('utf-8').splitlines():
                ls = l.split(' ')
                crate = ls[0].strip()
                path = ls[-1]
                if not os.path.exists(fd+'/'+crate+'.json'):
                    subprocess.run('cargo clean && CHAT_UNIT=1 cargorunner rudra', shell=True, capture_output=True, cwd=fd+'/'+path)
                    subprocess.run('mv preprocess.json {}.json'.format(crate), shell=True, capture_output=True, cwd=fd)
                if os.path.exists(fd+'/'+crate+'.json'):
                    args.append((fd, crate))
                    # run_each_target((load_analysis(fd+'/'+crate+'.json'), fd, crate))
        print(args)
        with multiprocessing.Pool(12) as p:
            p.map(run_single, args)
    else:
        fd = sys.argv[1]
        crate = sys.argv[2]
        data = load_analysis(fd+'/'+crate+'.json')
        run_each_target((data, fd, crate))
















