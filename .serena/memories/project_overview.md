# Median Filter Project Overview

## Cel projektu
Implementacja filtra medianowego do odszumiania obrazów w czterech wersjach:
1. **Sequential** - wersja sekwencyjna (baseline)
2. **Parallel** - równoległość z Rayon (zamiennik OpenMP)
3. **Distributed** - przetwarzanie rozproszone z MPI
4. **GPU** - akceleracja GPU przez Metal (macOS)

## Struktura projektu
```
median-filter/
├── Cargo.toml          # Konfiguracja i zależności
├── src/
│   ├── main.rs         # CLI, orchestracja, pomiary
│   ├── shared.rs       # Wspólne struktury i funkcje
│   ├── sequential.rs   # Implementacja sekwencyjna
│   ├── parallel.rs     # Implementacja równoległa (Rayon)
│   ├── distributed.rs  # Implementacja rozproszona (MPI)
│   └── gpu.rs          # Implementacja GPU (Metal)
├── results/            # Pliki CSV z pomiarami
│   ├── seq.csv
│   ├── par.csv
│   ├── dist.csv
│   └── gpu.csv
├── raport.ipynb        # Jupyter notebook z analizą
└── README.md
```

## Funkcjonalności
- Wczytywanie obrazów z pliku
- Dodawanie szumu salt-and-pepper (parametr 0-1)
- Filtrowanie medianowe z kernelem 3x3 lub 5x5
- Wybór metody przetwarzania (seq/par/dist/gpu)
- Pomiary czasu wykonania
- Zapis wyników do CSV (timestamp, parametry, czas, PSNR, SSIM)
- Raport w Jupyter z wykresami i analizą wydajności

## Algorytm filtra medianowego
Dla każdego piksela:
1. Pobierz sąsiedztwo (3x3 lub 5x5)
2. Obsłuż brzegi (mirror/replicate padding)
3. Zbierz wartości pikseli
4. Posortuj i wybierz medianę
5. Zapisz jako nowy kolor piksela
