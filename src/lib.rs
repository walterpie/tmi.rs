use client::Client;
use promise::Promise;

pub mod client;
pub mod object;
pub mod promise;
pub mod sys;

extern "Rust" {
    fn tmirs_main(client: Client) -> Promise;
}

pub mod prelude {
    pub use crate::client::Client;
    pub use crate::object::Object;
    pub use crate::promise::Promise;
}

// TODO: tmi.cxx should expect a possible error from this function
#[no_mangle]
pub extern "C" fn tmicxx_main(client: *mut sys::TmiClient) {
    let user_client = Client::new(client).expect("tmicxx_main called with NULL");
    let client = Client::new(client).expect("tmicxx_main called with NULL");

    let result = unsafe { tmirs_main(user_client) };

    unsafe {
        sys::tmi_on_action(client.as_ptr(), Some(client::on_action));
        sys::tmi_on_anongiftpaidupgrade(client.as_ptr(), Some(client::on_anongiftpaidupgrade));
        sys::tmi_on_ban(client.as_ptr(), Some(client::on_ban));
        sys::tmi_on_chat(client.as_ptr(), Some(client::on_chat));
        sys::tmi_on_cheer(client.as_ptr(), Some(client::on_cheer));
        sys::tmi_on_clearchat(client.as_ptr(), Some(client::on_clearchat));
        sys::tmi_on_connected(client.as_ptr(), Some(client::on_connected));
        sys::tmi_on_connecting(client.as_ptr(), Some(client::on_connecting));
        sys::tmi_on_disconnected(client.as_ptr(), Some(client::on_disconnected));
        sys::tmi_on_emoteonly(client.as_ptr(), Some(client::on_emoteonly));
        sys::tmi_on_emotesets(client.as_ptr(), Some(client::on_emotesets));
        sys::tmi_on_followersonly(client.as_ptr(), Some(client::on_followersonly));
        sys::tmi_on_giftpaidupgrade(client.as_ptr(), Some(client::on_giftpaidupgrade));
        sys::tmi_on_hosted(client.as_ptr(), Some(client::on_hosted));
        sys::tmi_on_hosting(client.as_ptr(), Some(client::on_hosting));
        sys::tmi_on_join(client.as_ptr(), Some(client::on_join));
        sys::tmi_on_logon(client.as_ptr(), Some(client::on_logon));
        sys::tmi_on_message(client.as_ptr(), Some(client::on_message));
        sys::tmi_on_messagedeleted(client.as_ptr(), Some(client::on_messagedeleted));
        sys::tmi_on_mod(client.as_ptr(), Some(client::on_mod));
        sys::tmi_on_mods(client.as_ptr(), Some(client::on_mods));
        sys::tmi_on_notice(client.as_ptr(), Some(client::on_notice));
        sys::tmi_on_part(client.as_ptr(), Some(client::on_part));
        sys::tmi_on_ping(client.as_ptr(), Some(client::on_ping));
        sys::tmi_on_pong(client.as_ptr(), Some(client::on_pong));
        sys::tmi_on_r9kbeta(client.as_ptr(), Some(client::on_r9kbeta));
        sys::tmi_on_raided(client.as_ptr(), Some(client::on_raided));
        sys::tmi_on_raw_message(client.as_ptr(), Some(client::on_raw_message));
        sys::tmi_on_reconnect(client.as_ptr(), Some(client::on_reconnect));
        sys::tmi_on_resub(client.as_ptr(), Some(client::on_resub));
        sys::tmi_on_roomstate(client.as_ptr(), Some(client::on_roomstate));
        sys::tmi_on_serverchange(client.as_ptr(), Some(client::on_serverchange));
        sys::tmi_on_slowmode(client.as_ptr(), Some(client::on_slowmode));
        sys::tmi_on_subgift(client.as_ptr(), Some(client::on_subgift));
        sys::tmi_on_submysterygift(client.as_ptr(), Some(client::on_submysterygift));
        sys::tmi_on_subscribers(client.as_ptr(), Some(client::on_subscribers));
        sys::tmi_on_subscription(client.as_ptr(), Some(client::on_subscription));
        sys::tmi_on_timeout(client.as_ptr(), Some(client::on_timeout));
        sys::tmi_on_unhost(client.as_ptr(), Some(client::on_unhost));
        sys::tmi_on_unmod(client.as_ptr(), Some(client::on_unmod));
        sys::tmi_on_vips(client.as_ptr(), Some(client::on_vips));
        sys::tmi_on_whisper(client.as_ptr(), Some(client::on_whisper));
    }

    result.or_else(|_, err| {
        eprintln!("an error occured in tmi.cxx: {}", err);
    });
}
