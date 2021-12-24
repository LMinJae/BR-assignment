pub mod account;
pub mod card;

#[cfg(test)]
mod tests {
    #[test]
    fn account_creation() {
        let a = crate::account::Account::new(10000);
        assert_eq!(a.balance(), 10000);
    }

    #[test]
    fn account_deposit() {
        let mut a = crate::account::Account::new(10000);
        assert_eq!(a.balance(), 10000);
        a.deposit(10000);
        assert_eq!(a.balance(), 20000);
        a.deposit(1000);
        assert_eq!(a.balance(), 21000);
        a.deposit(100);
        assert_eq!(a.balance(), 21100);
    }

    #[test]
    fn account_withdraw() {
        let mut a = crate::account::Account::new(10000);
        assert_eq!(a.balance(), 10000);
        a.withdraw(1000);
        assert_eq!(a.balance(), 9000);
        a.withdraw(4000);
        assert_eq!(a.balance(), 5000);
        a.withdraw(5001);
        assert_eq!(a.balance(), 5000);
        a.withdraw(5000);
        assert_eq!(a.balance(), 0);
    }

    #[test]
    fn account_fn() {
        let mut a = crate::account::Account::new(10000);
        assert_eq!(a.balance(), 10000);
        a.deposit(100);
        assert_eq!(a.balance(), 10100);
        a.withdraw(1000);
        assert_eq!(a.balance(), 9100);
        a.deposit(100);
        assert_eq!(a.balance(), 9200);
        a.withdraw(10000);
        assert_eq!(a.balance(), 9200);
    }

    #[test]
    fn verify_pin() {
        let c = crate::card::Card::new(b"1234", vec![]);
        assert_eq!(c.verify(b"0000"), false);
        assert_eq!(c.verify(b"1234"), true);
    }
}
