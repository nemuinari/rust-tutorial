// option, result, vec
pub fn execute() {
    println!("--- Option Example ---");
    option_example();

    println!("\n--- Result Example ---");
    result_example();

    println!("\n--- Vec Example ---");
    vec_example();
}

fn option_example() {
    let value: Option<i32> = Some(100);

    match value {
        Some(num) => println!("match: Value is {}", num),
        None => println!("match: No value"),
    }

    if let Some(num) = value {
        println!("if let: Value is {}", num);
    }
}

fn result_example() {
    let result: Result<i32, String> = Ok(200);

    if let Ok(code) = &result {
        println!("if let: Success with code {}", code);
    } else if let Err(err) = &result {
        println!("if let: Error is \"{}\"", err);
    }

    let _ = error_handling(result.clone());
}

fn error_handling(result: Result<i32, String>) -> Result<i32, String> {
    let code = result?;

    println!("Delegated: Code is {}", code);

    Ok(code)
}

fn vec_example() {
    let v = vec![1, 2, 3, 4, 5];

    println!("v[2] = {}", v[2]);

    for element in &v {
        println!("Element: {}", element);
    }
}
