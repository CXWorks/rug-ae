import json
import os
import multiprocessing
import subprocess
import sys
import json

def run_single_fd(fd):
    fin = subprocess.run('cargo ws list -l', shell=True, cwd=fd, capture_output=True)
    total = 0
    for l in fin.stdout.decode('utf-8').splitlines():
        ls = l.split(' ')
        crate = ls[0].strip()
        path = ls[-1]
        if os.path.exists(fd+'/{}_inject.log'.format(crate)):
            # record = {}
            # with open(fd+'/{}_base_res.json'.format(crate), 'r') as fp:
            #     record = json.load(fp)
            with open(fd+'/{}_inject.log'.format(crate), 'r') as fp:
                ls = fp.readlines()
                in_test = False
                target = ''
                count = 0
                res = ''
                for l in ls:
                    if l.startswith("ChatCompletion(id='"):
                        idx = l.find('total_tokens=')
                        num = int(l[idx+13:-3])
                        count += num
                        total += num
                    if l.startswith("repair err "):

                        print(count, l, end='')
                        count = 0
                    elif l.startswith("inject succeed"):
                        count = 0
    print(fd, total)

def run_single(fd):
    # print("python3.10 -u main.py {} {} > {}_{}.log 2>&1".format(fd, crate, fd, crate))
    subprocess.run("python3.10 -u main.py {} > {}/run.log 2>&1".format(fd, fd), shell=True)


if __name__ == '__main__':
    args = []
    if len(sys.argv) < 2:
        # os.chdir(sys.argv[1])
        for fd in os.listdir('.'):
            if not os.path.isdir(fd):
                continue
            # fd = sys.argv[1]
            fin = subprocess.run('cargo ws list -l', shell=True, cwd=fd, capture_output=True)
            if fin.returncode == 0:
                args.append(fd)
        # print(args)
        with multiprocessing.Pool(8) as p:
            p.map(run_single, args)
    else:
        fd = sys.argv[1]
        run_single_fd(fd)