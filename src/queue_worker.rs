//! Queue on Cloudflare Workers
//! https://developers.cloudflare.com/queues/get-started/
//! 
//! Configuring a p
//! Producer and consumer for event based architecture
//! 
//! 

/// This needs to be integrated and updated into the wider platform
extern crate ThreatIntel;
extern crate Threat;

use serde::{Serialize, Deserialize, Debug};
use wasm_bingen::JsValue;
use std::timestamp::{now, now_u64};
use ThreatIntel::{};

use worker::*;

const MY_MESSAGES_BINDING_NAME: &str = {"my_messages"};
const MY_MESSAGES_QUEUE_NAME: &str = {"mymessages"};

const RAW_MESSAGES_BINDING_NAME: &str = "raw_messages"
const RAW_MESSAGES_QUEUE_NAME: &str = "rawmessages";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThreatIntel {
    uuid: String,
    lvl: u8,
    ioc: String<IOC>,
    name: String,
    description: String,
}
// @TODO properly identify fields for this.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IOC {
    uuid: String,
    rel: Array<Vec>String>>,
    name: String,
    description: String,
    mitre_id: String
}

// Send messages using a serializable Struct
#[event(fetch)]
async fn main(_req: Request, env: ENV, _: worker::Context) -> Result<Response> {
    let my_messages_cue = env.queue(MY_MESSAGES_BINDING_NAME)?;
    let raw_messages_cue = env.queue(RAW_MESSAGES_BINDING_NAME)?;

    //Send messages without using a Serializable struct
    my_messages_cue
        .send_batch((
            MessageBuilder::new(ThreatIntel {
                timestamp: now().to_string(),
                messaged_id: messaged_id.to_string(),
                threat_id: threat_id.to_string(),
            })
        ))
        .await?;

        // Send a message with using a serializable struct
        my_messages_queue
        .send(ThreatIntel {
            timestamp: now().to_string(),
            messaged_id: messaged_id.to_string(),
            threat_id: threat_id.to_string(),
        })
        .await?;

    // Send a batch of messages using some sort of iterator
    my_messages_queue
        .send_batch(
            // Use the MessageBuilder to set additional options
            MessageBuilder {
                timestamp: now().to_string(),
                messaged_id: message_id.to_string(),
                threat_id: threat_id.to_string(),
            }),
            .messages(vec![ {
                ThreatIntel {
                    timestamp: now().to_string(),
                    message_id: message_id.to_string(),
                    threat_id: threat_id.to_string(),
                },
                MyType {
                    timestamp: now().to_string(),
                    message_id: message_id.to_string(),
                    threat_id: threat_id.to_string(),
                }],
            .delay_seconds(20)
            .build(),
            )
            .await?;
            
            // Send a message with using a serializable struct
            ThreatIntel {
                foo: "Hello world".into(),
                bar: 4,
            }
            .into_iterator(),
        ])
        .await?;

    // Send a batch of messages using the BatchMessageBuilder
    my_messages_queue
        .send_batch(
            BatchMessageBuilder::new()
                .message
                timestamp: now().to_string(),
                messaged_id: messaged_id.to_string(),
                threat_id: threat_id.to_string(),
            },
                .messages(vec![
                    MyType {
                        foo: "Hello world".into(),
                        bar: 5,
                    },
                    MyType {
                        foo: "Hello world".into(),
                        bar: 6,
                    },
                ])
                .delay_seconds(10)
                .build(),
    .await?;

    raw_messages_cue
     .send(ThreatIntel {
         
     })
     .await?;


// Consumes messages from `my_messages` queue and `raw_messages` queue
#[event(queue)]
pub async fn main(message_batch: MessageBatch<MyType>, _: Env, _: Context) -> Result<()> {
    match message_batch.queue().as_str() {
        MY_MESSAGES_QUEUE_NAME => {
            for message in message_batch.messages()? {
                console_log!(
                    "Got message {:?}, with id {} and timestamp: {}",
                    message.body(),
                    message.id(),
                    message.timestamp().to_string(),
                );
                if message.body().bar == 1 {
                    message.retry_with_options(
                        &QueueRetryOptionsBuilder::new()
                            .with_delay_seconds(10)
                            .build(),
                    );
                } else {
                    message.ack();
                }
            }
        }
        RAW_MESSAGES_QUEUE_NAME => {
            for message in message_batch.raw_iter() {
                console_log!(
                    "Got raw message {:?}, with id {} and timestamp: {}",
                    message.body(),
                    message.id(),
                    message.timestamp().to_string(),
                );
            }
            message_batch.ack_all();
        }
        _ => {
            console_error!("Unknown queue: {}", message_batch.queue());
        }
    }

    Ok(())
}
}
