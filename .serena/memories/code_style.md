# Code Style & Conventions

## Rust Standard Conventions
- **Nazewnictwo**:
  - `snake_case` dla funkcji, zmiennych, modułów
  - `CamelCase` dla typów, struktur, enumów
  - `SCREAMING_SNAKE_CASE` dla stałych
  
- **Formatowanie**: `cargo fmt` (rustfmt)
- **Linting**: `cargo clippy` (zero warnings policy)

## Struktura modułów
- Jeden plik = jedna implementacja filtra
- Spójny interfejs publiczny:
  ```rust
  pub fn apply_median_filter(image: &Image, kernel_size: usize) -> Image
  ```

## Dokumentacja
- Doc comments (`///`) dla wszystkich publicznych API
- Przykłady użycia w doc comments gdzie sensowne
- README.md z instrukcjami uruchomienia

## Obsługa błędów
- `Result<T, E>` dla operacji mogących się nie udać
- `panic!` tylko dla nieodwracalnych błędów
- Informacyjne komunikaty błędów

## Testy
- Unit testy w tym samym pliku (`#[cfg(test)]`)
- Integration testy w `tests/`
- Testy wydajnościowe (benchmarki) opcjonalnie

## Shared code
- Wspólne struktury i funkcje w `shared.rs`
- Unikanie duplikacji kodu między implementacjami
- Reużywalne utility functions (noise generation, I/O, metrics)
