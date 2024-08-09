import unittest

from rug_recur import prompt_built_in, compile_verify, load_analysis, prompt_with_src_only, prompt_with_bounds, \
    parse_log, counter, is_std, prompt_target, prompt_dep, prompt_struct, prompt_impls, prompt_rimpls, get_full_path, \
    get_real_path, gpt_request, init_global, compile_only


class TestStringMethods(unittest.TestCase):

    def test_built_in(self):
        global single_path_import, glob_path_import
        single_path_import = {}
        glob_path_import = {}
        prompt_built_in('','', 'std::fs::File', '','')

    def test_verify(self):
        code = """
        #[cfg(test)]
mod tests_prepare {
    use std::fs::File;
    
    #[test]
    fn sample() {
        let v1: File = File::create("path/to/file.txt").unwrap();
    }
}"""
        self.assertTrue(compile_verify('/home/cxworks/codespace/bincode', 'src/atomic.rs', code, 'tests_prepare', 'v1', 'std::fs::File'))

    def test_src(self):
        fd = '/home/cxworks/codespace/bincode'
        file = 'src/enc/impls.rs'
        data = load_analysis(fd +'/bincode.json')
        prompt_with_src_only('', 'enc::write::SliceWriter', 'enc::write::SliceWriter', data, fd, [], file)

    def test_all(self):
        import copy
        init = """You are an expert in Rust. I need your help to develop unit tests for the given function in the crate.I will give you the information about the target function and relevant definitions. I may give you the sample code to build the parameters, please strictly follow the sample code to construct the variable (you can change the variable names) and its use statements since these code are verified. Please only output the unit test(Rust code) for the targetfunction without any explainations and be strict about compiler checks and import paths. Please prepare the inital test data if necessary."""
        msgs=[
            {"role": "system", "content": init},

        ]
        fd = '/mnt/sda/xiang/workspace/tmp/bincode'
        file = 'src/atomic.rs'
        data = load_analysis(fd +'/bincode.json')
        ans = parse_log(fd + "/bincode.out.txt")
        crate = 'bincode'
        targets = data['targets']
        dependencies = data['dependencies']
        srcs = data['srcs']
        struct_to_trait = data['struct_to_trait']
        trait_to_struct = data['trait_to_struct']
        self_to_fn = data['self_to_fn']
        type_to_def_path = data['type_to_def_path']
        init_global(data)
        for f, vv in ans.items():
            file = f
            for (_, stmts, func_call, lifetimes, deps, candidates, target_func, tys) in vv:
                if target_func == 'atomic::<impl enc::Encode for std::sync::atomic::AtomicIsize>::encode':
                    final_prompt = ''
                    parent_def_path = target_func
                    src_pq = []
                    has_sample = set()
                    for idx, (ty, primitive) in enumerate(tys):
                        prompt = ''
                        local_src = []
                        if ty is not None:
                            def_path = ty
                            if ty in type_to_def_path:
                                def_path = type_to_def_path[ty]
                            if target_func in deps:
                                if ty in deps[target_func]:
                                    bounds = deps[target_func][ty]
                                    cans = []
                                    if ty in candidates[target_func]:
                                        cans = candidates[target_func][ty]
                                    prompt = prompt_with_bounds(parent_def_path, def_path, ty, bounds, cans, deps, candidates, data, crate, f, local_src, fd, set())
                            if len(prompt) == 0:
                                if is_std(ty):
                                    prompt = prompt_built_in(fd, parent_def_path, ty,  file)
                                else:
                                    prompt = prompt_with_src_only(parent_def_path, def_path, ty,  data, fd, local_src, f)
                            if prompt[0]:
                                has_sample.add(idx)
                                final_prompt += "For {}th argument, `{}` can be used, please use following sample code to construct it:\n```rust\n{}\n```\n".format(idx+1, prompt[3], prompt[1])
                            else:
                                src_pq.extend(local_src)
                                final_prompt += "For {}th argument, `{}` can be used, please use following description to construct it:\n```\n{}\n```\n".format(idx+1, prompt[3], prompt[2])
                        else:
                            final_prompt += "For {}th argument, its type is `{}`, please use some sample data to initialize it.\n".format(idx+1, primitive)
                    # request for unit test
                    target = target_func
                    deps = dependencies[target]
                    func_src = srcs[target][0]

                    pr_target = prompt_target.format(target, crate, file, target, func_src)
                    src_pq = set(src_pq)
                    single_test_template = """ 
#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_rug() {{
        {}
        
        {}
    }}
}}
                    """
                    for fc in reversed([x for x in func_call]):
                        params = ''
                        param_template = "let mut p{} = ... ;\n"
                        no_sample = set()
                        for idx, (ty, primitive) in enumerate(tys):
                            params += param_template.format(idx)
                            if idx not in has_sample:
                                no_sample.add(idx)
                        tests = single_test_template.format(params, fc)
                        output = pr_target
                        test_template = """
Please help me following steps on the code below to build the unit test:

1. fill in the {} variables in the following code using the samples without modifications and keep the type declarations
2. construct the variables {} based on hints if there isn't a sample
3. combine all the use statements inside the mod and remove the duplicated use, but don't add extra ones:

```rust
{}
```


                        """
                        output += test_template.format(", ".join(["p" +str(x) for x in has_sample]), ", ".join(["p" +str(x) for x in no_sample]), tests)
                        output += final_prompt
                        for dep in src_pq:
                            code = ''
                            file_loc = ''
                            if dep in srcs:
                                code += srcs[dep][0]
                                file_loc = " in {}".format(get_real_path(srcs[dep][1]))
                            if dep in self_to_fn:
                                for c in self_to_fn[dep]:
                                    if c not in 'CloneCopyDebug':
                                        code += c+'\n'
                            if len(code) > 0:
                                output += prompt_struct.format(get_full_path(dep), file_loc, code)
                        messages = copy.deepcopy(msgs)
                        final_prompt = output
                        print('='*40)
                        print(final_prompt)
                        # count += 1
                        # # print(fd, crate, exceed_16, exceed_128, count)
                        # if prompt_length + len(enc.encode(output)) <= 16350:
                        #     final_prompt = output
                        #     ok += 1
                        # else:
                        #     if prompt_length + len(enc.encode(output)) <= 32750:
                        #         exceed_16 += 1
                        #     if prompt_length + len(enc.encode(output)) <= 128000:
                        #         exceed_128 += 1
                        #     continue
                        messages.append({"role": "user", "content":final_prompt})
                        finished = False
                        count = 5
                        while not finished and count > 0:
                            has_ans , code = gpt_request(messages)
                            code = code.replace('```rust', '').replace('```Rust', '').replace('```', '')
                            if has_ans and compile_only(fd, file, code):
                                finished = True
                            else:
                                count -= 1
                        if finished:
                            return
                        # ty='E'
                        # bounds = deps[target_func][ty]
                        # cans = []
                        # if ty in candidates[target_func]:
                        #     cans = candidates[target_func][ty]
                        # print(prompt_with_bounds('enc::impls::<impl enc::Encode for isize>::encode', ty, ty, bounds, cans, deps, candidates,
                        #                    data, 'bincode', f, [], fd))
        # prompt_with_bounds('', '', '', data, fd, [], file)

    def test_rq(self):
        import json
        with open('e.json', 'r') as fp:
            msgs = json.load(fp)
            print(msgs)
            print(gpt_request(msgs))



if __name__ == '__main__':
    unittest.main()
