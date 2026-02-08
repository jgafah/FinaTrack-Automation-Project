pub fn run() {
    let income: f64 = 934.00;

    let rent = 450.00;
    let food = 200.00;
    let insurance = 120.00;
    let transport = 49.00;
    let personal = 65.50;

    let total_expenses = rent + food + insurance + transport + personal;
    let remaining = income - total_expenses;

    println!("Monthly Income: €{:.2}", income);
    println!("Total Expenses: €{:.2}", total_expenses);
    println!("Remaining Balance: €{:.2}", remaining);
}
