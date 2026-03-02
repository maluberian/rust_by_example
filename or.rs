
#[derive(Debug, Copy, Clone)] enum Payment { Cash, Check, CreditCard }

fn main() {
    println!("{:?}", None.or(Some(Payment::Cash)).or(Some(Payment::CreditCard)));

    let credit_fallback = || {
        println!("credit_fallback");
        Some(Payment::CreditCard)
    };
    println!("{:?}", None.or_else(credit_fallback).or(Some(Payment::CreditCard)));

    let mut orig: Option<Payment> = None;
    let cc_payment_type = Payment::CreditCard;
    let pref_payment_type = orig.get_or_insert(cc_payment_type);
    println!("new: {:?}", pref_payment_type);
    println!("orig: {:?}", orig);
}