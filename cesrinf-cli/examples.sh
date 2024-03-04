#!/bin/bash

cargo build --release

# valid
../target/release/cesrinf-cli AJ97qKeoQzmWJvqxmeuqIMQbRxHErlNBUsm9BJ2FKX6T
../target/release/cesrinf-cli AKrRTkeoyVnRBOJG03ZXeSroKhM7j_YyNurHuWxf8wgzAKrRTkeoyVnRBOJG03ZXeSroKhM7j_YyNurHuWxf8wgz
../target/release/cesrinf-cli 0BAcbTzvOldV43AGAW6yv1luylr4BPb06B8PiycQ1SbON09QPQW3812nzlzaitf-hmyCSG-mevc3_kOHDT3pj6AA
../target/release/cesrinf-cli 1AAJA3cK_P2CDlh-_EMFPvyqTPI1POkw-dr14DANx5JEXDCZ

# invalid
../target/release/cesrinf-cli AAA
