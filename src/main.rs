use std::fmt::format;

slint::include_modules!();

const TAXPER: f64 = 0.30;
const OWNERPER: f64 = 0.55;
const PROFIT: f64 = 0.05;
const OPEXPER: f64 = 0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.divide_income(move |string| {
        let ui = ui_handle.unwrap();
        let num = string.parse::<f64>().unwrap();
        let tax = num * TAXPER;
        let owner = num * OWNERPER;
        let profit = num * PROFIT;
        let opex = num * OPEXPER;
        let result = format!("args: {}\ntax: {}\nowner: {}\nprofit: {}\nopex: {}", num, tax, owner, profit, opex);
        ui.set_result(result.into());
    });

    ui.run()
}
