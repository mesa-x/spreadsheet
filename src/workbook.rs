//! The implementation of Workbooks for Mesa X
//!

// Copyright 2020 David Pollak
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::definitions::SheetIdentifier;
use atomic_counter::{AtomicCounter, RelaxedCounter};
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use tokio::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug, Clone, PartialEq)]
pub enum WorksheetCommand {
    Noop,
}
#[derive(Debug, Clone, PartialEq)]
pub enum CommandResponse {
    OkResp,
}

#[derive(Debug, Clone)]
pub struct CommandWrapper {
    command: WorksheetCommand,
    reply_channel: Option<Sender<CommandResponse>>,
}

#[derive(Debug)]
pub struct Workbook {
    sender_chan: Sender<CommandWrapper>,
    receiver_chan: Mutex<Receiver<CommandWrapper>>,
    command_cnt: RelaxedCounter,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StringError {
    msg: String,
}

impl std::fmt::Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for StringError {}

pub type ArcWorkbook = Arc<Workbook>;

impl Workbook {
    pub fn new() -> ArcWorkbook {
        let (tx, rx) = channel(100); // default backpressure TODO variable backpressure
        let ret = Workbook {
            sender_chan: tx,
            receiver_chan: Mutex::new(rx),
            command_cnt: RelaxedCounter::new(0),
        };

        let ret = Arc::new(ret);

        listen_for_commands(&ret);

        ret
    }

    pub fn get_command_count(&self) -> usize {
        self.command_cnt.get()
    }

    pub async fn send_command(
        &self,
        cmd: WorksheetCommand,
    ) -> Result<CommandResponse, Box<dyn std::error::Error>> {
        let (rc, mut info) = channel(1);
        let wrapper = CommandWrapper {
            command: cmd,
            reply_channel: Some(rc),
        };
        let send_chan = self.sender_chan.clone();
        send_chan.send(wrapper).await?;
        match info.recv().await {
            Some(x) => Ok(x),
            None => Err(Box::new(StringError {
                msg: "Failed to receive".into(),
            })),
        }
    }

    pub async fn get_sheets(&self) -> Vec<Box<dyn SheetIdentifier>> {
        vec![]
    }
}

/// Spawn a thread (not super keen about this, but whatever)
fn listen_for_commands(workbook: &ArcWorkbook) {
    let book = workbook.clone();

    spawn(move || {
        let f1 = async {
            match book.receiver_chan.lock() {
                Ok(mut rx) => {
                    while let Some(msg) = rx.recv().await {
                        // FIXME dispatch message
                        book.command_cnt.inc();
                        match msg.reply_channel {
                            Some(rc) => {
                                let _ = rc.send(CommandResponse::OkResp).await;
                                ()
                            }
                            None => (),
                        }
                    }
                }
                _ => (),
            };
        };

        let runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        runtime.block_on(f1);
    });
}
