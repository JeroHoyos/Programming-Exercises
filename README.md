# Programming Exercises

Personal competitive programming workspace. Problems from Codeforces, LeetCode, and LeetGPU.

## Structure

```text
codeforces/
  cpp/
  rust/
  python/
leetcode/
  cpp/
  rust/
  python/
leetgpu/
  cuda/
templates/
```

## New problem

```text
python new_problem.py <cf|lc|gpu> <name> --lang <cpp|rs|py|cu>
```

```bash
python new_problem.py cf 1234A --lang cpp
python new_problem.py lc two-sum --lang rs
python new_problem.py gpu matmul --lang cu
```

## Run

Place an `input.txt` next to the solution file, then:

```bash
python run.py codeforces/cpp/1234A.cpp
python run.py leetcode/rust/two-sum.rs --input input.txt
```

Compiles automatically for C++, Rust, and CUDA. Python runs directly.

## Languages

| Platform   | C++ | Rust | Python | CUDA |
|------------|-----|------|--------|------|
| Codeforces | ✓   | ✓    | ✓      |      |
| LeetCode   | ✓   | ✓    | ✓      |      |
| LeetGPU    |     |      |        | ✓    |

