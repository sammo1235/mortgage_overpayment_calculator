use std::env;
use std::process;
use mortgage_overpayment_calculator::MortgageParameters;
use mortgage_overpayment_calculator::round;
use mortgage_overpayment_calculator::build_mortgage;

pub mod mortgage_schedule;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mp = MortgageParameters::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);  
    });

    let mut mortgage_1 = build_mortgage(
        mp.principal, 
        mp.annual_rate_as_decimal, 
        mp.mortgage_term_length, 
        0.0
    );

    let Ok((total_repayment_cost, total_interest_paid)) = mortgage_1.run_schedule() else { return; };

    let mut mortgage_2 = build_mortgage(
        mp.principal, 
        mp.annual_rate_as_decimal, 
        mp.mortgage_term_length, 
        mp.overpayment_amount
    );

    match mortgage_2.run_schedule() {
        Ok((op_total_repayment_cost, op_total_interest_paid)) => {
            let total_payment_savings = total_repayment_cost - op_total_repayment_cost;
            let interest_saved = total_interest_paid - op_total_interest_paid;
            println!("\nTotal amount saved: £{}", round(total_payment_savings, 2));
            println!("\nInterest saved: £{}", round(interest_saved, 2));
        },
        Err(e) => {
            println!("Application error: {e}");
            process::exit(1);
        }
    }
}
