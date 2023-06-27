mod mortgage_schedule;

use mortgage_schedule::MortgageSchedule;

pub fn build_mortgage(principal: f64, annual_interest_rate: f64, term_periods: u32, overpayment_amount: f64) -> MortgageSchedule {
  MortgageSchedule {
      principal,
      overpayment_amount,
      running_principal: principal,
      monthly_interest_rate: annual_interest_rate/12.0,
      number_of_payments: term_periods,
  }
}

pub struct MortgageParameters {
  pub principal: f64,
  pub annual_rate_as_decimal: f64,
  pub mortgage_term_length: u32,
  pub overpayment_amount: f64,
}

impl MortgageParameters {
  pub fn build(args: &[String]) -> Result<MortgageParameters, &'static str> {
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

pub fn round(x: f64, decimals: u32) -> f64 {
  let y = 10i32.pow(decimals) as f64;
  (x*y).round() / y
}