FROM rust:1.48

WORKDIR "/opt/aoc"

COPY . .

ENTRYPOINT ["cargo", "run"]
