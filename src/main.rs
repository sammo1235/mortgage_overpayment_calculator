use std::env;
use std::process;
use std::error::Error;

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

struct MortgageParameters {
    principal: f64,
    annual_rate_as_decimal: f64,
    mortgage_term_length: u32,
    overpayment_amount: f64,
}

impl MortgageParameters {
    fn build(args: &[String]) -> Result<MortgageParameters, &'static str> {
        if args.len() < 4 {
            return Err("Not enough arguments. Please try again");
        }

        let principal: f64 = match args[1].parse() {
            Ok(n) => {
                n
            },
            Err(_) => {
                return Err("Starting principal of mortgage must be a float");
            },
        };
        
        let annual_rate_as_decimal: f64 = match args[2].parse() {
            Ok(n) => {
                n
            },
            Err(_) => {
                return Err("Annual Rate as Decimal must be a float");
            },
        };

        let mortgage_term_length: u32 = match args[3].parse() {
            Ok(n) => {
                n
            },
            Err(_) => {
                return Err("Term length of mortgage must be an integer");
            },
        };

        let overpayment_amount: f64 = match args[4].parse() {
            Ok(n) => {
                n
            },
            Err(_) => {
                return Err("Overpayment amount must be a float");
            },
        };

        Ok(MortgageParameters { principal, annual_rate_as_decimal, mortgage_term_length, overpayment_amount })
    }
}

fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i32.pow(decimals) as f64;
    (x*y).round() / y
}

fn build_mortgage(principal: f64, annual_interest_rate: f64, term_periods: u32, overpayment_amount: f64) -> MortgageSchedule {
    MortgageSchedule {
        principal,
        overpayment_amount,
        running_principal: principal,
        monthly_interest_rate: annual_interest_rate/12.0,
        number_of_payments: term_periods,
    }
}

struct MortgageSchedule {
    principal: f64,
    running_principal: f64,
    monthly_interest_rate: f64,
    number_of_payments: u32,
    overpayment_amount: f64,
}

impl MortgageSchedule {
    fn top_half(&self) -> f64 {
        self.monthly_interest_rate * (1.0 + self.monthly_interest_rate).powf(self.number_of_payments.into())
    }
    fn bottom_half(&self) -> f64 {
        ((1.0 + self.monthly_interest_rate).powf(self.number_of_payments.into())) - 1.0
    }
    fn payment_amount_equation(&self) -> f64 {
        self.principal * (self.top_half() / self.bottom_half())
    }
    fn run_schedule(&mut self) -> Result<(f64, f64), Box<dyn Error>> {
        
        let minimum_payment_amount = self.payment_amount_equation();
        println!("\nRunning schedule for overpayment amount £{}, monthly cost £{}",
            self.overpayment_amount,
            round(minimum_payment_amount + self.overpayment_amount, 2)
        );
        let mut total_interest_paid = 0.0;
        let mut total_repayment_cost = 0.0;
        let mut payment_count = 0;

        
        for _payment_number in 0..self.number_of_payments {
            payment_count += 1;
            // interest paid this month
            let interest_paid = self.running_principal * self.monthly_interest_rate;
            total_interest_paid += interest_paid;
            // principal paid this month
            let mut principal_paid = minimum_payment_amount - interest_paid;
            if self.overpayment_amount != 0.0 {
                principal_paid += self.overpayment_amount
            }
            
            // we pay off the principal
            self.running_principal -= principal_paid;

            total_repayment_cost += minimum_payment_amount;
            if self.overpayment_amount != 0.0 {
                total_repayment_cost += self.overpayment_amount;
            }

            if self.running_principal.round() <= 0.0 {
                println!("Mortgage paid in {} years, {} months", payment_count / 12, payment_count % 12);
                println!("Total Interest Paid: £{}", round(total_interest_paid, 2));
                println!("Total Amount Paid: £{}", round(total_repayment_cost, 2));
                break;
            }
        }
        Ok((total_repayment_cost, total_interest_paid))
    }
}