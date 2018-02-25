#!/bin/bash
cargo web deploy --release
sed -i "s/{{ver}}/$(sha1sum target/deploy/xjtlu-timetable.js | cut -d " " -f 1)/g" target/deploy/index.html