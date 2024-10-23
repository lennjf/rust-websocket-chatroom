

use chrono::Utc;
use common::{ChatMessage, WebSocketMessage, WebSocketMessageType};
use serde_json::json;
use yew::prelude::*;
use yew_hooks::use_websocket;
use crate::message_list::*;
use crate::send_dialog::*;
use crate::users_list::*;

mod message_list;
mod send_dialog;
mod users_list;

#[function_component]
fn App() -> Html {
    let messages_handle = use_state(Vec::default);
    let messages = (*messages_handle).clone();
    let users_handle = use_state(Vec::default);
    let users = (*users_handle).clone();
    let username_handle = use_state(String::default);
    let username = (*username_handle).clone();
   
    let ws = use_websocket("ws://127.0.0.1:8000".to_string());

    let mut cloned_messages = messages.clone();
    use_effect_with(ws.message.clone(), move |ws_message|{
        if let Some(ws_msg) = &**ws_message {
            let web_socket_message: WebSocketMessage = serde_json::from_str(&ws_msg).unwrap();
            match web_socket_message.message_type {
                WebSocketMessageType::NewMessage => {
                    let msg = web_socket_message.message.expect("missing message payload");
                    cloned_messages.push(msg);
                    messages_handle.set(cloned_messages);
                },
                WebSocketMessageType::UserList => {
                    let ulist = web_socket_message.users.expect("missing users payload");
                    users_handle.set(ulist);
                    messages_handle.set(cloned_messages);
                },
                WebSocketMessageType::UsernameChange => {
                    let username = web_socket_message.username.expect("missing username payload");
                    username_handle.set(username);
                }
            }
            
        }
    });

    let cloned_username = username.clone();
    let cloned_ws = ws.clone();
    let send_message_callback = Callback::from(move |msg: String|{
        let websocket_message = WebSocketMessage { 
            message_type: WebSocketMessageType::NewMessage, 
            message: Some(ChatMessage { 
                message: msg, 
                author: cloned_username.clone(), 
                created_at: Utc::now().naive_utc(),
            }), 
            users: None, 
            username: None 
        };
        cloned_ws.send(json!(websocket_message).to_string());
    });

    let cloned_ws = ws.clone();
    let change_username_callback = Callback::from(move |username: String|{
        let websocket_message = WebSocketMessage { 
            message_type: WebSocketMessageType::UsernameChange, 
            message: None, 
            users: None, 
            username: Some(username.clone()) 
        };
        cloned_ws.send(json!(websocket_message).to_string());

    });


    html!{
        <div class = "container-fluid"  >
            <div class = "row" >
                <div class = "col-sm-1"><UsersList users = {users} /></div>
                
                <div class = "col-sm-11"><MessageList messages= {messages} /></div>
            </div>
            <div class = "row" >
                if username.len() > 0 {
                    <SendDialog 
                        change_username_callback = {change_username_callback}
                        send_message_callback = {send_message_callback} 
                        username = {username}/>
                } 
            </div>



        </div>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
