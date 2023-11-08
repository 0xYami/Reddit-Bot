/**
 * This allows the program to sleep for a certain amount of time
 * @param time
 */

pub fn sleep(time: u64) {
    let time = time * 1000;
    let mut i = 0;

    while i < time {
        i += 1;
    }

    return;
}
