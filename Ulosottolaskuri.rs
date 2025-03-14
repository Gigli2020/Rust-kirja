use std::io;

fn main() {
    println!("Ulosottolaskuri");
    println!("Syötä kuukausittainen nettotulo (euroa):");

    let mut income_input = String::new();
    io::stdin()
        .read_line(&mut income_input)
        .expect("Virhe syötteen lukemisessa");
    let income: f64 = income_input
        .trim()
        .parse()
        .expect("Syötä kelvollinen numero");

    println!("Syötä kotitalouden jäsenten lukumäärä:");
    let mut persons_input = String::new();
    io::stdin()
        .read_line(&mut persons_input)
        .expect("Virhe syötteen lukemisessa");
    let persons: u32 = persons_input
        .trim()
        .parse()
        .expect("Syötä kelvollinen kokonaisluku");

    // Lasketaan perussuojamäärä käyttäjän syöttämien tietojen perusteella
    let basic_protection = calculate_basic_protection(persons);

    println!("Perussuojamäärä: {:.2} euroa", basic_protection);

    if income <= basic_protection {
        println!("Tuloistasi ei voi ulosmitata mitään, koska ne eivät ylitä suojamäärää.");
    } else {
        let garnishable = income - basic_protection;
        // Esimerkinomaisesti käytetään 20% ulosmittausprosenttina.
        let garnishment_rate = 0.20;
        let garnishment_amount = garnishable * garnishment_rate;

        println!("Ulosmitattava määrä: {:.2} euroa", garnishment_amount);
    }
}

/// Laskee perussuojamäärän kotitalouden jäsenten lukumäärän perusteella.
/// Esimerkkisäännöt:
/// - Yhden hengen kotitaloudessa suoja-arvo on 1000 euroa
/// - Kahden hengen kotitaloudessa 1300 euroa
/// - Kolmen hengen kotitaloudessa 1550 euroa
/// - Neljän tai useamman hengen kotitaloudessa lisätään 150 euroa jokaisesta lisähenkilöstä
fn calculate_basic_protection(persons: u32) -> f64 {
    match persons {
        1 => 1000.0,
        2 => 1300.0,
        3 => 1550.0,
        n if n >= 4 => 1550.0 + 150.0 * ((n - 3) as f64),
        _ => 1000.0, // oletustapaus
    }
}
