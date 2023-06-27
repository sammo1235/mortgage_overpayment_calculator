use std::error::Error;
use crate::round;

pub struct MortgageSchedule {
  pub principal: f64,
  pub running_principal: f64,
  pub monthly_interest_rate: f64,
  pub number_of_payments: u32,
  pub overpayment_amount: f64,
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
  pub fn run_schedule(&mut self) -> Result<(f64, f64), Box<dyn Error>> {
      
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
          // add overpayment
          if self.overpayment_amount != 0.0 {
              principal_paid += self.overpayment_amount
          }

          if principal_paid > self.running_principal {
            principal_paid = self.running_principal
          }
          
          // we pay off the principal
          self.running_principal -= principal_paid;

          total_repayment_cost += minimum_payment_amount;
          if self.overpayment_amount != 0.0 {
              total_repayment_cost += self.overpayment_amount;
          }

          if self.running_principal <= 0.0 {
              println!("Mortgage paid in {} years, {} months", payment_count / 12, payment_count % 12);
              println!("Total Interest Paid: £{}", round(total_interest_paid, 2));
              println!("Total Amount Paid: £{}", round(total_repayment_cost, 2));
              break;
          }
      }
      Ok((round(total_repayment_cost, 2), round(total_interest_paid, 2)))
  }
}

#[cfg(test)]
mod tests {
  use crate::mortgage_schedule::MortgageSchedule;

  #[test]
  fn small_loan_with_annual_rate() {
    let Ok((total_cost, total_interest)) = MortgageSchedule {
      principal: 10000.0, // £10k value
      overpayment_amount: 0.0,
      running_principal: 10000.0,
      monthly_interest_rate: 0.02, // 2% interest annual rate
      number_of_payments: 5, // 5 years
    }.run_schedule() else { return; };

    assert_eq!(total_cost, 10607.92);
    assert_eq!(total_interest, 607.92)

  }
  
  #[test]
  fn calculates_mortgage_without_overpaying() {
    let Ok((total_cost, total_interest)) = MortgageSchedule {
      principal: 100000.0, // £100k value
      overpayment_amount: 0.0,
      running_principal: 100000.0,
      monthly_interest_rate: 0.02/12.0, // 2% interest monthly rate
      number_of_payments: 35*12, // 35 years
    }.run_schedule() else { return; };

    assert_eq!(total_cost, 139130.36);
    assert_eq!(total_interest, 39130.36)
  }

  #[test]
  fn calculates_mortgage_with_overpaying() {
    let Ok((total_cost, total_interest)) = MortgageSchedule {
      principal: 100000.0, // £100k value
      overpayment_amount: 100.0, // £100 overpayment per month
      running_principal: 100000.0,
      monthly_interest_rate: 0.02/12.0, // 2% interest monthly rate
      number_of_payments: 35*12, // 35 years
    }.run_schedule() else { return; };

    assert_eq!(total_cost, 126791.25);
    assert_eq!(total_interest, 26511.89);
  }

  #[test]
  fn calculates_mortgage_without_overpaying_2() {
    let Ok((total_cost, total_interest)) = MortgageSchedule {
      principal: 300000.0, // £300k value
      overpayment_amount: 0.0,
      running_principal: 300000.0,
      monthly_interest_rate: 0.04/12.0, // 4% interest monthly rate
      number_of_payments: 30*12, // 30 years
    }.run_schedule() else { return; };

    assert_eq!(total_cost, 515608.52);
    assert_eq!(total_interest, 215608.52);
  }

  #[test]
  fn calculates_mortgage_with_overpaying_2() {
    let Ok((total_cost, total_interest)) = MortgageSchedule {
      principal: 300000.0, // £300k value
      overpayment_amount: 100.0,
      running_principal: 300000.0,
      monthly_interest_rate: 0.04/12.0, // 4% interest monthly rate
      number_of_payments: 30*12, // 30 years
    }.run_schedule() else { return; };

    assert_eq!(total_cost, 487254.19);
    assert_eq!(total_interest, 186861.93);
  }
}