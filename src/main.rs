slint::include_modules!();

const KEST: f64 = 0.275;
const MULTIPLIER: f64 = 0.725;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_calculate_target_price(move |aktienpreis, anzahl, gewuenschter_gewinn_prozent| {
        let ui = ui_handle.unwrap();
        let stockprice_num = aktienpreis.trim().parse::<f64>().unwrap();
        let amount_num = anzahl.trim().parse::<f64>().unwrap();
        let profit_percentage_num = gewuenschter_gewinn_prozent.trim().parse::<f64>().unwrap();

        let notwendige_steigerung = stockprice_num * profit_percentage_num / 100.0 / MULTIPLIER;
        let steuern_auf_erhoehung = notwendige_steigerung * amount_num * KEST;

        let summe_brutto = stockprice_num * amount_num + notwendige_steigerung * amount_num;
        let summe_auszahlung = summe_brutto - steuern_auf_erhoehung;
        let summe_gewinn = summe_auszahlung - (stockprice_num * amount_num);

        ui.set_results(format!(
            "Der Aktienkurs muss auf {:.2} steigen.\n\n\nSumme (brutto): {:.2}\nSumme Steuern: {:.2}\n\nSumme Auszahlung (netto): {:.2}\nSumme Gewinn: {:.2}",
            stockprice_num + notwendige_steigerung,
            summe_brutto,
            steuern_auf_erhoehung,
            summe_auszahlung,
            summe_gewinn
        ).into());
    });

    ui.run()
}
