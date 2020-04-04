use std::convert::TryInto;

use tmi_rs::prelude::*;

#[no_mangle]
pub fn tmirs_main(mut client: Client) -> Promise {
    client.on_chat(|client, channel, userstate, msg, self_| {
        if self_ {
            return;
        }

        let username = userstate.get("display-name").to_string();

        if msg.starts_with("!echo ") {
            let msg = &msg["!echo ".len()..];
            client
                .say(channel, &format!("@{}: {}", username, msg))
                .or_else(|_, _| {}); // do nothing
        }
    });
    client.connect()
}
