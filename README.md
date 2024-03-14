## Jak uruchomić
Aby uruchomić aplikację należy posiadać wersję `1.76.0-nightly` (wersja z funkcjami, które nie są ustabilizowane w normalnej wersji) języka Rust lub nowszą. By uzyskać wersję nighly należy wykonać polecenia:
```bash
rustup toolchain install nightly
rustup default nightly
```
Należy również dodać architekturę kompilacji (architektura wasm pozwala na uruchamianie kodu min. przez przeglądarki):
```bash
rustup target add wasm32-unknown-unknown
```
Do wykonywania migracji na bazie danych będzie potrzebne dodatkowe narzędzie biblioteki `sqlx` o nazwie `sqlx-cli` w wersji `0.7.1`, które możemy pobrać używając komendy (`cargo` jest menadżerem bibliotek oraz aplikacji terminalowych języka Rust)
```bash
cargo install sqlx-cli
```
By skompilować stronę oraz udostępnić ją w internecię będzie potrzebne narzędzie `cargo-lepts` w wersji `2.16.0`, które zainstalujemy komendą:
```
cargo install cargo-leptos --version "2.16.0"
``` 
Po zainstalowaniu wymaganych aplikacji należy pobrać kod z platformy github:
```
git clone https://github.com/Eg-Nickname/axum-bank.git
```
A następnie utworzyć plik `.env` zawierający dane do połączenia się aplikacji z bazą danych:
```env
DATABASE_URL="postgres://user:password@localhost/axum_bank"
```
By skompilować i udostępnić stronę należy wykonać polecenie:
```
cargo leptos serve
```

## How to run
To run code you need:
- Rust version `1.76.0-nightly` or newer and wasm compilation target
```bash
rustup toolchain install nightly
rustup default nightly
rustup target add wasm32-unknown-unknown
```
- sql-cli version `0.7.1`
```bash
cargo install sqlx-cli
```
- cargo-leptos version `2.16.0`
```bash
cargo install cargo-leptos --version "2.16.0"
```
- .env file containing database credentials 
```env
DATABASE_URL="postgres://user:password@localhost/axum_bank"
```

To run application use command:
```bash
cargo leptos serve
```