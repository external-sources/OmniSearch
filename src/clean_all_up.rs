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
use std::fs;

pub fn clean_all_up(workdir: &str, post_output: &str) {
    env::set_current_dir(workdir).expect("Unable to change directory");
    fs::remove_file(post_output).expect("Unable to delete post output file");

    let temp_files: Vec<String> = vec!["temp_host_file", "temp_easylist_file", "temp_matrix_file"]
        .into_iter()
        .map(|s| format!("/tmp/{}.*", s))
        .collect();

    for temp_file in temp_files {
        let files = glob::glob(&temp_file).expect("Failed to read glob pattern");
        for file in files {
            if let Ok(path) = file {
                fs::remove_file(path).expect("Unable to delete temp file");
            }
        }
    }
}