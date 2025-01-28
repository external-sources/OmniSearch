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

pub fn search_matrix(matrix_dir: &str, record: &str) {
    let temp_matrix_file = format!("/tmp/temp_matrix_file.{}", uuid::Uuid::new_v4());

    if std::path::Path::new(matrix_dir).is_dir() {
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!(
                "cd {} && git up -q || git pull --rebase -q && git grep -iE \"(^|\\.){}$\" \"source/\" | sed 's/source\\///g;s/\\.csv//g' > {}",
                matrix_dir, record, temp_matrix_file
            ))
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let content = fs::read_to_string(&temp_matrix_file).expect("Unable to read file");
            println!("{}", content);
        } else {
            eprintln!("Failed to search matrix");
        }

        fs::remove_file(temp_matrix_file).expect("Unable to delete temp file");
    }
}