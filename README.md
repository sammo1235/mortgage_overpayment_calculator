# mortgage_overpayment_calculator

A simple calculator to give a rough guide of how much you'd save on time and interest by overpaying your mortgage.

To use the calculator, use the command:

```
cargo run -- [loan_amount] [annual_rate_as_decimal] [term_length_in_months] [overpayment_amount]
```

For example, running the command:

```
cargo run -- 300000.0 0.03 420 250
```

gives the output:

```
Running schedule for overpayment amount £0, monthly cost £1154.55
Mortgage paid in 35 years, 0 months
Total Interest Paid: £184911.24
Total Amount Paid: £484911.24

Running schedule for overpayment amount £250, monthly cost £1404.55
Mortgage paid in 25 years, 6 months
Total Interest Paid: £129499.39
Total Amount Paid: £429792.47

Total amount saved: £55118.76

Interest saved: £55411.85
```
