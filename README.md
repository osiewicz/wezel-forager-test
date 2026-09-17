# wezel forager test
hello wezel

This repository exercises Wezel's GitHub Actions runner integration.

The `line-count` experiment records generated LLVM IR lines.

The intentionally small program provides a stable baseline for comparison.

Runs are assigned by Fiflok and executed by the repository workflow.

The workflow checks out the exact commit assigned to each run.

Results are uploaded to Wezel after the experiment completes.

The fixture is intentionally dependency-free for fast, deterministic runs.

Commit history is kept linear so regression attribution is easy to inspect.

This project is test data rather than a production application.

## Linear-history regression fixture

The second attribution trial is applied to `main` with a fast-forward.

Its commits remain individually reachable from the tracked branch.
