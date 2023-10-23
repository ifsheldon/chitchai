use dioxus::prelude::*;
use uuid::Uuid;

use crate::components::LeftSidebarEvent;
use crate::utils::datetime::DatetimeString;
use crate::utils::storage::StoredStates;

pub fn ChatHistorySidebar(cx: Scope) -> Element {
    let chat_event_handler = use_coroutine_handle::<LeftSidebarEvent>(cx).unwrap();
    let chats: Vec<(String, DatetimeString, Uuid)> = use_shared_state::<StoredStates>(cx)
        .unwrap()
        .read()
        .chats
        .iter()
        .map(|c| (c.topic.clone(), c.date.clone(), c.id))
        .collect();

    render! {
        div {
            class: "h-screen w-52 overflow-y-auto bg-slate-50 py-8 dark:bg-slate-900 sm:w-60",
            div {
                class: "flex items-start",
                h2 {
                    class: "inline px-5 text-lg font-medium text-slate-800 dark:text-slate-200",
                    "Chats"
                }
                span {
                    class: "rounded-full bg-blue-600 px-2 py-1 text-xs text-slate-200",
                    "{chats.len()}"
                }
            }
            div {
                class: "mx-2 mt-8 space-y-4",
                // chat list
                chats.into_iter().rev().map(|(title, date, id)| rsx!{
                    ChatHistoryItem {
                        on_click: move |_| {
                            chat_event_handler.send(LeftSidebarEvent::ChangeChat(id))
                        },
                        title: title,
                        date: date.0,
                    }
                })
            }
        }
    }
}

#[derive(Props)]
pub struct ChatHistoryItemProps<'a> {
    pub title: String,
    pub date: String,
    pub on_click: EventHandler<'a, MouseEvent>,
}

pub fn ChatHistoryItem<'a>(cx: Scope<'a, ChatHistoryItemProps>) -> Element<'a> {
    render! {
        button {
            onclick: |event| {
                cx.props.on_click.call(event);
            },
            class: "flex w-full flex-col gap-y-2 rounded-lg px-3 py-2 text-left transition-colors duration-200 hover:bg-slate-200 focus:outline-none dark:hover:bg-slate-800",
            h1 {
                class: "text-sm font-medium capitalize text-slate-700 dark:text-slate-200",
                "{cx.props.title}"
            }
            p {
                class: "text-xs text-slate-500 dark:text-slate-400",
                "{cx.props.date}"
            }
        }
    }
}
