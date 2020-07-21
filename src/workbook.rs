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
    reply_channel: Option<Sender<Box<CommandResponse>>>,
}

#[derive(Debug)]
pub struct Workbook {
    sender_chan: Sender<Box<CommandWrapper>>,
    receiver_chan: Arc<Mutex<Receiver<Box<CommandWrapper>>>>,
    command_cnt: Arc<Mutex<u64>>,
}

#[derive(Debug)]
pub struct StringError {
    msg: String,
}

impl std::fmt::Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
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
            receiver_chan: Arc::new(Mutex::new(rx)),
            command_cnt: Arc::new(Mutex::new(0)),
        };

        ret.listen_for_commands();

        let aret = Arc::new(ret);

        return aret;
    }

    pub fn get_command_count(&self) -> u64 {
        match self.command_cnt.lock() {
            Ok(r) => *r,
            _ => 0
        }
    }

    pub async fn send_command(
        &self,
        cmd: &WorksheetCommand,
    ) -> Result<CommandResponse, Box<dyn std::error::Error>> {
        let (rc, mut info) = channel(1);
        let wrapper = CommandWrapper {
            command: cmd.clone(),
            reply_channel: Some(rc),
        };
        let the_box = Box::new(wrapper);
        let mut send_chan = self.sender_chan.clone();
        send_chan.send(the_box).await?;
        match info.recv().await {
            Some(x) => Ok(*x),
            None => Err(Box::new(StringError {
                msg: "Failed to receive".into(),
            })),
        }
    }

    /// Spawn a thread (not super keen about this, but whatever)
    fn listen_for_commands(&self) {
        let rec_chan = self.receiver_chan.clone();
        let cnt_mut = self.command_cnt.clone();
        spawn(move || {
            let f1 = async {
                match rec_chan.lock() {
                    Ok(mut rx) => {
                        println!("Waiting for messages!");
                        while let Some(msg) = rx.recv().await {
                            println!("Message {:?}", msg.command);
                            match cnt_mut.lock() {
                                Ok(mut mg) => *mg += 1,
                                _ => ()
                            };
                            match msg.reply_channel {
                                Some(mut rc) => {
                                    let _ = rc.send(Box::new(CommandResponse::OkResp)).await;
                                    ()
                                }
                                None => (),
                            }
                        }
                    }
                    Err(e) => println!("Unable to unwrap receive channel {}", e),
                }
                println!("Done with listen loop");
            };

            let mut runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
            runtime.block_on(f1);
            println!("Finished blocking!");
        });
    }

    async fn get_sheets(&self) -> Vec<Box<dyn SheetIdentifier>> {
        vec![]
    }
}

fn listen_for_commands(workbook: &ArcWorkbook) {
    let wbc = workbook.clone();
    spawn(move || {
        let f1 = async {
            match wbc.as_ref().receiver_chan.clone().lock() {
                Ok(mut rx) => {
                    println!("Waiting for messages!");
                    while let Some(msg) = rx.recv().await {
                        println!("Message {:?}", msg);
                        match msg.reply_channel {
                            Some(mut rc) => {
                                let _ = rc.send(Box::new(CommandResponse::OkResp)).await;
                                ()
                            }
                            None => (),
                        }
                    }
                }
                Err(e) => println!("Unable to unwrap receive channel {}", e),
            }
        };

        let mut runtime = tokio::runtime::Runtime::new().expect("Unable to create a runtime");
        runtime.block_on(f1);
    });
}
