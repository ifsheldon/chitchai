use dioxus::prelude::*;

use crate::app::AppEvents;
use crate::components::{LeftSidebarEvent, SecondarySidebar};

pub fn IconSidebar(cx: Scope) -> Element {
    let now_active_secondary = use_shared_state::<SecondarySidebar>(cx).unwrap().read();
    let conversation_list_button_active = matches!(*now_active_secondary, SecondarySidebar::History);
    let user_profile_button_active = matches!(*now_active_secondary, SecondarySidebar::Profile);

    render! {
        div {
            class: "flex h-screen w-12 flex-col items-center space-y-8 border-r border-slate-300 bg-slate-50 py-8 dark:border-slate-700 dark:bg-slate-900 sm:w-16",
            Logo {},
            NewConversationButton {
                activatable: false,
                active: false,
            },
            ConversationListButton {
                activatable: true,
                active: conversation_list_button_active,
            },
            DiscoverButton {
                activatable: false,
                active: false,
            },
            UserProfileButton {
                activatable: true,
                active: user_profile_button_active,
            },
            SettingsButton {
                activatable: false,
                active: false,
            },
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
    activatable: bool,
    active: bool,
}

#[derive(Props)]
pub struct RawButtonProps<'a> {
    button_props: &'a ButtonProps,
    on_click: Option<EventHandler<'a, MouseEvent>>,
    on_click_active: Option<EventHandler<'a, MouseEvent>>,
    on_click_inactive: Option<EventHandler<'a, MouseEvent>>,
    children: Element<'a>,
}

pub fn RawButton<'a>(cx: Scope<'a, RawButtonProps<'a>>) -> Element<'a> {
    const BUTTON_INACTIVE_STYLE: &str = "rounded-lg p-1.5 text-slate-500 transition-colors duration-200 hover:bg-slate-200 focus:outline-none dark:text-slate-400 dark:hover:bg-slate-800";
    const BUTTON_ACTIVE_STYLE: &str = "rounded-lg bg-blue-100 p-1.5 text-blue-600 transition-colors duration-200 dark:bg-slate-800 dark:text-blue-600";
    let activatable = cx.props.button_props.activatable;
    let active = cx.props.button_props.active;
    render! {
        a {
            href: "#",
            class: if activatable && active {BUTTON_ACTIVE_STYLE} else {BUTTON_INACTIVE_STYLE},
            onclick: move |event| {
                if activatable {
                    match (active, &cx.props.on_click_active, &cx.props.on_click_inactive) {
                        (true, Some(on_click_active), _) => on_click_active.call(event),
                        (false, _, Some(on_click_inactive)) => on_click_inactive.call(event),
                        _ => {}
                    }
                } else {
                    if let Some(on_click) = &cx.props.on_click {
                        on_click.call(event)
                    }
                }
            },
            &cx.props.children
        }
    }
}

pub fn NewConversationButton(cx: Scope<ButtonProps>) -> Element {
    let chat_sidebar_event_handler = use_coroutine_handle::<LeftSidebarEvent>(cx).unwrap();
    render! {
        RawButton {
            button_props: &cx.props,
            on_click: move |_| chat_sidebar_event_handler.send(LeftSidebarEvent::NewChat),
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
    let chat_sidebar_event_handler = use_coroutine_handle::<LeftSidebarEvent>(cx).unwrap();
    render! {
        RawButton {
            on_click_active: move |_| chat_sidebar_event_handler.send(LeftSidebarEvent::DisableSecondary(SecondarySidebar::History)),
            on_click_inactive: move |_| chat_sidebar_event_handler.send(LeftSidebarEvent::EnableSecondary(SecondarySidebar::History)),
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
    // let chat_sidebar_event_handler = use_coroutine_handle::<LeftSidebarEvent>(cx).unwrap();
    render! {
        RawButton {
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
    // let chat_sidebar_event_handler = use_coroutine_handle::<LeftSidebarEvent>(cx).unwrap();
    render! {
        RawButton {
            // TODO: enble this after implementing profile sidebar
            // on_click_active: move |_| chat_sidebar_event_handler.send(LeftSidebarEvent::EnableSecondary(SecondarySidebar::Profile)),
            // on_click_inactive: move |_| chat_sidebar_event_handler.send(LeftSidebarEvent::DisableSecondary(SecondarySidebar::Profile)),
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

pub fn SettingsButton(cx: Scope<ButtonProps>) -> Element {
    let app_event_handler = use_coroutine_handle::<AppEvents>(cx).unwrap();
    render! {
        RawButton {
            button_props: &cx.props,
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