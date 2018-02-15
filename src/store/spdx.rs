// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License").
// You may not use this file except in compliance with the License.
// A copy of the License is located at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// or in the "license" file accompanying this file. This file is distributed
// on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
// express or implied. See the License for the specific language governing
// permissions and limitations under the License.

use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;

use failure::{err_msg, Error};
use walkdir::WalkDir;

use store::base::{LicenseEntry, Store};
use license::TextData;

impl Store {
    pub fn load_spdx(&mut self, dir: &str, include_texts: bool) -> Result<(), Error> {
        use json::{from_str, Value};

        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if !path.is_file() || path.extension().unwrap_or_else(|| OsStr::new("")) != "json" {
                continue;
            }

            let mut f = File::open(path)?;
            let mut data = String::new();
            f.read_to_string(&mut data)?;
            let val: Value = from_str(&data)?;

            let name = val["licenseId"]
                .as_str()
                .ok_or_else(|| err_msg("missing licenseId"))?;
            let text = val["licenseText"]
                .as_str()
                .ok_or_else(|| err_msg("missing licenseText"))?;
            //let template = val["standardLicenseTemplate"].as_str(); // unused
            let header = val["standardLicenseHeader"].as_str();

            info!("Processing {}", name);

            let content = if include_texts {
                TextData::new(text)
            } else {
                TextData::new(text).without_text()
            };

            let license = self.licenses
                .entry(name.to_owned())
                .or_insert_with(|| LicenseEntry::new(content));

            if let Some(header_text) = header {
                let header_data = if include_texts {
                    TextData::new(header_text)
                } else {
                    TextData::new(header_text).without_text()
                };
                license.headers = vec![header_data];
            }
        }

        Ok(())
    }
}
