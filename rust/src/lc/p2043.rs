struct Bank {
    balance: Vec<i64>
}


impl Bank {

    fn new(balance: Vec<i64>) -> Self {
        Bank {
            balance
        }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        if !self.check_account(account1) || !self.check_account(account2) || !self.check_balance(account1,money) {
            return false;
        }
        self.add_balance(account1, - money);
        self.add_balance(account2, money);
        true
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        if !self.check_account(account) {
            return false;
        }
        self.add_balance(account, money);
        true
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        if !self.check_account(account) || !self.check_balance(account, money)  {
            return false;
        }
        self.add_balance(account, -money);
        true

    }

    fn check_account(&self, account: i32) -> bool {
        if account <= 0 || account as usize > self.balance.len() {
            return false;
        }
        true
    }

    fn check_balance(&self, account: i32, money: i64) -> bool {
        if self.balance[(account - 1) as usize] < money {
            return false;
        }
        true
    }

    fn add_balance(&mut self, account: i32, money: i64) {
        self.balance[(account - 1) as usize] += money;
    }
}

/**
 * Your Bank object will be instantiated and called as such:
 */
#[test]
fn test() {
    let mut obj = Bank::new(vec![10, 100, 20, 50, 30]);
    assert!(obj.withdraw(3, 10));
    assert!(obj.transfer(5, 1, 20));
    assert!(obj.deposit(5, 20));
    assert!(!obj.transfer(3, 4, 15));
    assert!(!obj.withdraw(10, 50));
}
