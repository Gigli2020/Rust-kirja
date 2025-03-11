## Sulkeiset: Anonyymit funktiot, jotka voivat tallentaa ympäristönsä

Rustin **sulkeiset** (*closures*) ovat anonyymejä funktioita, jotka voidaan tallentaa muuttujaan tai välittää argumenttina toiselle funktiolle. Sulkeinen voidaan määrittää yhdessä paikassa ja suorittaa toisessa yhteydessä. Toisin kuin tavalliset funktiot, sulkeiset voivat tallentaa ja käyttää arvoja määrittelynsä ympäristöstä.

Sulkeiset tarjoavat tehokkaita tapoja uudelleenkäytettävyyden ja käyttäytymisen mukauttamisen toteuttamiseen. Tässä luvussa tutustumme niiden käyttöön ja hyötyihin.

### Sulkeisten ympäristön tallentaminen

Kuvitellaan skenaario: T-paitayrityksemme tarjoaa satunnaisesti ilmaisia, rajoitetun erän paitoja postituslistalle liittyneille käyttäjille. Käyttäjät voivat valita suosikkivärinsä, ja jos heidän värivalintansa on määritetty, he saavat sen värisen paidan. Jos väriä ei ole määritelty, he saavat yleisimmin varastossa olevan värin.

Voisimme toteuttaa tämän monin eri tavoin. Seuraavassa esimerkissä käytämme `ShirtColor`-enumia, jolla on kaksi vaihtoehtoa: `Red` ja `Blue`. Varastoa edustaa `Inventory`-rakenne, jossa on `shirts`-kenttä, joka sisältää `Vec<ShirtColor>`-vektorin varastossa olevista väreistä. `giveaway`-metodi määrittää käyttäjälle annettavan paidan värin.

<Listing number="13-1" file-name="src/main.rs" caption="T-paitayrityksen kampanjan toteutus">

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-01/src/main.rs}}
```

</Listing>

Tässä `giveaway`-metodissa käytämme `unwrap_or_else`-metodia `user_preference`-arvon käsittelyyn. Tämä metodi ottaa parametrina **sulkeisen**, jota suoritetaan vain, jos käyttäjällä ei ole värivalintaa.

Sulkeinen `|| self.most_stocked()` ei ota argumentteja (`||` tarkoittaa tyhjää argumenttilistaa). Sen sisällä kutsutaan `self.most_stocked()`-metodia, joka palauttaa suosituimman värin.

Tämän koodin suorittaminen tulostaa:

```console
{{#include ../listings/ch13-functional-features/listing-13-01/output.txt}}
```

Sulkeinen tallentaa muuttujan `self` viittauksen, mahdollistaen `most_stocked`-metodin kutsumisen. Tätä ei voisi tehdä tavallisella funktiolla, koska funktiot eivät voi tallentaa ympäristönsä muuttujia samalla tavalla.

### Sulkeisten tyyppipäätelmät ja annotaatiot

Sulkeisille ei yleensä tarvitse määrittää tyyppiannotaatiota parametreille tai palautusarvolle, koska Rust pystyy päättelemään ne. Tämä eroaa tavallisista funktioista, joissa tyyppiannotaatiot ovat pakollisia, koska ne muodostavat julkisen rajapinnan.

Jos kuitenkin haluat tehdä tyypit eksplisiittisiksi, voit määrittää ne seuraavasti:

```rust
let add_one_v1 = |x: u32| -> u32 { x + 1 };
let add_one_v2 = |x| { x + 1 };
let add_one_v3 = |x| x + 1;
```

Sulkeisten syntaksi on joustava, ja Rust päättelee puuttuvat tiedot niiden käytöstä.

### Viitteiden tallentaminen ja omistajuuden siirtäminen

Sulkeiset voivat käyttää arvoja kolmella tavalla, jotka vastaavat tapoja, joilla funktiot voivat vastaanottaa parametreja:

1. **Lainaamalla muuttumattomana** – `&T`
2. **Lainaamalla muuttuvana** – `&mut T`
3. **Ottamalla omistajuuden** – `T`

Esimerkiksi seuraava sulkeinen lainaa `list`-vektorin muuttumattomana:

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-04/src/main.rs}}
```

Tulostus:

```console
{{#include ../listings/ch13-functional-features/listing-13-04/output.txt}}
```

Jos haluamme muokata `list`-vektoria, sulkeisen on lainattava se muuttuvana:

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-05/src/main.rs}}
```

Tulostus:

```console
{{#include ../listings/ch13-functional-features/listing-13-05/output.txt}}
```

Tässä tapauksessa sulkeinen tarvitsee **mutattavan viitteen** `list`-muuttujaan. Tästä syystä Rust ei salli muita viittauksia `list`-muuttujaan ennen kuin sulkeinen on käytetty.

Jos sulkeinen tarvitsee **ottaa omistajuuden** arvosta, käytetään `move`-avainsanaa:

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-06/src/main.rs}}
```

Tämä on erityisen hyödyllistä, kun siirrämme sulkeisen uudelle säikeelle (*thread*). `move` varmistaa, että sulkeinen omistaa kaikki tarvitsemansa arvot.

### `Fn`-traitit ja sulkeisten ominaisuudet

Sulkeiset voivat toteuttaa yhden tai useamman seuraavista trait-määrityksistä:

- **`FnOnce`** – Sulkeinen voidaan kutsua **vain kerran**, jos se **siirtää omistajuuden**.
- **`FnMut`** – Sulkeinen voidaan kutsua useita kertoja, mutta se **voi muuttaa ympäristöään**.
- **`Fn`** – Sulkeinen voidaan kutsua useita kertoja **ilman sivuvaikutuksia**.

Esimerkiksi `unwrap_or_else` käyttää `FnOnce`-traitia, koska sulkeista käytetään enintään kerran:

```rust,ignore
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

Toisaalta `sort_by_key` tarvitsee `FnMut`-traitin, koska se kutsuu sulkeista useita kertoja:

```rust,noplayground
{{#rustdoc_include ../listings/ch13-functional-features/listing-13-07/src/main.rs}}
```

Sulkeinen `|r| r.width` ei siirrä omistajuutta tai muokkaa ympäristöään, joten se toimii `FnMut`-traitin kanssa.

---

Tässä osiossa opimme:

- **Sulkeisten käyttämisen ympäristön arvojen kanssa**.
- **Sulkeisten tyyppipäätelmät ja annotaatiot**.
- **Omistajuuden ja viittausten hallinnan sulkeisissa**.
- **`Fn`-traitien vaikutukset sulkeisten käyttöön**.

Seuraavaksi tutustumme **iteraattoreihin**, jotka mahdollistavat tehokkaan tiedon käsittelyn Rustissa!
