//! Send a string text message to webhook

#![warn(missing_docs)]

use log::{debug, info};
use reqwest::{Client, Error as ReqError, Response};
use serde::Serialize;

/// Webhook Sender
pub struct WebhookSender {
    url: String,
    web_client: Client,
}

impl WebhookSender {
    /// Create a new webhook sender
    pub fn new<S: Into<String>>(webhook_url: S) -> Self {
        let url = webhook_url.into();
        debug!("created a new webhook-sender for url: '{}'", url);
        Self {
            url,
            web_client: Client::new(),
        }
    }

    /// post string message
    pub async fn post(&self, message: &str) -> Result<Response, ReqError> {
        info!("message to post: {}", message);
        let msg = get_sample_msg(message);
        debug!("message encoded as json: {:?}", msg);
        self.web_client.post(&self.url).json(&msg).send().await
    }
}

fn get_sample_msg(message: &str) -> JsonMessage {
    let mut msg = get_msg_template();
    msg.attachments
        .get_mut(0)
        .unwrap()
        .content
        .body
        .get_mut(0)
        .unwrap()
        .text
        .push_str(message);
    msg
}

const fn get_msg_template() -> JsonMessage {
    JsonMessage {
        r#type: "message",
        attachments: [{
            Attachment {
                content_type: "application/vnd.microsoft.card.adaptive",
                content_url: "",
                content: {
                    Content {
                        schema: "http://adaptivecards.io/schemas/adaptive-card.json",
                        r#type: "AdaptiveCard",
                        version: "1,2",
                        body: [Body {
                            r#type: "TextBlock",
                            text: String::new(),
                        }],
                    }
                },
            }
        }],
    }
}

#[derive(Debug, Serialize)]
struct JsonMessage {
    r#type: &'static str,
    attachments: [Attachment; 1],
}

#[derive(Debug, Serialize)]
struct Attachment {
    #[serde(rename = "contentType")]
    content_type: &'static str,
    #[serde(rename = "contentUrl")]
    content_url: &'static str,
    content: Content,
}

#[derive(Debug, Serialize)]
struct Content {
    #[serde(rename = "$schema")]
    schema: &'static str,
    r#type: &'static str,
    version: &'static str,
    body: [Body; 1],
}

#[derive(Debug, Serialize)]
struct Body {
    r#type: &'static str,
    text: String,
}
