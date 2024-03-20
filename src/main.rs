use futures::prelude::*;
use irc::client::prelude::*;
use std::io;
use std::process;
use std::thread;

const SERVER: &'static str = "chat.freenode.net";

#[tokio::main]
async fn main() -> irc::error::Result<()> {
    /*
    println!("What channel in {} would you like to talk in?", SERVER);
    let mut channel = String::new();

    io::stdin()
        .read_line(&mut channel)
        .expect("Could not read input");
    let channel = channel.trim().to_string();
    */
    println!("What nickname would you like to use?");
    let channel = String::from("#rust-spam");
    let mut nickname = String::new();

    io::stdin()
        .read_line(&mut nickname)
        .expect("Could not read input");
    let nickname = nickname.trim().to_string();

    let config = Config {
        nickname: Some(nickname.to_string()),
        server: Some(SERVER.to_string()),
        channels: vec![channel.to_string()],
        ..Default::default()
    };

    let mut client = Client::from_config(config).await?;
    client.identify()?;

    let mut stream = client.stream()?;
    let _sender = client.sender();

    thread::spawn(move || {
        send_message(client, &channel);
    });

    while let Some(message) = stream.next().await.transpose()? {
        print!("{}", parse_message(&message, &nickname.to_string()));
    }

    Ok(())
}

fn parse_message(message: &Message, nickname: &String) -> String {
    let command = &message.command;
    let message_sender = &message.source_nickname();
    //println!("COMMAND: {:?}", command);

    match command {
        Command::PRIVMSG(sent_to, msg_text) => {
            if let Some(message_sender) = message_sender {
                if sent_to == nickname.trim() {
                    format!("PM from {}: {}\n", message_sender, msg_text)
                } else {
                    format!("{}: {}\n", message_sender, msg_text)
                }
            } else {
                String::new()
            }
        }
        Command::NOTICE(_sent_to, notice_text) => {
            format!("NOTICE: {}\n", notice_text)
        }
        Command::Response(_response_type, text_vec) => {
            format!("{}\n", text_vec.last().unwrap())
        }
        Command::MOTD(motd) => {
            if let Some(motd) = motd {
                format!("MOTD: {}\n", motd)
            } else {
                String::from("No MOTD?\n")
            }
        }
        Command::PING(_, _) => String::new(),
        Command::JOIN(_, _, _) => {
            if let Some(message_sender) = message_sender {
                format!("{} joined.\n", message_sender)
            } else {
                format!("Someone joined. Don't ask me who.\n")
            }
        }
        Command::QUIT(_) => {
            format!("")
        }
        _ => format!("{:?}", message),
    }
}

fn send_message(client: Client, channel: &str) {
    loop {
        let mut user_message = String::new();
        io::stdin()
            .read_line(&mut user_message)
            .expect("Could not read user input.");
        let user_message = user_message.trim();

        if user_message.starts_with('/') {
            let command = user_message.replacen("/", "", 1);
            let mut command = command.split(' ');

            if let Some(command) = command.nth(0) {
                match command {
                    "quit" => match client.send_quit("Closed by user.") {
                        Ok(_) => {
                            println!("Quitting.");
                            process::exit(0);
                        }
                        Err(_) => {
                            println!("Could not quit.");
                        }
                    },
                    _ => {}
                }
            }
        } else {
            client
                .send_privmsg(channel, user_message)
                .expect("Message failed to send.");
        }
    }
}
