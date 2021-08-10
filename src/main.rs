// Copyright 2019-present, the HuggingFace Inc. team, The Google AI Language Team and Facebook, Inc.
// Copyright 2019 Guillaume Becquin
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//     http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.extern crate anyhow;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()>{
    let mut in_args : Vec<String> = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        in_args.push(line?);
    }

    let result: String = rustlib::rusttagr::rust_tag_r(in_args);

    //write to a file
    println!("{}", result);

    Ok(())
}