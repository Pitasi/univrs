build:
    cargo build

run:
    cargo run

dev:
    DEV=true cargo watch -x run

tailwind:
    npx tailwindcss --output static/tailwind.css -w -m

css-dist:
    npx tailwindcss --output static/tailwind.css -m
    npx lightningcss-cli --minify --bundle --targets '>= 0.25%' --nesting static/style.css -o static/style.dist.css
