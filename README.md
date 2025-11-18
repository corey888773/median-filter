# Median Filter - Image Denoising

Projekt implementujący filtr medianowy do usuwania szumu impulsowego (salt-and-pepper) z obrazów. Zawiera cztery różne implementacje:
- **Sequential (seq)**: Implementacja sekwencyjna
- **Parallel (par)**: Implementacja równoległa z użyciem Rayon
- **GPU (gpu)**: Implementacja GPU z użyciem WGPU/WGSL
- **Distributed (dist)**: Implementacja rozproszona z użyciem MPI

## Wymagania

### Kompilacja i uruchomienie
- **Rust** (edition 2021)
- **Open MPI** (dla wersji distributed)
  ```bash
  brew install open-mpi  # macOS
  ```

### Raport i analiza
- **Python 3.14+**
- **Jupyter Lab**
- Biblioteki: pandas, matplotlib, seaborn, numpy

## Instalacja

1. Sklonuj repozytorium:
```bash
git clone <repository-url>
cd median-filter
```

2. Zbuduj projekt:
```bash
cargo build --release
```

3. Przygotuj środowisko Python dla raportu:
```bash
python3 -m venv venv
source venv/bin/activate
pip install jupyter pandas matplotlib seaborn numpy
```

## Użycie

### Podstawowe użycie

```bash
./target/release/median-filter -i <input> -o <output> [OPTIONS]
```

**Parametry:**
- `-i, --input <PATH>`: Ścieżka do obrazu wejściowego
- `-o, --output <PATH>`: Ścieżka do obrazu wyjściowego
- `-n, --noise <LEVEL>`: Poziom szumu (0.0-1.0, domyślnie 0.1)
- `-m, --method <METHOD>`: Metoda filtrowania (seq/par/gpu/dist)
- `-k, --kernel <SIZE>`: Rozmiar kernela (3 lub 5, domyślnie 3)

### Przykłady

**Sequential (sekwencyjny):**
```bash
./target/release/median-filter -i image.jpg -o output.jpg -n 0.1 -m seq -k 3
```

**Parallel (równoległy):**
```bash
./target/release/median-filter -i image.jpg -o output.jpg -n 0.1 -m par -k 3
```

**GPU:**
```bash
./target/release/median-filter -i image.jpg -o output.jpg -n 0.1 -m gpu -k 3
```

**Distributed (MPI) z 4 procesami:**
```bash
mpirun -np 4 ./target/release/median-filter -i image.jpg -o output.jpg -n 0.1 -m dist -k 3
```

## Benchmarki

Uruchom automatyczne benchmarki dla wszystkich metod:

```bash
./run_benchmarks.sh
```

Skrypt wykonuje:
- 10 runów dla każdej konfiguracji
- Testy dla kernel 3x3 i 5x5
- Testy MPI dla 2, 4, 8 procesów
- Wyniki zapisywane do `results/results.csv`

## Raport

Wygeneruj raport z analizą wydajności:

```bash
source venv/bin/activate
jupyter lab analiza.ipynb
```

Raport zawiera:
- Statystyki czasów przetwarzania (mean, std, min, max)
- Wykresy porównawcze dla wszystkich metod
- Analizę przyspieszenia (speedup) względem wersji sekwencyjnej
- Analizę skalowalności MPI
- Metryki jakości (PSNR, SSIM)
- Przykłady obrazów przed/po odszumianiu

## Metryki jakości

Program oblicza dwie metryki jakości odszumiania:

- **PSNR (Peak Signal-to-Noise Ratio)**: Wyższe wartości = lepsza jakość (typowo 20-50 dB)
- **SSIM (Structural Similarity Index)**: Zakres -1 do 1, gdzie 1 = identyczne obrazy (dobre wartości > 0.9)

## Struktura projektu

```
median-filter/
├── src/
│   ├── main.rs           # CLI i główna logika
│   ├── shared.rs         # Wspólne funkcje (noise, median, PSNR, SSIM)
│   ├── sequential.rs     # Implementacja sekwencyjna
│   ├── parallel.rs       # Implementacja równoległa (Rayon)
│   ├── gpu.rs            # Implementacja GPU (WGPU/WGSL)
│   └── distributed.rs    # Implementacja rozproszona (MPI)
├── results/              # Wyniki benchmarków i obrazy
├── analiza.ipynb         # Jupyter notebook z analizą
├── run_benchmarks.sh     # Skrypt do automatycznych testów
└── README.md             # Ten plik
```

## Wyniki wydajności

Przykładowe wyniki dla obrazu 1920x1080, kernel 3x3, noise 0.1:

| Metoda      | Czas (ms) | Speedup | PSNR (dB) | SSIM  |
|-------------|-----------|---------|-----------|-------|
| Sequential  | ~3500     | 1.0x    | ~28.0     | ~0.88 |
| Parallel    | ~440      | ~8.0x   | ~28.0     | ~0.88 |
| GPU         | ~235      | ~15.0x  | ~28.0     | ~0.88 |
| Distributed | ~2070     | ~1.7x   | ~28.0     | ~0.88 |

*Uwaga: Wyniki mogą się różnić w zależności od sprzętu*