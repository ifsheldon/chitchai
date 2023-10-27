use std::collections::HashMap;

use transprompt::utils::llm::openai::ChatMsg;
use uuid::Uuid;

use crate::agents::{AgentConfig, AgentID, AgentInstance, AgentName, AgentType};
use crate::utils::datetime::DatetimeString;

pub type LinkedChatHistory = Vec<MessageID>;

#[derive(Clone, Copy, Hash, PartialEq, Debug, Eq)]
pub struct MessageID(pub(crate) Uuid);

impl MessageID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct MessageManager {
    pub(crate) messages: HashMap<MessageID, ChatMsg>,
}

impl MessageManager {
    pub fn insert(&mut self, msg: ChatMsg) -> MessageID {
        let id = MessageID::new();
        self.messages.insert(id.clone(), msg);
        id
    }

    pub fn remove(&mut self, id: &MessageID) -> Option<ChatMsg> {
        self.messages.remove(id)
    }

    pub fn get(&self, id: &MessageID) -> Option<&ChatMsg> {
        self.messages.get(id)
    }

    pub fn get_mut(&mut self, id: &MessageID) -> Option<&mut ChatMsg> {
        self.messages.get_mut(id)
    }

    pub fn update(&mut self, id: &MessageID, msg: ChatMsg) -> Option<ChatMsg> {
        self.messages.insert(id.clone(), msg)
    }
}


#[derive(Debug, PartialEq)]
pub struct Chat {
    pub(crate) id: Uuid,
    pub message_manager: MessageManager,
    pub topic: String,
    pub date: DatetimeString,
    pub agents: HashMap<AgentID, AgentInstance>,
}

impl Chat {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn default_chat_and_configs() -> (Self, HashMap<AgentName, AgentConfig>) {
        let mut name_to_configs = HashMap::new();
        let mut message_manager = MessageManager::default();
        // init two assistants named Alice and Bob
        let alice = AgentName::Named("Alice".to_string());
        let assistant_alice = AgentInstance::default_assistant(
            alice.clone(),
            &mut message_manager,
        );
        let bob = AgentName::Named("Bob".to_string());
        let assistant_bob = AgentInstance::default_assistant(
            bob.clone(),
            &mut message_manager,
        );
        // init a user whose history is empty and will be displayed by default
        let user = AgentInstance::default_user();
        name_to_configs.insert(alice, assistant_alice.config.clone());
        name_to_configs.insert(bob, assistant_bob.config.clone());
        name_to_configs.insert(user.get_name(), user.config.clone());
        let agents = HashMap::from([
            (assistant_alice.id, assistant_alice),
            (assistant_bob.id, assistant_bob),
            (user.id, user),
        ]);
        let chat = Self {
            id: Uuid::new_v4(),
            message_manager,
            topic: "New Chat".to_string(),
            date: DatetimeString::get_now(),
            agents,
        };
        (chat, name_to_configs)
    }

    pub fn default() -> Self {
        let (chat, _) = Self::default_chat_and_configs();
        chat
    }

    pub fn user_agent_ids<B: FromIterator<AgentID>>(&self) -> B {
        self
            .agents
            .iter()
            .filter_map(|(id, agent)| {
                if agent.config.agent_type == AgentType::User {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn assistant_agent_ids<B: FromIterator<AgentID>>(&self) -> B {
        self
            .agents
            .iter()
            .filter_map(|(id, agent)| {
                if let AgentType::Assistant { .. } = agent.config.agent_type {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn agent_ids(&self) -> Vec<AgentID> {
        self.agents.keys().cloned().collect()
    }
}

impl Clone for Chat {
    fn clone(&self) -> Self {
        Self {
            id: Uuid::new_v4(),
            message_manager: self.message_manager.clone(),
            topic: self.topic.clone(),
            date: DatetimeString::get_now(),
            agents: self.agents.clone(),
        }
    }
}
