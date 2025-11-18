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

## Specyfikacja sprzętu testowego

Testy wydajnościowe zostały przeprowadzone na następującej konfiguracji:

- **Komputer**: MacBook Pro 16-inch, Nov 2023
- **Procesor**: Apple M3 Pro
- **GPU**: Apple M3 Pro (zintegrowane GPU, Metal API)
- **Pamięć RAM**: 36 GB
- **Dysk**: Macintosh HD
- **System operacyjny**: macOS Sequoia 15.7.2
- **Kompilator**: Rust edition 2021 (release mode z optymalizacjami)
- **MPI**: Open MPI 5.0.8
- **GPU Backend**: Metal (przez WGPU 27.0.1)

## Opis działania algorytmu

### Filtr medianowy

Filtr medianowy to nieliniowy filtr cyfrowy używany do usuwania szumu z obrazów, szczególnie skuteczny w przypadku szumu impulsowego (salt-and-pepper). Algorytm działa następująco:

1. **Dla każdego piksela w obrazie**:
   - Wyznacz okno (kernel) o rozmiarze k×k wokół piksela (np. 3×3 lub 5×5)
   - Zbierz wszystkie wartości pikseli w tym oknie
   - Posortuj zebrane wartości
   - Zastąp wartość środkowego piksela medianą z posortowanej listy

2. **Obsługa brzegów**: Używamy mirror padding - piksele poza granicami obrazu są odbijane lustrzanie

3. **Szum salt-and-pepper**: Losowo wybrane piksele są ustawiane na 0 (czarny) lub 255 (biały)

### Implementacje

#### 1. Sequential (`src/sequential.rs`)

Podstawowa implementacja sekwencyjna:
- Iteruje przez każdy piksel obrazu w pętli
- Dla każdego piksela zbiera wartości z okna k×k
- Sortuje wartości i wybiera medianę
- Prosta, ale wolna dla dużych obrazów

**Złożoność**: O(W × H × k² × log(k²)), gdzie W×H to wymiary obrazu

#### 2. Parallel (`src/parallel.rs`)

Implementacja równoległa z użyciem biblioteki Rayon:
- Dzieli obraz na wiersze
- Każdy wątek przetwarza swoje wiersze niezależnie
- Wykorzystuje wszystkie dostępne rdzenie procesora
- Używa `par_iter()` do automatycznej paralelizacji

**Przyspieszenie**: ~8x na Apple M3 Pro (zależy od liczby rdzeni)

**Kod kluczowy**:
```rust
output_data.par_chunks_mut(width * 3)
    .enumerate()
    .for_each(|(y, row)| {
        // Przetwarzanie wiersza
    });
```

#### 3. GPU (`src/gpu.rs`)

Implementacja GPU z użyciem WGPU (WebGPU API) i shadera WGSL:
- **Backend**: Metal (natywne API GPU dla Apple Silicon)
- Przenosi obraz do pamięci GPU
- Każdy piksel przetwarzany przez osobny wątek GPU
- Shader WGSL wykonuje sortowanie i wybór mediany
- Format danych: RGB spakowane do u32 (R << 16 | G << 8 | B)

**Przyspieszenie**: ~15x na Apple M3 Pro GPU (Metal backend)

**Shader WGSL** (`src/gpu.rs`):
- Każdy workgroup przetwarza fragment obrazu
- Sortowanie bąbelkowe w shaderze (wystarczające dla małych kerneli)
- Wykorzystanie shared memory dla wydajności

**Uwaga**: WGPU automatycznie wybiera backend (Metal na macOS, Vulkan na Linux, DirectX na Windows)

#### 4. Distributed (`src/distributed.rs`)

Implementacja rozproszona z użyciem MPI:
- Dzieli obraz na poziome paski między procesy
- Każdy proces przetwarza swój fragment
- Wymiana "ghost rows" (wierszy brzegowych) między procesami
- Root process (rank 0) zbiera wyniki i składa obraz

**Komunikacja MPI**:
1. Root rozsyła wymiary obrazu do wszystkich procesów
2. Root wysyła fragmenty obrazu do każdego procesu
3. Procesy wymieniają ghost rows z sąsiadami
4. Każdy proces przetwarza swój fragment
5. Root zbiera przetworzone fragmenty

**Przyspieszenie**: ~1.7x dla 4 procesów (overhead komunikacji MPI)

**Uwaga**: MPI jest zaprojektowane dla klastrów - na jednej maszynie overhead komunikacji może przewyższać korzyści z paralelizacji.

### Wspólne funkcje (`src/shared.rs`)

#### Generowanie szumu
```rust
pub fn add_noise(img: &mut Image, noise_level: f32)
```
- Losowo wybiera piksele (zgodnie z `noise_level`)
- Ustawia je na 0 (czarny) lub 255 (biały) z prawdopodobieństwem 50/50

#### Obliczanie mediany
```rust
pub fn median(values: &mut [u8]) -> u8
```
- Sortuje tablicę wartości
- Zwraca środkowy element

#### Mirror padding
```rust
pub fn get_pixel_with_padding(img: &Image, x: i32, y: i32) -> [u8; 3]
```
- Obsługuje piksele poza granicami obrazu
- Odbija współrzędne lustrzanie

#### Metryki jakości

**PSNR (Peak Signal-to-Noise Ratio)**:
```
MSE = średnia((oryginalny - przetworzony)²)
PSNR = 20 × log₁₀(255) - 10 × log₁₀(MSE)
```
- Wyższe wartości = lepsza jakość
- Typowe wartości: 20-50 dB

**SSIM (Structural Similarity Index)**:
- Porównuje strukturę, luminancję i kontrast
- Obliczane w oknach 8×8 z krokiem 8
- Zakres: -1 do 1 (1 = identyczne obrazy)
- Dobre wartości: > 0.9

### Przepływ programu (`src/main.rs`)

1. **Parsowanie argumentów CLI** (clap)
2. **Wczytanie obrazu** (image crate)
3. **Dodanie szumu** salt-and-pepper
4. **Wybór metody filtrowania**:
   - Sequential: bezpośrednie wywołanie
   - Parallel: bezpośrednie wywołanie
   - GPU: inicjalizacja WGPU, transfer danych, wykonanie shadera
   - Distributed: inicjalizacja MPI, podział danych, komunikacja
5. **Pomiar czasu** (std::time::Instant)
6. **Obliczenie PSNR/SSIM** (porównanie z oryginalnym obrazem)
7. **Zapis wyniku** do pliku i CSV
8. **Wyświetlenie statystyk** w konsoli