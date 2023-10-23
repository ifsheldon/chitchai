use dioxus::prelude::*;
use futures_util::StreamExt;
use uuid::Uuid;

use crate::app::{AppEvents, ChatId};
use crate::utils::datetime::DatetimeString;
use crate::utils::storage::StoredStates;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChatSidebarEvent {
    ToggleChatHistory,
    ChangeChat(Uuid),
    NewChat,
    EnterDiscovery,
    EnterUserProfile,
}

async fn event_handler(mut rx: UnboundedReceiver<ChatSidebarEvent>,
                       show_chat: UseState<bool>,
                       showing_chat_id: UseSharedState<ChatId>) {
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
            ChatSidebarEvent::ChangeChat(chat_id) => {
                // TODO: disable switching when a reply stream is receiving
                log::info!("Changing to Chat {}", chat_id);
                showing_chat_id.write().0 = chat_id;
            }
            _ => log::warn!("Unknown event: {:?}", event),
        }
    }
}

pub fn ChatSidebar(cx: Scope) -> Element {
    let show_chat_history = use_state(cx, || false);
    let showing_chat_id = use_shared_state::<ChatId>(cx).unwrap();
    use_coroutine(cx, |rx| event_handler(rx, show_chat_history.to_owned(), showing_chat_id.to_owned()));
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

pub struct NowActive(pub Option<usize>);

pub fn IconSidebar(cx: Scope) -> Element {
    let now_active = use_state(cx, || NowActive(None));
    render! {
        div {
            class: "flex h-screen w-12 flex-col items-center space-y-8 border-r border-slate-300 bg-slate-50 py-8 dark:border-slate-700 dark:bg-slate-900 sm:w-16",
            Logo {},
            NewConversationButton {},
            ConversationListButton {
                preemption: Some(now_active.to_owned()),
                idx: 0,
            },
            DiscoverButton {
                preemption: Some(now_active.to_owned()),
                idx: 1,
            },
            UserProfileButton {
                preemption: Some(now_active.to_owned()),
                idx: 2,
            },
            SettingsButton {},
        }
    }
}

pub fn ChatHistorySidebar(cx: Scope) -> Element {
    let chat_event_handler = use_coroutine_handle::<ChatSidebarEvent>(cx).unwrap();
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
                            chat_event_handler.send(ChatSidebarEvent::ChangeChat(id))
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


#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    #[props(! optional)]
    preemption: Option<UseState<NowActive>>,
    idx: usize,
}

#[derive(Props)]
pub struct RawButtonProps<'a> {
    button_props: Option<&'a ButtonProps>,
    on_click: EventHandler<'a, MouseEvent>,
    children: Element<'a>,
}

pub fn RawButton<'a>(cx: Scope<'a, RawButtonProps<'a>>) -> Element<'a> {
    const BUTTON_INACTIVE_STYLE: &str = "rounded-lg p-1.5 text-slate-500 transition-colors duration-200 hover:bg-slate-200 focus:outline-none dark:text-slate-400 dark:hover:bg-slate-800";
    const BUTTON_ACTIVE_STYLE: &str = "rounded-lg bg-blue-100 p-1.5 text-blue-600 transition-colors duration-200 dark:bg-slate-800 dark:text-blue-600";
    let preemptive = cx.props.button_props
        .is_some_and(|p|
            p.preemption.is_some());
    let active = cx.props.button_props
        .is_some_and(|p|
            p.preemption
                .as_ref()
                .is_some_and(|s|
                    s.get().0
                        .is_some_and(|i| i == p.idx)));
    render! {
        a {
            href: "#",
            class: if preemptive && active {BUTTON_ACTIVE_STYLE} else {BUTTON_INACTIVE_STYLE},
            onclick: move |event| {
                cx.props.on_click.call(event);
                if preemptive {
                    let button_props = cx.props.button_props.unwrap();
                    let preemption = button_props.preemption.as_ref().unwrap();
                    if active {
                        preemption.set(NowActive(None));
                    } else {
                        preemption.set(NowActive(Some(button_props.idx)));
                    };
                }
            },
            &cx.props.children
        }
    }
}

pub fn NewConversationButton(cx: Scope) -> Element {
    let chat_sidebar_event_handler = use_coroutine_handle::<ChatSidebarEvent>(cx).unwrap();
    render! {
        RawButton {
            on_click: move |_| chat_sidebar_event_handler.send(ChatSidebarEvent::NewChat),
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

pub fn ConversationListButton(cx: Scope<ButtonProps>) -> Element {
    let chat_sidebar_event_handler = use_coroutine_handle::<ChatSidebarEvent>(cx).unwrap();
    // FIXME: keep the button sync with the chat history sidebar
    render! {
        RawButton {
            on_click: move |_| chat_sidebar_event_handler.send(ChatSidebarEvent::ToggleChatHistory),
            button_props: &cx.props,
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


pub fn DiscoverButton(cx: Scope<ButtonProps>) -> Element {
    let chat_sidebar_event_handler = use_coroutine_handle::<ChatSidebarEvent>(cx).unwrap();
    render! {
        RawButton {
            on_click: move |_| chat_sidebar_event_handler.send(ChatSidebarEvent::EnterDiscovery),
            button_props: &cx.props,
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


pub fn UserProfileButton(cx: Scope<ButtonProps>) -> Element {
    let chat_sidebar_event_handler = use_coroutine_handle::<ChatSidebarEvent>(cx).unwrap();
    render! {
        RawButton {
            on_click: move |_| chat_sidebar_event_handler.send(ChatSidebarEvent::EnterUserProfile),
            button_props: &cx.props,
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
        RawButton {
            on_click: |_| app_event_handler.send(AppEvents::ToggleSettingsSidebar),
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
