---
description: Safely split an oversized file into modules under 250 lines without changing behavior
---
1. Load `modular-file-size`. Run `check_file_size.sh` to list offenders.
2. Ensure `verify.sh` is green; add characterization tests if coverage is missing.
3. Extract one cohesive responsibility at a time, moving code verbatim. Run `verify.sh --fast` after each.
4. One commit per extraction. No behavior changes in this task.
