# Advent of Code Solutions

A collection of solutions for [Advent of Code](https://adventofcode.com/) challenges across multiple years and programming languages.

## Directory Structure

```
Advent_of_Code/
├── 2015/                    # Advent of Code 2015 solutions
│   ├── Day3/               # Day 3 solutions (Rust)
│   ├── Day4/               # Day 4 solutions (Rust)
│   └── Day5/               # Day 5 solutions (Rust)
│
├── 2025/                    # Advent of Code 2025 solutions
│   ├── day_01/             # Day 1 solutions (Rust)
│   │   ├── part_01/        # Part 1 solution
│   │   └── part_02/        # Part 2 solution
│   ├── day_02/             # Day 2 solutions (Rust)
│   ├── day_03/             # Day 3 solutions (Rust)
│   ├── day_04/             # Day 4 solutions (Rust)
│   ├── day_05/             # Day 5 solutions (Rust)
│   └── day_06/             # Day 6 solutions (Rust)
│
└── README.md                # This file
```

## Content Overview

### 2015 Solutions

The 2015 solutions are implemented in **Rust** and include compiled binaries alongside source code.

- **Day 3**: Includes two solutions (`main.rs` and `main2.rs`) with separate input files
- **Day 4**: Includes two solutions (`main.rs` and `main2.rs`) with compiled binaries (`main` and `main2`)
- **Day 5**: Includes two solutions (`main.rs` and `main2.rs`)

Each day typically contains:
- Input data (`input.txt`, `input2.txt`)
- Rust source files (`.rs`)
- Compiled executables (where present)

### 2025 Solutions

The 2025 solutions follow a more structured organization with explicit part separation, implemented in **Rust**.

**Days Available**: 1-6 (ongoing)

Each day is organized as:
```
day_XX/
├── part_01/
│   ├── main_part_one.rs    # Part 1 solution
│   ├── input.txt           # Puzzle input
│   └── input_test.txt      # Test input
│
└── part_02/
    ├── main_part_two.rs    # Part 2 solution
    ├── input.txt           # Puzzle input
    └── input_test.txt      # Test input
```

## File Naming Conventions

### 2015 Solutions
- Single directory per day containing both parts
- Solutions named `main.rs` and `main2.rs` (typically part 1 and part 2 respectively)
- Input files: `input.txt`, `input2.txt`

### 2025 Solutions
- Separate `part_01/` and `part_02/` directories per day
- Solutions named `main_part_one.rs`, `main_part_two.rs`, or `main.rs`
- Test inputs: `input_test.txt`
- Actual puzzle inputs: `input.txt`

## Languages Used

- **Rust** (`.rs`): Primary language for all solutions
  - 2015: 6 solution files
  - 2025: 11+ solution files (ongoing)

## Total Statistics

- **Total files**: 74
- **Year range**: 2015, 2025
- **Days completed**:
  - 2015: Days 3, 4, 5
  - 2025: Days 1-6

## Running Solutions

To run a Rust solution (2025 example):

```bash
cd 2025/day_01/part_01/
rustc main_part_one.rs -o main
./main < input.txt
```

For 2015 solutions with pre-compiled binaries:

```bash
cd 2015/Day3/
./main < input.txt
```

## About Advent of Code

[Advent of Code](https://adventofcode.com/) is an annual event featuring daily programming puzzles throughout December. Each puzzle has two parts, and solutions typically involve algorithmic problem-solving and programming skills.

## Notes

- Test inputs are provided for verification purposes
- Compiled binaries are included in the 2015 solutions directory
- Solutions are individual implementations without shared dependencies or libraries
