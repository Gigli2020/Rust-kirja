## Tietotyypit

Rust on **staattisesti tyypitetty kieli**, mikä tarkoittaa, että jokaisen muuttujan ja arvon tyyppi tunnetaan kääntäjän aikana. Tämä auttaa tunnistamaan mahdolliset virheet jo ennen ohjelman suoritusta.

Tässä luvussa opimme:

- Rustin **skaalarityypit** (kokonaisluvut, liukuluvut, booleanit, merkit)
- **Yhdistelmätietotyypit** (tuplet ja taulukot)
- Miten **tyypit määritellään ja muutetaan**

Rust päättää useimmiten muuttujan tyypin automaattisesti, mutta joskus saatat haluta määrittää tyypin itse.

### Skaalarityypit

Rust tukee neljää perustietotyyppiä, jotka edustavat yksittäisiä arvoja:

| Tyyppi       | Kuvaus |
|-------------|------------|
| **Kokonaisluvut** | Esim. `i32`, `u64` |
| **Liukuluvut** | Esim. `f32`, `f64` |
| **Booleanit** | `true` tai `false` |
| **Merkit** | Esim. `'a'`, `'😊'` |

### Kokonaisluvut

Kokonaisluvut voivat olla **allekirjoitettuja (`i`)** tai **allekirjoittamattomia (`u`)**. Esimerkiksi:

- `i8`, `i16`, `i32`, `i64`, `i128`, `isize` → Allekiroitettuja
- `u8`, `u16`, `u32`, `u64`, `u128`, `usize` → Allekiroittamattomia

Jos et määritä tyyppiä, Rust käyttää **oletuksena `i32`-tyyppiä**.

```rust
let x: i32 = 42;
let y: u8 = 255;
```

### Liukuluvut

Rust tukee **`f32`** ja **`f64`** tyyppejä. **Oletuksena Rust käyttää `f64`**, koska se on nopeampi useimmilla moderneilla prosessoreilla.

```rust
let x = 2.5; // Oletuksena f64
let y: f32 = 3.14;
```

### Boolean

Boolean-tyypillä (`bool`) on kaksi mahdollista arvoa: `true` tai `false`.

```rust
let is_rust_fun: bool = true;
```

### Merkki (char)

Merkki (`char`) on yksittäinen Unicode-merkki:

```rust
let heart_eyed_cat = '😻';
let letter = 'R';
```

### Yhdistelmätietotyypit

Rust tukee kahta pääasiallista yhdistelmätietotyyppiä:

1. **Tuplet** – Ryhmittää eri tyyppisiä arvoja yhteen muuttujaan.
2. **Taulukot** – Ryhmittää samantyyppisiä arvoja yhteen muuttujaan.

### Tuplet

Tuplessa voi olla eri tyyppisiä arvoja:

```rust
let tup: (i32, f64, char) = (500, 6.4, 'R');

// Arvojen hakeminen tuplesta
let (x, y, z) = tup;
println!("y:n arvo on: {}", y);

// Hakeminen indeksin perusteella
let first_value = tup.0;
```

### Taulukot

Taulukko (`array`) sisältää **saman tyyppisiä arvoja** ja sillä on kiinteä koko.

```rust
let numbers: [i32; 3] = [1, 2, 3];
let same_values = [5; 4]; // Vastaa: [5, 5, 5, 5]
```

Taulukon elementtiin viitataan indeksillä:

```rust
let first = numbers[0];
let second = numbers[1];
```

Jos yrität käyttää indeksin ulkopuolista arvoa, Rust antaa **runtime-virheen**!

---

Tässä osiossa opimme:

- Rustin **skaalarityypit** (kokonaisluvut, liukuluvut, booleanit, merkit)
- **Tuplet ja taulukot**, jotka yhdistävät useita arvoja

Seuraavaksi käsittelemme **funktioita**! 🚀
