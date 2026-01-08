# taskr-webapp

This repo contains the web frontend, backend, and API of the taskr platform application.

# Security

**Leakage will not be tolerated.**
- In order to keep intellectual property over this repository, we forbid the disclosure/leakage of any code/asset hosted within.

*More security measures will be added later.*

# Hosting

> [!WARNING]
> Windows is **not** supported, never will be.
> If you are on Windows and want to host the web application, consider using [WSL](). 

First, install the prerequisites using your package manager:
- `git`
- `npm`
- `rust` (using [rust-up]())

Then clone the repository and `cd` into it:
```sh
git clone https://github.com/taskrllc/taskr-webapp.git
cd taskr-webapp
```

And finally run the application locally:
```sh
npm run dev-all
```

Or build it:
```sh
npm run build-all
```

# Contributing

> [!NOTE]
> We use pull requests in order to merge commits. Only the CTO is authorized to approve/disapprove pull requests.

- If you are working on a **small change/fix** -> commit to the `devel` branch.
- If you are working on a **large addition/feature** -> create a new branch dedicated to said feature, forked from the `devel` branch. When the commit is ready, merge into the `devel` branch.

> [!WARNING]
> Never directly commit to the `master` branch. Your pull request will be closed.
