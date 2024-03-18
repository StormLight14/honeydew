use futures::prelude::*;
use irc::client::prelude::*;
use std::io;
use std::thread;

const SERVER: &'static str = "chat.freenode.net";

#[tokio::main]
async fn main() -> irc::error::Result<()> {
    println!("What channel in {} would you like to talk in?", SERVER);
    let mut channel = String::new();

    io::stdin()
        .read_line(&mut channel)
        .expect("Could not read input");

    println!("What nickname would you like to use?");
    let mut nickname = String::new();

    io::stdin()
        .read_line(&mut nickname)
        .expect("Could not read input");

    let config = Config {
        nickname: Some(nickname.trim().to_string()),
        server: Some(SERVER.to_string()),
        channels: vec![channel.trim().to_string()],
        ..Default::default()
    };

    let mut client = Client::from_config(config).await?;
    client.identify()?;

    let mut stream = client.stream()?;
    let sender = client.sender();

    println!("{:?}", sender);

    thread::spawn(move || {
        send_message(client, channel.trim());
    });

    println!("non blocking.");

    while let Some(message) = stream.next().await.transpose()? {
        //println!("{:?}", message.command);

        match message.command {
            Command::JOIN(channel, _key1, _key2) => {
                println!("Someone joined {}!", channel);
            }
            _ => {
                print!("{}", parse_message(&message));
            }
        }
    }

    Ok(())
}

fn parse_message(message: &Message) -> String {
    let message_str = message.to_string();

    if message_str.starts_with(":") && message_str.contains("PRIVMSG") {
        let split_msg = message_str.splitn(3, " ").collect::<Vec<&str>>();
        let first_part = split_msg[0].split("~").collect::<Vec<&str>>();
        let last_part = split_msg[2].splitn(2, " ").collect::<Vec<&str>>();
        let sender_username = first_part[0].replace(":", "");
        let sender_message = last_part[1];

        //println!("{:?}", split_msg);

        return format!("{}: {}", sender_username, sender_message);
    }

    String::new()
}

fn send_message(client: Client, channel: &str) {
    loop {
        let mut user_message = String::new();
        io::stdin()
            .read_line(&mut user_message)
            .expect("Could not read user input.");

        client
            .send_privmsg(channel, user_message.trim())
            .expect("Message failed to send.");
    }
}
