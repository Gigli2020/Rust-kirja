## Liite A: Avainsanat

Seuraava luettelo sisältää avainsanoja, jotka ovat varattuina Rust-kielelle joko nykyiseen tai tulevaan käyttöön. Näitä ei voi käyttää tunnisteina (paitsi raakatunnisteina, kuten käsitellään kohdassa "[Raakatunnisteet][raw-identifiers]<!-- ignore -->").  
Tunnisteet ovat nimiä, joita käytetään funktioissa, muuttujissa, parametreissa, rakenteiden kentissä, moduleissa, paketeissa, vakioissa, makroissa, staattisissa arvoissa, attribuuteissa, tyypeissä, rajapinnoissa (traits) tai elinaikojen hallinnassa.

[raw-identifiers]: #raw-identifiers

### Käytössä olevat avainsanat

Seuraava on lista tällä hetkellä käytössä olevista avainsanoista ja niiden toiminnoista:

- `as` - suorittaa primitiivisen tyypin muunnoksen, määrittää tietyn rajapinnan sisältävän kohteen tai uudelleennimeää kohteita `use`-lausunnoissa  
- `async` - palauttaa `Future`-olion estämättä nykyistä säiettä  
- `await` - keskeyttää suorituksen, kunnes `Future`-olion tulos on valmis  
- `break` - poistuu silmukasta välittömästi  
- `const` - määrittää vakioita tai vakio-osoittimia  
- `continue` - jatkaa seuraavaan silmukka-iteraatioon  
- `crate` - moduulipolussa viittaa paketin juureen  
- `dyn` - mahdollistaa dynaamisen viittaamisen rajapintaolioihin  
- `else` - varavaihtoehto `if`- ja `if let` -rakenteille  
- `enum` - määrittää luettelotyypin  
- `extern` - linkittää ulkoiseen funktioon tai muuttujaan  
- `false` - looginen epätosi-vakio  
- `fn` - määrittää funktion tai funktioviittauksen tyypin  
- `for` - käy läpi iteraattorista saatuja arvoja, toteuttaa rajapinnan tai määrittää korkeatasoisen eliniän  
- `if` - suorittaa ehdollisen haarautumisen  
- `impl` - toteuttaa sisäänrakennettua tai rajapinnan funktioita  
- `in` - osa `for`-silmukan syntaksia  
- `let` - sitoo muuttujan  
- `loop` - suorittaa silmukan loputtomasti  
- `match` - vertailee arvoa eri kaavoihin  
- `mod` - määrittää moduulin  
- `move` - siirtää sulkeuman muuttujat omistukseen  
- `mut` - määrittää muuttuvuuden viittauksissa, osoittimissa tai sidonnoissa  
- `pub` - määrittää näkyvyyden julkiseksi rakenteiden kentissä, `impl`-lohkoissa tai moduleissa  
- `ref` - sitoo muuttujan viittauksena  
- `return` - palauttaa funktion suorituksesta  
- `Self` - viittaa tämänhetkiseen tyyppiin, jota määritellään tai toteutetaan  
- `self` - viittaa metodin kohteeseen tai nykyiseen moduuliin  
- `static` - määrittää globaalin muuttujan tai arvon, joka säilyy ohjelman suoritusajan läpi  
- `struct` - määrittää rakenteen  
- `super` - viittaa nykyisen moduulin ylämoduuliin  
- `trait` - määrittää rajapinnan  
- `true` - looginen tosi-vakio  
- `type` - määrittää tyyppialiasin tai assosioidun tyypin  
- `union` - määrittää [unionin][union]<!-- ignore -->; tämä on avainsana vain unionin yhteydessä  
- `unsafe` - merkitsee turvattoman koodin, funktion, rajapinnan tai toteutuksen  
- `use` - tuo symboleja näkyviin; tarkentaa kaappauksia geneerisille ja elinikärajoille  
- `where` - määrittää ehdot, jotka rajoittavat tyyppiä  
- `while` - suorittaa ehdollisen silmukan  

[union]: ../reference/items/unions.html

### Tulevaisuuden varatut avainsanat

Seuraavat avainsanat eivät vielä ole käytössä, mutta ne on varattu Rustin mahdolliseen tulevaan käyttöön:

- `abstract`
- `become`
- `box`
- `do`
- `final`
- `gen`
- `macro`
- `override`
- `priv`
- `try`
- `typeof`
- `unsized`
- `virtual`
- `yield`

### Raakatunnisteet

**Raakatunnisteet** ovat syntaksi, jonka avulla voit käyttää avainsanoja tunnisteina siellä, missä niitä normaalisti ei sallittaisi. Raakatunniste muodostetaan lisäämällä avainsanan eteen `r#`-etuliite.

Esimerkiksi `match` on avainsana. Jos yrität kääntää seuraavan funktion, joka käyttää `match`-sanaa funktion nimenä:

```rust,ignore,does_not_compile
fn match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}
```

Saat seuraavan virheilmoituksen:

```text
error: expected identifier, found keyword `match`
 --> src/main.rs:4:4
  |
4 | fn match(needle: &str, haystack: &str) -> bool {
  |    ^^^^^ expected identifier, found keyword
```

Virhe kertoo, että et voi käyttää `match`-sanaa funktion nimenä. Tämän voi kuitenkin kiertää käyttämällä raakatunnisteita:

```rust
fn r#match(needle: &str, haystack: &str) -> bool {
    haystack.contains(needle)
}

fn main() {
    assert!(r#match("foo", "foobar"));
}
```

Tämä koodi kääntyy ilman virheitä. Huomaa `r#`-etuliite sekä funktion määrittelyssä että sen kutsussa `main`-funktiossa.

Raakatunnisteet mahdollistavat minkä tahansa sanan käytön tunnisteena, vaikka sana olisi muuten varattu avainsanaksi. Tämä antaa ohjelmoijalle enemmän vapautta valita muuttujien ja funktioiden nimet sekä helpottaa integraatiota muiden ohjelmointikielten kanssa, joissa nämä sanat eivät ole avainsanoja. Lisäksi raakatunnisteet mahdollistavat erilaisten Rust-versioiden yhteensopivuuden. Esimerkiksi `try` ei ollut avainsana Rust 2015 -versiossa, mutta siitä tuli sellainen Rust 2018, 2021 ja 2024 -versioissa. Jos riippuvuus käyttää vanhempaa Rust-versiota, saatat joutua käyttämään raakatunnistetta (`r#try`) kutsuessasi sen funktioita. Katso lisää tietoa Rustin eri versioista kohdasta [Liite E][appendix-e]<!-- ignore -->.

[appendix-e]: appendix-05-editions.html
