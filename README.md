# Description
**cfg_loader** is a simple library for loading configuration/dotenv files usually used during development or testing environments.

## Example

Make some file that stores the variables :
```
# .env

SOMECONNECTIONSTRING = abc@user:pass/thing
SOMESECRETSALT = table
```

Then you can access the variables in any config or .env file using `cfg_loader::load` :
``` rust
# main.rs

let config: cfg_loader::VariableMap = cfg_loader::load(".env"); // File path is from the project's root directory.
let salt: Option<&str> = config.get("SOMESECRETSALT");

assert_eq!("table", salt.unwrap())
```

There is no limit as to how many files you can get variable maps from. This may be useful if you want different environments that will load different variables.

```
# root
.
+-- Cargo.toml
|
+-- src
|   +-- ...
|
+-- cfg
    +-- cfg
    +-- dev.env
    +-- prod.env
```

Using pattern matching, you can return an environment-specific configuration.
``` rust
# main.rs

let env: Option<&str> = cfg_loader::load("cfg").get("ENVIRONMENT");

let config: cfg_loader::VariableMap = match env.unwrap() {
    "development" => cfg_loader::load("dev.env"),
    "production" => cfg_loader::load("prod.env"),
    _ => panic!()
};

let connection_string: = config.get("SOMECONNECTIONSTRING").unwrap();
```