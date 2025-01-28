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

fn fetch_github_issues(query: &str, repo: &str, test: &str) -> String {
    let curl_command = if env::var("CURL_USE_PROXY").unwrap_or_else(|_| "true".to_string()) == "true" {
        format!("curl -x {}", env::var("CURL_PROXY_ADDRESS").unwrap_or_else(|_| "socks5h://127.0.0.1:9050".to_string()))
    } else {
        "curl".to_string()
    };

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "{} -sL -G 'https://api.github.com/search/issues?q={}&repo={}' -H 'Accept: application/vnd.github.text-match+json' | jq '.items[ ] {} | \"+ <\" + .html_url + \">\"' -r",
            curl_command, query, repo, test
        ))
        .output()
        .expect("Failed to fetch GitHub issues");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn search_youtrack(domain: &str, auth: &str) -> String {
    let curl_command = if env::var("CURL_USE_PROXY").unwrap_or_else(|_| "true".to_string()) == "true" {
        format!("curl -x {}", env::var("CURL_PROXY_ADDRESS").unwrap_or_else(|_| "socks5h://127.0.0.1:9050".to_string()))
    } else {
        "curl".to_string()
    };

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "{} -sL -G 'https://{}/api/issues?fields=id,idReadable,summary&query=summary:{}+project:{{Matrix}}' -H 'Accept: application/json' -H 'Authorization: Bearer {}' -H 'Cache-Control: no-cache' -H 'Content-Type: application/json' | jq -r '.[] | select(.summary|test(\"^{}$\")) | \"+ \" + .idReadable + \" (<https://{}/issue/\" + .idReadable + \"/\" + .summary + \")\"'",
            curl_command, domain, env::var("rgxRecord").unwrap_or_else(|_| "".to_string()), auth, env::var("RECORD").unwrap_or_else(|_| "".to_string()), domain
        ))
        .output()
        .expect("Failed to search YouTrack");

    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn search_issues() {
    println!("## Known Issues\n");

    let my_privacy_dns_issue = search_youtrack("kb.mypdns.org", &env::var("KB_YOUTRACK_API_TOKEN").unwrap_or_else(|_| "".to_string()));
    let my_privacy_dns_backup_issue = search_youtrack("mypdns.youtrack.cloud", &env::var("MY_YOUTRACK_API_TOKEN").unwrap_or_else(|_| "".to_string()));

    let matrix_github_issue = fetch_github_issues(
        &format!("{}%20in:title%20type:issue", env::var("RECORD").unwrap_or_else(|_| "".to_string())),
        "mypdns/matrix",
        "| select(.title|test(\"^{}$\"))"
    );

    let phishing_database_issue = fetch_github_issues(
        &env::var("RECORD").unwrap_or_else(|_| "".to_string()),
        "Phishing-Database/phishing",
        ""
    );

    let phishing_db_issue = fetch_github_issues(
        &env::var("RECORD").unwrap_or_else(|_| "".to_string()),
        "Phishing-Database/Phishing.Database",
        ""
    );

    let other_github_issues = fetch_github_issues(
        &format!("{}+type:issue%20-repo:mypdns/matrix&sort=indexed", env::var("RECORD").unwrap_or_else(|_| "".to_string())),
        "",
        ""
    );

    fn print_issues(issues: &str, repo: &str) {
        if !issues.is_empty() {
            println!("{}", issues);
        } else {
            println!("+ There are no known issues for `{}` in [{}]", env::var("RECORD").unwrap_or_else(|_| "".to_string()), repo);
        }
    }

    print_issues(&my_privacy_dns_issue, "My Privacy DNS");
    print_issues(&my_privacy_dns_backup_issue, "My Privacy DNS (Issue Backup)");
    print_issues(&matrix_github_issue, "Matrix:GitHub");
    print_issues(&phishing_database_issue, "phishing (Database)");
    print_issues(&phishing_db_issue, "Phishing.Database");
    print_issues(&other_github_issues, "other issues on GitHub");

    println!("");
}