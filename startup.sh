#!/bin/bash

nohup cargo run >output.log 2>&1 &
bash add_rustfmt_git_hook.sh &