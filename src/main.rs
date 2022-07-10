use std::{io::{self, Write}, process, thread, time};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let query = format!(r#"{{"query": "query Channel_Query{{channel:user(login: \"{}\"){{stream{{id}}}}}}"}}"#, input("Enter channel name: "));

    let client = reqwest::blocking::Client::new();

    let resp = client.get("https://www.twitch.tv").send()?.text()?;
    let client_id = resp
        .split(r#"clientId=""#)
        .skip(1)
        .next().unwrap()
        .split('"')
        .next().unwrap();
    
    let minute = time::Duration::new(60, 0);
    loop {
        println!("Channel is still live! Next check in 60secs");
        let resp = client.post("https://gql.twitch.tv/gql")
            .header("client-id", client_id)
            .body(query.clone())
            .send()?.text()?;
    
        if resp.contains("null") {
            process::Command::new("cmd")
                .args(&["/C", "shutdown -s"])
                .output()
                .expect("failed to execute process");
            process::exit(0);
        }
        thread::sleep(minute);
    }
}

fn input(prompt: &str) -> String
{
    print!("{}", prompt);
    io::stdout().flush().expect("Can't print to stdout!");
    let mut ret = String::new();
    io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
    ret.trim().to_string()
}