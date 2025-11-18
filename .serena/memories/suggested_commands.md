# Suggested Commands

## Build & Run
```bash
# Build projektu
cargo build

# Build w trybie release (optymalizacje)
cargo build --release

# Uruchomienie (debug)
cargo run -- --input image.png --noise 0.1 --method seq --kernel 3 --output output.png

# Uruchomienie (release, szybsze)
cargo run --release -- --input image.png --noise 0.1 --method par --kernel 5 --output output.png
```

## CLI Examples
```bash
# Sekwencyjna wersja, kernel 3x3, szum 10%
cargo run --release -- -i input.png -n 0.1 -m seq -k 3 -o output_seq.png

# Równoległa wersja (Rayon), kernel 5x5, szum 20%
cargo run --release -- -i input.png -n 0.2 -m par -k 5 -o output_par.png

# Distributed (MPI) - wymaga kompilacji z feature "mpi"
cargo run --release --features mpi -- -i input.png -n 0.1 -m dist -k 3 -o output_dist.png

# GPU (Metal) - wymaga kompilacji z feature "gpu"
cargo run --release --features gpu -- -i input.png -n 0.1 -m gpu -k 3 -o output_gpu.png
```

## Testing
```bash
# Uruchomienie wszystkich testów
cargo test

# Testy z outputem
cargo test -- --nocapture

# Testy konkretnego modułu
cargo test sequential
```

## Code Quality
```bash
# Formatowanie kodu
cargo fmt

# Sprawdzenie formatowania (CI)
cargo fmt -- --check

# Linting
cargo clippy

# Linting wszystkich targets
cargo clippy --all-targets

# Linting z warnings as errors
cargo clippy -- -D warnings
```

## Utilities (macOS/Darwin)
```bash
# Listowanie plików
ls -la

# Znajdowanie plików
find . -name "*.rs"

# Grep w plikach
grep -r "median" src/

# Git
git status
git add .
git commit -m "message"
git push
```

## Jupyter Notebook
```bash
# Uruchomienie Jupyter
jupyter notebook raport.ipynb
```
