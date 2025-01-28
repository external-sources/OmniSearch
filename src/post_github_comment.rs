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

use std::process::Command;

pub fn post_github_comment(github_comment_body: &str, url: &str) {
    let api_url = if url.contains("/issues/") {
        url.replace("/issues/", "/repos/").replace("https://github.com/", "https://api.github.com/repos/") + "/comments"
    } else if url.contains("/pull/") {
        url.replace("/pull/", "/repos/").replace("https://github.com/", "https://api.github.com/repos/") + "/comments"
    } else {
        println!("Invalid GitHub URL");
        return;
    };

    let json_data = format!("{{\"body\": \"{}\"}}", github_comment_body);
    let output = Command::new("curl")
        .arg("-s")
        .arg("-X")
        .arg("POST")
        .arg("-H")
        .arg("Authorization: token $GITHUB_TOKEN")
        .arg("-H")
        .arg("Content-Type: application/json")
        .arg("-d")
        .arg(json_data)
        .arg(&api_url)
        .output()
        .expect("Failed to execute curl");

    let response = String::from_utf8_lossy(&output.stdout);
    let http_code = String::from_utf8_lossy(&output.stderr);

    if !http_code.contains("20") {
        let message = response.clone();
        let status = response.clone();
        println!("HTTP Code: {}", http_code);
        println!("Message: {}", message);
        println!("Status: {}", status);
    } else {
        println!("HTTP Code: {}", http_code);
    }
}