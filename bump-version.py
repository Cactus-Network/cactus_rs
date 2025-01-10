#!/usr/bin/env python3

# usage:
# bump-version <new-version-string> <previous-release-tag>

import os
import re
import sys
from pathlib import Path
from typing import Callable

v = sys.argv[1]
tag = sys.argv[2]

our_crates = [
    "crates/cactus-bls",
    "crates/clvm-traits",
    "crates/cactus-traits",
    "crates/cactus_py_streamable_macro",
    "crates/cactus_streamable_macro",
    "crates/cactus-protocol",
    "crates/cactus-tools",
    "crates/clvm-utils",
    "crates/clvm-derive",
    "crates/cactus-puzzles",
    "crates/cactus-client",
    "crates/cactus-ssl",
    "crates/cactus-consensus",
    "crates/cactus-consensus/fuzz",
    "crates/cactus-puzzles/fuzz",
    "crates/clvm-utils/fuzz",
]

def crates_with_changes() -> set[str]:
    ret = set()
    for c in our_crates:
        diff = os.popen(f"git diff {tag} -- {c}").read().strip()
        if len(diff) > 0:
            ret.add(c)
    # the python wheel is the top-level build target, we always want to bump its
    # version
    ret.add("wheel")
    return ret

def update_cargo(name: str, crates: set[str]) -> None:
    subst = ""
    with open(f"{name}/Cargo.toml") as f:
        for line in f:
            split = line.split()
            if split == []:
                subst += line
                continue

            if split[0] == "version" and name in crates:
                line = f'version = "{v}"\n'
            elif split[0] in crates and line.startswith(split[0] + " = "):
                line = re.sub('version = "([>=^]?)\d+\.\d+\.\d+"', f'version = "\\g<1>{v}"', line)
            subst += line

    with open(f"{name}/Cargo.toml", "w") as f:
        f.write(subst)


crates = crates_with_changes()
# always update the root crate (cactus)
crates.add(".")
crates.add("cactus")

crate_names = set([Path(n).name for n in crates])

print("bumping version of crates:")
for c in crate_names:
    print(f" - {c}")

for c in our_crates:
    update_cargo(c, crate_names)

update_cargo(".", crate_names)
update_cargo("wheel", crate_names)
