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
mod search_host_sources;
mod search_easylist;
mod search_matrix;
mod search_http_header;
mod clean_all_up;
mod search_dns;
mod show_version;
mod parse_arguments;
mod search_issues;
mod post_github_comment;
mod search_rpz;
mod show_help;
mod ctrl_c;

use search_host_sources::search_host_sources;
use search_easylist::search_easylist;
use search_matrix::search_matrix;
use search_http_header::search_http_header;
use clean_all_up::clean_all_up;

fn main() {
    // Example usage
    let host_source_dir = "/path/to/host/source/dir";
    let easylist_dir = "/path/to/easylist/dir";
    let matrix_dir = "/path/to/matrix/dir";
    let record = "example.com";
    let repo_root_dir = "/path/to/repo/root/dir";
    let curl_command = "curl";
    let workdir = "/path/to/workdir";
    let post_output = "/tmp/post_output";

    search_host_sources(host_source_dir, record);
    search_easylist(easylist_dir, record, repo_root_dir);
    search_matrix(matrix_dir, record);
    search_http_header(record, curl_command);
    clean_all_up(workdir, post_output);
}