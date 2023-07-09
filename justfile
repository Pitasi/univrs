dev:
    DEV=true cargo watch -x run

tailwind:
    npx tailwindcss --output static/tailwind.css -w
