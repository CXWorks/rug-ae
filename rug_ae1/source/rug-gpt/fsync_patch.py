import os
import sys
import copy

def collect_tests():
    ans = {}
    with open('run.log', 'r') as fp:
        target_file = ''
        code = ''
        in_code = False
        for l in fp.readlines():
            if l.startswith("The target function is "):
                for ls in l.split(' '):
                    if ls.endswith(".rs"):
                        target_file = ls
                        break
            else:
                if in_code:
                    if l.startswith("```"):
                        in_code = False
                    else:
                        code += l
                else:
                    if l.startswith("```rust") or l.startswith("```Rust") or l.startswith("```"):
                        code = ''
                        in_code = True
                    elif l.startswith("unit gen succeed"):
                        if target_file not in ans:
                            ans[target_file] = []
                        ans[target_file].append(code)
                        code = ''
    return ans


def apply(fd:str, ans:dict):
    counter = 0
    for f, tests in ans.items():
        with open(fd+'/'+f, 'r+') as fp:
            origins = fp.readlines()
            mutate = copy.deepcopy(origins)
            for test in tests:
                counter += 1
                code = test.replace('mod tests {', 'mod tests_rug_{} {{'.format(counter))
                mutate.append(code)
            fp.truncate(0)
            fp.seek(0)
            fp.writelines(mutate)
            fp.flush()
            os.fsync(fp.fileno())



if __name__ == '__main__':
    os.chdir(sys.argv[1])
    ans = collect_tests()
    for k, v in ans.items():
        for vv in v:
            print(k)
            print(vv)
    # apply(sys.argv[1], ans)
