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

pub fn search_dns(record: &str) {
    println!("### DNS lookup");
    println!("\n```bash");

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("dig +short +tls @91.239.100.100 -t NS {}", record))
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let content = String::from_utf8_lossy(&output.stdout);
        println!("{}", content);
    } else {
        eprintln!("Failed to perform DNS lookup");
    }

    println!("```");
    println!("");
}