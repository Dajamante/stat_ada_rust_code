# Rusty Regex Types Counter

Have you ever wondered how many times you use Vec, String or &mut in your Rust project, or Record in your Ada project? Probably not, but I needed it for my thesis.

Rusty Regex Type Counter is a small Rust program that reads through a directory and its subdirectories, looking for certain patterns using regular expressions. Specifically, it counts the occurrences of different patterns such as numbers, arrays, references, strings, and various heap-allocated types.

## Getting Started

Run the program: 

```
$ cargo run -- --code-base <path-to-code-base>
```

## How it Works

The program creates a list of regular expressions to use for pattern matching, then loops through each subdirectory in the specified directory, processing each file in the directory and its subdirectories.


Once all files have been processed, the program adds up the counts from all hashmaps and creates a final hashmap with the total counts for each pattern. Finally, the program prints out the final hashmap to the console.


## Results

Once the program finishes running, it will print out a bunch of statistics to the console. You can then use the plotter or Tikz for plotting.

```yaml
{
    "String": 18613,
    "Array": 9291,
    "Numbers": 304618,
    "Struct": 48955,
    "Vec": 14278,
    "Box/Rc/Arc": 6971,
    "Enum": 15040
}
```

## Scripts

### Git hash script

Structure of the directory:

```
rust$ tree -L 2
.
├── cratesio
│   ├── cfg-if
│   ├── libc
│   ├── proc-macro2
│   ├── quote
│   ├── rand
│   └── syn
├── librs
│   ├── clap
│   ├── json
│   ├── log
│   ├── serde
│   ├── thiserror
│   └── tokio
├── log.sh
└── rustc
    ├── Cargo.lock
    ├── compiler
    ├── CONTRIBUTING.md
    ├── COPYRIGHT
    ├── README.md
    └── x.py
```

Script that iterates through the sub directories and get the git hash. This example is for Rust.
```
#!/bin/bash

# Three sub repos in rust/ repo
for dir in $(find . -maxdepth 3 -name '.git' -type d -printf '%h\n'); do
    cd "$dir" || continue
    last_commit=$(git rev-parse --short=7 HEAD)
    echo "$dir: $last_commit"
    # Print only the last line relevant to me
    rust_lines=$(cloc . | grep "Rust" | awk '{print $NF}')    
    echo "Rust lines: $rust_lines"
    cd - > /dev/null || continue
done

```