#!/usr/bin/env python3
import argparse
import subprocess
import sys
from pathlib import Path

COMPILERS = {
    ".cpp": lambda src, out: ["g++", "-O2", "-o", str(out), str(src)],
    ".rs":  lambda src, out: ["rustc", "-O", "-o", str(out), str(src)],
    ".cu":  lambda src, out: ["nvcc", "-O2", "-o", str(out), str(src)],
}

def compile(src: Path) -> Path:
    suffix = ".exe" if sys.platform == "win32" else ""
    out = src.with_suffix(suffix)
    result = subprocess.run(COMPILERS[src.suffix](src, out))
    if result.returncode != 0:
        sys.exit("Compilation failed.")
    return out

def main():
    parser = argparse.ArgumentParser(description="Run a solution with input from a txt file.")
    parser.add_argument("file", help="Solution file (e.g. codeforces/cpp/1234A.cpp)")
    parser.add_argument("--input", "-i", help="Input file (default: input.txt next to solution)")
    args = parser.parse_args()

    src = Path(args.file)
    if not src.exists():
        sys.exit(f"File not found: {src}")

    input_file = Path(args.input) if args.input else src.parent / "input.txt"
    if not input_file.exists():
        sys.exit(f"Input file not found: {input_file}\nCreate it and add your test cases.")

    stdin = input_file.read_text()

    ext = src.suffix
    if ext in COMPILERS:
        cmd = [str(compile(src))]
    elif ext == ".py":
        cmd = [sys.executable, str(src)]
    else:
        sys.exit(f"Unsupported extension: {ext}")

    print(f"--- output ---")
    subprocess.run(cmd, input=stdin, text=True)

if __name__ == "__main__":
    main()
