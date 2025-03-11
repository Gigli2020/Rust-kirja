## Mikä on omistajuus?

_Omistajuus_ on joukko sääntöjä, jotka määräävät, miten Rust-ohjelma hallitsee muistia.
Kaikki ohjelmat joutuvat hallitsemaan muistinkäyttöään suorituksen aikana.
Joissakin kielissä on roskienkerääjä, joka säännöllisesti vapauttaa käyttämättömän muistin ohjelman suorittaessa;
toisissa kielissä ohjelmoijan on itse varattava ja vapautettava muisti manuaalisesti.
Rust käyttää kolmatta lähestymistapaa: muisti hallitaan omistajuusjärjestelmällä, jonka sääntöjä kääntäjä tarkistaa.
Jos sääntöjä rikotaan, ohjelma ei käänny.
Omistajuus ei hidasta ohjelman suorittamista.

Koska omistajuus on monille ohjelmoijille uusi käsite, sen omaksuminen vie aikaa.
Hyvä uutinen on, että mitä enemmän harjoittelet Rustia ja sen omistajuussääntöjä,
sitä helpommaksi turvallisen ja tehokkaan koodin kirjoittaminen tulee.
Jatka harjoittelua!

Kun ymmärrät omistajuuden, sinulla on vahva pohja Rustin ainutlaatuisten ominaisuuksien ymmärtämiselle.
Tässä luvussa opit omistajuudesta käyttämällä esimerkkejä hyvin yleisestä tietorakenteesta: merkkijonoista.

### Omistajuussäännöt

Ensinnäkin tarkastellaan omistajuuden perussääntöjä. Muista nämä säännöt, kun käsittelemme esimerkkejä:

- Jokaisella arvolla Rustissa on _omistaja_.
- Vain yksi omistaja voi olla kerrallaan.
- Kun omistaja poistuu näkyvyysalueelta (scope), arvo poistetaan muistista.

### Muuttujan näkyvyysalue (Scope)

Nyt kun olemme käyneet läpi Rustin perussyntaksia, emme sisällytä `fn main() {` -riviä kaikkiin esimerkkeihin.
Jos seuraat mukana, varmista, että sijoitat esimerkit `main`-funktion sisään manuaalisesti.

Ensimmäisenä omistajuusesimerkkinä tarkastellaan muuttujan _näkyvyysaluetta_ (scope). Tämä tarkoittaa ohjelman osaa,
jossa muuttuja on voimassa:

```rust
let s = "hello";
```

Muuttuja `s` viittaa merkkijonoliteraltiin, joka on kovakoodattu ohjelman lähdekoodiin.
Se on voimassa siitä kohdasta lähtien, missä se on määritetty, kunnes näkyvyysalue päättyy.
Tässä luvussa tarkastelemme, miten Rust hallitsee muistin varaamista ja vapauttamista omistajuussääntöjen avulla.

