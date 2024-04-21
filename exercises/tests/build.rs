//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
    // 运行到test7时，time是在[TEST_FOO, TEST_FOO + 10]之间，
    // 也就是cargo在这一段时间内完成运行
    let your_command = format!(
        // "Your command here with {}, please checkout exercises/tests/build.rs",
        // timestamp
        "rustc-env=TEST_FOO={}", timestamp.to_string()
    );
    println!("cargo:{}", your_command);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    let your_command = "rustc-cfg=feature=\"pass\"";
    println!("cargo:{}", your_command);
}
