default:
  just --list
test:
  cargo nextest run
new-branch issue base="main":
  gh issue develop {{issue}} --base {{base}} --checkout
