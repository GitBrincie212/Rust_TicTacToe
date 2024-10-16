use std::io::{stdout, Write};

pub(crate) fn prompt(input_str: &String) -> String {
    use std::io::{stdin};

    let mut input = String::new();
    print!("{}", input_str);
    let _ = stdout().flush();
    stdin().read_line(&mut input).unwrap();
    input
}

#[macro_export]
macro_rules! clean_screen {
    () => {
        println!("\x1B[2J\x1B[1;1H");
    };
}