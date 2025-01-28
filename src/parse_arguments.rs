/*
 * <Program Name>: <Brief Description of the Program>
 * Copyright (C) 2025. My Privacy DNS
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use std::env;

pub struct Config {
    pub options: Vec<String>,
    pub post_comment: bool,
    pub issue_url: Option<String>,
    pub curl_use_proxy: bool,
    pub curl_proxy_address: String,
    pub record: Option<String>,
}

pub fn parse_arguments() -> Config {
    let args: Vec<String> = env::args().collect();
    let mut options = Vec::new();
    let mut post_comment = false;
    let mut issue_url = None;
    let mut curl_use_proxy = true;
    let mut curl_proxy_address = "socks5h://127.0.0.1:9050".to_string();
    let mut record = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].to_lowercase().as_str() {
            "-r" | "--rpz" => options.push("rpz".to_string()),
            "-i" | "--issues" => options.push("issues".to_string()),
            "--dns" => options.push("dns".to_string()),
            "--hs" => options.push("host_sources".to_string()),
            "-e" | "--easylist" => options.push("easylist".to_string()),
            "--hh" | "--http" => options.push("http_header".to_string()),
            "-m" => options.push("matrix".to_string()),
            "-c" | "--pc" => {
                post_comment = true;
                if i + 1 < args.len() {
                    issue_url = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "-x" | "--proxy" => curl_use_proxy = true,
            "--proxy-host" => {
                if i + 1 < args.len() {
                    curl_proxy_address = args[i + 1].clone();
                    i += 1;
                }
            }
            "--no-proxy" => curl_use_proxy = false,
            "-h" | "--help" => {
                // Call show_help function here
                super::show_help::show_help();
                std::process::exit(0);
            }
            "-v" | "--version" => {
                // Call show_version function here
                super::show_version::show_version();
                std::process::exit(0);
            }
            _ => {
                record = Some(args[i].clone());
            }
        }
        i += 1;
    }

    if record.is_none() {
        // Call show_help function here
        super::show_help::show_help();
        std::process::exit(1);
    }

    Config {
        options,
        post_comment,
        issue_url,
        curl_use_proxy,
        curl_proxy_address,
        record,
    }
}