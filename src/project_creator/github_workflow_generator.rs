pub const GITHUB_WORKFLOW_CONTENTS: &str = r#"name: Odin

on:
  push:
    branches: ["master"]
  pull_request:
    branches: ["master"]

jobs:
  test:
    name: Test
    runs-on: ubuntu-latest
    steps:
      - uses: laytan/setup-odin@v2
        with:
            token: ${{ secrets.GITHUB_TOKEN }}
      - uses: actions/checkout@v4
      - name: Run tests
        run: make test
"#;
