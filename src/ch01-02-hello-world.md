## Hello, World!

Nyt kun olet asentanut Rustin, on aika kirjoittaa ensimmäinen Rust-ohjelmasi.
On perinteistä, että uutta ohjelmointikieltä opetellessa ensimmäinen ohjelma
tulostaa ruudulle tekstin `Hello, world!`, joten teemme saman täällä!

> Huom: Tämä kirja olettaa, että olet perillä komentorivin käytöstä. Rust ei
> aseta erityisiä vaatimuksia siitä, mitä editoria tai työkaluja käytät tai missä koodisi sijaitsee.
> Jos haluat käyttää komentorivin sijaan suosikki-IDE:täsi, se on täysin sallittua. Monissa IDE:issä on nykyään Rust-tuki;
> tarkista IDE:si dokumentaatio lisätietoja varten. Rust-tiimi on panostanut **`rust-analyzer`**-työkaluun,
> joka mahdollistaa erinomaisen IDE-tuotetuen. Katso lisätietoja [Liitteestä D][devtools]<!-- ignore -->.

### Projektihakemiston luominen

Aloitat luomalla hakemiston, johon tallennat Rust-koodisi. Rust ei välitä,
missä koodisi sijaitsee, mutta tämän kirjan harjoituksia ja projekteja varten
suosittelemme luomaan **_projects_**-hakemiston kotihakemistoosi ja pitämään kaikki projektit siellä.

Avaa terminaali ja suorita seuraavat komennot luodaksesi **_projects_**-hakemiston
sekä **"Hello, world!"** -projektihakemiston sen sisään.

Linuxilla, macOS:llä ja Windowsin PowerShellillä:

```console
$ mkdir ~/projects
$ cd ~/projects
$ mkdir hello_world
$ cd hello_world
```

Windowsin CMD:ssä:

```cmd
> mkdir "%USERPROFILE%\projects"
> cd /d "%USERPROFILE%\projects"
> mkdir hello_world
> cd hello_world
```

### Rust-ohjelman kirjoittaminen ja suorittaminen

Seuraavaksi luo uusi lähdetiedosto ja nimeä se **_main.rs_**. Rust-tiedostot päättyvät aina **.rs**-päätteeseen.
Jos tiedostonimi koostuu useammasta sanasta, käytä alaviivaa erottamaan ne toisistaan, esim. **_hello_world.rs_**.

Avaa nyt **_main.rs_**-tiedosto ja kirjoita siihen seuraava koodi (Listaus 1-1):

<Listing number="1-1" file-name="main.rs" caption="Ohjelma, joka tulostaa `Hello, world!`">

```rust
fn main() {
    println!("Hello, world!");
}
```

</Listing>

Tallenna tiedosto ja siirry takaisin terminaaliin **_~/projects/hello_world_**-hakemistossa. Käännä ja suorita ohjelma:

Linuxilla tai macOS:llä:

```console
$ rustc main.rs
$ ./main
Hello, world!
```

Windowsissa suorita ohjelma komennolla:

```powershell
> rustc main.rs
> .\main.exe
Hello, world!
```

Riippumatta käyttöjärjestelmästäsi, terminaalissa tulisi näkyä:

```text
Hello, world!
```

Jos et näe tätä tulostetta, katso [vianmääritysosio][troubleshooting]<!-- ignore -->.
Jos ohjelma tulosti `Hello, world!`, olet juuri kirjoittanut ensimmäisen Rust-ohjelmasi – onnittelut! 🎉

### Rust-ohjelman anatomia

Käydään läpi ohjelman osat yksityiskohtaisesti. Ensimmäinen koodilohko:

```rust
fn main() {

}
```

Nämä rivit määrittävät funktion nimeltä **`main`**. Tämä funktio on erityinen: **kaikki Rust-ohjelmat käynnistyvät `main`-funktiosta**.
Ensimmäinen rivi määrittää funktion, jolla ei ole parametreja eikä se palauta mitään.

Funktion runko on suljettu **`{}`**-merkkien sisään. Rust vaatii aaltosulkeet jokaisen funktion ympärille.
On hyvä tyyli sijoittaa avauskaarisulje `main`-funktion määrittelyrivin perään ja jättää yksi välilyönti väliin.

> Huom: Rust-projektien koodityylin yhtenäistämiseen voit käyttää **`rustfmt`**-työkalua, joka muotoilee koodin
> automaattisesti. **`rustfmt`** on sisällytetty Rustin viralliseen jakeluun.

Funktion **`main`** sisällä on yksi rivi koodia:

```rust
println!("Hello, world!");
```

Tämä rivi tekee kaiken työn: se **tulostaa tekstin ruudulle**. Kolme tärkeää asiaa:

1. **`println!` on Rust-makro**. Jos se olisi funktio, sen nimi olisi `println` ilman huutomerkkiä (`!`).
2. **"Hello, world!"** on merkkijono, joka välitetään **`println!`**-makrolle.
3. **Rivi päättyy puolipisteeseen (`;`)**, mikä päättää lausekkeen ja siirtyy seuraavaan.

### Kääntäminen ja ajaminen ovat erillisiä vaiheita

Rust on **etukäteen käännettävä (ahead-of-time compiled)** kieli, mikä tarkoittaa, että ohjelma on **käännettävä ennen suorittamista**.
Ohjelma käännetään komennolla:

```console
$ rustc main.rs
```

Kääntämisen jälkeen syntyy suoritettava **binaaritiedosto**. Näet sen hakemistosta:

Linux/macOS:

```console
$ ls
main  main.rs
```

Windows CMD:

```cmd
> dir /B
main.exe
main.pdb
main.rs
```

Tiedostojen joukossa on **_main_**- (tai Windowsissa **_main.exe_**) -tiedosto, joka voidaan suorittaa komennolla:

```console
$ ./main    # tai Windowsissa: .\main.exe
```

**Dynaamisesti tulkittavissa kielissä** (kuten Python, Ruby tai JavaScript) ohjelmat ajetaan **suoraan ilman kääntämistä**.  
Rust toimii toisin: **voit kääntää ohjelman kerran ja jakaa sen ilman, että vastaanottajan täytyy asentaa Rustia.**

`rustc`-komennolla kääntäminen riittää pieniin ohjelmiin, mutta suuremmissa projekteissa haluat hallita riippuvuuksia
ja tehdä kehityksestä tehokkaampaa. Tässä kohtaa **Cargo-työkalu** tulee avuksi! Seuraavaksi tutustumme **Cargoon**.

[troubleshooting]: ch01-01-installation.html#troubleshooting
[devtools]: appendix-04-useful-development-tools.html
