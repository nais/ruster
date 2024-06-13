# nais ruster: intro og cheat sheet
Dette er en praktisk guide til Rust.
Her lærer du å installere Rust, lage nye prosjekter, strukturere filer, importere dependencies, og mange andre ting.
Vi skal oppdager hvordan Rust sitt typesystem og tidvis funksjonelle syntaks ofte fører til elegante design, korrekt kode, færre bugs, og fornøyde utviklere.
Hvis du kan Go, vil du sette pris på de praktiske sammenlikningene av hvordan man gjør en spesifikk ting både i Rust.
Rust kan gjøre alt som Go kan, og mye mer.

## Basics
Start med å legge inn alt av software og dependencies for å komme i gang med utviklingen. På slutten av denne modulen skal du kunne sette opp et nytt Rust-prosjekt og kompilere det.

### Rustup
Rustup er Rust sin toolchain-pakkemanager og brukes for å legge inn compileren for din arkitektur.

```bash
# Mac OS X
brew install rustup

# Arch Linux
pacman -S rustup

# Alt annet
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Og deretter init:
```bash
rustup-init
```

### Installere toolchain med Rustup
Her installeres cargo, som fungerer som compiler, linker, pakkemanager, etc.

```bash
# Installere toolchain for native arch
rustup update

# Installere toolchain for WebAssembly (valgfritt)
rustup target add wasm32-unknown-unknown
```

### Cargo
Cargo er one-stop-shop for bygg, test, dependencies.

Kommandolinjeverktøyene `cargo` og `go` likner på hverandre:

```bash
# Dependencies
cargo add PAKKE
cargo add -F native-tls PAKKE  # med features
cargo remove PAKKE

# Kompilering
cargo build
cargo build --release

# Andre ting
cargo run
cargo test
cargo bench
cargo update
```

### Filstruktur på prosjektet
* All kildekode går i `src/*.rs` og eventuelt underkataloger
* `src/lib.rs` for libraries
* `src/main.rs` for prosjekter som lager en binær
* `src/bin/*.rs` for prosjekter som lager flere binærer, samme stilen som `cmd/*/main.go` for Go

### Moduler
Når koden blir stor kan den brytes ned i moduler.
Hver `*.rs` fil er sin egen modul. Moduler kan nestes i filtreet.
Moduler kan deklareres i `main.rs`:

```rust
// foo.rs
// bar.rs
// mymod/submodule.rs
pub mod foo;
mod bar;
mod mymod::submodule; // verify
```

Moduler kan også implementeres inni en annen modulfil:

```rust
// main.rs

fn main() {
  crate::mymodule::foo(); // 42
}

mod mymodule {
    fn foo() -> usize {
        42
    }
}
```

### Doc på cargo.rs
For alle pakker som er publisert gjennom cargo finner man dokumentasjon på https://cargo.rs.

Litt liknende konsept som https://pkg.go.dev.

Man kan også bruke docs.rs til å lese dokumentasjon for pakker: https://docs.rs.

### RustRover
IntelliJ sitt IDE for Rust.

```bash
# Mac OS X
brew install --cask rustrover

# Arch Linux
paru -S rustup

# Alt annet
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```
### Hello world
```bash
cargo new MITTPROSJEKT
cd MITTPROSJEKT
cargo run
```

## Typesystemet
* Vanlige typer: int, byte, string, vectors, hashmaps
* Result<T> og Option<T> og mangel på NULL
* camelCase, snake_case, FirstCase, UPPERCASE
* Enums og enums med structs
* Higher order functions (closures)
* Iterators/funksjonell programmering
* Mutable, references
* Makroer

## Avanserte konsepter
* Ownership: Hvem eier variabelen og verdien?
* Krysskompilering med cross
* Lifetimes
* Async
* Default variabler
* Linting, static check
* Testing
* std
    * Path, filesystem, File I/O
    * Child processes
    * Threads, concurrency, locks, mutexes
    * Channels (multiple producers, single consumer)

## Libs
### Kubernetes
* https://github.com/kube-rs/kube (ikke testet)

### Serialisering (JSON/YAML)
* https://serde.rs/
* https://github.com/ethiraric/yaml-rust2 (ikke testet)

### GraphQL
* Server: https://github.com/graphql-rust/juniper (ikke testet)
* Klient: https://github.com/graphql-rust/graphql-client (ikke testet)

### Protobuf & gRPC
* https://github.com/tokio-rs/prost (Proto + gRPC, ikke testet)

### HTTP server/client
* https://docs.rs/axum/latest/axum/index.html (Web application framework. Cirka Go-chi + snacks)
* https://docs.rs/tower/latest/tower/ (middleware)
* https://docs.rs/hyper/latest/hyper/ (ikke testet, low level http)
* https://crates.io/crates/reqwest (ikke testet)

### Postgres
* https://docs.rs/sqlx/latest/sqlx/ (async sql toolkit for Rust, likner på sqlc)

### Asynchronous runtime
Tokio is an event-driven, non-blocking I/O platform for writing asynchronous applications with the Rust programming language.
* https://docs.rs/tokio/latest/tokio/

### Configuration
* https://docs.rs/config/latest/config/ (ikke testet)

### Command-line interfaces
* https://docs.rs/clap/latest/clap/

### Pub/sub
* https://docs.rs/google-cloud-pubsub/latest/google_cloud_pubsub/ (ikke testet)

### Kafka

## Sammenlikne med Go
* Fordeler: økosystem, ergonomi, typesystem, funksjonell syntaks, match arms
* Ulemper: fighting the borrow checker, styrete å holde tunga rett i munnen i concurrency
* Gotchas hvor Go kan skyte deg i foten
* Slipper å bruke `context` siden async har en annen semantikk i Rust
* Tar litt lengre tid å kompilere
* Kompilatoren hjelper deg veldig
* Mye bedre generics

## Videre arbeid
* Jobbe litt med Rustlings - et program for læring, hvor man skriver Rust fra scratch, med progressivt mer avanserte teknikker
* Skrive en av kodeoppgavene til NAV
* Fra Christer: naisdevice approval page

## Utklipp
* https://doc.rust-lang.org/rust-by-example/

jmaker https://news.ycombinator.com/item?id=40159988

> In my experience, Go appears simple as long as you’re solving simple tasks. Once you need to model fairly complex semantics, whose structure and detail depend on your business domain, Go code turns illegible, with lots of hackily lumped together bloated nonsensical channels and for-selects stemming from async IO and multi-threading forced into the goroutines syntax, with hard to formulate structured concurrency. And then you bolt on your validation logic on top of it, with several Go idiosyncrasies, and then you need to explain the distinct consumer-side interfaces, underwhelming generics and myriads of conventions based rules. One could claim that’s not how one writes “good” Go code. But that’s what depends on the complexity and dynamism of your domain logic and the folks you work with. After Java, production grade Go for larger projects is terrible. Null pointer method receivers, interface reification. And now the experiment with the functorial ranges, increasingly with more and more genetics, all without any syntactic sugar… hard to read, hard to explain. Oh and the GC is rather inefficient compared to most JDK distributions. Compared to modern Java or Kotlin, for my project requirements, I don’t want to do as much Go as I used to. JDK Mission Control and Flight Recorder alone with great IDEs and profilers are golden. And you pay in RAM for JDK apps.
> 
> As for Rust, it just requires more upfront cost, but makes you consider potential failures at every step. Debugging production code isn’t fun, fuzz tests aren’t enough to assure quality to the extent Rust’s type system can. I trust rustc much more than I trust myself or other devs. If the code passes rustc, and the type semantics are correct, I’m 3/4 certain the code is correct, that’s a very high watermark, albeit at the cost of the upfront effort to satisfy the type constraints. Much more efficient in terms of dev time than debugging Python or Go at runtime.
> 
> Anyway, one can easily run many Python packages on GraalVM Truffle for direct interfacing with Java, Ruby, R, and via PyO3 with Rust, for which you have the native Polars data frame library, which all cover a good chunk of Python programs. Rust also teaches one to do proper ownership accounting for the variable bindings, great for C++ devs and for heap escape analysis by inspection for Go devs alike. Lots of Go bugs result from lack of understanding of ownership and implicit reference binding as opposed to copy and move semantics. Some of that was now “improved” with Go 1.22 for closures in loops.
