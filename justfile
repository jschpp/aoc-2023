set shell := ["powershell.exe", "-c"]

work day part:
    cargo watch -x "check -p {{day}}" -s "just test {{part}} -p {{day}}" -s "just lint {{day}}"
lint day:
    cargo clippy -p {{day}}
test part +FLAGS='-p day-01':
    cargo test {{FLAGS}} {{part}}
create day:
    cargo generate --path ./daily-template --name {{day}}