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

    let Ok((_total_repayment_cost, total_interest_paid, total_time_taken)) = mortgage_1.run_schedule() else { return; };

    let mut mortgage_2 = build_mortgage(
        mp.principal, 
        mp.annual_rate_as_decimal, 
        mp.mortgage_term_length, 
        mp.overpayment_amount
    );

    match mortgage_2.run_schedule() {
        Ok((_op_total_repayment_cost, op_total_interest_paid, op_total_time_taken)) => {
            let time_saved = total_time_taken - op_total_time_taken;
            let interest_saved = total_interest_paid - op_total_interest_paid;

            println!("\nTime saved: {} years {} months", time_saved / 12, time_saved % 12);
            println!("Interest saved: £{}", round(interest_saved, 2));
        },
        Err(e) => {
            println!("Application error: {e}");
            process::exit(1);
        }
    }
}
