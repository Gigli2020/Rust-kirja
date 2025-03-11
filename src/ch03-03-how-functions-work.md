## Funktiot

Funktiot ovat yleisiä Rust-koodissa. Olet jo nähnyt yhden kielen tärkeimmistä funktioista: `main`-funktion, joka on monien ohjelmien aloituspiste. Olet myös nähnyt `fn`-avainsanan, joka mahdollistaa uusien funktioiden määrittämisen.

Rust käyttää _snake case_ -tyyliä funktioiden ja muuttujien nimissä, mikä tarkoittaa, että kaikki kirjaimet ovat pieniä ja sanat erotetaan alaviivoilla. Tässä on esimerkki funktion määrittämisestä:

<span class="filename">Tiedostonimi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-16-functions/src/main.rs}}
```

Määritämme funktion kirjoittamalla `fn`, sitten funktion nimen ja sulut. Aaltosulkeet kertovat kääntäjälle, missä funktion runko alkaa ja loppuu.

Voimme kutsua mitä tahansa määrittämäämme funktiota kirjoittamalla sen nimen ja sulut. Koska `another_function` on määritetty ohjelmassa, sitä voidaan kutsua `main`-funktion sisältä. Huomaa, että määritimme `another_function`-funktion _main_-funktion jälkeen lähdekoodissa; olisimme voineet määrittää sen myös ennen. Rustille ei ole väliä, missä kohdin funktiosi määritetään, kunhan ne on määritetty näkyvässä alueessa.

Aloitetaan uusi binääriprojekti nimeltään _functions_ ja tutkitaan funktioita tarkemmin. Sijoita `another_function`-esimerkki _src/main.rs_-tiedostoon ja suorita se. Sinun pitäisi nähdä seuraava tulostus:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-16-functions/output.txt}}
```

Rivit suoritetaan `main`-funktion sisällä siinä järjestyksessä kuin ne esiintyvät. Ensin tulostetaan “Hello, world!” -viesti, ja sitten kutsutaan `another_function`, joka tulostaa oman viestinsä.

### Parametrit

Voimme määrittää funktioita, joilla on _parametreja_, eli erikoismuuttujia, jotka ovat osa funktion määrittelyä. Kun funktiolla on parametreja, voit antaa niille konkreettisia arvoja. Teknisesti konkreettisia arvoja kutsutaan _argumenteiksi_, mutta arjessa ihmiset käyttävät sanoja _parametri_ ja _argumentti_ usein toistensa synonyymeinä.

Tässä versiossa `another_function`-funktiosta lisäämme parametrin:

<span class="filename">Tiedostonimi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/src/main.rs}}
```

Suorita ohjelma; sen pitäisi antaa seuraava tulostus:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/output.txt}}
```

Funktion `another_function` määrittelyssä on yksi parametri nimeltä `x`. Sen tyyppi on `i32`. Kun välitämme arvon `5` funktiolle `another_function`, `println!`-makro sijoittaa `5`:n siihen kohtaan, jossa `{x}` oli muotoilujonossa.

Funktion parametrit _on pakko_ määrittää tyyppeineen. Tämä on Rustin suunnitteluperiaate: koska funktioiden parametrit vaativat tyyppimäärittelyn, kääntäjä ei juuri koskaan tarvitse muita vihjeitä koodista sen selvittämiseksi, mitä tyyppiä tarkoitetaan. Tämä myös parantaa virheilmoituksia.

Jos määrität useita parametreja, erota ne pilkulla, kuten tässä:

<span class="filename">Tiedostonimi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/src/main.rs}}
```

Tämä funktio, `print_labeled_measurement`, ottaa kaksi parametria: `value` on `i32`, ja `unit_label` on `char`. Funktio tulostaa molemmat arvot.

### Lausekkeet ja lauseet

Funktion runko koostuu sarjasta lauseita, jotka voivat päättyä lausekkeeseen. Rust on lausekepohjainen kieli, joten tämä ero on tärkeä ymmärtää.

- **Lauseet** suorittavat jonkin toiminnon, mutta eivät palauta arvoa.
- **Lausekkeet** palauttavat arvon.

Olemme jo käyttäneet sekä lauseita että lausekkeita. Esimerkiksi `let y = 6;` on lause.

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-01/src/main.rs}}
```

Funktion määritys on myös lause. Mutta funktiokutsut ovat lausekkeita.

Koska lauseet eivät palauta arvoa, et voi sijoittaa `let`-lausetta muuttujaan:

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/src/main.rs}}
```

Tämä aiheuttaa virheen, koska `let y = 6;` ei palauta arvoa.

Lausekkeet, kuten `5 + 6`, palauttavat arvon. Esimerkiksi lohko:

```rust
{
    let x = 3;
    x + 1
}
```

on lauseke, joka palauttaa arvon `4`, joka sitoutuu muuttujaan `y`.

### Funktiot ja palautusarvot

Funktiot voivat palauttaa arvoja. Rustissa funktio palauttaa viimeisen lausekkeen arvon.

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/src/main.rs}}
```

Tässä funktiossa `five` ei sisällä mitään muuta kuin `5`, mikä on täysin kelvollinen funktio Rustissa. Funktion paluuarvo määritellään `-> i32`.

Toinen esimerkki:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-22-function-parameter-and-return/src/main.rs}}
```

Tulostaa `The value of x is: 6`. Mutta jos lisäisimme puolipisteen riville `x + 1`, saisimme virheen, koska lauseet eivät palauta arvoa.

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/src/main.rs}}
```

Rustin virheilmoitus ehdottaa puolipisteen poistamista, mikä korjaisi virheen.
