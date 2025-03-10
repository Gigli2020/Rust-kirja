## Hello, Cargo!

Cargo on Rustin rakennusjärjestelmä ja pakettienhallintaohjelma. Suurin osa Rust-kehittäjistä käyttää tätä työkalua hallitakseen Rust-projektejaan, 
koska Cargo hoitaa monia tehtäviä puolestasi, kuten koodisi kääntämisen, kirjastojen lataamisen ja niiden kääntämisen. (Näitä kirjastoja kutsutaan 
**riippuvuuksiksi**).

Yksinkertaisimmat Rust-ohjelmat, kuten aiemmin kirjoittamamme `Hello, world!`, eivät käytä riippuvuuksia. Jos olisimme rakentaneet projektin 
Cargoa käyttäen, se hyödyntäisi vain Cargoa kääntämiseen. Kun Rust-projektisi monimutkaistuu ja lisäät riippuvuuksia, Cargo tekee hallinnasta huomattavasti helpompaa.

Koska **valtaosa Rust-projekteista käyttää Cargoa**, tämä kirja olettaa, että käytät sitä myös. Cargo asennetaan automaattisesti Rustin mukana, jos 
käytit [asennusohjeen][installation]<!-- ignore --> mukaisia virallisia asennusmenetelmiä. Jos asensit Rustin toisella tavalla, voit tarkistaa, onko Cargo asennettu suorittamalla:

```console
$ cargo --version
```

Jos näet versionumeron, Cargo on käytettävissäsi! Jos saat virheen, kuten `command not found`, tarkista käyttämäsi asennusmenetelmä ja asenna Cargo tarvittaessa erikseen.

### Projektin luominen Cargolla

Luodaan uusi projekti Cargoa käyttäen ja katsotaan, miten se eroaa alkuperäisestä `Hello, world!` -projektista. Siirry **projects**-hakemistoon tai 
mihin tahansa, missä haluat säilyttää koodisi. Suorita seuraava komento:

```console
$ cargo new hello_cargo
$ cd hello_cargo
```

Ensimmäinen komento luo uuden hakemiston ja projektin nimeltä **hello_cargo**. Cargo sijoittaa projektin tiedostot hakemistoon, jolla on sama nimi.

Kun siirryt **hello_cargo**-hakemistoon ja tarkastelet tiedostoja, huomaat, että Cargo on luonut seuraavat:

- **Cargo.toml** -tiedosto
- **src/** -hakemisto, joka sisältää **main.rs**-tiedoston
- **.gitignore**-tiedosto (jos käytössäsi on Git)

Cargo alustaa uuden Git-repositorion oletuksena. Jos suoritat `cargo new` -komennon olemassa olevan Git-repositorion sisällä, Cargo ei luo Git-tiedostoja.

> Huom: Cargo tukee myös muita versionhallintajärjestelmiä tai voit poistaa versionhallinnan käytöstä `--vcs`-lipulla.

Avaa **Cargo.toml**-tiedosto, jonka sisältö vastaa seuraavaa (Listaus 1-2):

<Listing number="1-2" file-name="Cargo.toml" caption="Cargo.toml-tiedoston sisältö">

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2024"

[dependencies]
```

</Listing>

Tiedosto käyttää **TOML**-muotoilua (**Tom’s Obvious, Minimal Language**), jota Cargo käyttää asetustiedostoissaan.

Tiedoston ensimmäinen osa `[package]` on otsikko, joka osoittaa, että alla olevat rivit määrittävät paketin asetuksia. Tässä tapauksessa määrittelemme:

- Projektin nimi (**name**)
- Versio (**version**)
- Rust-versio (**edition**), jota projekti käyttää

[dependencies]-osio listaa projektin mahdolliset riippuvuudet. Tässä projektissa niitä ei ole, mutta myöhemmissä projekteissa niitä lisätään.

Avaa nyt **src/main.rs**-tiedosto:

```rust
fn main() {
    println!("Hello, world!");
}
```

Cargo on luonut automaattisesti `Hello, world!` -ohjelman puolestasi!

### Cargon käyttäminen ohjelman kääntämiseen ja suorittamiseen

Käännä ja suorita ohjelma **Cargolla** seuraavilla komennoilla:

```console
$ cargo build
$ ./target/debug/hello_cargo
```

Tämä kääntää ohjelman ja sijoittaa sen **target/debug/**-hakemistoon. Voit myös käyttää:

```console
$ cargo run
```

Tämä kääntää ohjelman ja suorittaa sen yhdellä komennolla:

```console
Hello, world!
```

Cargo käyttää **älykästä käännösmekanismia** ja **ei käännä uudelleen**, jos koodia ei ole muutettu.

Voit tarkistaa, toimiiko koodisi ilman koko ohjelman kääntämistä käyttämällä:

```console
$ cargo check
```

`cargo check` on huomattavasti nopeampi kuin `cargo build`, koska se ei luo suoritettavaa ohjelmaa.

### Julkaisuversion rakentaminen

Kun ohjelma on valmis tuotantoon, voit kääntää sen optimoiduksi **julkaisuversiona** seuraavalla komennolla:

```console
$ cargo build --release
```

Tämä luo suoritettavan tiedoston **target/release/**-hakemistoon, joka on huomattavasti nopeampi mutta vie enemmän aikaa kääntää.

### Yhteenveto

Olet nyt oppinut:

- Luomaan uuden projektin Cargolla: `cargo new`
- Kääntämään projektin: `cargo build`
- Kääntämään ja ajamaan yhdellä komennolla: `cargo run`
- Tarkistamaan koodin ilman kääntämistä: `cargo check`
- Kääntämään julkaisuversion: `cargo build --release`

Cargo tarjoaa yhtenäisen kehityskokemuksen käyttöjärjestelmästä riippumatta. Tästä eteenpäin tämä kirja olettaa, että käytät Cargoa!

[installation]: ch01-01-installation.html#installation
[toml]: https://toml.io
[appendix-e]: appendix-05-editions.html
[cargo]: https://doc.rust-lang.org/cargo/
