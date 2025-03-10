## Liite D - Hyödylliset kehitystyökalut

Tässä liitteessä käsittelemme joitakin hyödyllisiä kehitystyökaluja, joita Rust-projekti tarjoaa. Käymme läpi automaattisen muotoilun, nopeita tapoja korjata varoituksia, linterin sekä IDE-integroinnin.

### Koodin automaattinen muotoilu `rustfmt`-työkalulla

`rustfmt`-työkalu muotoilee koodisi yhteisön hyväksymän koodityylin mukaisesti. Monet yhteistyöhankkeet käyttävät `rustfmt`-työkalua estääkseen kiistat siitä, mitä tyyliä Rust-koodissa tulisi käyttää – kaikki muotoilevat koodinsa tällä työkalulla.

Asenna `rustfmt` seuraavalla komennolla:

```console
$ rustup component add rustfmt
```

Tämä komento asentaa sekä `rustfmt`:n että `cargo-fmt`:n, aivan kuten Rust asentaa sekä `rustc`:n että `cargo`:n. Voit muotoilla minkä tahansa Cargo-projektin seuraavasti:

```console
$ cargo fmt
```

Tämä komento muotoilee kaiken nykyisen käännösyksikön Rust-koodin. Sen pitäisi muuttaa vain koodityyliä, ei koodin toiminnallisuutta. Lisätietoja `rustfmt`-työkalusta löytyy sen [dokumentaatiosta][rustfmt].

[rustfmt]: https://github.com/rust-lang/rustfmt

### Koodin korjaaminen `rustfix`-työkalulla

Rust-asennuksen mukana tuleva `rustfix`-työkalu voi automaattisesti korjata kääntäjän varoitukset, joiden ratkaisu on selkeä. Esimerkiksi alla olevassa koodissa:

<span class="filename">Tiedostonimi: src/main.rs</span>

```rust
fn main() {
    let mut x = 42;
    println!("{x}");
}
```

Muuttuja `x` on määritelty **mut**-avainsanalla, mutta sitä ei koskaan muokata. Rust antaa tästä varoituksen:

```console
$ cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: variable does not need to be mutable
 --> src/main.rs:2:9
  |
2 |     let mut x = 0;
  |         ----^
  |         |
  |         help: remove this `mut`
  |
  = note: `#[warn(unused_mut)]` on by default
```

Varoitus ehdottaa, että `mut`-avainsana poistetaan. Tämä voidaan tehdä automaattisesti `rustfix`-työkalulla suorittamalla seuraava komento:

```console
$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

Kun tarkastelemme tiedostoa **src/main.rs**, huomaamme, että `cargo fix` on päivittänyt koodin:

```rust
fn main() {
    let x = 42;
    println!("{x}");
}
```

Muuttuja `x` on nyt muuttumaton, eikä varoitusta enää näy.

`cargo fix` -komentoa voi myös käyttää koodin siirtämiseen eri Rust-versioiden välillä. Rustin eri versiot käsitellään [liitteessä E][editions].

[editions]: appendix-05-editions.md

### Lisää tarkastuksia Clippyllä

Clippy on kokoelma lintereitä, jotka analysoivat koodiasi ja auttavat löytämään yleisiä virheitä sekä parantamaan Rust-koodia.

Asenna Clippy seuraavasti:

```console
$ rustup component add clippy
```

Aja Clippy missä tahansa Cargo-projektissa:

```console
$ cargo clippy
```

Jos kirjoitat esimerkiksi ohjelman, joka käyttää likimääräistä matemaattista vakioarvoa, kuten piitä:

<span class="filename">Tiedostonimi: src/main.rs</span>

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("ympyrän pinta-ala on {}", x * r * r);
}
```

Suorittamalla `cargo clippy` saat seuraavan virheilmoituksen:

```text
error: approximate value of `f{32, 64}::consts::PI` found
 --> src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  |
  = note: `#[deny(clippy::approx_constant)]` on by default
  = help: consider using the constant directly
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#approx_constant
```

Clippy ilmoittaa, että Rustissa on jo tarkempi `PI`-vakio, jota tulisi käyttää.

Korjaamalla koodin:

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("ympyrän pinta-ala on {}", x * r * r);
}
```

Virheilmoitus katoaa, ja Clippy ei löydä huomautettavaa.

Lisätietoja Clippystä löytyy sen [dokumentaatiosta][clippy].

[clippy]: https://github.com/rust-lang/rust-clippy

### IDE-integraatio `rust-analyzer`-työkalulla

Rust-yhteisö suosittelee käyttämään [`rust-analyzer`][rust-analyzer]<!-- ignore --> -työkalua IDE-integraatioon. Tämä työkalu sisältää kääntäjälähtöisiä apuvälineitä ja tukee [Language Server Protocol][lsp]<!-- ignore --> -määritystä, jonka avulla eri IDE:t ja ohjelmointikielet voivat kommunikoida keskenään.

Asennusohjeet löytyvät `rust-analyzer`-projektin [kotisivulta][rust-analyzer]<!-- ignore -->. Asennuksen jälkeen saat käyttöösi ominaisuuksia kuten automaattisen täydennyksen, siirtymisen määrityksiin ja reaaliaikaiset virheilmoitukset.

[rust-analyzer]: https://rust-analyzer.github.io
[lsp]: http://langserver.org/
[vscode]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
