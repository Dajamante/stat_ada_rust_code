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
