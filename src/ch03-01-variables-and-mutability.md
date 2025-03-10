## Muuttujat ja muuttumattomuus

Kuten mainittiin [“Arvojen tallentaminen muuttujilla”][storing-values-with-variables]<!-- ignore --> -osiossa, 
muuttujat ovat oletusarvoisesti muuttumattomia. Tämä on yksi Rustin tavoista kannustaa kirjoittamaan koodia, 
joka hyödyntää turvallisuutta ja rinnakkaisuuden helppoutta. Sinulla on kuitenkin mahdollisuus tehdä muuttujista myös 
muuttuvia. Tarkastellaan, miksi Rust suosii muuttumattomuutta ja milloin saatat haluta käyttää muuttujia.

Kun muuttuja on muuttumaton, sen arvoa ei voi muuttaa sen jälkeen, kun se on kerran sidottu nimeen. 
Kokeillaan tätä luomalla uusi projekti nimeltä **variables**:

```console
$ cargo new variables
$ cd variables
```

Avaa **src/main.rs** ja kirjoita seuraava koodi:

```rust,ignore,does_not_compile
fn main() {
    let x = 5;
    x = 6; // Tämä aiheuttaa virheen
    println!("Arvo on: {}", x);
}
```

Käännä ohjelma:

```console
$ cargo run
```

Saat virheilmoituksen, joka kertoo, että muuttuja `x` on muuttumaton:

```text
error: cannot assign twice to immutable variable `x`
```

### Miksi muuttumattomuus on hyödyllistä?

On tärkeää, että saamme **käännösaikaisia virheitä**, kun yritämme muuttaa muuttumatonta arvoa. 
Jos jokin osa koodista olettaa, että arvo ei muutu, ja toinen osa koodia muuttaa sen, tämä voi johtaa 
vaikeasti löydettäviin bugeihin. Rustin kääntäjä **takaa**, että muuttumaton arvo ei muutu, 
joten ohjelmoijan ei tarvitse itse seurata sitä.

Mutta muuttuvuus voi olla hyödyllistä! Voimme tehdä muuttujasta muuttuvan lisäämällä `mut`-avainsanan:

```rust
fn main() {
    let mut x = 5;
    println!("Arvo on: {}", x);
    x = 6;
    println!("Uusi arvo on: {}", x);
}
```

Nyt kääntäjä sallii arvon muuttamisen.

### Vakiot

Vakiot (`const`) ovat aina muuttumattomia, eikä niitä voi määrittää `mut`-avainsanalla. 
Vakioita käytetään arvoihin, jotka pysyvät samana koko ohjelman suorituksen ajan.

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

Vakiot kirjoitetaan **suuraakkosin** ja sanat erotetaan alaviivalla.

### Varjostaminen (Shadowing)

Rust tukee **varjostamista**, jossa voit määrittää **uuden muuttujan samalla nimellä** kuin aiempi muuttuja:

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("Sisäisen lohkon x: {}", x);
    }

    println!("Ulkopuolisen lohkon x: {}", x);
}
```

Tässä `x` muuttuu useita kertoja, mutta joka kerta se luodaan **uudelleen** `let`-avainsanalla. 
Toisin kuin `mut`, varjostaminen voi myös **muuttaa muuttujan tyypin**:

```rust
fn main() {
    let spaces = "   ";
    let spaces = spaces.len(); // Nyt spaces on numero
}
```

`mut`-avainsanalla tämä ei onnistu, koska Rust ei salli muuttujan **tyypin muuttamista** ilman uutta määrittelyä.

---

Tässä osiossa opimme:

- Rustin muuttujat ovat oletusarvoisesti **muuttumattomia**
- `mut` mahdollistaa muuttujien **muokkaamisen**
- **Vakiot** (`const`) pysyvät aina samana
- **Varjostaminen** mahdollistaa saman muuttujanimen uudelleenkäytön

Seuraavaksi perehdymme Rustin **tietotyyppeihin**! 🚀

[storing-values-with-variables]: ch02-00-guessing-game-tutorial.html#storing-values-with-variables
[data-types]: ch03-02-data-types.html#data-types
[const-eval]: ../reference/const_eval.html
