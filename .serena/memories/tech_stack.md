# Tech Stack

## Język
- **Rust** (edition 2021)

## Główne zależności
- `image = "0.24"` - wczytywanie/zapisywanie obrazów
- `rayon = "1.8"` - równoległość (zamiennik OpenMP)
- `clap = { version = "4.4", features = ["derive"] }` - CLI
- `csv = "1.3"` - zapis wyników do CSV
- `serde = { version = "1.0", features = ["derive"] }` - serializacja
- `rand = "0.8"` - generowanie szumu
- `chrono = "0.4"` - timestampy

## Opcjonalne zależności (features)
- `rsmpi = "0.7"` - MPI dla wersji distributed (feature: "mpi")
- `metal = "0.27"` lub `wgpu = "0.18"` - GPU (feature: "gpu")

## Platforma
- macOS (Metal dla GPU)
- Darwin system

## Narzędzia deweloperskie
- `cargo` - build system
- `rustfmt` - formatowanie kodu
- `clippy` - linter
- Jupyter - raport (Python + pandas, matplotlib)
