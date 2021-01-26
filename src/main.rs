// Copyright 2021 Nikhil Benesch. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE file at the
// root of this repository, or online at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::error::Error;
use std::process::{self, Stdio, Command};
use std::io::BufReader;
use cargo_metadata::{Message, MetadataCommand};

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {}", e);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let metadata = MetadataCommand::new().exec()?;
    let package = match metadata.root_package() {
        Some(p) => p,
        None => return Err("cargo out-dir must be run from within a crate".into()),
    };
    let mut command = Command::new("cargo")
        .args(&["check", "--message-format=json", "--quiet"])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let reader = BufReader::new(command.stdout.take().unwrap());
    for message in Message::parse_stream(reader) {
        match message? {
            Message::BuildScriptExecuted(script) => {
                if script.package_id == package.id {
                    println!("{}", script.out_dir.display());
                    return Ok(());
                }
            },
            _ => ()
        }
    }
    Err(format!("crate {} did not run a build script", package.name).into())
}
