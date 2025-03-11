## Ohjausrakenne

Kyky suorittaa koodia ehdon perusteella ja toistaa koodia, kunnes ehto muuttuu `false`:ksi, on perusperiaate useimmissa ohjelmointikielissä. Yleisimmät rakenteet, joilla voit hallita ohjelman suorituksen kulkua Rustissa, ovat `if`-lausekkeet ja silmukat.

### `if`-lausekkeet

`if`-lausekkeella voit haarauttaa koodin suoritusta ehtojen perusteella. Voit antaa ehdon ja määrittää: "Jos tämä ehto täyttyy, suorita tämä koodilohko. Jos ehto ei täyty, älä suorita tätä koodilohkoa."

Luo uusi projekti nimeltään _branches_ hakemistoon _projects_ ja lisää seuraava koodi tiedostoon _src/main.rs_:

<span class="filename">Tiedostonimi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-26-if-true/src/main.rs}}
```

Kaikki `if`-lausekkeet alkavat avainsanalla `if`, jota seuraa ehto. Tässä tapauksessa ehto tarkistaa, onko muuttujan `number` arvo alle 5. Jos ehto on `true`, ohjelma suorittaa aaltosulkeiden sisällä olevan koodin. Ehtolausekkeeseen liittyviä koodilohkoja kutsutaan joskus _haaroiksi_, kuten myös `match`-lausekkeessa käsitellyt vaihtoehdot luvussa 2.

Voimme myös sisällyttää `else`-haaran, kuten tässä teemme, jolloin ohjelma suorittaa vaihtoehtoisen koodilohkon, jos ehto on `false`. Jos `else`-haaraa ei ole ja ehto on `false`, ohjelma vain ohittaa `if`-lohkon ja jatkaa seuraavaan kohtaan.

### Monimutkaisemmat ehdot `else if`-rakenteella

Voit käyttää useita ehtoja yhdistämällä `if` ja `else` `else if` -rakenteeseen:

<span class="filename">Tiedostonimi: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-30-else-if/src/main.rs}}
```

Kun ohjelma suoritetaan, se tarkistaa jokaisen `if`-lauseen järjestyksessä ja suorittaa ensimmäisen, jonka ehto on `true`. Muut ehtovaihtoehdot ohitetaan.

### `if` osana `let`-lausetta

Koska `if` on lauseke, voimme käyttää sitä `let`-lausessa muuttujan arvon määrittämiseen:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-02/src/main.rs}}
```

Muuttuja `number` saa arvonsa `if`-lausekkeesta riippuen. Muista, että `if`-lausekkeen haarojen tulee palauttaa sama tietotyyppi.

### Toisto silmukoilla

Ohjelmissa on usein tarpeen suorittaa koodilohko useita kertoja. Rust tarjoaa kolme silmukkatyyppiä: `loop`, `while` ja `for`.

#### `loop`-silmukka

`loop` suorittaa koodilohkoa ikuisesti, kunnes se keskeytetään:

```rust,ignore
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-loop/src/main.rs}}
```

Voit keskeyttää silmukan `break`-avainsanalla ja hypätä seuraavaan iteraatioon `continue`-avainsanalla.

#### Paluuarvon palauttaminen `loop`-silmukasta

Silmästä voi myös palauttaa arvon `break`-avainsanan avulla:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-33-return-value-from-loop/src/main.rs}}
```

Tässä ohjelmassa `counter` kasvaa, kunnes se saavuttaa arvon 10, jolloin `break` palauttaa arvon `counter * 2`.

#### Silmukoiden etiketit

Kun käytetään sisäkkäisiä silmukoita, `break` ja `continue` vaikuttavat oletuksena vain sisimpään silmukkaan. Voit käyttää _silmukkaetikettiä_ määrittääksesi, mihin silmukkaan `break` tai `continue` vaikuttaa:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-32-5-loop-labels/src/main.rs}}
```

Etiketti `'counting_up` määrittää, että `break 'counting_up;` keskeyttää ulomman silmukan.

### `while`-silmukka

`while` suorittaa koodilohkoa niin kauan kuin ehto on `true`:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-03/src/main.rs}}
```

Tämä on kätevä tapa kirjoittaa ehtoon perustuvia silmukoita ilman tarvetta `loop`- ja `break`-rakenteelle.

### `for`-silmukka

Voit käydä kokoelman läpi `for`-silmukan avulla:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-05/src/main.rs}}
```

Tämä tapa on turvallisempi ja tehokkaampi kuin `while`-silmukka, koska se ei vaadi manuaalista indeksien hallintaa.

Voimme myös käyttää `for`-silmukkaa ja `rev`-metodia laskeutumiseen:

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-34-for-range/src/main.rs}}
```

### Yhteenveto

Tässä luvussa opit muuttujista, yksinkertaisista ja yhdistetyistä tietotyypeistä, funktioista, kommenteista, `if`-lausekkeista ja silmukoista. Kokeile seuraavia harjoituksia:

- Muunna lämpötiloja Fahrenheitin ja Celsiusin välillä.
- Luo ohjelma, joka laskee _n_:nnen Fibonacci-luvun.
- Tulosta joululaulun “The Twelve Days of Christmas” sanat hyödyntäen silmukoita.

Seuraavaksi tutustumme ainutlaatuiseen Rustin ominaisuuteen: omistajuuteen.

[comparing-the-guess-to-the-secret-number]: ch02-00-guessing-game-tutorial.html#comparing-the-guess-to-the-secret-number
[quitting-after-a-correct-guess]: ch02-00-guessing-game-tutorial.html#quitting-after-a-correct-guess
