## Metodisyntaksi

_Metodit_ ovat samankaltaisia kuin funktiot: ne määritellään `fn`-avainsanalla, voivat sisältää parametreja ja palautusarvon, ja ne suorittavat määriteltyä koodia kutsuttaessa. Toisin kuin funktiot, metodit määritellään rakenteen, luettelotyypin (_enum_) tai rajapinnan (_trait_) yhteyteen, ja niiden ensimmäinen parametri on aina `self`, joka viittaa rakenteen ilmentymään.

### Metodin määrittely

Muutetaan `area`-funktio metodiksi, joka määritellään `Rectangle`-rakenteen sisällä:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-13/src/main.rs}}
```

Metodi määritellään `impl`-lohkon sisällä, joka sitoo metodin `Rectangle`-tyyppiin. Metodin ensimmäinen parametri on `&self`, mikä tarkoittaa, että se viittaa `Rectangle`-ilmentymään mutta ei ota omistajuutta.

`main`-funktiossa voimme kutsua metodia piste-merkinnällä (`.`):

```rust
println!("The area of the rectangle is {} square pixels.", rect1.area());
```

Metodin käyttämisessä on se etu, että kaikki `Rectangle`-tyyppiin liittyvät toiminnot löytyvät yhdestä `impl`-lohkosta, eikä käyttäjän tarvitse etsiä eri puolilta koodia.

### Metodit, joilla on useita parametreja

Lisätään `Rectangle`-rakenteelle metodi `can_hold`, joka tarkistaa, mahtuuko toinen suorakulmio toisen sisään:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-15/src/main.rs:here}}
```

Tämä metodi ottaa parametrina viittauksen toiseen `Rectangle`-ilmentymään ja vertaa mittoja keskenään.

### Liittyvät funktiot

Kaikki `impl`-lohkon sisällä määritellyt funktiot ovat _liittyviä funktioita_ (_associated functions_). Ne voivat olla metodeja, mutta jos niillä ei ole `self`-parametria, niitä kutsutaan ilman ilmentymää.

Esimerkiksi voimme luoda `Rectangle`-rakenteelle uuden liittyvän funktion `square`, joka luo neliön:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/no-listing-03-associated-functions/src/main.rs:here}}
```

Tällainen funktio kutsutaan `Rectangle::square(3);`-muodossa.

### Useita `impl`-lohkoja

Voimme jakaa metodit useisiin `impl`-lohkoihin:

```rust
{{#rustdoc_include ../listings/ch05-using-structs-to-structure-related-data/listing-05-16/src/main.rs:here}}
```

Tämä on sallittua, ja siitä voi olla hyötyä esimerkiksi silloin, kun eri metodit liittyvät eri ominaisuuksiin.

## Yhteenveto

Rakenteet mahdollistavat mukautettujen tietotyyppien luomisen. `impl`-lohkojen avulla voimme määritellä rakenteeseen liittyviä funktioita ja metodeja, jotka määrittävät sen käyttäytymisen.

Seuraavaksi tutustumme Rustin _enum_-ominaisuuteen, joka tarjoaa toisen tavan luoda mukautettuja tyyppejä.
