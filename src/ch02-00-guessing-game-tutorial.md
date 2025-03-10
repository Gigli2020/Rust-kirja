# Ohjelmoidaan Arvauspeli

Hypätään suoraan Rustin pariin työstämällä yhdessä käytännön projekti! Tämä luku esittelee sinulle muutamia yleisiä Rust-käsitteitä näyttämällä, 
miten voit käyttää niitä oikeassa ohjelmassa. Opit muun muassa `let`-määrittelyt, `match`-lausekkeet, metodit, assosioidut funktiot, ulkoiset kirjastot ja paljon muuta! 
Seuraavissa luvuissa perehdymme näihin aiheisiin tarkemmin, mutta tässä luvussa harjoittelemme vain perusasioita.

Toteutamme klassisen aloittelijatason ohjelmointitehtävän: **arvauspeli**. Näin se toimii: ohjelma generoi satunnaisen kokonaisluvun väliltä 1–100. 
Sen jälkeen se pyytää käyttäjää syöttämään arvauksen. Kun arvaus on annettu, ohjelma kertoo, onko se liian suuri vai liian pieni. Jos arvaus on oikein, ohjelma 
tulostaa onnitteluviestin ja sulkeutuu.

## Uuden projektin luominen

Luo uusi projekti siirtymällä _projects_-hakemistoon, jonka loit luvussa 1, ja luo uusi projekti käyttäen Cargoa:

```console
$ cargo new guessing_game
$ cd guessing_game
```

Ensimmäinen komento `cargo new` luo uuden projektin nimeltä **guessing_game**. Toinen komento siirtää sinut uuteen projektihakemistoon.

Tarkastellaan luotua **Cargo.toml**-tiedostoa:

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2024"

[dependencies]
```

Cargo generoi sinulle automaattisesti "Hello, world!" -ohjelman. Tarkista **src/main.rs**-tiedosto:

```rust
fn main() {
    println!("Hello, world!");
}
```

Nyt voimme kääntää ja suorittaa ohjelman käyttäen `cargo run` -komentoa:

```console
$ cargo run
```

`cargo run` on hyödyllinen komento, kun haluat nopeasti testata ohjelmaa muutosten jälkeen.

Avaa **src/main.rs**-tiedosto uudelleen. Kaikki ohjelmakoodimme kirjoitetaan tähän tiedostoon.

## Käyttäjän arvauksen käsittely

Ensimmäinen osa arvauspeliä pyytää käyttäjältä syötteen, käsittelee sen ja tarkistaa, että se on odotetussa muodossa. Aloitetaan mahdollistamalla käyttäjän syöttää arvaus:

```rust
use std::io;

fn main() {
    println!("Arvaa luku!");

    println!("Syötä arvauksesi:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Virhe luettaessa riviä");

    println!("Arvasit: {guess}");
}
```

Tässä koodissa käytämme **`std::io`**-kirjastoa saadaksemme käyttäjän syötteen ja tulostamme sen näytölle. Tarkastellaan seuraavaksi, mitä tämä koodi tekee rivi riviltä.

### Muuttujien käyttö

Seuraava rivi luo muuttujan käyttäjän syötteen tallentamiseksi:

```rust
let mut guess = String::new();
```

Tämä määrittelee muuttujan `guess`, joka on **muuttuva (`mut`)** ja alustetaan tyhjällä **String**-tyypillä.

### Käyttäjän syötteen lukeminen

Tässä kohtaa kutsumme **`io::stdin().read_line(&mut guess)`**, joka lukee käyttäjän syötteen:

```rust
io::stdin()
    .read_line(&mut guess)
    .expect("Virhe luettaessa riviä");
```

Rust edellyttää, että välitämme **viittauksen (&)** `guess`-muuttujaan, jotta funktio voi muokata sitä. `expect`-kutsu varmistaa, että ohjelma kaatuu selkeällä virheilmoituksella, jos syötteen lukeminen epäonnistuu.

### Tulostus

Viimeisenä ohjelma tulostaa käyttäjän syötteen:

```rust
println!("Arvasit: {guess}");
```

Tässä käytämme **`{}`**-paikantimia tulostaaksemme `guess`-muuttujan arvon.

## Satunnaisen luvun generointi

Nyt lisätään satunnaisen luvun generointi käyttäen **rand**-kirjastoa. Lisää **rand** Cargo-riippuvuuksiin:

```toml
[dependencies]
rand = "0.8.5"
```

Sitten päivitetään **src/main.rs**:

```rust
use std::io;
use rand::Rng;

fn main() {
    println!("Arvaa luku!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Syötä arvauksesi:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Virhe luettaessa riviä");

    println!("Arvasit: {guess}");
}
```

Tämä lisää satunnaisen luvun generoinnin ja tallentaa sen `secret_number`-muuttujaan.

## Arvauksen vertaaminen salaisuuteen

Nyt lisätään vertailu käyttäjän arvauksen ja salaisen numeron välillä:

```rust
use std::cmp::Ordering;

match guess.cmp(&secret_number) {
    Ordering::Less => println!("Liian pieni!"),
    Ordering::Greater => println!("Liian suuri!"),
    Ordering::Equal => {
        println!("Voitit!");
        break;
    }
}
```

## Silmukan lisääminen

Nyt voimme antaa käyttäjän arvata useita kertoja:

```rust
loop {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Virhe luettaessa riviä");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Liian pieni!"),
        Ordering::Greater => println!("Liian suuri!"),
        Ordering::Equal => {
            println!("Voitit!");
            break;
        }
    }
}
```

Tämä mahdollistaa **uusien arvauksien syöttämisen** niin kauan, kunnes käyttäjä arvaa oikein!

## Yhteenveto

Tässä luvussa rakensimme **arvauspelin** ja opimme Rustin peruskäsitteitä, kuten muuttujia, syötteen lukemista, `match`-lausekkeita ja Cargo-riippuvuuksia.

Seuraavaksi siirrymme tarkastelemaan Rustin perusrakenteita tarkemmin!

