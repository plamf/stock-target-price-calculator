slint::include_modules!();

const KEST: f64 = 0.275;
const MULTIPLIER: f64 = 0.725;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_calculate_target_price(move |aktienpreis, anzahl, gewuenschterGewinnProzent| {
        let ui = ui_handle.unwrap();
        let stockpriceNum: f64 = aktienpreis.trim().parse().unwrap();
        let amountNum: f64 = anzahl.trim().parse().unwrap();
        let profitPercentageNum: f64 = gewuenschterGewinnProzent.trim().parse().unwrap();

        let notwendigeSteigerung = stockpriceNum * (profitPercentageNum / 100.0) / MULTIPLIER;
        let investitionsSumme = stockpriceNum * amountNum;
        let gewuenschterWertProStueck = stockpriceNum + notwendigeSteigerung;
        let steuern = notwendigeSteigerung * KEST;
        let kestInProzent = KEST * 100.0;
        let nettogewinn = notwendigeSteigerung - steuern;
        let summeAuszahlungBrutto = investitionsSumme + (notwendigeSteigerung * amountNum);
        let summeSteuern = steuern * amountNum;
        let summeGewinn = (notwendigeSteigerung - steuern) * amountNum;
        let summeAuszahlung =
            investitionsSumme + (notwendigeSteigerung * amountNum) - (steuern * amountNum);

        let result_text = format!(
            "Ergebnis pro Stueck:\n
Der Aktienkurs muss auf {:.2} steigen.\n
Von der Differenz ({:.2}) werden {:.1}% ({:.2}) Steuern abgezogen.\n
Davon bleibt ein Nettogewinn von {:.2} pro Stueck.\n\n\n
Ergebnis Gesamt:\n
Summe Auszahlung (brutto): {:.2}\n
Summe Steuern: {:.2}\n
Summe Gewinn: {:.2}\n
Summe Auszahlung (netto): {:.2}
",
            gewuenschterWertProStueck,
            notwendigeSteigerung,
            kestInProzent,
            steuern,
            nettogewinn,
            summeAuszahlungBrutto,
            summeSteuern,
            summeGewinn,
            summeAuszahlung
        );

        ui.set_results(result_text.into());
    });

    ui.run()
}
