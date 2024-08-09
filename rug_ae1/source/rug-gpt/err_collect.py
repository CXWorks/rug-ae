import subprocess
import os
import copy
import json


def parse_file(fd:str):
    with open(fd+'/inject.log', 'r') as fp:
        ls = fp.readlines()
        total = 0
        succeed = 0
        err = 0
        err_ty = {}
        err_sg_ty = {}
        local_sg_ty = {}
        for l in ls:
            if l.startswith('========================================'):
                all_k_400 = True
                all_fix = True
                for k in local_sg_ty.keys():
                    if k not in err_sg_ty:
                        err_sg_ty[k] = 0
                    err_sg_ty[k] += 1
                    if not errno.startswith('E04'):
                        all_k_400 = False
                    else:
                        all_fix = all_fix and local_sg_ty[errno]
                if len(local_sg_ty) > 0:
                    print(local_sg_ty, 'all import', all_k_400, 'all 400 fix', all_fix)
                local_sg_ty = {}
            try:
                info = json.loads(l)
                errno = info['message']['code']['code']
                has_fix = False
                try:
                    for child in info['message']['children']:
                        for span in child['spans']:
                            if span['suggested_replacement']:
                                has_fix=True
                except:
                    pass
                if errno not in err_ty:
                    err_ty[errno] = 0
                err_ty[errno] += 1
                if errno not in local_sg_ty:
                    local_sg_ty[errno] = has_fix
                local_sg_ty[errno] = local_sg_ty[errno] and has_fix
            except:
                pass
        for k in local_sg_ty.keys():
            if k not in err_sg_ty:
                err_sg_ty[k] = 0
            err_sg_ty[k] += 1
        print(local_sg_ty)
        return err, succeed, total, err_ty, err_sg_ty


if __name__ == '__main__':
    all = {}
    for fd in os.listdir('.'):
        if os.path.isdir(fd) and os.path.exists(fd+'/inject.log'):
            print('='*80)
            err, succeed, total, err_ty, err_sg_ty = parse_file(fd)
            for k,v in err_sg_ty.items():
                if k not in all:
                    all[k] = 0
                all[k] += v
            print(fd, err, succeed, total, err_ty, err_sg_ty)
    res = {k: v for k, v in sorted(all.items(), key=lambda item: item[1], reverse=True)}
    print(res)