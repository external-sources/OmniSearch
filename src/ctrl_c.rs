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

use rand::distributions::{Distribution, Uniform};
use rand::Rng;

pub fn ctrl_c() {
    println!("Ctrl + C happened!");
    println!("Whoa! Captain Marvel just made a surprise appearance! CTRL+C is her way of saying, 'Time to show some skin!'");

    let phrases = [
        "Oops! Did you just Ctrl+Z my pizza order while Wonder Woman flexes?",
        "No way! That was a total system crash, and Captain Marvel just flashed her superpowers!",
        "Bruh, you just hit the escape button while Miss America shows off her dazzling smile!",
        "Yikes! Did you just debug my snack time while Wonder Woman strikes a pose?",
        "Whoa! Captain Marvel just made a surprise appearance! CTRL+C is her way of saying, 'Time to show some skin!'"
    ];

    let mut rng = rand::thread_rng(); // Use thread_rng() correctly
    let dist = Uniform::from(0..phrases.len());
    let random_index = dist.sample(&mut rng);
    let random_phrase = phrases[random_index];
    println!("{}", random_phrase);

    clean_all_up();
    std::process::exit(1);
}

fn clean_all_up() {
    // Cleaning up temporary files and restoring the working directory
    // Implement the cleanup logic here
}

fn main() {
    println!("Hello, world!");
    // Simulate Ctrl+C event for demonstration
    ctrl_c();
}