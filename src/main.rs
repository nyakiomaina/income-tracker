use slint::SharedString;

slint::include_modules!();

const EXPENSES: f64 = 0.50;
const SAVINGS: f64 = 0.25;
const MISCELLANEOUS: f64 = 0.25;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.on_divide_income(move |string: SharedString| {
        let ui: AppWindow = ui_handle.unwrap();
        let num: f64 = string.trim().parse().unwrap();
        let expenses: f64 = num * EXPENSES;
        let savings: f64 = num * SAVINGS;
        let miscellaneous: f64 = num * MISCELLANEOUS;
        let results: String = format!("Expenses: KSH{:.2}\nSavings: KSH{:.2}\nMiscellaneous: KSH{:.2}", expenses, savings, miscellaneous);
        ui.set_results(results.into());
    });

    ui.run()
}
