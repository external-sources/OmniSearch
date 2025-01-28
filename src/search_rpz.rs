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
use std::process::Command;

pub fn search_rpz(record: &str) {
    let script_dir = env::var("SCRIPT_DIR").expect("SCRIPT_DIR environment variable not set");

    // Source user settings
    let user_settings_script = format!("{}/functions/userSettings.sh", script_dir);
    let source_command = format!("source {}", user_settings_script);
    let output = Command::new("bash")
        .arg("-c")
        .arg(&source_command)
        .output()
        .expect("Failed to source user settings script");

    if !output.status.success() {
        eprintln!("Failed to source user settings script: {}", String::from_utf8_lossy(&output.stderr));
        return;
    }

    // Database query
    let pda_db_host = env::var("PDA_DB_HOST").expect("PDA_DB_HOST environment variable not set");
    let pda_db_user = env::var("PDA_DB_USER").expect("PDA_DB_USER environment variable not set");
    let pda_db_pass = env::var("PDA_DB_PASS").expect("PDA_DB_PASS environment variable not set");
    let pda_db_name = env::var("PDA_DB_NAME").expect("PDA_DB_NAME environment variable not set");
    let rpz_domain_name = env::var("RPZ_DOMAIN_NAME").expect("RPZ_DOMAIN_NAME environment variable not set");

    let rgx_rpz_domain_name = rpz_domain_name.replace('.', "\\.");

    let sql_query = format!(
        "SELECT REPLACE(name, '.{}', '') AS name FROM .domains WHERE name LIKE '%.{}'",
        rgx_rpz_domain_name, rgx_rpz_domain_name
    );

    let zone_output = Command::new("mariadb")
        .arg("-h")
        .arg(&pda_db_host)
        .arg("-u")
        .arg(&pda_db_user)
        .arg(format!("-p{}", pda_db_pass))
        .arg("-D")
        .arg(&pda_db_name)
        .arg("-e")
        .arg(&sql_query)
        .output()
        .expect("Failed to execute SQL query");

    if !zone_output.status.success() {
        eprintln!("Failed to execute SQL query: {}", String::from_utf8_lossy(&zone_output.stderr));
        return;
    }

    let zone = String::from_utf8_lossy(&zone_output.stdout)
        .split('\n')
        .collect::<Vec<&str>>()
        .join("|");
    let zone = zone.trim_end_matches('|');

    let sql_query = format!(
        "SELECT name, type, content FROM records WHERE name REGEXP '(^|\\.){}\\.(rpz-nsdname\\.)?({})\\.mypdns\\.cloud$' AND content <> '' ORDER BY domain_id, name;",
        regex::escape(record), zone
    );

    let rpz_output = Command::new("mariadb")
        .arg("-h")
        .arg(&pda_db_host)
        .arg("-u")
        .arg(&pda_db_user)
        .arg(format!("-p{}", pda_db_pass))
        .arg("-D")
        .arg(&pda_db_name)
        .arg("-e")
        .arg(&sql_query)
        .output()
        .expect("Failed to execute SQL query");

    if !rpz_output.status.success() {
        eprintln!("Failed to execute SQL query: {}", String::from_utf8_lossy(&rpz_output.stderr));
        return;
    }

    let rpz = String::from_utf8_lossy(&rpz_output.stdout);
    println!("### Response Policy Zone - RPZ\n");

    if rpz.trim().is_empty() {
        println!("Did not find any matching RPZ records\n");
    } else {
        println!("Found these RPZ records in My Privacy DNS\n");
        println!("| Domain records | Type | content |");
        println!("| :--- | :---: | :---: |");

        for line in rpz.lines().skip(1) {
            let fields: Vec<&str> = line.split('\t').collect();
            if fields.len() >= 3 {
                println!("| `{}` | {} | {} |", fields[0], fields[1], fields[2]);
            }
        }
    }
}