use dioxus::prelude::*;

use crate::agents::AgentName::{AssistantDefault, Named};
use crate::components::MessageCard;
use crate::utils::{assistant_msg, user_msg};
use crate::utils::storage::Announcements;

const ANNOUNCEMENTS: &str = include_str!("announcements.toml");

pub fn AnnouncementPage(cx: Scope) -> Element {
    let Announcements { mut announcement } = toml::from_str(ANNOUNCEMENTS).unwrap();
    announcement.sort_by(|a, b| b.date.cmp(&a.date));
    let messages = announcement
        .into_iter()
        .map(|a| {
            let command = format!("Please help me write an announcement with title `{}`", a.title);
            let announcement = format!(r#"# {}

## By {}

## {}

{}
"#, a.title, a.author, a.date, a.content);
            let command_msg = user_msg(command, Named(a.author.clone()));
            let announcement_msg = assistant_msg(announcement, AssistantDefault);
            [command_msg, announcement_msg]
        })
        .flatten()
        .collect::<Vec<_>>();

    render! {
        div {
            class: "flex h-screen w-screen flex-col relative",
            div {
                class: "flex flex-col h-full space-y-6 bg-slate-200 text-sm leading-6 text-slate-900 shadow-sm dark:bg-slate-900 dark:text-slate-300 sm:text-base sm:leading-7",
                div {
                    class: "overflow-auto max-h-[100vh] flex-grow dark:scrollbar dark:scrollbar-thumb-slate-700 dark:scrollbar-track-slate-900",
                    messages
                        .into_iter()
                        .map(|msg| rsx! {
                                MessageCard {
                                    chat_msg: msg
                                }
                            })
                }
            }
        }
    }
}