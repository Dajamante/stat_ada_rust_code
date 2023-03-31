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

## Regexes used

### Rust regexes

#### Numbers: 
```rust
r#"\b(?:\d+(?:\.\d+)?|\d+(?:_\d+)+)\b"#
```
This regex matches integers and floats, as well as numbers with underscore since it is allowed by Rust. Because it is fenced by boundaries, it should not match type declarations, or numbers inside a struct (please see next regex for enum and struct).

It will match the any number.

```rust
//match
let a = 5;
let b = 10_000;
let c = 3.14;
//not match
let d:f32;
```
#### enum and struct

A very simple regex matching declaration of enums and structs.

```rust
struct Point {
    x: f32,
    y: f32,
}
enum Color {
    //...
}
```
  


#### array: 

`r#"\[(\s*\d+\s*(?:,\s*\d+\s*)*)\]"#`

This regex matches array literals, but not their declaration.

The rationale is that more often then not the compiler does not need the declaration and can figure out the type. But in case we have the declaration, we don't want to match twice.

```rust
// matches array, but not [i32;5]
let a : [i32; 5] = [1, 2, 3, 4, 5];
```

#### References

`r"&[mut\s]*\w+"`

This regex matches reference types in Rust, including mutable and immutable references.

```rust
let x = &x;
let y = &mut x;
```

#### Heap types

`"Box<[^<>]+>|Rc<[^<>]+>|Arc<[^<>]+>")`

This regex matches the declaration of Rust heap-allocated types, specifically Box, Rc, and Arc, followed by `<`.

```rust
let boxed_value: Box<i32> = Box::new(5);
let rc_value: Rc<String> = Rc::new(String::from("Hello"));
let arc_value: Arc<Vec<u8>> = Arc::new(vec![1, 2, 3]);
```

#### Dynamic vectors

This regex matches dynamic `Vec` types and the `vec![]` macro in Rust.

```rust
let v = vec![1, 2, 3];
let w: Vec<i32> = Vec::new();
```

#### String

This pattern matches instances where a String is created or manipulated in Rust. Usually with a method `.to_string()`, `String::new()` or the `format!` macro.

String::: This part of the regex pattern matches any method or associated function called on the String type. Examples include `String::new()` and `String::from("...")` or `...to_string()`. The pattern searches for the "String::" substring in the code.

```rust
et name = ”Yacouba”;
let age = 12;
let formatted = format!(”{} is {} years old”, name, age);
```
### SPARK regexes


#### Number

`Integer|Float|Fixed|Decimal|Modular|Natural|Positive|Long|range \d .. \d`

Ada/SPARK is stricter than Rust in the declarations, and requires a type declaration. This pattern match the possible numeric types.

This pattern matches for example:

```Ada
type Integer is range -2_147_483_648 .. 2_147_483_647;
type Positive is range 1 .. Integer'Last;
type Float is digits 6;
```


#### struct

This regex matches record types in Ada, which are similar to structs in other languages. The pattern is record looks for the "is record" 

```ada
type Point is record
    X : Integer;
    Y : Integer;
end record;
```

#### enum

This regex matches enumeration types in Ada.
This pattern looks for a type, a name, the word is and an immediate parenthesis.

```ada
type Color is (Red, Green, Blue);
```
#### arrays

`type [a-zA-Z_]+ is array\(\d.."`
This regex matches array declarations in Ada. In the same manner, it combines keywords type, is and array, opening parenthesis and a numeric type, to not match on Dynamic vectors.

Example:

```ada
type Int_Array is array (1 .. 10) of Integer;
```

#### Dynamic vectors

This regex matches unconstrained arrays in Ada, specifically those using range <> or the Containers.Vector package.

```ada
type Int_Vector is array (Integer range <>) of Integer;
package Int_Vectors is new Containers.Vectors (Index_Type => Integer, Element_Type => Integer);
```


The pattern range <>|Containers.Vector looks for either the "range <>" substring, which represents an unconstrained array index, or the "Containers.Vector" substring, which is used for instantiating the Containers.Vectors generic package.

#### References regex

This pattern looks memory allocation and access in Ada. It looks either for "access" (for access types, the Ada/SPARK pointers) or "new" (for memory allocation).

```ada
type Int_Ptr is access Integer;
X : aliased Integer := 42;
Ptr : Int_Ptr := X'Access;
```

#### Unbounded strings.

This regex matches unbounded strings in Ada.

```ada
use Ada.Strings.Unbounded;
S : Unbounded_String := To_Unbounded_String("Hello, world!");
```


The pattern Strings.Unbounded looks for the "Strings.Unbounded" substring, which indicates the use of the Ada.Strings.Unbounded package and its Unbounded_String type.

