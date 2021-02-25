#!/bin/sh
mkdir outputs
cargo run < samples/sample1 > outputs/output1
cargo run < samples/sample2 > outputs/output2
cargo run < samples/sample3 > outputs/output3
cargo run < samples/sample4 > outputs/output4
cargo run < samples/sample5 > outputs/output5
cargo run < samples/sample6 > outputs/output6
