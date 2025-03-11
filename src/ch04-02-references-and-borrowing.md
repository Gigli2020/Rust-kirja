## Viittaukset ja lainaaminen

Edellisen luvun esimerkissä (Listing 4-5) jouduimme palauttamaan `String`-arvon kutsuvalle funktiolle, jotta voisimme käyttää sitä `calculate_length`-funktion kutsun jälkeen. Tämä johtui siitä, että `String` siirtyi `calculate_length`-funktion omistukseen. Sen sijaan voimme tarjota viittauksen `String`-arvoon.  

_Viittaus_ on kuin osoitin siinä mielessä, että se on muistiosoite, jota seuraamalla voimme käyttää kyseiseen osoitteeseen tallennettua dataa. Tämä data on jonkin muun muuttujan omistuksessa. Toisin kuin osoitin, viittaus on aina turvallinen: se osoittaa aina kelvolliseen tietyn tyyppiseen arvoon koko viittauksen eliniän ajan.

Näin voimme määrittää ja käyttää `calculate_length`-funktiota siten, että se ottaa viittauksen parametrinaan eikä siirrä omistajuutta:

<span class="filename">Tiedostonimi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:all}}
```

Ensinnäkin, huomaa, että tuplakoodia ei enää tarvita muuttujan määrityksessä eikä funktion palautusarvossa. Lisäksi annamme `&s1`-viittauksen `calculate_length`-funktiolle, ja funktion määrityksessä otamme `&String`-viittauksen `String`-arvon sijaan. Et-merkki (`&`) tarkoittaa _viittausta_, jonka avulla voimme käyttää arvoa ilman, että omistajuus siirtyy.

### Viittauksen käyttämisen periaatteet

Rust kutsuu viittauksen käyttöä _lainaamiseksi_. Aivan kuten oikeassa elämässä voit lainata jotain toiselta henkilöltä, mutta et omista sitä. Kun olet valmis, palautat sen takaisin.

Mutta mitä tapahtuu, jos yritämme muuttaa lainattua arvoa? Kokeile seuraavaa koodia:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-06/src/main.rs}}
```

Tämä ei toimi, koska viittaukset ovat oletusarvoisesti muuttumattomia. Emme voi muuttaa lainattua arvoa.

### Muutettavat viittaukset

Voimme kuitenkin sallia muuttamisen käyttämällä _muutettavaa viittausta_:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-09-fixes-listing-04-06/src/main.rs}}
```

Lisäsimme `mut`-avainsanan `s`-muuttujan määrittelyyn ja käytimme `&mut s`-viittausta `change`-funktion kutsussa. Myös funktion määritystä muutettiin hyväksymään muuttuva viittaus `some_string: &mut String`. Tämä tekee selväksi, että funktio voi muuttaa lainaamaansa arvoa.

Mutta muuttuvilla viittauksilla on rajoitus: samanaikaisesti voi olla vain yksi muuttuva viittaus tiettyyn arvoon. Seuraava koodi ei toimi:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/src/main.rs:here}}
```

Tämä rajoitus estää _data race_ -ongelmat, jotka voivat aiheuttaa arvaamatonta käytöstä.

Rust estää myös muuttuvan ja muuttumattoman viittauksen käyttämisen samaan aikaan:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/src/main.rs}}
```

Jos tarvitset molemmat, voit rajata niiden elinkaaret erillisiin lohkoihin:

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-13-reference-scope-ends/src/main.rs:here}}
```

### Riippuvat viittaukset

Muissa ohjelmointikielissä voi helposti syntyä _riippuvia viittauksia_ (dangling references), jotka osoittavat muistialueelle, joka on jo vapautettu. Rust estää tämän käännösaikaisella virheellä:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/src/main.rs}}
```

Rust ei anna palauttaa viittausta, jos siihen liittyvä data on jo vapautettu.

### Viittausten säännöt

Yhteenvetona viittauksista:

- Voit omistaa _yhden_ muuttuvan viittauksen _tai_ useita muuttumattomia viittauksia samanaikaisesti.
- Viittausten on aina osoitettava kelvolliseen arvoon.

Seuraavaksi tarkastelemme erilaista viittaustyyppiä: _viipaleita_ (slices).
