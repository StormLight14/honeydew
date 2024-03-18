use futures::prelude::*;
use irc::client::prelude::*;
use std::io;

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

    while let Some(message) = stream.next().await.transpose()? {
        println!("{:?}", message.command);

        match message.command {
            Command::JOIN(channel, _key1, _key2) => {
                println!("Someone joined {}! :3", channel);
            }
            _ => {
                println!("{}", parse_message(&message));
            }
        }
    }

    Ok(())
}

fn parse_message(message: &Message) -> String {
    let message_str = message.to_string();

    if message_str.starts_with(":") && message_str.contains("PRIVMSG") {
        let split_msg = message_str.splitn(3, " ").collect::<Vec<&str>>();
        println!("{:?}", split_msg);

        return format!(
            "{} | {}: {}",
            split_msg[2],
            split_msg[1].replace(":", " "),
            split_msg[3].replace(":", " ")
        );
    }

    String::new()
}
