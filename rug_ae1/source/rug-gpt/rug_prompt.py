import subprocess
import subprocess as sp
import os
import copy
import re
import json
import sys
from openai import OpenAI
import tiktoken
import time
import multiprocessing


msgs=[
    {"role": "system", "content": "You are an expert in Rust. I need your help to develop unit tests for the given function in the crate."
                                  "I will give you the information about the target function and relevant definitions. Please only output the unit test(Rust code) for the target"
                                  "function without any explainations and be strict about compiler checks and import paths. I will give you the extra info about generic args and def paths, please try to use them in the code. "
                                  "Try to use the definition path directly instead of `use` statement to import mods."},

]


def parse_log(file:str):
    ans = {}

    if not os.path.exists(file):
        print(file)
        raise Exception()
    with open(file, 'r') as fp:
        ls = fp.readlines()
        i = 0
        while i < len(ls):
            if ls[i].startswith("----"):
                lifetimes = ''
                idx = ls[i+1].find(' ')
                target_file = ls[i+1][:idx]
                target_func = ls[i+1][idx+1:-1]
                i+=1
                if ls[i+1].startswith('\''):
                    lifetimes = ls[i+1][:-1]
                    i+=1

                assert ls[i+1].startswith("deps:")
                deps = json.loads(ls[i+1][5:])
                assert ls[i+2].startswith("candidates:")
                candidates = json.loads(ls[i+2][11:])
                i+=2
                stmts = []
                j = i+1
                tys = []
                func_call=set()
                while j < len(ls) and not ls[j].startswith("-----"):
                    if ls[j].startswith('+'):
                        func_call.add(ls[j])
                    else:
                        stmts.append(ls[j])
                        if '//' in ls[j]:
                            tys.append(ls[j].split('//')[1].strip())
                        else:
                            # primitve type
                            tys.append(None)
                    j+=1
                i = j
                if target_file not in ans:
                    ans[target_file] = []
                ans[target_file].append((target_file, stmts, func_call, lifetimes, deps, candidates, target_func, tys))
            else:
                i+=1
    return ans


def is_std(s:str):
    if s.startswith("&mut") or s.startswith("& mut "):
        s = s[s.find("mut")+4:]
    elif s.startswith("&"):
        s = s[max(s.find(" ")+1, 1):]
    if s.startswith('std::') or s.startswith('core::') or s.startswith('alloc::'):
        return True
    else:
        return False


def load_analysis(f: str):
    ans = None
    with open(f, 'r') as fp:
        ans = json.load(fp)
    return ans


counter = 0



def prompt_with_bounds(parent_def_path, def_path, ty, bounds, cans, deps, candidates, data, crate, file, src_pq, idx = -1):
    if 'RUG_ANY' in cans and len(cans) > 1:
        cans = [x for x in filter(lambda x: x!='RUG_ANY', cans)]
    found = False
    concrete_can = {}
    std_count = 0
    has_succeed = False
    for can in filter(lambda x: not ((x.startswith('<') or '::<' in x) and x.endswith('>')) and x != 'std::io::Stdin',cans):
        found = True
        if is_std(can):
            std_count += 1
            if std_count < 3:
                concrete_can[can] = prompt_built_in(parent_def_path, def_path, can, crate, file)
        elif can == 'RUG_ANY':
            concrete_can[can]= prompt_built_any(parent_def_path, def_path, ty, crate, file)
        else:
            if len(deps.get(can, [])) == 0:
                # no other depends
                # prompt directly
                concrete_can[can] = prompt_with_src_only(parent_def_path, ty, can, data, crate, src_pq)
            else:

                concrete_can[can] = prompt_pre_context(parent_def_path, def_path, can, data, crate, file,src_pq)
                map = {}
                for k, vs in deps[can].items():
                    map[k] = prompt_with_bounds(can, can, k, vs, candidates.get(can, {}).get(k, []), deps, candidates, data, crate, file, src_pq)
                for k, v in map.items():
                    concrete_can[can] +=  v
    if found:
        prompt = "For `{}` type in `{}`, we have {} candidates: `{}`\n".format(get_full_path(ty), get_full_path(parent_def_path), len(concrete_can), "`, `".join([get_full_path(x) for x in concrete_can.keys()]))
        if len(concrete_can) == 1 and 'RUG_ANY' in concrete_can:
            prompt = "For `{}` type in `{}`, we don't find explicit bounds.\n".format(get_full_path(ty), get_full_path(parent_def_path))
        for can, v in concrete_can.items():
            prompt +=v+"\n"
            src_pq.append(can)
        return prompt
    if not found:
        for can in filter(lambda x: (x.startswith('<') or '::<' in x) and x.endswith('>'),cans):
            #assert len(deps.get(can, {})) == 1
            for nty, nbounds in deps.get(can, {}).items():
                found = True
                src_pq.append(nty)
                src_pq.append(can)
                return "For `{}` type in `{}`, `{}` can be used: \n".format(get_full_path(ty), get_full_path(parent_def_path), get_full_path(can)) + prompt_with_bounds(can, can, nty, nbounds, candidates.get(can, {}).get(nty, []), deps, candidates, data, crate, file, src_pq)
    if not found:
        for x in bounds:
            src_pq.append(x)
        return "For `{}` type in `{}`, you need to write a concrete implementation that satisfied bounds: `{}`.\n".format(get_full_path(ty), get_full_path(parent_def_path), ", ".join([get_full_path(x) for x in bounds]))
    assert False




def prompt_built_any(parent_def_path, def_path, ty, crate, file):
    prompt="The `{}` in `{}` doesn't have type bounds. It might have other implicit bounds".format(get_full_path(ty), get_full_path(def_path))
    return prompt

def prompt_built_in(parent_def_path, def_path, ty, crate, file):
    prompt="the `{}` can be used in {}. ".format(get_full_path(ty), get_full_path(parent_def_path))
    return prompt


def get_real_path(s:str):
    return s[s.find("\"")+1:s.rfind("\"")]


def prompt_pre_context(parent_def_path, def_path, can, data, crate, file, src_pq):
    global counter
    counter += 1
    var_name = 'v'+str(counter)
    targets = data['targets']
    dependencies = data['dependencies']
    srcs = data['srcs']
    struct_to_trait = data['struct_to_trait']
    trait_to_struct = data['trait_to_struct']
    self_to_fn = data['self_to_fn']
    type_to_def_path = data['type_to_def_path']
    struct_constructor = data['struct_constructor']
    prompt="for `{}` used as `{}`, ".format(get_full_path(can), get_full_path(def_path))
    cons = set([get_full_path(x) for x in filter(lambda x: x not in ['clone'], struct_constructor.get(can, []))])
    if len(cons) > 0:
        if len(cons) > 1 and 'default' in cons:
            cons.remove('default')
        prompt+="try to use constructor functions like `{}` to build `{}`. ".format(", ".join(cons), get_full_path(can))
    src_pq.append(can)
    return prompt


def prompt_with_src_only(parent_def_path, def_path, ty, data, crate,  src_pq, idx = -1):
    src_pq.append(ty)
    struct_constructor = data['struct_constructor']
    prompt="the `{}` satisfies `{}` in `{}`. ".format(get_full_path(ty), get_full_path(def_path), get_full_path(parent_def_path))
    cons = set([get_full_path(x) for x in filter(lambda x: x not in ['clone'], struct_constructor.get(ty, []))])
    if len(cons) > 0:
        if len(cons) > 1 and 'default' in cons:
            cons.remove('default')
        prompt+="Try to use constructor functions like `{}` to build `{}`. ".format(", ".join(cons), get_full_path(ty))
    return prompt


def get_full_path(ty:str):
    if ty in single_path_import:
        return single_path_import[ty]
    for k, v in glob_path_import:
        if ty.startswith(k):
            t = ''
            if len(v)>1:
                t = v
            assert not (t+ty[len(k)+2:]).startswith("::")
            return t+ty[len(k)+2:]
    return ty


def prepare_deps_prompts(args):
    data=args[0]
    fd=args[1]
    crate=args[2]
    global single_path_import, glob_path_import
    ans = parse_log(fd + "/" + crate +".out.txt")
    targets = data['targets']
    dependencies = data['dependencies']
    srcs = data['srcs']
    struct_to_trait = data['struct_to_trait']
    trait_to_struct = data['trait_to_struct']
    self_to_fn = data['self_to_fn']
    type_to_def_path = data['type_to_def_path']
    single_path_import = data['single_path_import']
    glob = data['glob_path_import']
    glob_path_import = []
    glob_path_import = [(x, glob[x]) for x in glob]
    glob_path_import.sort(key=lambda x: len(x[0]), reverse=True)
    idx = 0
    total = 0
    good = 0
    template_total = 0
    template_good = 0
    dep_prompts = {}
    for f, vv in ans.items():
        file = f
        workspace = ''
        if not file.startswith('src/'):
            workspace = file[:file.find('/')]
        total += len(vv)
        for (_, fixed, func_call, lifetimes, deps, candidates, target_func, tys) in vv:
            src_pq = []
            final_prompt = ''
            parent_def_path = target_func
            for idx, ty in enumerate(tys):
                prompt = ''
                if ty is not None:
                    def_path = ty
                    if ty in type_to_def_path:
                        def_path = type_to_def_path[ty]
                    depends = []
                    if def_path in dependencies:
                        depends = [x for x in filter(lambda x: x!= ty and not is_std(x)
                                                     , dependencies[def_path])]
                    if target_func in deps:
                        if ty in deps[target_func]:
                            bounds = deps[target_func][ty]
                            cans = []
                            if ty in candidates[target_func]:
                                cans = candidates[target_func][ty]
                            prompt = prompt_with_bounds(parent_def_path, def_path, ty, bounds, cans, deps, candidates, data, crate, f, src_pq, idx)
                    if len(prompt) == 0:
                        if is_std(ty):
                            prompt = prompt_built_in(parent_def_path, def_path, ty, crate,  file)
                        else:
                            prompt = prompt_with_src_only(parent_def_path, def_path, ty,  data, crate, src_pq, idx)
                    if prompt[0]:

                        final_prompt += "For {}th argument, ".format(idx+1) + prompt +"\n"
            if file not in dep_prompts:
                dep_prompts[file] = {}
            #print(file)
            if target_func not in dep_prompts[file]:
                dep_prompts[file][target_func] = (final_prompt, set(src_pq))
            else:
                assert False
            # dep_prompts[(file, target_func)] = final_prompt
    # with normal prompt
    return dep_prompts



msgs=[
    {"role": "system", "content": "You are an expert in Rust. I need your help to develop unit tests for the given function in the crate."
                                  "I will give you the information about the target function and relevant definitions. Please only output the unit test(Rust code) for the target"
                                  "function without any explainations and be strict about compiler checks and import paths."},

]



prompt_target= """The target function is `{}` in `{}` crate's `{}` file, its definition path is `{}` and source code is like below:
```rust
{}
```

"""

prompt_dep= """The bounds and generic parameter info is shown below:
```
{}
```

"""

prompt_struct = """ The relevant definition, and method of `{}`{} are shown below:
```rust
{}
```
"""

prompt_impls = """The `{}` impls `{}` traits.
"""

prompt_rimpls = """The `{}` trait has `{}` that implements it.
"""


def run_each_target(args):
    global uid
    data=args[0]
    fd=args[1]
    crate=args[2]
    uid = 0
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
    with open('{}/{}_rq.log'.format(fd,crate), 'w') as sys.stdout:
        dep_prompts = prepare_deps_prompts(args)
        for target, meta in targets.items():
            prompt_length = len(enc.encode(msgs[0]['content']))
            func_name = meta[0]
            filename = meta[1][meta[1].find('"')+1:meta[1].rfind('"')]
            if filename.startswith("/home") or target.endswith(">::fmt"):
                continue

            deps = dependencies[target]
            func_src = srcs[target][0]
            output = ''
            output_less = ''
            pr_target = prompt_target.format(func_name, crate, filename, target, func_src)
            output += pr_target
            dep_prompt = dep_prompts[filename][target][0]
            src_pq = dep_prompts[filename][target][1]
            output += prompt_dep.format( dep_prompt)
            for dep in deps:
                if dep in struct_to_trait:
                    output += prompt_impls.format(get_full_path(dep), ','.join([get_full_path(x) for x in struct_to_trait[dep]]))
                if dep in trait_to_struct and not is_std(dep):
                    output += prompt_rimpls.format(get_full_path(dep), ','.join([get_full_path(x) for x in trait_to_struct[dep]]))
            output_less = copy.deepcopy(output)
            output_min = copy.deepcopy(output)
            for dep in deps:
                src_pq.add(dep)
            for dep in src_pq:
                code = ''
                file_loc = ''
                if dep in srcs:
                    code += srcs[dep][0]
                    file_loc = " in {}".format(get_real_path(srcs[dep][1]))
                    if len(code) > 0:
                        output_min += prompt_struct.format(get_full_path(dep), file_loc, code)
                if dep in self_to_fn:
                    if dep not in func_src and len(code) > 0:
                        output_less += prompt_struct.format(get_full_path(dep),file_loc,  code)
                    for c in self_to_fn[dep]:
                        if c not in 'CloneCopyDebug':
                            code += c+'\n'
                    if dep in func_src and len(code) > 0:
                        output_less += prompt_struct.format(get_full_path(dep),file_loc,  code)
                if len(code) > 0:
                    output += prompt_struct.format(get_full_path(dep), file_loc, code)
            messages = copy.deepcopy(msgs)
            final_prompt = ''
            count += 1
            # print(fd, crate, exceed_16, exceed_128, count)
            if prompt_length + len(enc.encode(output)) <= 16350:
                final_prompt = output
                ok += 1
            else:
                if prompt_length + len(enc.encode(output)) <= 32750:
                    exceed_16 += 1
                if prompt_length + len(enc.encode(output)) <= 128000:
                    exceed_128 += 1
                continue
            messages.append({"role": "user", "content":final_prompt})
            finished = False
            my_counter = 3
            while not finished:
                try:
                    #client = OpenAI(api_key='')
                    response = client.chat.completions.create(
                        model="gpt-3.5-turbo-16k",
                        # model= "gpt-4-1106-preview",
                        presence_penalty=-1,
                        messages = messages,
                    )
                    msg = response.choices[0].message.content
                    print("="*40)
                    print(messages[-1]['content'])
                    print('-'*20)
                    print(msg)
                    (status, idx, code, err) = compile_run(fd, crate, msg, data, target, meta, my_counter)
                    ans[target] = msg
                    if status:
                        finished = True
                        print('inject succeed', target, filename, my_counter)
                    elif idx >= 0:
                        my_counter -= 1
                        if my_counter == 0:
                            print('inject err', target, filename, my_counter)
                            break
                        messages.append({"role": "assistant", "content":msg})
                        messages.append({"role": "user", "content":'I put your code as below starting from {} line in {}, please revise based on the compiler'
                                                                   'error message and try to resolve compilation errors.\n```rust\n{}\n```\n```error\n{}\n```'.format(idx, filename, code, err)})
                    time.sleep(1)
                except Exception as e:
                    print('err', e)
                    if "This model's maximum context length is " in str(e):
                        print('inject err', target)
                        break
                    if "Connection err" in str(e):
                        client = OpenAI(api_key='')
                    time.sleep(15)
    with open(fd+'/'+crate+'.gpt.json','w') as fp:
        json.dump(ans, fp)
    sys.stdout = sys.__stdout__
    print(crate,count, ok, exceed_16, exceed_128)


def compile_run(fd, crate, msg, data, target, meta, my_counter):
    global uid
    uid += 1
    func_name = meta[0]
    filename = meta[1][meta[1].find('"')+1:meta[1].rfind('"')]
    program = msg
    program = program.replace('```rust', '').replace('```Rust', '').replace('```', '')
    ls = program.splitlines(keepends=True)
    if my_counter == 3:
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
    print('after modified')
    print("".join(ls))
    code = "".join(ls)
    # try inject
    if not os.path.exists(fd+'/'+filename):
        print('file not found', target, fd, filename)
        return (False, -1, code, None)
    with open(fd+'/'+filename, 'r+') as fp:
        origins = fp.readlines()
        le = len(origins)
        mutate = copy.deepcopy(origins)
        mutate.extend(ls)
        fp.truncate(0)
        fp.seek(0)
        fp.writelines(mutate)
        fp.flush()
        ret = subprocess.run("cargo test --no-run", shell=True, cwd=fd+'/'+path, capture_output=True)
        if ret.returncode == 0:
            # print('inject succeed', fd, crate, target)
            return (True, le, code ,None)
        else:
            fp.truncate(0)
            fp.seek(0)
            fp.writelines(origins)
            fp.flush()
            print('='*40)
            # print('inject err', target, filename, func_name)
            print(''.join(ls))
            print(ret.stderr.decode('utf-8'))
            return (False, le, code,ret.stderr.decode('utf-8'))



def run_single(args):
    fd = args[0]
    crate = args[1]
    # print("python3.10 -u main.py {} {} > {}_{}.log 2>&1".format(fd, crate, fd, crate))
    subprocess.run("python3.10 -u main.py {} {} > {}_{}.log 2>&1".format(fd, crate, fd, crate), shell=True)


if __name__ == '__main__':
    args = []
    if len(sys.argv) < 3:
        # os.chdir(sys.argv[1])
        for fd in os.listdir('.'):
            if not os.path.isdir(fd):
                continue
            # fd = sys.argv[1]
            fin = subprocess.run('cargo ws list -l', shell=True, cwd=fd, capture_output=True)
            for l in fin.stdout.decode('utf-8').splitlines():
                ls = l.split(' ')
                crate = ls[0].strip()
                path = ls[-1]
                if not os.path.exists(fd+'/'+crate+'.json'):
                    subprocess.run('cargo clean && CHAT_UNIT=1 cargorunner rudra', shell=True, capture_output=True, cwd=fd+'/'+path)
                    subprocess.run('mv preprocess.json {}.json'.format(crate), shell=True, capture_output=True, cwd=fd)
                if not os.path.exists(fd+"/"+crate+".out.txt"):
                    fin = subprocess.run('cargo clean && UNIT_GEN=s1 cargorunner rudra', shell=True, capture_output=True, cwd=fd+'/'+path)
                    with open(fd+"/"+crate+".out.txt", 'w') as fp:
                        fp.writelines("\n".join(fin.stdout.decode("utf-8").splitlines()))
                if os.path.exists(fd+'/'+crate+'.json'):
                    args.append((fd, crate))
                    run_each_target((load_analysis(fd+'/'+crate+'.json'), fd, crate))
        # print(args)
        # with multiprocessing.Pool(4) as p:
        #     p.map(run_single, args)
    else:
        fd = sys.argv[1]
        crate = sys.argv[2]
        data = load_analysis(fd+'/'+crate+'.json')
        run_each_target((data, fd, crate))
