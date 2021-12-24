pub mod account;
pub mod card;

use std::collections::HashMap;

use uuid::Uuid;

struct Session {
    pub state: u8,
    pub card_number: String,
    pub verified: bool,
    pub account_number: String,
}

pub trait Bank {
    fn create_session(&mut self) -> Uuid;

    fn insert_card(&mut self, session: &Uuid, card_number: String) -> Result<(), String>;
    fn verify_pin(&mut self, session: &Uuid, pin: &[u8; 4]) -> Result<bool, String>;
    fn account_list(&mut self, session: &Uuid) -> Result<Vec<String>, String>;
    fn account_select(&mut self, session: &Uuid, account_number: String) -> Result<(), String>;

    fn balance(&mut self, session: &Uuid) -> Result<u64, String>;
    fn deposit(&mut self, session: &Uuid, amount: u64) -> Result<u64, String>;
    fn withdraw(&mut self, session: &Uuid, amount: u64) -> Result<u64, String>;
}

pub struct Dummy<'a> {
    accounts: HashMap<String, account::Account<u64>>,
    cards: HashMap<String, card::Card<'a>>,
    sessions: HashMap<Uuid, Session>,
}

impl<'a> Dummy<'a> {
    pub fn new() -> Dummy<'a> {
        let mut rst = Dummy {
            accounts: HashMap::new(),
            cards: HashMap::new(),
            sessions: HashMap::new(),
        };

        // Insertion dummy datas
        rst.accounts.insert(
            "10010001000".to_string(),
            crate::account::Account::new(10000)
        );
        rst.accounts.insert(
            "10010001001".to_string(),
            crate::account::Account::new(12000)
        );
        rst.accounts.insert(
            "10010001002".to_string(),
            crate::account::Account::new(30000)
        );
        rst.accounts.insert(
            "10010001003".to_string(),
            crate::account::Account::new(50000)
        );

        rst.cards.insert(
            "1234567887654321".to_string(),
            crate::card::Card::new(b"3579", vec![
                "10010001000".to_string(),
                "10010001001".to_string(),
                "10010001002".to_string(),
                "10010001003".to_string(),
            ])
        );

        rst.accounts.insert(
            "10010002000".to_string(),
            crate::account::Account::new(1000)
        );
        rst.accounts.insert(
            "10010002001".to_string(),
            crate::account::Account::new(3000)
        );
        rst.accounts.insert(
            "10010002002".to_string(),
            crate::account::Account::new(500)
        );
        rst.accounts.insert(
            "10010002003".to_string(),
            crate::account::Account::new(10000)
        );

        rst.cards.insert(
            "8765432112345678".to_string(),
            crate::card::Card::new(b"1470", vec![
                "10010002000".to_string(),
                "10010002001".to_string(),
                "10010002002".to_string(),
                "10010002003".to_string(),
            ])
        );

        rst
    }
}

impl<'a> Bank for Dummy<'a> {
    fn create_session(&mut self) -> Uuid {
        let uuid = Uuid::new_v4();

        let _ = &self.sessions.insert(
            uuid,
            Session {
                state: 1,
                card_number: "".to_string(),
                verified: false,
                account_number: "".to_string(),
            }
        );

        uuid
    }

    fn insert_card(&mut self, session: &Uuid, card_number: String) -> Result<(), String> {
        match self.sessions.get_mut(session) {
            Some(s) => {
                if 1 != s.state {
                    return Err("Incorrect access".to_string())
                }

                s.state = 2;
                s.card_number = card_number;

                return Ok(())
            },
            None => {
                return Err("Session not exist".to_string())
            }
        }
    }

    fn verify_pin(&mut self, session: &Uuid, pin: &[u8; 4]) -> Result<bool, String> {
        match self.sessions.get_mut(session) {
            Some(s) => {
                if 2 != s.state {
                    return Err("Incorrect access".to_string())
                }

                match self.cards.get(&s.card_number) {
                    Some(c) => {
                        let rst = c.verify(pin);
                        if true == rst {
                            s.state = 3;
                            s.verified = true;
                        }
                        return Ok(rst)
                    },
                    None => {
                        return Err("Card not exist".to_string())
                    }
                }
            },
            None => {
                return Err("Session not exist".to_string())
            }
        }
    }

    fn account_list(&mut self, session: &Uuid) -> Result<Vec<String>, String> {
        match self.sessions.get_mut(session) {
            Some(s) => {
                if 3 != s.state || false == s.verified {
                    return Err("Incorrect access".to_string())
                }

                match self.cards.get(&s.card_number) {
                    Some(c) => {
                        return Ok(c.account_list())
                    },
                    None => {
                        return Err("Card not exist".to_string())
                    }
                }
            },
            None => {
                return Err("Session not exist".to_string())
            }
        }
    }

    fn account_select(&mut self, session: &Uuid, account_number: String) -> Result<(), String> {
        match self.sessions.get_mut(session) {
            Some(s) => {
                if 3 != s.state || false == s.verified {
                    return Err("Incorrect access".to_string())
                }

                match self.cards.get(&s.card_number) {
                    Some(c) => {
                        let mut contains = false;

                        for a in c.account_list() {
                            println!("{}, {}, {}", a, account_number, a.eq(&account_number));
                            if a.eq(&account_number) {
                                contains = true;
                                break;
                            }
                        }

                        match contains {
                            false => {
                                return Err("Account not exist".to_string())
                            },
                            true => {
                                s.state = 4;
                                s.account_number = account_number;

                                return Ok(())
                            },
                        }
                    },
                    _ => {
                        return Err("Unknown Error".to_string())
                    }
                }
            },
            None => {
                return Err("Session not exist".to_string())
            }
        }
    }

    fn balance(&mut self, session: &Uuid) -> Result<u64, String> {
        match self.sessions.get_mut(session) {
            Some(s) => {
                if 4 != s.state {
                    return Err("Incorrect access".to_string())
                }

                match self.accounts.get(&s.account_number) {
                    Some(a) => {
                        return Ok(a.balance())
                    },
                    _ => {
                        return Err("Unknown Error".to_string())
                    }
                }
            },
            None => {
                return Err("Session not exist".to_string())
            }
        }
    }

    fn deposit(&mut self, session: &Uuid, amount: u64) -> Result<u64, String> {
        match self.sessions.get_mut(session) {
            Some(s) => {
                if 4 != s.state {
                    return Err("Incorrect access".to_string())
                }

                match self.accounts.get_mut(&s.account_number) {
                    Some(a) => {
                        return a.deposit(amount)
                    },
                    _ => {
                        return Err("Unknown Error".to_string())
                    }
                }
            },
            None => {
                return Err("Session not exist".to_string())
            }
        }
    }

    fn withdraw(&mut self, session: &Uuid, amount: u64) -> Result<u64, String> {
        match self.sessions.get_mut(session) {
            Some(s) => {
                if 4 != s.state {
                    return Err("Incorrect access".to_string())
                }

                match self.accounts.get_mut(&s.account_number) {
                    Some(a) => {
                        return a.withdraw(amount)
                    },
                    _ => {
                        return Err("Unknown Error".to_string())
                    }
                }
            },
            None => {
                return Err("Session not exist".to_string())
            }
        }
    }
}

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
        a.deposit(10000).unwrap();
        assert_eq!(a.balance(), 20000);
        a.deposit(1000).unwrap();
        assert_eq!(a.balance(), 21000);
        a.deposit(100).unwrap();
        assert_eq!(a.balance(), 21100);
    }

    #[test]
    fn account_withdraw() {
        let mut a = crate::account::Account::new(10000);
        assert_eq!(a.balance(), 10000);
        a.withdraw(1000).unwrap();
        assert_eq!(a.balance(), 9000);
        a.withdraw(4000).unwrap();
        assert_eq!(a.balance(), 5000);
        let _ = a.withdraw(5001);  // Err case: Drop check
        assert_eq!(a.balance(), 5000);
        a.withdraw(5000).unwrap();
        assert_eq!(a.balance(), 0);
    }

    #[test]
    fn account_fn() {
        let mut a = crate::account::Account::new(10000);
        assert_eq!(a.balance(), 10000);
        a.deposit(100).unwrap();
        assert_eq!(a.balance(), 10100);
        a.withdraw(1000).unwrap();
        assert_eq!(a.balance(), 9100);
        a.deposit(100).unwrap();
        assert_eq!(a.balance(), 9200);
        let _ = a.withdraw(10000);  // Err case: Drop check
        assert_eq!(a.balance(), 9200);
    }

    #[test]
    fn verify_pin() {
        let c = crate::card::Card::new(b"1234", vec![]);
        assert_eq!(c.verify(b"0000"), false);
        assert_eq!(c.verify(b"1234"), true);
    }
}
