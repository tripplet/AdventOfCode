# Use PGO in rust
    cargo rustc --bin aoc -- -Cprofile-generate --release
or
    cargo rustc --bin aoc --release -- -Cprofile-generate=C:\\tmp

merge profile data after execution
    llvm-profdata.exe merge -o C:\tmp\merged.profdata C:\tmp\default_17262501441562561664_0.profraw

cargo rustc --bin aoc --release -- "-Cprofile-use=C:\tmp\merged.profdata"

have fun