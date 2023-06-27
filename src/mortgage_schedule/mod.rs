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