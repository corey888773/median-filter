# Task Completion Checklist

Po zakończeniu implementacji każdej funkcjonalności, wykonaj:

## 1. Formatowanie
```bash
cargo fmt
```
- Upewnij się, że kod jest sformatowany zgodnie z rustfmt

## 2. Linting
```bash
cargo clippy --all-targets
```
- Napraw wszystkie warningi
- Zero warnings policy

## 3. Testy jednostkowe
```bash
cargo test
```
- Wszystkie testy muszą przechodzić
- Dodaj nowe testy dla nowej funkcjonalności

## 4. Weryfikacja funkcjonalna
- Uruchom program z przykładowymi danymi
- Sprawdź czy obrazy wyjściowe są poprawne
- Zweryfikuj czy pliki CSV są generowane

## 5. Testy wydajnościowe
```bash
# Porównaj czasy dla różnych metod
cargo run --release -- -i test.png -n 0.1 -m seq -k 3 -o out_seq.png
cargo run --release -- -i test.png -n 0.1 -m par -k 3 -o out_par.png
```
- Sprawdź czy wersja równoległa jest szybsza
- Zweryfikuj poprawność pomiarów w CSV

## 6. Build release
```bash
cargo build --release
```
- Upewnij się, że build release działa bez błędów

## 7. Dokumentacja
- Zaktualizuj README.md jeśli dodano nowe funkcjonalności
- Dodaj doc comments do nowych publicznych API

## 8. Git
```bash
git status
git add .
git commit -m "Descriptive commit message"
```
- Commit z opisowym komunikatem
- Push tylko po weryfikacji powyższych kroków
