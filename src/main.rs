//
use std::io::{self, Write};

const MINUMUM_DEPOSIT: i32 = 100;
const MINIMUM_WITHDRAWAL: i32 = 100;
const MINIMUM_ACCOUNT_BALANCE_AFTER_WITHDRAWAL: i32 = 50;
const MAXIMUM_LOAN_PERCENTAGE: f32 = 1.75;

fn flush_output() {
    let _ = io::stdout().flush();
}

struct AccoutnDetails {
    balance: u32,
    loan_balance: u32,
    loan_limit: u32,
    account_name: String,
    account_exists: bool,
    total_deposits: u32,
    total_withdrawal: u32,
    account_password: String,
}

//Initialize acoount details for all accounts.
impl AccoutnDetails {
    fn new_account() -> AccoutnDetails {
        AccoutnDetails {
            balance: 0,
            loan_balance: 0,
            loan_limit: 0,
            account_name: "_".to_string(),
            account_exists: false,
            total_deposits: 0,
            total_withdrawal: 0,
            account_password: "Giberish".to_string(),
        }
    }
}

fn get_loan_limit(user_account: &mut AccoutnDetails) {
    if user_account.balance == 0 {
        return;
    }

    let mut loan_limit_percentage = user_account.balance as f32
        + (1.0 + (user_account.total_withdrawal as f32 / user_account.total_withdrawal as f32));

    /*

        Max loan limit percentage is 1.75.
        Loan limit is calculated by adding loan_limit_percentage got from
        total withdrawals / total_deposits
    */
    if loan_limit_percentage > MAXIMUM_LOAN_PERCENTAGE {
        loan_limit_percentage = MAXIMUM_LOAN_PERCENTAGE;
    }

    let loan_limit =
        user_account.balance as f32 + (user_account.loan_balance as f32 * loan_limit_percentage);
    user_account.loan_limit = loan_limit as u32;
}

fn help_command(args: &Vec<&str>) {
    let argument_lenght = args.len();
    // Manually match explanantion to correct index of argument
    let argument_explanation = vec![
        " -> Type help for more information",
        " -> Creates an account. 'create' + 'account_name'",
        " -> Deposit money to signed in account. 'deposit' + 'amount'",
        " -> Withdraw from signed in account. 'withdraw' + 'ammount'",
        " -> Take loan from signed in account. 'borrow' + 'ammount'",
        " -> Repay previously loaned money. 'repay' + 'ammount'",
        " -> Details of account. 'details'",
        " -> hello",
    ];
    let mut position = 1;
    println!();
    for (arg, expanation) in args.iter().zip(argument_explanation.iter()) {
        println!("\t{arg}{expanation}");
        position += 1;
        // ensure only help argument found in args are printed and no
        // excess documentation is given
        if position > argument_lenght {
            break;
        }
    }
}

fn print_account_details(user_account: &AccoutnDetails) {
    println!();
    println!("Account Name: {}", user_account.account_name.trim());
    println!("Account Balance: {}", user_account.balance);
    println!("Loan balance: {}", user_account.loan_balance);
    println!("Loan limit: {}", user_account.loan_limit);
}

fn create_account(user_account: &mut AccoutnDetails, value: &str) {
    if value == "" {
        println!("Failed to create account. No account name");
    }

    user_account.account_name = value.to_string();
    user_account.account_exists = true;

    println!("Successfully created account.");
    print_account_details(user_account);
}

fn deposit_money(user_account: &mut AccoutnDetails, value: &str) {
    // Ensure account is created
    if !user_account.account_exists {
        println!("Account does not exist. No account to add funds to");
        return;
    }
    let mut deposit_ammount = 0;
    // Try to typecast the second value and check for int
    match value.parse::<i32>() {
        Ok(number) => {
            if number < MINUMUM_DEPOSIT {
                println!("Failed: Ammount must be grater than {MINUMUM_DEPOSIT}");
            } else {
                deposit_ammount = number;
                print!("Successfully deposited {deposit_ammount}. ");
                flush_output();
            }
        }
        Err(_) => println!("Failed, ammount entered is not a number or less that 100"),
    }

    user_account.balance = user_account.balance + deposit_ammount as u32;
    user_account.total_deposits = user_account.total_deposits + deposit_ammount as u32;

    println!("New account balance: {}", user_account.balance)
}

fn withdraw_money(user_account: &mut AccoutnDetails, value: &str) {
    if !user_account.account_exists {
        println!("Account does not exist. No account to withdraw from");
        return;
    }

    let mut withdraw_ammount = 0;

    match value.parse::<i32>() {
        Ok(number) => {
            if number < MINIMUM_WITHDRAWAL {
                println!("Failed: Ammount must be grater than {MINIMUM_WITHDRAWAL}");
            } else {
                withdraw_ammount = number;
                if (user_account.balance as i32 - withdraw_ammount)
                    < MINIMUM_ACCOUNT_BALANCE_AFTER_WITHDRAWAL
                {
                    println!(
                        "Failed, account balance must remain above {MINIMUM_ACCOUNT_BALANCE_AFTER_WITHDRAWAL} Try a lower ammount"
                    );
                    return;
                }
                print!("Successfully withdrew {withdraw_ammount}. ");
                flush_output();
            }
        }
        Err(_) => {
            println!("Failed, ammount entered is not a number or less that {withdraw_ammount}")
        }
    }

    user_account.balance = user_account.balance - withdraw_ammount as u32;
    user_account.total_withdrawal = user_account.total_withdrawal + withdraw_ammount as u32;

    println!("New account balance: {}", user_account.balance)
}

fn go_to_bank_transaction(buffer: &mut str, user_account: &mut AccoutnDetails) -> bool {
    let arguments = vec![
        "help", "create", "deposit", "withdraw", "borrow", "repay", "details",
    ];
    /* Split argument and evaluate the forst item for supported commands */
    let mut user_arguments = Vec::new();

    for parts in buffer.split_whitespace() {
        user_arguments.push(parts);
    }
    if arguments.contains(&user_arguments[0]) != true {
        println!("[Invalid Argument]");
        return false;
    }

    // Commands needing one argument.
    if user_arguments.len() == 1 {
        match user_arguments[0] {
            "help" => help_command(&arguments),
            "create" => println!("Falied to create account. No account name"),
            "deposit" => println!("Failed to deposit. No ammount added"),
            "withdraw" => println!("Failed to withdraw. No ammont added"),

            _ => println!("[Invalid Argument]"),
        }
        // commands with more than one argument
    } else if user_arguments.len() > 1 {
        match user_arguments[0] {
            "create" => {
                if user_account.account_exists == true {
                    println!(
                        "Account exists: [Account name: {}]",
                        user_account.account_name
                    );
                    return false;
                } else {
                    create_account(user_account, &user_arguments[1]);
                }
            }
            "deposit" => {
                deposit_money(user_account, &user_arguments[1]);
            }
            "withdraw" => withdraw_money(user_account, &user_arguments[1]),
            _ => println!("[Invalid Argument]"),
        }
    }

    /* match user_arguments[0] {
        "deposit" => {
            if user_arguments.len() > 0 {
                deposit_money(&mut user_account, &user_arguments[1])
            } else {
                println!("Failed to create account. No account name");
            }
        }
        // "withdraw" => withdraw_money(),
        // "borrow" => borrow_money(),
        // "repay" => repay_loan(),
        // "details" => account_details(),
        _ => println!("[Invalid Argument]"),
    } */

    return true;
}

fn input_true(buffer: &mut String) -> bool {
    io::stdin().read_line(buffer).expect("Failed to read line!");
    if buffer.trim().is_empty() {
        return false;
    }
    return true;
}

fn get_input(user_account: &mut AccoutnDetails) -> i8 {
    let mut buffer = String::new();
    buffer.clear();

    print!("> ");
    flush_output();
    //skip if input is empty.
    if !input_true(&mut buffer) {
        return -1;
    }

    if !go_to_bank_transaction(&mut buffer, user_account) {
        return -1;
    }

    return 0;
}

fn main() {
    let mut user_account = AccoutnDetails::new_account();
    loop {
        if get_input(&mut user_account) != 0 {
            get_input(&mut user_account);
        }
    }
}

/*
    1. Create account
    2. Deposit
    3. withdraw
    4. Take a loan
*/
