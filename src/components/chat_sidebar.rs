use dioxus::prelude::*;
use futures_util::StreamExt;

use crate::app::AppEvents;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChatSidebarEvent {
    ToggleChatHistory,
    NewChat,
    EnterDiscovery,
    EnterUserProfile,
}

async fn event_handler(mut rx: UnboundedReceiver<ChatSidebarEvent>, show_chat: UseState<bool>) {
    while let Some(event) = rx.next().await {
        match event {
            ChatSidebarEvent::ToggleChatHistory => show_chat.modify(|s| !(*s)),
            ChatSidebarEvent::NewChat => {
                // TODO: implement adding a new chat
                log::info!("NewChat");
            }
            ChatSidebarEvent::EnterDiscovery => {
                // TODO: implement entering discovery
                log::info!("EnterDiscovery");
            }
            ChatSidebarEvent::EnterUserProfile => {
                // TODO: implement entering user profile
                log::info!("EnterUserProfile");
            }
            _ => log::warn!("Unknown event: {:?}", event),
        }
    }
}

pub fn ChatSidebar(cx: Scope) -> Element {
    let show_chat_history = use_state(cx, || false);
    use_coroutine(cx, |rx| event_handler(rx, show_chat_history.to_owned()));
    render! {
        aside {
            class: "flex",
            IconSidebar {}
            if *show_chat_history.get() {
                rsx! {
                    ChatHistorySidebar {}
                }
            }
        }
    }
}

pub fn IconSidebar(cx: Scope) -> Element {
    render! {
        div {
            class: "flex h-screen w-12 flex-col items-center space-y-8 border-r border-slate-300 bg-slate-50 py-8 dark:border-slate-700 dark:bg-slate-900 sm:w-16",
            Logo(cx),
            NewConversationButton(cx),
            ConversationListButton(cx),
            DiscoverButton(cx),
            UserProfileButton(cx),
            SettingsButton(cx),
        }
    }
}

pub fn ChatHistorySidebar(cx: Scope) -> Element {
    let FAKE_ITEMS: &[ChatRecord] = &[
        ChatRecord {
            title: "Tailwind Classes".to_string(),
            date: "12 Mar".to_string(),
        },
        ChatRecord {
            title: "explain quantum computing".to_string(),
            date: "10 Feb".to_string(),
        },
        ChatRecord {
            title: "How to create ERP Diagram".to_string(),
            date: "22 Jan".to_string(),
        },
        ChatRecord {
            title: "API Scaling Strategies".to_string(),
            date: "1 Jan".to_string(),
        },
    ];

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
                    "{FAKE_ITEMS.len()}"
                }
            }
            div {
                class: "mx-2 mt-8 space-y-4",
                // chat list
                FAKE_ITEMS.iter().map(|item| rsx!{
                    ChatHistoryItem {
                        chat_record: item.clone(),
                    }
                })
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChatRecord {
    pub title: String,
    pub date: String,
}

#[inline_props]
pub fn ChatHistoryItem(cx: Scope, chat_record: ChatRecord) -> Element {
    render! {
        button {
            class: "flex w-full flex-col gap-y-2 rounded-lg px-3 py-2 text-left transition-colors duration-200 hover:bg-slate-200 focus:outline-none dark:hover:bg-slate-800",
            h1 {
                class: "text-sm font-medium capitalize text-slate-700 dark:text-slate-200",
                "{chat_record.title}"
            }
            p {
                class: "text-xs text-slate-500 dark:text-slate-400",
                "{chat_record.date}"
            }
        }
    }
}

pub fn Logo(cx: Scope) -> Element {
    render! {
        a {
            href: "#",
            class: "mb-1",
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                class: "h-7 w-7 text-blue-600",
                fill: "currentColor",
                stroke_width: "1",
                view_box: "0 0 24 24",
                path {
                    d: "M20.553 3.105l-6 3C11.225 7.77 9.274 9.953 8.755 12.6c-.738 3.751 1.992 7.958 2.861 8.321A.985.985 0 0012 21c6.682 0 11-3.532 11-9 0-6.691-.9-8.318-1.293-8.707a1 1 0 00-1.154-.188zm-7.6 15.86a8.594 8.594 0 015.44-8.046 1 1 0 10-.788-1.838 10.363 10.363 0 00-6.393 7.667 6.59 6.59 0 01-.494-3.777c.4-2 1.989-3.706 4.728-5.076l5.03-2.515A29.2 29.2 0 0121 12c0 4.063-3.06 6.67-8.046 6.965zM3.523 5.38A29.2 29.2 0 003 12a6.386 6.386 0 004.366 6.212 1 1 0 11-.732 1.861A8.377 8.377 0 011 12c0-6.691.9-8.318 1.293-8.707a1 1 0 011.154-.188l6 3A1 1 0 018.553 7.9z",
                }
            }
        }
    }
}

pub fn NewConversationButton(cx: Scope) -> Element {
    let chat_sidebar_event_handler = use_coroutine_handle::<ChatSidebarEvent>(cx).unwrap();
    render! {
        a {
            href: "#",
            class: "rounded-lg p-1.5 text-slate-500 transition-colors duration-200 hover:bg-slate-200 focus:outline-none dark:text-slate-400 dark:hover:bg-slate-800",
            onclick: |_| chat_sidebar_event_handler.send(ChatSidebarEvent::NewChat),
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                class: "h-6 w-6",
                view_box: "0 0 24 24",
                stroke_width: "2",
                stroke: "currentColor",
                fill: "none",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path {
                    stroke: "none",
                    d: "M0 0h24v24H0z",
                    fill: "none",
                }
                path {
                    d: "M8 9h8",
                }
                path {
                    d: "M8 13h6",
                }
                path {
                    d: "M12.01 18.594l-4.01 2.406v-3h-2a3 3 0 0 1 -3 -3v-8a3 3 0 0 1 3 -3h12a3 3 0 0 1 3 3v5.5",
                }
                path {
                    d: "M16 19h6",
                }
                path {
                    d: "M19 16v6",
                }
            }
        }
    }
}

pub fn ConversationListButton(cx: Scope) -> Element {
    let chat_sidebar_event_handler = use_coroutine_handle::<ChatSidebarEvent>(cx).unwrap();
    render! {
        a {
            href: "#",
            class: "rounded-lg bg-blue-100 p-1.5 text-blue-600 transition-colors duration-200 dark:bg-slate-800 dark:text-blue-600",
            onclick: |_| chat_sidebar_event_handler.send(ChatSidebarEvent::ToggleChatHistory),
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                class: "h-6 w-6",
                view_box: "0 0 24 24",
                stroke_width: "2",
                stroke: "currentColor",
                fill: "none",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path {
                    stroke: "none",
                    d: "M0 0h24v24H0z",
                    fill: "none",
                }
                path {
                    d: "M21 14l-3 -3h-7a1 1 0 0 1 -1 -1v-6a1 1 0 0 1 1 -1h9a1 1 0 0 1 1 1v10",
                }
                path {
                    d: "M14 15v2a1 1 0 0 1 -1 1h-7l-3 3v-10a1 1 0 0 1 1 -1h2",
                }
            }
        }
    }
}

pub fn DiscoverButton(cx: Scope) -> Element {
    let chat_sidebar_event_handler = use_coroutine_handle::<ChatSidebarEvent>(cx).unwrap();
    render! {
        a {
            href: "#",
            class: "rounded-lg p-1.5 text-slate-500 transition-colors duration-200 hover:bg-slate-200 focus:outline-none dark:text-slate-400 dark:hover:bg-slate-800",
            onclick: |_| chat_sidebar_event_handler.send(ChatSidebarEvent::EnterDiscovery),
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                class: "h-6 w-6",
                view_box: "0 0 24 24",
                stroke_width: "2",
                stroke: "currentColor",
                fill: "none",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path {
                    stroke: "none",
                    d: "M0 0h24v24H0z",
                    fill: "none",
                }
                path {
                    d: "M10 10m-7 0a7 7 0 1 0 14 0a7 7 0 1 0 -14 0",
                }
                path {
                    d: "M21 21l-6 -6",
                }
            }
        }
    }
}

pub fn UserProfileButton(cx: Scope) -> Element {
    let chat_sidebar_event_handler = use_coroutine_handle::<ChatSidebarEvent>(cx).unwrap();
    render! {
        a {
            href: "#",
            class: "rounded-lg p-1.5 text-slate-500 transition-colors duration-200 hover:bg-slate-200 focus:outline-none dark:text-slate-400 dark:hover:bg-slate-800",
            onclick: |_| chat_sidebar_event_handler.send(ChatSidebarEvent::EnterUserProfile),
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                class: "h-6 w-6",
                view_box: "0 0 24 24",
                stroke_width: "2",
                stroke: "currentColor",
                fill: "none",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path {
                    stroke: "none",
                    d: "M0 0h24v24H0z",
                    fill: "none",
                }
                path {
                    d: "M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0",
                }
                path {
                    d: "M12 10m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0",
                }
                path {
                    d: "M6.168 18.849a4 4 0 0 1 3.832 -2.849h4a4 4 0 0 1 3.834 2.855",
                }
            }
        }
    }
}

pub fn SettingsButton(cx: Scope) -> Element {
    let app_event_handler = use_coroutine_handle::<AppEvents>(cx).unwrap();
    render! {
        a {
            href: "#",
            class: "rounded-lg p-1.5 text-slate-500 transition-colors duration-200 hover:bg-slate-200 focus:outline-none dark:text-slate-400 dark:hover:bg-slate-800",
            onclick: |_| app_event_handler.send(AppEvents::ToggleSettingsSidebar),
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                class: "h-6 w-6",
                view_box: "0 0 24 24",
                stroke_width: "2",
                stroke: "currentColor",
                fill: "none",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path {
                    stroke: "none",
                    d: "M0 0h24v24H0z",
                    fill: "none",
                }
                path {
                    d: "M19.875 6.27a2.225 2.225 0 0 1 1.125 1.948v7.284c0 .809 -.443 1.555 -1.158 1.948l-6.75 4.27a2.269 2.269 0 0 1 -2.184 0l-6.75 -4.27a2.225 2.225 0 0 1 -1.158 -1.948v-7.285c0 -.809 .443 -1.554 1.158 -1.947l6.75 -3.98a2.33 2.33 0 0 1 2.25 0l6.75 3.98h-.033z",
                }
                path {
                    d: "M12 12m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0",
                }
            }
        }
    }
}
