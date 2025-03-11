## I/O-projektin parantaminen

Nyt kun olemme oppineet iteraattoreista, voimme parantaa luvussa 12 tehtyä I/O-projektia käyttämällä iteraattoreita koodin selkeyttämiseen ja tiivistämiseen. Katsotaan, kuinka voimme parantaa `Config::build`- ja `search`-funktioiden toteutusta.

### `clone`-kutsun poistaminen iteraattorin avulla

Listing 12-6:ssa loimme `Config`-rakenteen käyttämällä viipaletta `String`-arvoja, ja kopioimme (`clone`) arvot, jotta `Config`-instanssi omistaisi ne. **Listing 13-17** toistaa `Config::build`-funktion alkuperäisen toteutuksen **Listing 12-23**:sta:

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-23-reproduced/src/lib.rs:ch13}}
```

Tuolloin sanoimme, ettei `clone`-kutsujen tehokkuuteen kannata kiinnittää huomiota vielä, mutta nyt on aika optimoida se!

`clone`-kutsuja tarvittiin, koska `args`-parametri oli viipale `String`-arvoja, mutta `build`-funktio ei omistanut `args`-viipaletta. Jotta `Config` voisi omistaa `query`- ja `file_path`-kentät, sen täytyi **kopioida ne** (`clone`).

Nyt voimme muuttaa `build`-funktion käyttämään **iteraattoria** viipaleen sijaan. Tämä parantaa luettavuutta, koska voimme käyttää iteraattoria arvojen hakemiseen suoraan ilman viittausindeksointia.

Kun `Config::build` ottaa iteraattorin omistukseensa ja lopettaa indeksien käytön, voimme **siirtää (`move`)** arvot `Config`-rakenteeseen ilman `clone`-kutsua.

### Suoraan palautetun iteraattorin käyttäminen

Avaa I/O-projektisi _src/main.rs_-tiedosto. Se näyttää tältä:

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-12-24-reproduced/src/main.rs:ch13}}
```

Muokataan `main`-funktion alkua **Listing 13-18** mukaiseksi:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-18/src/main.rs:here}}
```

Tämä ei vielä käänny, koska meidän täytyy päivittää myös `Config::build`-funktio.

Tässä uudessa toteutuksessa `env::args` palauttaa **iteraattorin**! Sen sijaan, että keräisimme arvot vektoriin ja välittäisimme viipaleen `Config::build`-funktiolle, välitämme nyt iteraattorin **suoraan**.

Seuraavaksi päivitetään `Config::build`-funktion määritys _src/lib.rs_-tiedostossa **Listing 13-19** mukaiseksi:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-19/src/lib.rs:here}}
```

Standardikirjaston dokumentaatio `env::args`-funktiolle kertoo, että se palauttaa **`std::env::Args`**-tyypin iteraattorin, joka toteuttaa `Iterator`-traitin ja palauttaa `String`-arvoja.

Päivitämme `Config::build`-funktion siten, että `args` on **geneerinen tyyppi**, joka toteuttaa `Iterator<Item = String>` -traitin. Tämä tarkoittaa, että se voi olla mikä tahansa iteraattori, joka palauttaa `String`-arvoja.

Koska otamme `args`-iteraattorin omistukseen ja **käytämme sitä muuttuvana**, lisäämme `mut`-määreen.

### `Iterator`-traitin metodien käyttö indeksien sijaan

Seuraavaksi päivitetään `Config::build`-funktion runko. Koska `args` on iteraattori, voimme kutsua sen `next`-metodia! **Listing 13-20** päivittää **Listing 12-23**:n käyttämään `next`-metodia:

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-20/src/lib.rs:here}}
```

Ensimmäinen `next`-kutsu ohittaa ohjelman nimen. Toinen `next`-kutsu asettaa `query`-muuttujan arvon, ja jos arvoa ei ole, palautamme virheen. Sama tehdään `file_path`-muuttujalle.

### Koodin yksinkertaistaminen iteraattorisovittimilla

Voimme parantaa myös `search`-funktion toteutusta käyttämällä iteraattorisovittimia. **Listing 13-21** näyttää alkuperäisen toteutuksen **Listing 12-19**:stä:

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:ch13}}
```

Voimme kirjoittaa tämän tiiviimmin käyttäen `filter`-iteraattoria, mikä myös poistaa tarpeen käyttää muuttuvaa `results`-vektoria. **Listing 13-22** näyttää parannetun version:

```rust,ignore
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-22/src/lib.rs:here}}
```

Tämä toimii kuten aiempi toteutus, mutta käyttää `filter`-metodia, joka säilyttää vain ne rivit, jotka täyttävät ehdon `line.contains(query)`. Lopuksi `collect` kerää tulokset vektoriin.

Voit tehdä saman muutoksen myös `search_case_insensitive`-funktiolle.

### Looppien ja iteraattorien valitseminen

Pitäisikö meidän valita alkuperäinen toteutus **Listing 13-21** vai iteraattoriversio **Listing 13-22**?

Useimmat Rust-ohjelmoijat suosivat **iteraattoreita**, koska ne:

- Tiivistävät koodia ja vähentävät muuttuvan tilan käyttöä.
- Selkeyttävät tarkoitusta: sen sijaan, että hallitsemme **loopin yksityiskohtia**, keskitymme **tehtävään**.
- Voivat **parantaa suorituskykyä**, koska Rust optimoi iteraattorit erittäin tehokkaasti.

Mutta onko iteraattoriversio todella yhtä tehokas kuin perinteinen `for`-silmukka? Intuitiivisesti voisi ajatella, että matalan tason silmukka on nopeampi. Seuraavaksi tutkimme suorituskykyä ja optimointia Rustissa!
