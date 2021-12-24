Implement a simple ATM controller
===

Write code for a simple ATM. It doesn't need any UI (either graphical or console), but a controller should be implemented and tested.

# Requirements
At least the following flow should be implemented:

Insert Card => PIN number => Select Account => See Balance/Deposit/Withdraw

For simplification, there are only 1 dollar bills in this world, no cents. Thus account balance can be represented in integer.

Your code doesn't need to integrate with a real bank system, but keep in mind that we may want to integrate it with a real bank system in the future. It doesn't have to integrate with a real cash bin in the ATM, but keep in mind that we'd want to integrate with that in the future. And even if we integrate it with them, we'd like to test our code. Implementing bank integration and ATM hardware like cash bin and card reader is not a scope of this task, but testing the controller part (not including bank system, cash bin etc) is within the scope.

A bank API wouldn't give the ATM the PIN number, but it can tell you if the PIN number is correct or not.

Based on your work, another engineer should be able to implement the user interface. You don't need to implement any REST API, RPC, network communication etc, but just functions/classes/methods, etc.

You can simplify some complex real world problems if you think it's not worth illustrating in the project.

# Instruction to clone
```sh
git clone https://github.com/LMinJae/BR-assignment
```

# build and run tests
```sh
# If need rust install, using docker image
alias cargo="docker run -it --rm -v .:/mnt -w /mnt rust cargo"

# Run test cases
cargo test

# Run sample ATM CLI
cargo run
```

Initial dummy data
```
# Format
Card(PIN)
- Account(Initial balance)
...
---

1234567887654321(3579)
- 10010001000(10000)
- 10010001001(12000)
- 10010001002(30000)
- 10010001003(50000)

8765432112345678(1470)
- 10010002000(1000)
- 10010002001(3000)
- 10010002002(500)
- 10010002003(10000)
```

# Memo
For real application, sensitive data(such as card number and PIN number) will be wipe from memory.

So, i select non script programming language, for handling memory easily in the future.

And rust was easy to integrate with javascript as wasm.
Also most of programming languages that used in production, support native interfaces.

this project splited 3 parts.
- bank

	dummy system of backend system

- bank-api

	api that passing to another engineer who implement ATM

- atm

	sample interactive command-line interface for test api
