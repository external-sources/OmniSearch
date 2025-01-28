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
use std::fs;

pub fn search_easylist(easylist_dir: &str, record: &str, repo_root_dir: &str) {
    let temp_easylist_file = format!("/tmp/temp_easylist_file.{}", uuid::Uuid::new_v4());

    if !std::path::Path::new(easylist_dir).exists() {
        Command::new("sh")
            .arg("-c")
            .arg(format!(
                "cd {} && mkdir -p easylist && git clone git@github.com:easylist/easylist.git {} --quiet",
                repo_root_dir, easylist_dir
            ))
            .output()
            .expect("Failed to clone Easylist repository");
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(format!(
                "cd {} && git merge --ff-only @{{u}} -q || git pull --rebase -q && git grep -iE \"((^|\\.|\\|\\||~){}(\\^|#|\\$))\" > {}",
                easylist_dir, record, temp_easylist_file
            ))
            .output()
            .expect("Failed to search Easylist");
    }

    let content = fs::read_to_string(&temp_easylist_file).expect("Unable to read file");
    println!("{}", content);
    fs::remove_file(temp_easylist_file).expect("Unable to delete temp file");
}