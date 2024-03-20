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
    let _sender = client.sender();

    thread::spawn(move || {
        send_message(client, channel.trim());
    });

    while let Some(message) = stream.next().await.transpose()? {
        println!("{}", message);
        println!("{}", parse_message(&message, &nickname));
    }

    Ok(())
}

fn parse_message(message: &Message, nickname: &String) -> String {
    let command = &message.command;
    let message_sender = &message.source_nickname();

    if let Some(message_sender) = message_sender {
        match command {
            Command::PRIVMSG(sent_to, msg_text) => {
                if sent_to == nickname {
                    format!("PM from {}: {}", message_sender, msg_text)
                } else {
                    format!("{}: {}", message_sender, msg_text)
                }
            }
            Command::NOTICE(_sent_to, notice_text) => {
                format!("NOTICE: {}", notice_text)
            }
            Command::QUIT(_) => {
                format!("")
            }
            _ => format!("{}", message),
        }
    } else {
        String::new()
    }
}

fn send_message(client: Client, channel: &str) {
    loop {
        let mut user_message = String::new();
        io::stdin()
            .read_line(&mut user_message)
            .expect("Could not read user input.");
        let user_message = user_message.trim();

        client
            .send_privmsg(channel, user_message)
            .expect("Message failed to send.");
    }
}
