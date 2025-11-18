# Implementation Plan

## Fazy implementacji

### Faza 1: MVP (Minimum Viable Product)
1. **Poprawić Cargo.toml**
   - Edition 2021
   - Podstawowe dependencies: image, clap, csv, serde, rand, chrono

2. **Zaimplementować shared.rs**
   - Struktura Image (wrapper na image::RgbImage)
   - Funkcja dodawania szumu salt-and-pepper
   - I/O (load/save image)
   - Utility: median calculation, padding

3. **Zaimplementować sequential.rs**
   - Podstawowy filtr medianowy
   - Obsługa kernel 3x3 i 5x5
   - Mirror padding dla brzegów

4. **Zaimplementować main.rs**
   - CLI z clap (derive API)
   - Timing (std::time::Instant)
   - Zapis do CSV
   - Orchestracja: load -> noise -> filter -> save -> measure

5. **Testy podstawowe**
   - Unit testy dla median calculation
   - Integration test: end-to-end

### Faza 2: Równoległość (Rayon)
6. **Zaimplementować parallel.rs**
   - Rayon par_iter() po wierszach
   - Każdy wątek przetwarza wiersz
   - Shared read-only access do input image

7. **Testy wydajnościowe**
   - Porównanie seq vs par
   - Różne rozmiary obrazów
   - Speedup analysis

### Faza 3: Distributed (MPI) - opcjonalne
8. **Zaimplementować distributed.rs**
   - Podział obrazu na części (row-wise)
   - MPI scatter/gather
   - Ghost cells dla brzegów między procesami

9. **Testy distributed**
   - Uruchomienie z mpirun
   - Weryfikacja poprawności
   - Scalability tests

### Faza 4: GPU (Metal) - opcjonalne
10. **Zaimplementować gpu.rs**
    - Metal shader/kernel
    - Każdy wątek = jeden piksel
    - Shared memory dla kafli (optymalizacja)

11. **Testy GPU**
    - Weryfikacja poprawności
    - Porównanie z CPU
    - GPU utilization

### Faza 5: Raport
12. **Zbieranie danych**
    - Różne rozmiary obrazów (512x512, 1024x1024, 2048x2048)
    - Różne kernel sizes (3x3, 5x5)
    - Różne poziomy szumu (0.05, 0.1, 0.2)
    - Wszystkie metody (seq/par/dist/gpu)

13. **Jupyter notebook**
    - Wczytanie CSV
    - Wykresy: czas vs rozmiar obrazu
    - Speedup charts
    - PSNR/SSIM analysis
    - Wnioski i obserwacje

## Priorytet
**seq -> par -> dist -> gpu -> raport**

Najpierw działająca wersja sekwencyjna, potem optymalizacje.
