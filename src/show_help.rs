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

pub fn show_help() {
    println!("Usage: dsearch [OPTIONS] DOMAIN");
    println!();
    println!("Options:");
    println!("  -r, --rpz         Search and output results from RPZ");
    println!("  -i, --issues      Search and output results from ISSUES");
    println!("      --dns         Search and output results from DNS");
    println!("      --hs          Search and output results from Host-Sources");
    println!("  -e, --easylist    Search and output results from Easylist");
    println!("      --hh, --http  Search and output results from HTTP header");
    println!("  -c, --pc          Post the output to GitHub issue as comment to the given URL");
    println!("  -x, --proxy       Use proxy for network requests");
    println!("      --no-proxy    Unset default proxy settings");
    println!("      --proxy-host  Set the proxy address (protocol://host:port)");
    println!("  -h, --help        Display this help message");
    println!("  -m                Search and output results from Matrix");
    println!("  -v, --version     Display the script version");
}