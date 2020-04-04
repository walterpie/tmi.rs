use std::cell::RefCell;
use std::ffi::{c_void, CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_int, c_ushort};
use std::ptr::{self, NonNull};
use std::rc::Rc;
use std::u64;

use crate::object::Object;
use crate::promise::Promise;
use crate::sys::{self, TmiClient, TmiObject};

type OnAction = Box<dyn Fn(&mut Client, &str, &Object, &str, bool)>;
type OnAnongiftpaidupgrade = Box<dyn Fn(&mut Client, &str, &str, &Object)>;
type OnBan = Box<dyn Fn(&mut Client, &str, &str, &Object)>;
type OnChat = Box<dyn Fn(&mut Client, &str, &Object, &str, bool)>;
type OnCheer = Box<dyn Fn(&mut Client, &str, &Object, &str)>;
type OnClearchat = Box<dyn Fn(&mut Client, &str)>;
type OnConnected = Box<dyn Fn(&mut Client, &str, u16)>;
type OnConnecting = Box<dyn Fn(&mut Client, &str, u16)>;
type OnDisconnected = Box<dyn Fn(&mut Client, &str)>;
type OnEmoteonly = Box<dyn Fn(&mut Client, &str, bool)>;
type OnEmotesets = Box<dyn Fn(&mut Client, &str, &Object)>;
type OnFollowersonly = Box<dyn Fn(&mut Client, &str, bool, usize)>;
type OnGiftpaidupgrade = Box<dyn Fn(&mut Client, &str, &str, &str, &Object)>;
type OnHosted = Box<dyn Fn(&mut Client, &str, &str, usize, bool)>;
type OnHosting = Box<dyn Fn(&mut Client, &str, &str, usize)>;
type OnJoin = Box<dyn Fn(&mut Client, &str, &str, bool)>;
type OnLogon = Box<dyn Fn(&mut Client)>;
type OnMessage = Box<dyn Fn(&mut Client, &str, &Object, &str, bool)>;
type OnMessagedeleted = Box<dyn Fn(&mut Client, &str, &str, &str, &Object)>;
type OnMod = Box<dyn Fn(&mut Client, &str, &str)>;
type OnMods = Box<dyn Fn(&mut Client, &str, &Object)>;
type OnNotice = Box<dyn Fn(&mut Client, &str, &str, &str)>;
type OnPart = Box<dyn Fn(&mut Client, &str, &str, bool)>;
type OnPing = Box<dyn Fn(&mut Client)>;
type OnPong = Box<dyn Fn(&mut Client, f64)>;
type OnR9kbeta = Box<dyn Fn(&mut Client, &str, bool)>;
type OnRaided = Box<dyn Fn(&mut Client, &str, &str, usize)>;
type OnRawMessage = Box<dyn Fn(&mut Client, &Object, &Object)>;
type OnReconnect = Box<dyn Fn(&mut Client)>;
type OnResub = Box<dyn Fn(&mut Client, &str, &str, usize, &str, &Object, &Object)>;
type OnRoomstate = Box<dyn Fn(&mut Client, &str, &Object)>;
type OnServerchange = Box<dyn Fn(&mut Client, &str)>;
type OnSlowmode = Box<dyn Fn(&mut Client, &str, bool, usize)>;
type OnSubgift = Box<dyn Fn(&mut Client, &str, &str, usize, &str, &Object, &Object)>;
type OnSubmysterygift = Box<dyn Fn(&mut Client, &str, &str, usize, &Object, &Object)>;
type OnSubscribers = Box<dyn Fn(&mut Client, &str, bool)>;
type OnSubscription = Box<dyn Fn(&mut Client, &str, &str, &Object, &str, &Object)>;
type OnTimeout = Box<dyn Fn(&mut Client, &str, &str, usize, &Object)>;
type OnUnhost = Box<dyn Fn(&mut Client, &str, usize)>;
type OnUnmod = Box<dyn Fn(&mut Client, &str, &str)>;
type OnVips = Box<dyn Fn(&mut Client, &str, &Object)>;
type OnWhisper = Box<dyn Fn(&mut Client, &str, &Object, &str, bool)>;

pub(crate) unsafe extern "C" fn on_action(
    client: *mut TmiClient,
    channel: *const c_char,
    userstate: *mut TmiObject,
    msg: *const c_char,
    self_: c_int,
) {
    let channel = CStr::from_ptr(channel);
    let msg = CStr::from_ptr(msg);
    let userstate = Object::new(false, userstate).expect("NULL object passed to callback");
    let self_ = self_ != 0;
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_action {
        callback(
            &mut client,
            &channel.to_string_lossy(),
            &userstate,
            &msg.to_string_lossy(),
            self_,
        );
    }
    mem::forget(userstate);
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_anongiftpaidupgrade(
    client: *mut TmiClient,
    channel: *const c_char,
    username: *const c_char,
    userstate: *mut TmiObject,
) {
    let channel = CStr::from_ptr(channel);
    let username = CStr::from_ptr(username);
    let userstate = Object::new(false, userstate).expect("NULL object passed to callback");
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_anongiftpaidupgrade {
        callback(
            &mut client,
            &channel.to_string_lossy(),
            &username.to_string_lossy(),
            &userstate,
        );
    }
    mem::forget(userstate);
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_ban(
    client: *mut TmiClient,
    channel: *const c_char,
    username: *const c_char,
    userstate: *mut TmiObject,
) {
    let channel = CStr::from_ptr(channel);
    let username = CStr::from_ptr(username);
    let userstate = Object::new(false, userstate).expect("NULL object passed to callback");
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_ban {
        callback(
            &mut client,
            &channel.to_string_lossy(),
            &username.to_string_lossy(),
            &userstate,
        );
    }
    mem::forget(userstate);
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_chat(
    client: *mut TmiClient,
    channel: *const c_char,
    userstate: *mut TmiObject,
    msg: *const c_char,
    self_: c_int,
) {
    let channel = CStr::from_ptr(channel);
    let msg = CStr::from_ptr(msg);
    let userstate = Object::new(false, userstate).expect("NULL object passed to callback");
    let self_ = self_ != 0;
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_chat {
        callback(
            &mut client,
            &channel.to_string_lossy(),
            &userstate,
            &msg.to_string_lossy(),
            self_,
        );
    }
    mem::forget(userstate);
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_cheer(
    client: *mut TmiClient,
    channel: *const c_char,
    userstate: *mut TmiObject,
    msg: *const c_char,
) {
    let channel = CStr::from_ptr(channel);
    let msg = CStr::from_ptr(msg);
    let userstate = Object::new(false, userstate).expect("NULL object passed to callback");
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_cheer {
        callback(
            &mut client,
            &channel.to_string_lossy(),
            &userstate,
            &msg.to_string_lossy(),
        );
    }
    mem::forget(userstate);
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_clearchat(client: *mut TmiClient, channel: *const c_char) {
    let channel = CStr::from_ptr(channel);
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_clearchat {
        callback(&mut client, &channel.to_string_lossy());
    }
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_connected(
    client: *mut TmiClient,
    address: *const c_char,
    port: c_ushort,
) {
    let port = port as u16;
    let address = CStr::from_ptr(address);
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_connected {
        callback(&mut client, &address.to_string_lossy(), port);
    }
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_connecting(
    client: *mut TmiClient,
    address: *const c_char,
    port: c_ushort,
) {
    let port = port as u16;
    let address = CStr::from_ptr(address);
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_connecting {
        callback(&mut client, &address.to_string_lossy(), port);
    }
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_disconnected(client: *mut TmiClient, reason: *const c_char) {
    let reason = CStr::from_ptr(reason);
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_disconnected {
        callback(&mut client, &reason.to_string_lossy());
    }
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_emoteonly(
    client: *mut TmiClient,
    channel: *const c_char,
    enabled: c_int,
) {
    let channel = CStr::from_ptr(channel);
    let enabled = enabled != 0;
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_emoteonly {
        callback(&mut client, &channel.to_string_lossy(), enabled);
    }
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_emotesets(
    client: *mut TmiClient,
    sets: *const c_char,
    obj: *mut TmiObject,
) {
    let sets = CStr::from_ptr(sets);
    let obj = Object::new(false, obj).expect("NULL object passed to callback");
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_emotesets {
        callback(&mut client, &sets.to_string_lossy(), &obj);
    }
    mem::forget(obj);
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_followersonly(
    client: *mut TmiClient,
    channel: *const c_char,
    enabled: c_int,
    length: c_int,
) {
    let channel = CStr::from_ptr(channel);
    let length = length as usize;
    let enabled = enabled != 0;
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_followersonly {
        callback(&mut client, &channel.to_string_lossy(), enabled, length);
    }
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_giftpaidupgrade(
    client: *mut TmiClient,
    channel: *const c_char,
    username: *const c_char,
    sender: *const c_char,
    userstate: *mut TmiObject,
) {
    let channel = CStr::from_ptr(channel);
    let username = CStr::from_ptr(username);
    let sender = CStr::from_ptr(sender);
    let userstate = Object::new(false, userstate).expect("NULL object passed to callback");
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_giftpaidupgrade {
        callback(
            &mut client,
            &channel.to_string_lossy(),
            &username.to_string_lossy(),
            &sender.to_string_lossy(),
            &userstate,
        );
    }
    mem::forget(userstate);
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_hosted(
    client: *mut TmiClient,
    channel: *const c_char,
    username: *const c_char,
    viewers: c_int,
    autohost: c_int,
) {
    let channel = CStr::from_ptr(channel);
    let username = CStr::from_ptr(username);
    let viewers = viewers as usize;
    let autohost = autohost != 0;
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_hosted {
        callback(
            &mut client,
            &channel.to_string_lossy(),
            &username.to_string_lossy(),
            viewers,
            autohost,
        );
    }
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_hosting(
    client: *mut TmiClient,
    channel: *const c_char,
    target: *const c_char,
    viewers: c_int,
) {
    let channel = CStr::from_ptr(channel);
    let target = CStr::from_ptr(target);
    let viewers = viewers as usize;
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_hosting {
        callback(
            &mut client,
            &channel.to_string_lossy(),
            &target.to_string_lossy(),
            viewers,
        );
    }
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_join(
    client: *mut TmiClient,
    channel: *const c_char,
    username: *const c_char,
    self_: c_int,
) {
    let channel = CStr::from_ptr(channel);
    let username = CStr::from_ptr(username);
    let self_ = self_ != 0;
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_join {
        callback(
            &mut client,
            &channel.to_string_lossy(),
            &username.to_string_lossy(),
            self_,
        );
    }
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_logon(client: *mut TmiClient) {
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_logon {
        callback(&mut client);
    }
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_message(
    client: *mut TmiClient,
    channel: *const c_char,
    userstate: *mut TmiObject,
    msg: *const c_char,
    self_: c_int,
) {
    let channel = CStr::from_ptr(channel);
    let msg = CStr::from_ptr(msg);
    let userstate = Object::new(false, userstate).expect("NULL object passed to callback");
    let self_ = self_ != 0;
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_message {
        callback(
            &mut client,
            &channel.to_string_lossy(),
            &userstate,
            &msg.to_string_lossy(),
            self_,
        );
    }
    mem::forget(userstate);
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_messagedeleted(
    client: *mut TmiClient,
    channel: *const c_char,
    username: *const c_char,
    deleted: *const c_char,
    userstate: *mut TmiObject,
) {
    let channel = CStr::from_ptr(channel);
    let username = CStr::from_ptr(username);
    let deleted = CStr::from_ptr(deleted);
    let userstate = Object::new(false, userstate).expect("NULL object passed to callback");
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_messagedeleted {
        callback(
            &mut client,
            &channel.to_string_lossy(),
            &username.to_string_lossy(),
            &deleted.to_string_lossy(),
            &userstate,
        );
    }
    mem::forget(userstate);
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_mod(
    client: *mut TmiClient,
    channel: *const c_char,
    username: *const c_char,
) {
    let channel = CStr::from_ptr(channel);
    let username = CStr::from_ptr(username);
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_mod {
        callback(
            &mut client,
            &channel.to_string_lossy(),
            &username.to_string_lossy(),
        );
    }
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_mods(
    client: *mut TmiClient,
    channel: *const c_char,
    mods: *mut TmiObject,
) {
    let channel = CStr::from_ptr(channel);
    let mods = Object::new(false, mods).expect("NULL object passed to callback");
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_mods {
        callback(&mut client, &channel.to_string_lossy(), &mods);
    }
    mem::forget(mods);
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_notice(
    client: *mut TmiClient,
    channel: *const c_char,
    msgid: *const c_char,
    msg: *const c_char,
) {
    let channel = CStr::from_ptr(channel);
    let msgid = CStr::from_ptr(msgid);
    let msg = CStr::from_ptr(msg);
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_notice {
        callback(
            &mut client,
            &channel.to_string_lossy(),
            &msgid.to_string_lossy(),
            &msg.to_string_lossy(),
        );
    }
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_part(
    client: *mut TmiClient,
    channel: *const c_char,
    username: *const c_char,
    self_: c_int,
) {
    let channel = CStr::from_ptr(channel);
    let username = CStr::from_ptr(username);
    let self_ = self_ != 0;
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_part {
        callback(
            &mut client,
            &channel.to_string_lossy(),
            &username.to_string_lossy(),
            self_,
        );
    }
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_ping(client: *mut TmiClient) {
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_ping {
        callback(&mut client);
    }
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_pong(client: *mut TmiClient, latency: f64) {
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_pong {
        callback(&mut client, latency);
    }
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_r9kbeta(
    client: *mut TmiClient,
    channel: *const c_char,
    enabled: c_int,
) {
    let channel = CStr::from_ptr(channel);
    let enabled = enabled != 0;
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_r9kbeta {
        callback(&mut client, &channel.to_string_lossy(), enabled);
    }
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_raided(
    client: *mut TmiClient,
    channel: *const c_char,
    username: *const c_char,
    viewers: c_int,
) {
    let channel = CStr::from_ptr(channel);
    let username = CStr::from_ptr(username);
    let viewers = viewers as usize;
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_raided {
        callback(
            &mut client,
            &channel.to_string_lossy(),
            &username.to_string_lossy(),
            viewers,
        );
    }
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_raw_message(
    client: *mut TmiClient,
    msg_cloned: *mut TmiObject,
    msg: *mut TmiObject,
) {
    let msg_cloned = Object::new(false, msg_cloned).expect("NULL object passed to callback");
    let msg = Object::new(false, msg).expect("NULL object passed to callback");
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_raw_message {
        callback(&mut client, &msg_cloned, &msg);
    }
    mem::forget(msg);
    mem::forget(msg_cloned);
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_reconnect(client: *mut TmiClient) {
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_reconnect {
        callback(&mut client);
    }
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_resub(
    client: *mut TmiClient,
    channel: *const c_char,
    username: *const c_char,
    streak: c_int,
    msg: *const c_char,
    userstate: *mut TmiObject,
    methods: *mut TmiObject,
) {
    let channel = CStr::from_ptr(channel);
    let username = CStr::from_ptr(username);
    let msg = CStr::from_ptr(msg);
    let userstate = Object::new(false, userstate).expect("NULL object passed to callback");
    let methods = Object::new(false, methods).expect("NULL object passed to callback");
    let streak = streak as usize;
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_resub {
        callback(
            &mut client,
            &channel.to_string_lossy(),
            &username.to_string_lossy(),
            streak,
            &msg.to_string_lossy(),
            &userstate,
            &methods,
        );
    }
    mem::forget(methods);
    mem::forget(userstate);
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_roomstate(
    client: *mut TmiClient,
    channel: *const c_char,
    state: *mut TmiObject,
) {
    let channel = CStr::from_ptr(channel);
    let state = Object::new(false, state).expect("NULL object passed to callback");
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_roomstate {
        callback(&mut client, &channel.to_string_lossy(), &state);
    }
    mem::forget(state);
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_serverchange(client: *mut TmiClient, channel: *const c_char) {
    let channel = CStr::from_ptr(channel);
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_serverchange {
        callback(&mut client, &channel.to_string_lossy());
    }
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_slowmode(
    client: *mut TmiClient,
    channel: *const c_char,
    enabled: c_int,
    length: c_int,
) {
    let channel = CStr::from_ptr(channel);
    let length = length as usize;
    let enabled = enabled != 0;
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_slowmode {
        callback(&mut client, &channel.to_string_lossy(), enabled, length);
    }
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_subgift(
    client: *mut TmiClient,
    channel: *const c_char,
    username: *const c_char,
    streak: c_int,
    recipient: *const c_char,
    methods: *mut TmiObject,
    userstate: *mut TmiObject,
) {
    let channel = CStr::from_ptr(channel);
    let username = CStr::from_ptr(username);
    let recipient = CStr::from_ptr(recipient);
    let methods = Object::new(false, methods).expect("NULL object passed to callback");
    let userstate = Object::new(false, userstate).expect("NULL object passed to callback");
    let streak = streak as usize;
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_subgift {
        callback(
            &mut client,
            &channel.to_string_lossy(),
            &username.to_string_lossy(),
            streak,
            &recipient.to_string_lossy(),
            &methods,
            &userstate,
        );
    }
    mem::forget(userstate);
    mem::forget(methods);
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_submysterygift(
    client: *mut TmiClient,
    channel: *const c_char,
    username: *const c_char,
    num: c_int,
    methods: *mut TmiObject,
    userstate: *mut TmiObject,
) {
    let channel = CStr::from_ptr(channel);
    let username = CStr::from_ptr(username);
    let methods = Object::new(false, methods).expect("NULL object passed to callback");
    let userstate = Object::new(false, userstate).expect("NULL object passed to callback");
    let num = num as usize;
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_submysterygift {
        callback(
            &mut client,
            &channel.to_string_lossy(),
            &username.to_string_lossy(),
            num,
            &methods,
            &userstate,
        );
    }
    mem::forget(userstate);
    mem::forget(methods);
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_subscribers(
    client: *mut TmiClient,
    channel: *const c_char,
    enabled: c_int,
) {
    let channel = CStr::from_ptr(channel);
    let enabled = enabled != 0;
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_subscribers {
        callback(&mut client, &channel.to_string_lossy(), enabled);
    }
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_subscription(
    client: *mut TmiClient,
    channel: *const c_char,
    username: *const c_char,
    methods: *mut TmiObject,
    msg: *const c_char,
    userstate: *mut TmiObject,
) {
    let channel = CStr::from_ptr(channel);
    let username = CStr::from_ptr(username);
    let msg = CStr::from_ptr(msg);
    let methods = Object::new(false, methods).expect("NULL object passed to callback");
    let userstate = Object::new(false, userstate).expect("NULL object passed to callback");
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_subscription {
        callback(
            &mut client,
            &channel.to_string_lossy(),
            &username.to_string_lossy(),
            &methods,
            &msg.to_string_lossy(),
            &userstate,
        );
    }
    mem::forget(userstate);
    mem::forget(methods);
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_timeout(
    client: *mut TmiClient,
    channel: *const c_char,
    username: *const c_char,
    duration: c_int,
    userstate: *mut TmiObject,
) {
    let channel = CStr::from_ptr(channel);
    let username = CStr::from_ptr(username);
    let userstate = Object::new(false, userstate).expect("NULL object passed to callback");
    let duration = duration as usize;
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_timeout {
        callback(
            &mut client,
            &channel.to_string_lossy(),
            &username.to_string_lossy(),
            duration,
            &userstate,
        );
    }
    mem::forget(userstate);
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_unhost(
    client: *mut TmiClient,
    channel: *const c_char,
    viewers: c_int,
) {
    let channel = CStr::from_ptr(channel);
    let viewers = viewers as usize;
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_unhost {
        callback(&mut client, &channel.to_string_lossy(), viewers);
    }
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_unmod(
    client: *mut TmiClient,
    channel: *const c_char,
    username: *const c_char,
) {
    let channel = CStr::from_ptr(channel);
    let username = CStr::from_ptr(username);
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_unmod {
        callback(
            &mut client,
            &channel.to_string_lossy(),
            &username.to_string_lossy(),
        );
    }
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_vips(
    client: *mut TmiClient,
    channel: *const c_char,
    vips: *mut TmiObject,
) {
    let channel = CStr::from_ptr(channel);
    let vips = Object::new(false, vips).expect("NULL object passed to callback");
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_vips {
        callback(&mut client, &channel.to_string_lossy(), &vips);
    }
    mem::forget(vips);
    mem::forget(client);
}

pub(crate) unsafe extern "C" fn on_whisper(
    client: *mut TmiClient,
    from: *const c_char,
    userstate: *mut TmiObject,
    msg: *const c_char,
    self_: c_int,
) {
    let from = CStr::from_ptr(from);
    let msg = CStr::from_ptr(msg);
    let userstate = Object::new(false, userstate).expect("NULL object passed to callback");
    let self_ = self_ != 0;
    let userdata = sys::tmi_userdata(client);
    let mut client =
        Client::with_userdata(client, userdata).expect("NULL client passed to callback");
    let userdata = client.userdata.clone();
    if let Some(callback) = &userdata.borrow().on_whisper {
        callback(
            &mut client,
            &from.to_string_lossy(),
            &userstate,
            &msg.to_string_lossy(),
            self_,
        );
    }
    mem::forget(userstate);
    mem::forget(client);
}

pub struct Userdata {
    on_action: Option<OnAction>,
    on_anongiftpaidupgrade: Option<OnAnongiftpaidupgrade>,
    on_ban: Option<OnBan>,
    on_chat: Option<OnChat>,
    on_cheer: Option<OnCheer>,
    on_clearchat: Option<OnClearchat>,
    on_connected: Option<OnConnected>,
    on_connecting: Option<OnConnecting>,
    on_disconnected: Option<OnDisconnected>,
    on_emoteonly: Option<OnEmoteonly>,
    on_emotesets: Option<OnEmotesets>,
    on_followersonly: Option<OnFollowersonly>,
    on_giftpaidupgrade: Option<OnGiftpaidupgrade>,
    on_hosted: Option<OnHosted>,
    on_hosting: Option<OnHosting>,
    on_join: Option<OnJoin>,
    on_logon: Option<OnLogon>,
    on_message: Option<OnMessage>,
    on_messagedeleted: Option<OnMessagedeleted>,
    on_mod: Option<OnMod>,
    on_mods: Option<OnMods>,
    on_notice: Option<OnNotice>,
    on_part: Option<OnPart>,
    on_ping: Option<OnPing>,
    on_pong: Option<OnPong>,
    on_r9kbeta: Option<OnR9kbeta>,
    on_raided: Option<OnRaided>,
    on_raw_message: Option<OnRawMessage>,
    on_reconnect: Option<OnReconnect>,
    on_resub: Option<OnResub>,
    on_roomstate: Option<OnRoomstate>,
    on_serverchange: Option<OnServerchange>,
    on_slowmode: Option<OnSlowmode>,
    on_subgift: Option<OnSubgift>,
    on_submysterygift: Option<OnSubmysterygift>,
    on_subscribers: Option<OnSubscribers>,
    on_subscription: Option<OnSubscription>,
    on_timeout: Option<OnTimeout>,
    on_unhost: Option<OnUnhost>,
    on_unmod: Option<OnUnmod>,
    on_vips: Option<OnVips>,
    on_whisper: Option<OnWhisper>,
}

impl Userdata {
    pub fn new() -> Self {
        Self {
            on_action: None::<OnAction>,
            on_anongiftpaidupgrade: None::<OnAnongiftpaidupgrade>,
            on_ban: None::<OnBan>,
            on_chat: None::<OnChat>,
            on_cheer: None::<OnCheer>,
            on_clearchat: None::<OnClearchat>,
            on_connected: None::<OnConnected>,
            on_connecting: None::<OnConnecting>,
            on_disconnected: None::<OnDisconnected>,
            on_emoteonly: None::<OnEmoteonly>,
            on_emotesets: None::<OnEmotesets>,
            on_followersonly: None::<OnFollowersonly>,
            on_giftpaidupgrade: None::<OnGiftpaidupgrade>,
            on_hosted: None::<OnHosted>,
            on_hosting: None::<OnHosting>,
            on_join: None::<OnJoin>,
            on_logon: None::<OnLogon>,
            on_message: None::<OnMessage>,
            on_messagedeleted: None::<OnMessagedeleted>,
            on_mod: None::<OnMod>,
            on_mods: None::<OnMods>,
            on_notice: None::<OnNotice>,
            on_part: None::<OnPart>,
            on_ping: None::<OnPing>,
            on_pong: None::<OnPong>,
            on_r9kbeta: None::<OnR9kbeta>,
            on_raided: None::<OnRaided>,
            on_raw_message: None::<OnRawMessage>,
            on_reconnect: None::<OnReconnect>,
            on_resub: None::<OnResub>,
            on_roomstate: None::<OnRoomstate>,
            on_serverchange: None::<OnServerchange>,
            on_slowmode: None::<OnSlowmode>,
            on_subgift: None::<OnSubgift>,
            on_submysterygift: None::<OnSubmysterygift>,
            on_subscribers: None::<OnSubscribers>,
            on_subscription: None::<OnSubscription>,
            on_timeout: None::<OnTimeout>,
            on_unhost: None::<OnUnhost>,
            on_unmod: None::<OnUnmod>,
            on_vips: None::<OnVips>,
            on_whisper: None::<OnWhisper>,
        }
    }
}

pub struct Client {
    inner: NonNull<TmiClient>,
    userdata: Rc<RefCell<Userdata>>,
}

impl Client {
    pub(crate) fn new(ptr: *mut TmiClient) -> Option<Self> {
        let inner = NonNull::new(ptr)?;
        Some(Client {
            inner,
            userdata: Rc::new(RefCell::new(Userdata::new())),
        })
    }

    pub(crate) unsafe fn with_userdata(ptr: *mut TmiClient, userdata: *mut c_void) -> Option<Self> {
        let inner = NonNull::new(ptr)?;
        Some(Client {
            inner,
            userdata: Rc::from_raw(userdata as *mut RefCell<Userdata>),
        })
    }

    pub(crate) fn as_ptr(&self) -> *mut TmiClient {
        self.inner.as_ptr()
    }

    pub fn connect(self) -> Promise {
        let userdata = Rc::into_raw(self.userdata);
        let inner = self.inner.as_ptr();

        let promise = unsafe { sys::tmi_connect(inner, userdata as *mut c_void) };
        Promise::new(promise).expect("tmi_connect returned NULL")
    }

    pub fn action(&mut self, channel: &str, message: &str) -> Promise {
        let client = self.as_ptr();
        let message = CString::new(message).expect("invalid cstring passed into message");
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe { sys::tmi_client_action(client, channel.as_ptr(), message.as_ptr()) };
        Promise::new(promise).expect("tmi_client_action returned NULL")
    }

    pub fn ban(&mut self, channel: &str, username: &str, reason: Option<&str>) -> Promise {
        let client = self.as_ptr();
        let reason =
            reason.map(|reason| CString::new(reason).expect("invalid cstring passed into reason"));
        let username = CString::new(username).expect("invalid cstring passed into username");
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe {
            sys::tmi_client_ban(
                client,
                channel.as_ptr(),
                username.as_ptr(),
                reason.map(|cstr| cstr.as_ptr()).unwrap_or(ptr::null()),
            )
        };
        Promise::new(promise).expect("tmi_client_ban returned NULL")
    }

    pub fn clear(&mut self, channel: &str) -> Promise {
        let client = self.as_ptr();
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe { sys::tmi_client_clear(client, channel.as_ptr()) };
        Promise::new(promise).expect("tmi_client_clear returned NULL")
    }

    pub fn color(&mut self, color: &str) -> Promise {
        let client = self.as_ptr();
        let color = CString::new(color).expect("invalid cstring passed into color");
        let promise = unsafe { sys::tmi_client_color(client, color.as_ptr()) };
        Promise::new(promise).expect("tmi_client_color returned NULL")
    }

    pub fn commercial(&mut self, channel: &str, seconds: u64) -> Promise {
        let client = self.as_ptr();
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe { sys::tmi_client_commercial(client, channel.as_ptr(), seconds as _) };
        Promise::new(promise).expect("tmi_client_commercial returned NULL")
    }

    pub fn deletemessage(&mut self, channel: &str, uuid: &str) -> Promise {
        let client = self.as_ptr();
        let uuid = CString::new(uuid).expect("invalid cstring passed into uuid");
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise =
            unsafe { sys::tmi_client_deletemessage(client, channel.as_ptr(), uuid.as_ptr()) };
        Promise::new(promise).expect("tmi_client_deletemessage returned NULL")
    }

    pub fn emoteonly(&mut self, channel: &str) -> Promise {
        let client = self.as_ptr();
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe { sys::tmi_client_emoteonly(client, channel.as_ptr()) };
        Promise::new(promise).expect("tmi_client_emoteonly returned NULL")
    }

    pub fn emoteonlyoff(&mut self, channel: &str) -> Promise {
        let client = self.as_ptr();
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe { sys::tmi_client_emoteonlyoff(client, channel.as_ptr()) };
        Promise::new(promise).expect("tmi_client_emoteonlyoff returned NULL")
    }

    pub fn followersonly(&mut self, channel: &str, length: Option<u64>) -> Promise {
        let client = self.as_ptr();
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe {
            sys::tmi_client_followersonly(client, channel.as_ptr(), length.unwrap_or(u64::MAX) as _)
        };
        Promise::new(promise).expect("tmi_client_followersonly returned NULL")
    }

    pub fn followersonlyoff(&mut self, channel: &str) -> Promise {
        let client = self.as_ptr();
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe { sys::tmi_client_followersonlyoff(client, channel.as_ptr()) };
        Promise::new(promise).expect("tmi_client_followersonlyoff returned NULL")
    }

    pub fn host(&mut self, channel: &str, target: &str) -> Promise {
        let client = self.as_ptr();
        let target = CString::new(target).expect("invalid cstring passed into target");
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe { sys::tmi_client_host(client, channel.as_ptr(), target.as_ptr()) };
        Promise::new(promise).expect("tmi_client_host returned NULL")
    }

    pub fn join(&mut self, channel: &str) -> Promise {
        let client = self.as_ptr();
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe { sys::tmi_client_join(client, channel.as_ptr()) };
        Promise::new(promise).expect("tmi_client_join returned NULL")
    }

    pub fn mod_(&mut self, channel: &str, username: &str) -> Promise {
        let client = self.as_ptr();
        let username = CString::new(username).expect("invalid cstring passed into username");
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe { sys::tmi_client_mod(client, channel.as_ptr(), username.as_ptr()) };
        Promise::new(promise).expect("tmi_client_mod returned NULL")
    }

    pub fn mods(&mut self, mods: &str) -> Promise {
        let client = self.as_ptr();
        let mods = CString::new(mods).expect("invalid cstring passed into mods");
        let promise = unsafe { sys::tmi_client_mods(client, mods.as_ptr()) };
        Promise::new(promise).expect("tmi_client_mods returned NULL")
    }

    pub fn part(&mut self, channel: &str) -> Promise {
        let client = self.as_ptr();
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe { sys::tmi_client_part(client, channel.as_ptr()) };
        Promise::new(promise).expect("tmi_client_part returned NULL")
    }

    pub fn ping(&mut self) -> Promise {
        let client = self.as_ptr();
        let promise = unsafe { sys::tmi_client_ping(client) };
        Promise::new(promise).expect("tmi_client_ping returned NULL")
    }

    pub fn r9kbeta(&mut self, channel: &str) -> Promise {
        let client = self.as_ptr();
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe { sys::tmi_client_r9kbeta(client, channel.as_ptr()) };
        Promise::new(promise).expect("tmi_client_r9kbeta returned NULL")
    }

    pub fn r9kbetaoff(&mut self, channel: &str) -> Promise {
        let client = self.as_ptr();
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe { sys::tmi_client_r9kbetaoff(client, channel.as_ptr()) };
        Promise::new(promise).expect("tmi_client_r9kbetaoff returned NULL")
    }

    pub fn raw(&mut self, msg: &str) -> Promise {
        let client = self.as_ptr();
        let msg = CString::new(msg).expect("invalid cstring passed into msg");
        let promise = unsafe { sys::tmi_client_raw(client, msg.as_ptr()) };
        Promise::new(promise).expect("tmi_client_raw returned NULL")
    }

    pub fn say(&mut self, channel: &str, msg: &str) -> Promise {
        let client = self.as_ptr();
        let msg = CString::new(msg).expect("invalid cstring passed into msg");
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe { sys::tmi_client_say(client, channel.as_ptr(), msg.as_ptr()) };
        Promise::new(promise).expect("tmi_client_say returned NULL")
    }

    pub fn slow(&mut self, channel: &str, length: Option<u64>) -> Promise {
        let client = self.as_ptr();
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe {
            sys::tmi_client_slow(client, channel.as_ptr(), length.unwrap_or(u64::MAX) as _)
        };
        Promise::new(promise).expect("tmi_client_slow returned NULL")
    }

    pub fn slowoff(&mut self, channel: &str) -> Promise {
        let client = self.as_ptr();
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe { sys::tmi_client_slowoff(client, channel.as_ptr()) };
        Promise::new(promise).expect("tmi_client_slowoff returned NULL")
    }

    pub fn subscribers(&mut self, channel: &str) -> Promise {
        let client = self.as_ptr();
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe { sys::tmi_client_subscribers(client, channel.as_ptr()) };
        Promise::new(promise).expect("tmi_client_subscribers returned NULL")
    }

    pub fn subscribersoff(&mut self, channel: &str) -> Promise {
        let client = self.as_ptr();
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe { sys::tmi_client_subscribersoff(client, channel.as_ptr()) };
        Promise::new(promise).expect("tmi_client_subscribersoff returned NULL")
    }

    pub fn timeout(
        &mut self,
        channel: &str,
        username: &str,
        length: Option<u64>,
        reason: Option<&str>,
    ) -> Promise {
        let client = self.as_ptr();
        let reason =
            reason.map(|reason| CString::new(reason).expect("invalid cstring passed into reason"));
        let username = CString::new(username).expect("invalid cstring passed into username");
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe {
            sys::tmi_client_timeout(
                client,
                channel.as_ptr(),
                username.as_ptr(),
                length.unwrap_or(u64::MAX) as _,
                reason.map(|cstr| cstr.as_ptr()).unwrap_or(ptr::null()),
            )
        };
        Promise::new(promise).expect("tmi_client_timeout returned NULL")
    }

    pub fn unban(&mut self, channel: &str, username: &str) -> Promise {
        let client = self.as_ptr();
        let username = CString::new(username).expect("invalid cstring passed into username");
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe { sys::tmi_client_unban(client, channel.as_ptr(), username.as_ptr()) };
        Promise::new(promise).expect("tmi_client_unban returned NULL")
    }

    pub fn unhost(&mut self, channel: &str) -> Promise {
        let client = self.as_ptr();
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe { sys::tmi_client_unhost(client, channel.as_ptr()) };
        Promise::new(promise).expect("tmi_client_unhost returned NULL")
    }

    pub fn unmod(&mut self, channel: &str) -> Promise {
        let client = self.as_ptr();
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe { sys::tmi_client_unmod(client, channel.as_ptr()) };
        Promise::new(promise).expect("tmi_client_unmod returned NULL")
    }

    pub fn unvip(&mut self, channel: &str, username: &str) -> Promise {
        let client = self.as_ptr();
        let username = CString::new(username).expect("invalid cstring passed into username");
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe { sys::tmi_client_unvip(client, channel.as_ptr(), username.as_ptr()) };
        Promise::new(promise).expect("tmi_client_unvip returned NULL")
    }

    pub fn vip(&mut self, channel: &str, username: &str) -> Promise {
        let client = self.as_ptr();
        let username = CString::new(username).expect("invalid cstring passed into username");
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe { sys::tmi_client_vip(client, channel.as_ptr(), username.as_ptr()) };
        Promise::new(promise).expect("tmi_client_vip returned NULL")
    }

    pub fn vips(&mut self, channel: &str) -> Promise {
        let client = self.as_ptr();
        let channel = CString::new(channel).expect("invalid cstring passed into channel");
        let promise = unsafe { sys::tmi_client_vips(client, channel.as_ptr()) };
        Promise::new(promise).expect("tmi_client_vips returned NULL")
    }

    pub fn whisper(&mut self, username: &str, msg: &str) -> Promise {
        let client = self.as_ptr();
        let msg = CString::new(msg).expect("invalid cstring passed into msg");
        let username = CString::new(username).expect("invalid cstring passed into username");
        let promise = unsafe { sys::tmi_client_whisper(client, username.as_ptr(), msg.as_ptr()) };
        Promise::new(promise).expect("tmi_client_whisper returned NULL")
    }

    pub fn on_action<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &Object, &str, bool) + 'static,
    {
        self.userdata.borrow_mut().on_action = Some(Box::new(callback));
    }

    pub fn on_anongiftpaidupgrade<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &str, &Object) + 'static,
    {
        self.userdata.borrow_mut().on_anongiftpaidupgrade = Some(Box::new(callback));
    }

    pub fn on_ban<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &str, &Object) + 'static,
    {
        self.userdata.borrow_mut().on_ban = Some(Box::new(callback));
    }

    pub fn on_chat<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &Object, &str, bool) + 'static,
    {
        self.userdata.borrow_mut().on_chat = Some(Box::new(callback));
    }

    pub fn on_cheer<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &Object, &str) + 'static,
    {
        self.userdata.borrow_mut().on_cheer = Some(Box::new(callback));
    }

    pub fn on_clearchat<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str) + 'static,
    {
        self.userdata.borrow_mut().on_clearchat = Some(Box::new(callback));
    }

    pub fn on_connected<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, u16) + 'static,
    {
        self.userdata.borrow_mut().on_connected = Some(Box::new(callback));
    }

    pub fn on_connecting<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, u16) + 'static,
    {
        self.userdata.borrow_mut().on_connecting = Some(Box::new(callback));
    }

    pub fn on_disconnected<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str) + 'static,
    {
        self.userdata.borrow_mut().on_disconnected = Some(Box::new(callback));
    }

    pub fn on_emoteonly<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, bool) + 'static,
    {
        self.userdata.borrow_mut().on_emoteonly = Some(Box::new(callback));
    }

    pub fn on_emotesets<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &Object) + 'static,
    {
        self.userdata.borrow_mut().on_emotesets = Some(Box::new(callback));
    }

    pub fn on_followersonly<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, bool, usize) + 'static,
    {
        self.userdata.borrow_mut().on_followersonly = Some(Box::new(callback));
    }

    pub fn on_giftpaidupgrade<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &str, &str, &Object) + 'static,
    {
        self.userdata.borrow_mut().on_giftpaidupgrade = Some(Box::new(callback));
    }

    pub fn on_hosted<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &str, usize, bool) + 'static,
    {
        self.userdata.borrow_mut().on_hosted = Some(Box::new(callback));
    }

    pub fn on_hosting<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &str, usize) + 'static,
    {
        self.userdata.borrow_mut().on_hosting = Some(Box::new(callback));
    }

    pub fn on_join<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &str, bool) + 'static,
    {
        self.userdata.borrow_mut().on_join = Some(Box::new(callback));
    }

    pub fn on_logon<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client) + 'static,
    {
        self.userdata.borrow_mut().on_logon = Some(Box::new(callback));
    }

    pub fn on_message<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &Object, &str, bool) + 'static,
    {
        self.userdata.borrow_mut().on_message = Some(Box::new(callback));
    }

    pub fn on_messagedeleted<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &str, &str, &Object) + 'static,
    {
        self.userdata.borrow_mut().on_messagedeleted = Some(Box::new(callback));
    }

    pub fn on_mod<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &str) + 'static,
    {
        self.userdata.borrow_mut().on_mod = Some(Box::new(callback));
    }

    pub fn on_mods<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &Object) + 'static,
    {
        self.userdata.borrow_mut().on_mods = Some(Box::new(callback));
    }

    pub fn on_notice<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &str, &str) + 'static,
    {
        self.userdata.borrow_mut().on_notice = Some(Box::new(callback));
    }

    pub fn on_part<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &str, bool) + 'static,
    {
        self.userdata.borrow_mut().on_part = Some(Box::new(callback));
    }

    pub fn on_ping<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client) + 'static,
    {
        self.userdata.borrow_mut().on_ping = Some(Box::new(callback));
    }

    pub fn on_pong<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, f64) + 'static,
    {
        self.userdata.borrow_mut().on_pong = Some(Box::new(callback));
    }

    pub fn on_r9kbeta<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, bool) + 'static,
    {
        self.userdata.borrow_mut().on_r9kbeta = Some(Box::new(callback));
    }

    pub fn on_raided<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &str, usize) + 'static,
    {
        self.userdata.borrow_mut().on_raided = Some(Box::new(callback));
    }

    pub fn on_raw_message<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &Object, &Object) + 'static,
    {
        self.userdata.borrow_mut().on_raw_message = Some(Box::new(callback));
    }

    pub fn on_reconnect<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client) + 'static,
    {
        self.userdata.borrow_mut().on_reconnect = Some(Box::new(callback));
    }

    pub fn on_resub<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &str, usize, &str, &Object, &Object) + 'static,
    {
        self.userdata.borrow_mut().on_resub = Some(Box::new(callback));
    }

    pub fn on_roomstate<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &Object) + 'static,
    {
        self.userdata.borrow_mut().on_roomstate = Some(Box::new(callback));
    }

    pub fn on_serverchange<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str) + 'static,
    {
        self.userdata.borrow_mut().on_serverchange = Some(Box::new(callback));
    }

    pub fn on_slowmode<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, bool, usize) + 'static,
    {
        self.userdata.borrow_mut().on_slowmode = Some(Box::new(callback));
    }

    pub fn on_subgift<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &str, usize, &str, &Object, &Object) + 'static,
    {
        self.userdata.borrow_mut().on_subgift = Some(Box::new(callback));
    }

    pub fn on_submysterygift<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &str, usize, &Object, &Object) + 'static,
    {
        self.userdata.borrow_mut().on_submysterygift = Some(Box::new(callback));
    }

    pub fn on_subscribers<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, bool) + 'static,
    {
        self.userdata.borrow_mut().on_subscribers = Some(Box::new(callback));
    }

    pub fn on_subscription<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &str, &Object, &str, &Object) + 'static,
    {
        self.userdata.borrow_mut().on_subscription = Some(Box::new(callback));
    }

    pub fn on_timeout<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &str, usize, &Object) + 'static,
    {
        self.userdata.borrow_mut().on_timeout = Some(Box::new(callback));
    }

    pub fn on_unhost<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, usize) + 'static,
    {
        self.userdata.borrow_mut().on_unhost = Some(Box::new(callback));
    }

    pub fn on_unmod<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &str) + 'static,
    {
        self.userdata.borrow_mut().on_unmod = Some(Box::new(callback));
    }

    pub fn on_vips<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &Object) + 'static,
    {
        self.userdata.borrow_mut().on_vips = Some(Box::new(callback));
    }

    pub fn on_whisper<F>(&mut self, callback: F)
    where
        F: Fn(&mut Client, &str, &Object, &str, bool) + 'static,
    {
        self.userdata.borrow_mut().on_whisper = Some(Box::new(callback));
    }
}
