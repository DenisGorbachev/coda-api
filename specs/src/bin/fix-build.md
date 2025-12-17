# Fix the build on GitHub Actions

## Background

* I'm running a build of this repository in GitHub Actions
* The build on Rust `stable` succeeds
* The build on Rust `beta` fails
* The build on Rust `nightly` fails
* More than approximately a month ago, the builds on `nightly` and `beta` succeeded
* Less than approximately a month ago, the build on `nightly` started to fail, but the build on `beta` succeeded
* Now, the build on `beta` fails, too

## Tasks

* Diagnose the root cause
* Suggest the ways to fix it

## Files

.github/workflows/ci.yml

```yaml
name: CI
on: [ push, pull_request ]

env:
  RUST_BACKTRACE: 1
  GH_TOKEN: ${{ secrets.GITHUB_TOKEN }}

jobs:
  test:
    name: test
    runs-on: ubuntu-latest
    strategy:
      fail-fast: false
      matrix:
        rust:
          - stable
          - beta
    timeout-minutes: 45
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: ${{ matrix.rust }}
          components: clippy, rustfmt
      - uses: jdx/mise-action@v2
        with:
          version: 2025.10.11
      - name: Remove rust from mise
        run: |
          # use the rust version that was installed in the previous step
          mise rm rust
      - uses: Swatinem/rust-cache@v2.8.0
      - run: lefthook run --force pre-commit
      - run: cargo hack test --feature-powerset
      - uses: wagoid/commitlint-github-action@v6
        with:
          failOnWarnings: true
          failOnErrors: true
```

Build log

```text
2025-12-16T23:20:13.6786056Z Current runner version: '2.330.0'
2025-12-16T23:20:13.6819703Z ##[group]Runner Image Provisioner
2025-12-16T23:20:13.6820999Z Hosted Compute Agent
2025-12-16T23:20:13.6821871Z Version: 20251202.455
2025-12-16T23:20:13.6822883Z Commit: 6c10caca4910e198df60de23adf20ad317c474e3
2025-12-16T23:20:13.6824006Z Build Date: 2025-12-02T15:56:59Z
2025-12-16T23:20:13.6825232Z Worker ID: {000d1299-ece3-467d-a6d4-f87c8a6bd16a}
2025-12-16T23:20:13.6826385Z ##[endgroup]
2025-12-16T23:20:13.6827154Z ##[group]Operating System
2025-12-16T23:20:13.6828463Z Ubuntu
2025-12-16T23:20:13.6829144Z 24.04.3
2025-12-16T23:20:13.6829934Z LTS
2025-12-16T23:20:13.6830789Z ##[endgroup]
2025-12-16T23:20:13.6831666Z ##[group]Runner Image
2025-12-16T23:20:13.6832608Z Image: ubuntu-24.04
2025-12-16T23:20:13.6833603Z Version: 20251208.163.1
2025-12-16T23:20:13.6835428Z Included Software: https://github.com/actions/runner-images/blob/ubuntu24/20251208.163/images/ubuntu/Ubuntu2404-Readme.md
2025-12-16T23:20:13.6838473Z Image Release: https://github.com/actions/runner-images/releases/tag/ubuntu24%2F20251208.163
2025-12-16T23:20:13.6840183Z ##[endgroup]
2025-12-16T23:20:13.6842013Z ##[group]GITHUB_TOKEN Permissions
2025-12-16T23:20:13.6845242Z Contents: read
2025-12-16T23:20:13.6846103Z Metadata: read
2025-12-16T23:20:13.6847105Z Packages: read
2025-12-16T23:20:13.6848159Z ##[endgroup]
2025-12-16T23:20:13.6851202Z Secret source: Actions
2025-12-16T23:20:13.6852833Z Prepare workflow directory
2025-12-16T23:20:13.7579410Z Prepare all required actions
2025-12-16T23:20:13.7635241Z Getting action download info
2025-12-16T23:20:14.0914413Z Download action repository 'actions/checkout@v4' (SHA:34e114876b0b11c390a56381ad16ebd13914f8d5)
2025-12-16T23:20:14.2104777Z Download action repository 'dtolnay/rust-toolchain@stable' (SHA:4be9e76fd7c4901c61fb841f559994984270fce7)
2025-12-16T23:20:14.3730628Z Download action repository 'jdx/mise-action@v2' (SHA:c37c93293d6b742fc901e1406b8f764f6fb19dac)
2025-12-16T23:20:14.6879813Z Download action repository 'Swatinem/rust-cache@v2.8.0' (SHA:98c8021b550208e191a6a3145459bfc9fb29c4c0)
2025-12-16T23:20:15.0854449Z Download action repository 'wagoid/commitlint-github-action@v6' (SHA:b948419dd99f3fd78a6548d48f94e3df7f6bf3ed)
2025-12-16T23:20:15.4171749Z Complete job name: test (beta)
2025-12-16T23:20:15.4620761Z ##[group]Pull down action image 'wagoid/commitlint-github-action:6.2.1'
2025-12-16T23:20:15.4673089Z ##[command]/usr/bin/docker pull wagoid/commitlint-github-action:6.2.1
2025-12-16T23:20:15.8456119Z 6.2.1: Pulling from wagoid/commitlint-github-action
2025-12-16T23:20:15.8458491Z c6a83fedfae6: Pulling fs layer
2025-12-16T23:20:15.8459612Z d9aac50bc34e: Pulling fs layer
2025-12-16T23:20:15.8460792Z 0150f131fd2f: Pulling fs layer
2025-12-16T23:20:15.8462206Z c0ce3bd8f303: Pulling fs layer
2025-12-16T23:20:15.8463399Z c838a2e7b927: Pulling fs layer
2025-12-16T23:20:15.8464368Z 6d77938f3f85: Pulling fs layer
2025-12-16T23:20:15.8465366Z 22b1a6226a7b: Pulling fs layer
2025-12-16T23:20:15.8466294Z eee6acead8dc: Pulling fs layer
2025-12-16T23:20:15.8467306Z 89eacf6857e8: Pulling fs layer
2025-12-16T23:20:15.8468659Z 6d77938f3f85: Waiting
2025-12-16T23:20:15.8469541Z 22b1a6226a7b: Waiting
2025-12-16T23:20:15.8470473Z c838a2e7b927: Waiting
2025-12-16T23:20:15.8471510Z eee6acead8dc: Waiting
2025-12-16T23:20:15.8472395Z 89eacf6857e8: Waiting
2025-12-16T23:20:15.8473427Z c0ce3bd8f303: Waiting
2025-12-16T23:20:16.0113915Z 0150f131fd2f: Verifying Checksum
2025-12-16T23:20:16.0116881Z 0150f131fd2f: Download complete
2025-12-16T23:20:16.0332517Z c6a83fedfae6: Download complete
2025-12-16T23:20:16.1060406Z c0ce3bd8f303: Verifying Checksum
2025-12-16T23:20:16.1062507Z c0ce3bd8f303: Download complete
2025-12-16T23:20:16.1499872Z c6a83fedfae6: Pull complete
2025-12-16T23:20:16.1863508Z c838a2e7b927: Verifying Checksum
2025-12-16T23:20:16.1865620Z c838a2e7b927: Download complete
2025-12-16T23:20:16.1871036Z 6d77938f3f85: Verifying Checksum
2025-12-16T23:20:16.1872946Z 6d77938f3f85: Download complete
2025-12-16T23:20:16.2605115Z d9aac50bc34e: Verifying Checksum
2025-12-16T23:20:16.2608971Z d9aac50bc34e: Download complete
2025-12-16T23:20:16.2803441Z 22b1a6226a7b: Download complete
2025-12-16T23:20:16.3391421Z 89eacf6857e8: Verifying Checksum
2025-12-16T23:20:16.3393587Z 89eacf6857e8: Download complete
2025-12-16T23:20:17.0177397Z eee6acead8dc: Verifying Checksum
2025-12-16T23:20:17.0178628Z eee6acead8dc: Download complete
2025-12-16T23:20:17.7324474Z d9aac50bc34e: Pull complete
2025-12-16T23:20:17.7791791Z 0150f131fd2f: Pull complete
2025-12-16T23:20:17.7938000Z c0ce3bd8f303: Pull complete
2025-12-16T23:20:17.9995499Z c838a2e7b927: Pull complete
2025-12-16T23:20:18.0103876Z 6d77938f3f85: Pull complete
2025-12-16T23:20:18.0302499Z 22b1a6226a7b: Pull complete
2025-12-16T23:20:22.3174010Z eee6acead8dc: Pull complete
2025-12-16T23:20:22.3273963Z 89eacf6857e8: Pull complete
2025-12-16T23:20:22.3312481Z Digest: sha256:86a04e0a99128551a7555c269d2b675c3c85f61358cf7dd558f6b873b66f561a
2025-12-16T23:20:22.3324512Z Status: Downloaded newer image for wagoid/commitlint-github-action:6.2.1
2025-12-16T23:20:22.3337924Z docker.io/wagoid/commitlint-github-action:6.2.1
2025-12-16T23:20:22.3363327Z ##[endgroup]
2025-12-16T23:20:22.3633665Z ##[group]Run actions/checkout@v4
2025-12-16T23:20:22.3634507Z with:
2025-12-16T23:20:22.3634791Z   repository: DenisGorbachev/coda-api
2025-12-16T23:20:22.3635334Z   token: ***
2025-12-16T23:20:22.3635674Z   ssh-strict: true
2025-12-16T23:20:22.3636007Z   ssh-user: git
2025-12-16T23:20:22.3636263Z   persist-credentials: true
2025-12-16T23:20:22.3636633Z   clean: true
2025-12-16T23:20:22.3636919Z   sparse-checkout-cone-mode: true
2025-12-16T23:20:22.3637207Z   fetch-depth: 1
2025-12-16T23:20:22.3637531Z   fetch-tags: false
2025-12-16T23:20:22.3638011Z   show-progress: true
2025-12-16T23:20:22.3638336Z   lfs: false
2025-12-16T23:20:22.3638610Z   submodules: false
2025-12-16T23:20:22.3638915Z   set-safe-directory: true
2025-12-16T23:20:22.3639478Z env:
2025-12-16T23:20:22.3639801Z   RUST_BACKTRACE: 1
2025-12-16T23:20:22.3640279Z   GH_TOKEN: ***
2025-12-16T23:20:22.3640582Z ##[endgroup]
2025-12-16T23:20:22.4708670Z Syncing repository: DenisGorbachev/coda-api
2025-12-16T23:20:22.4710357Z ##[group]Getting Git version info
2025-12-16T23:20:22.4710805Z Working directory is '/home/runner/work/coda-api/coda-api'
2025-12-16T23:20:22.4784344Z [command]/usr/bin/git version
2025-12-16T23:20:22.4803446Z git version 2.52.0
2025-12-16T23:20:22.4829557Z ##[endgroup]
2025-12-16T23:20:22.4843552Z Temporarily overriding HOME='/home/runner/work/_temp/25899f06-7e2a-46e1-a0ee-02482482c94b' before making global git config changes
2025-12-16T23:20:22.4844521Z Adding repository directory to the temporary git global config as a safe directory
2025-12-16T23:20:22.4856572Z [command]/usr/bin/git config --global --add safe.directory /home/runner/work/coda-api/coda-api
2025-12-16T23:20:22.4902320Z Deleting the contents of '/home/runner/work/coda-api/coda-api'
2025-12-16T23:20:22.4905671Z ##[group]Initializing the repository
2025-12-16T23:20:22.4909967Z [command]/usr/bin/git init /home/runner/work/coda-api/coda-api
2025-12-16T23:20:22.5038357Z hint: Using 'master' as the name for the initial branch. This default branch name
2025-12-16T23:20:22.5039375Z hint: will change to "main" in Git 3.0. To configure the initial branch name
2025-12-16T23:20:22.5040281Z hint: to use in all of your new repositories, which will suppress this warning,
2025-12-16T23:20:22.5040979Z hint: call:
2025-12-16T23:20:22.5041321Z hint:
2025-12-16T23:20:22.5041678Z hint: 	git config --global init.defaultBranch <name>
2025-12-16T23:20:22.5042043Z hint:
2025-12-16T23:20:22.5042389Z hint: Names commonly chosen instead of 'master' are 'main', 'trunk' and
2025-12-16T23:20:22.5042995Z hint: 'development'. The just-created branch can be renamed via this command:
2025-12-16T23:20:22.5043438Z hint:
2025-12-16T23:20:22.5043673Z hint: 	git branch -m <name>
2025-12-16T23:20:22.5043934Z hint:
2025-12-16T23:20:22.5044544Z hint: Disable this message with "git config set advice.defaultBranchName false"
2025-12-16T23:20:22.5045925Z Initialized empty Git repository in /home/runner/work/coda-api/coda-api/.git/
2025-12-16T23:20:22.5057137Z [command]/usr/bin/git remote add origin https://github.com/DenisGorbachev/coda-api
2025-12-16T23:20:22.5091693Z ##[endgroup]
2025-12-16T23:20:22.5092354Z ##[group]Disabling automatic garbage collection
2025-12-16T23:20:22.5096701Z [command]/usr/bin/git config --local gc.auto 0
2025-12-16T23:20:22.5124993Z ##[endgroup]
2025-12-16T23:20:22.5125619Z ##[group]Setting up auth
2025-12-16T23:20:22.5132825Z [command]/usr/bin/git config --local --name-only --get-regexp core\.sshCommand
2025-12-16T23:20:22.5162703Z [command]/usr/bin/git submodule foreach --recursive sh -c "git config --local --name-only --get-regexp 'core\.sshCommand' && git config --local --unset-all 'core.sshCommand' || :"
2025-12-16T23:20:22.5585271Z [command]/usr/bin/git config --local --name-only --get-regexp http\.https\:\/\/github\.com\/\.extraheader
2025-12-16T23:20:22.5617168Z [command]/usr/bin/git submodule foreach --recursive sh -c "git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :"
2025-12-16T23:20:22.5834256Z [command]/usr/bin/git config --local --name-only --get-regexp ^includeIf\.gitdir:
2025-12-16T23:20:22.5864594Z [command]/usr/bin/git submodule foreach --recursive git config --local --show-origin --name-only --get-regexp remote.origin.url
2025-12-16T23:20:22.6094093Z [command]/usr/bin/git config --local http.https://github.com/.extraheader AUTHORIZATION: basic ***
2025-12-16T23:20:22.6127268Z ##[endgroup]
2025-12-16T23:20:22.6128257Z ##[group]Fetching the repository
2025-12-16T23:20:22.6136286Z [command]/usr/bin/git -c protocol.version=2 fetch --no-tags --prune --no-recurse-submodules --depth=1 origin +301809a52ba98ee5de9cc1be07c4e6eecabd649e:refs/remotes/origin/main
2025-12-16T23:20:22.8654785Z From https://github.com/DenisGorbachev/coda-api
2025-12-16T23:20:22.8655707Z  * [new ref]         301809a52ba98ee5de9cc1be07c4e6eecabd649e -> origin/main
2025-12-16T23:20:22.8692083Z ##[endgroup]
2025-12-16T23:20:22.8692498Z ##[group]Determining the checkout info
2025-12-16T23:20:22.8694759Z ##[endgroup]
2025-12-16T23:20:22.8700270Z [command]/usr/bin/git sparse-checkout disable
2025-12-16T23:20:22.8743529Z [command]/usr/bin/git config --local --unset-all extensions.worktreeConfig
2025-12-16T23:20:22.8769712Z ##[group]Checking out the ref
2025-12-16T23:20:22.8774240Z [command]/usr/bin/git checkout --progress --force -B main refs/remotes/origin/main
2025-12-16T23:20:22.8927918Z Switched to a new branch 'main'
2025-12-16T23:20:22.8931254Z branch 'main' set up to track 'origin/main'.
2025-12-16T23:20:22.8936919Z ##[endgroup]
2025-12-16T23:20:22.8971004Z [command]/usr/bin/git log -1 --format=%H
2025-12-16T23:20:22.8992696Z 301809a52ba98ee5de9cc1be07c4e6eecabd649e
2025-12-16T23:20:22.9253222Z ##[group]Run dtolnay/rust-toolchain@stable
2025-12-16T23:20:22.9253503Z with:
2025-12-16T23:20:22.9253675Z   toolchain: beta
2025-12-16T23:20:22.9253881Z   components: clippy, rustfmt
2025-12-16T23:20:22.9254091Z env:
2025-12-16T23:20:22.9254252Z   RUST_BACKTRACE: 1
2025-12-16T23:20:22.9254622Z   GH_TOKEN: ***
2025-12-16T23:20:22.9254805Z ##[endgroup]
2025-12-16T23:20:22.9352181Z ##[group]Run : parse toolchain version
2025-12-16T23:20:22.9352547Z [36;1m: parse toolchain version[0m
2025-12-16T23:20:22.9352802Z [36;1mif [[ -z $toolchain ]]; then[0m
2025-12-16T23:20:22.9353277Z [36;1m  # GitHub does not enforce `required: true` inputs itself. https://github.com/actions/runner/issues/1070[0m
2025-12-16T23:20:22.9353754Z [36;1m  echo "'toolchain' is a required input" >&2[0m
2025-12-16T23:20:22.9354016Z [36;1m  exit 1[0m
2025-12-16T23:20:22.9354306Z [36;1melif [[ $toolchain =~ ^stable' '[0-9]+' '(year|month|week|day)s?' 'ago$ ]]; then[0m
2025-12-16T23:20:22.9354695Z [36;1m  if [[ Linux == macOS ]]; then[0m
2025-12-16T23:20:22.9355167Z [36;1m    echo "toolchain=1.$((($(date -v-$(sed 's/stable \([0-9]*\) \(.\).*/\1\2/' <<< $toolchain) +%s)/60/60/24-16569)/7/6))" >> $GITHUB_OUTPUT[0m
2025-12-16T23:20:22.9355788Z [36;1m  else[0m
2025-12-16T23:20:22.9356132Z [36;1m    echo "toolchain=1.$((($(date --date "${toolchain#stable }" +%s)/60/60/24-16569)/7/6))" >> $GITHUB_OUTPUT[0m
2025-12-16T23:20:22.9356508Z [36;1m  fi[0m
2025-12-16T23:20:22.9356773Z [36;1melif [[ $toolchain =~ ^stable' 'minus' '[0-9]+' 'releases?$ ]]; then[0m
2025-12-16T23:20:22.9357227Z [36;1m  echo "toolchain=1.$((($(date +%s)/60/60/24-16569)/7/6-${toolchain//[^0-9]/}))" >> $GITHUB_OUTPUT[0m
2025-12-16T23:20:22.9357778Z [36;1melif [[ $toolchain =~ ^1\.[0-9]+$ ]]; then[0m
2025-12-16T23:20:22.9358220Z [36;1m  echo "toolchain=1.$((i=${toolchain#1.}, c=($(date +%s)/60/60/24-16569)/7/6, i+9*i*(10*i<=c)+90*i*(100*i<=c)))" >> $GITHUB_OUTPUT[0m
2025-12-16T23:20:22.9358622Z [36;1melse[0m
2025-12-16T23:20:22.9358845Z [36;1m  echo "toolchain=$toolchain" >> $GITHUB_OUTPUT[0m
2025-12-16T23:20:22.9359100Z [36;1mfi[0m
2025-12-16T23:20:22.9396068Z shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
2025-12-16T23:20:22.9396404Z env:
2025-12-16T23:20:22.9396568Z   RUST_BACKTRACE: 1
2025-12-16T23:20:22.9396919Z   GH_TOKEN: ***
2025-12-16T23:20:22.9397095Z   toolchain: beta
2025-12-16T23:20:22.9397273Z ##[endgroup]
2025-12-16T23:20:22.9527866Z ##[group]Run : construct rustup command line
2025-12-16T23:20:22.9528170Z [36;1m: construct rustup command line[0m
2025-12-16T23:20:22.9528562Z [36;1mecho "targets=$(for t in ${targets//,/ }; do echo -n ' --target' $t; done)" >> $GITHUB_OUTPUT[0m
2025-12-16T23:20:22.9529116Z [36;1mecho "components=$(for c in ${components//,/ }; do echo -n ' --component' $c; done)" >> $GITHUB_OUTPUT[0m
2025-12-16T23:20:22.9529542Z [36;1mecho "downgrade=" >> $GITHUB_OUTPUT[0m
2025-12-16T23:20:22.9561270Z shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
2025-12-16T23:20:22.9561582Z env:
2025-12-16T23:20:22.9561753Z   RUST_BACKTRACE: 1
2025-12-16T23:20:22.9562098Z   GH_TOKEN: ***
2025-12-16T23:20:22.9562274Z   targets: 
2025-12-16T23:20:22.9562469Z   components: clippy, rustfmt
2025-12-16T23:20:22.9562682Z ##[endgroup]
2025-12-16T23:20:22.9640130Z ##[group]Run : set $CARGO_HOME
2025-12-16T23:20:22.9640373Z [36;1m: set $CARGO_HOME[0m
2025-12-16T23:20:22.9640658Z [36;1mecho CARGO_HOME=${CARGO_HOME:-"$HOME/.cargo"} >> $GITHUB_ENV[0m
2025-12-16T23:20:22.9671493Z shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
2025-12-16T23:20:22.9671809Z env:
2025-12-16T23:20:22.9671976Z   RUST_BACKTRACE: 1
2025-12-16T23:20:22.9672345Z   GH_TOKEN: ***
2025-12-16T23:20:22.9672523Z ##[endgroup]
2025-12-16T23:20:22.9746471Z ##[group]Run : install rustup if needed
2025-12-16T23:20:22.9746756Z [36;1m: install rustup if needed[0m
2025-12-16T23:20:22.9747018Z [36;1mif ! command -v rustup &>/dev/null; then[0m
2025-12-16T23:20:22.9748135Z [36;1m  curl --proto '=https' --tlsv1.2 --retry 10 --retry-connrefused --location --silent --show-error --fail https://sh.rustup.rs | sh -s -- --default-toolchain none -y[0m
2025-12-16T23:20:22.9748820Z [36;1m  echo "$CARGO_HOME/bin" >> $GITHUB_PATH[0m
2025-12-16T23:20:22.9749080Z [36;1mfi[0m
2025-12-16T23:20:22.9780739Z shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
2025-12-16T23:20:22.9781054Z env:
2025-12-16T23:20:22.9781220Z   RUST_BACKTRACE: 1
2025-12-16T23:20:22.9781547Z   GH_TOKEN: ***
2025-12-16T23:20:22.9781740Z   CARGO_HOME: /home/runner/.cargo
2025-12-16T23:20:22.9781969Z ##[endgroup]
2025-12-16T23:20:22.9856210Z ##[group]Run rustup toolchain install beta --component clippy --component rustfmt --profile minimal --no-self-update
2025-12-16T23:20:22.9856943Z [36;1mrustup toolchain install beta --component clippy --component rustfmt --profile minimal --no-self-update[0m
2025-12-16T23:20:22.9888943Z shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
2025-12-16T23:20:22.9889247Z env:
2025-12-16T23:20:22.9889424Z   RUST_BACKTRACE: 1
2025-12-16T23:20:22.9889778Z   GH_TOKEN: ***
2025-12-16T23:20:22.9889986Z   CARGO_HOME: /home/runner/.cargo
2025-12-16T23:20:22.9890225Z   RUSTUP_PERMIT_COPY_RENAME: 1
2025-12-16T23:20:22.9890599Z ##[endgroup]
2025-12-16T23:20:23.3036432Z info: syncing channel updates for 'beta-x86_64-unknown-linux-gnu'
2025-12-16T23:20:23.6865382Z info: latest update on 2025-12-15, rust version 1.93.0-beta.2 (eb937a317 2025-12-14)
2025-12-16T23:20:23.6866040Z info: downloading component 'cargo'
2025-12-16T23:20:23.7745427Z info: downloading component 'clippy'
2025-12-16T23:20:23.8394521Z info: downloading component 'rust-std'
2025-12-16T23:20:24.0128805Z info: downloading component 'rustc'
2025-12-16T23:20:24.3702951Z info: downloading component 'rustfmt'
2025-12-16T23:20:24.4220033Z info: installing component 'cargo'
2025-12-16T23:20:25.1344872Z info: installing component 'clippy'
2025-12-16T23:20:25.5069687Z info: installing component 'rust-std'
2025-12-16T23:20:27.5140498Z info: installing component 'rustc'
2025-12-16T23:20:32.0335895Z info: installing component 'rustfmt'
2025-12-16T23:20:32.3162535Z 
2025-12-16T23:20:32.3255515Z   beta-x86_64-unknown-linux-gnu installed - rustc 1.93.0-beta.2 (eb937a317 2025-12-14)
2025-12-16T23:20:32.3256064Z 
2025-12-16T23:20:32.3304313Z ##[group]Run rustup default beta
2025-12-16T23:20:32.3304578Z [36;1mrustup default beta[0m
2025-12-16T23:20:32.3336299Z shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
2025-12-16T23:20:32.3336600Z env:
2025-12-16T23:20:32.3336766Z   RUST_BACKTRACE: 1
2025-12-16T23:20:32.3337268Z   GH_TOKEN: ***
2025-12-16T23:20:32.3337468Z   CARGO_HOME: /home/runner/.cargo
2025-12-16T23:20:32.3337813Z ##[endgroup]
2025-12-16T23:20:32.3437528Z info: using existing install for 'beta-x86_64-unknown-linux-gnu'
2025-12-16T23:20:32.3719681Z info: default toolchain set to 'beta-x86_64-unknown-linux-gnu'
2025-12-16T23:20:32.3720177Z 
2025-12-16T23:20:32.3806924Z   beta-x86_64-unknown-linux-gnu unchanged - rustc 1.93.0-beta.2 (eb937a317 2025-12-14)
2025-12-16T23:20:32.3807526Z 
2025-12-16T23:20:32.3847606Z ##[group]Run : create cachekey
2025-12-16T23:20:32.3848325Z [36;1m: create cachekey[0m
2025-12-16T23:20:32.3848769Z [36;1mDATE=$(rustc +beta --version --verbose | sed -ne 's/^commit-date: \(20[0-9][0-9]\)-\([01][0-9]\)-\([0-3][0-9]\)$/\1\2\3/p')[0m
2025-12-16T23:20:32.3849335Z [36;1mHASH=$(rustc +beta --version --verbose | sed -ne 's/^commit-hash: //p')[0m
2025-12-16T23:20:32.3849780Z [36;1mecho "cachekey=$(echo $DATE$HASH | head -c12)" >> $GITHUB_OUTPUT[0m
2025-12-16T23:20:32.3881892Z shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
2025-12-16T23:20:32.3882207Z env:
2025-12-16T23:20:32.3882369Z   RUST_BACKTRACE: 1
2025-12-16T23:20:32.3882739Z   GH_TOKEN: ***
2025-12-16T23:20:32.3882940Z   CARGO_HOME: /home/runner/.cargo
2025-12-16T23:20:32.3883157Z ##[endgroup]
2025-12-16T23:20:32.4256676Z ##[group]Run : disable incremental compilation
2025-12-16T23:20:32.4257000Z [36;1m: disable incremental compilation[0m
2025-12-16T23:20:32.4257452Z [36;1mif [ -z "${CARGO_INCREMENTAL+set}" ]; then[0m
2025-12-16T23:20:32.4258015Z [36;1m  echo CARGO_INCREMENTAL=0 >> $GITHUB_ENV[0m
2025-12-16T23:20:32.4258266Z [36;1mfi[0m
2025-12-16T23:20:32.4289686Z shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
2025-12-16T23:20:32.4289991Z env:
2025-12-16T23:20:32.4290151Z   RUST_BACKTRACE: 1
2025-12-16T23:20:32.4290572Z   GH_TOKEN: ***
2025-12-16T23:20:32.4290766Z   CARGO_HOME: /home/runner/.cargo
2025-12-16T23:20:32.4290988Z ##[endgroup]
2025-12-16T23:20:32.4382583Z ##[group]Run : enable colors in Cargo output
2025-12-16T23:20:32.4382876Z [36;1m: enable colors in Cargo output[0m
2025-12-16T23:20:32.4383153Z [36;1mif [ -z "${CARGO_TERM_COLOR+set}" ]; then[0m
2025-12-16T23:20:32.4383445Z [36;1m  echo CARGO_TERM_COLOR=always >> $GITHUB_ENV[0m
2025-12-16T23:20:32.4383699Z [36;1mfi[0m
2025-12-16T23:20:32.4413706Z shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
2025-12-16T23:20:32.4414022Z env:
2025-12-16T23:20:32.4414191Z   RUST_BACKTRACE: 1
2025-12-16T23:20:32.4414547Z   GH_TOKEN: ***
2025-12-16T23:20:32.4414746Z   CARGO_HOME: /home/runner/.cargo
2025-12-16T23:20:32.4414969Z   CARGO_INCREMENTAL: 0
2025-12-16T23:20:32.4415326Z ##[endgroup]
2025-12-16T23:20:32.4478894Z ##[group]Run : enable Cargo sparse registry
2025-12-16T23:20:32.4479192Z [36;1m: enable Cargo sparse registry[0m
2025-12-16T23:20:32.4479509Z [36;1m# implemented in 1.66, stabilized in 1.68, made default in 1.70[0m
2025-12-16T23:20:32.4480127Z [36;1mif [ -z "${CARGO_REGISTRIES_CRATES_IO_PROTOCOL+set}" -o -f "/home/runner/work/_temp"/.implicit_cargo_registries_crates_io_protocol ]; then[0m
2025-12-16T23:20:32.4480759Z [36;1m  if rustc +beta --version --verbose | grep -q '^release: 1\.6[89]\.'; then[0m
2025-12-16T23:20:32.4481250Z [36;1m    touch "/home/runner/work/_temp"/.implicit_cargo_registries_crates_io_protocol || true[0m
2025-12-16T23:20:32.4481731Z [36;1m    echo CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse >> $GITHUB_ENV[0m
2025-12-16T23:20:32.4482164Z [36;1m  elif rustc +beta --version --verbose | grep -q '^release: 1\.6[67]\.'; then[0m
2025-12-16T23:20:32.4482657Z [36;1m    touch "/home/runner/work/_temp"/.implicit_cargo_registries_crates_io_protocol || true[0m
2025-12-16T23:20:32.4483120Z [36;1m    echo CARGO_REGISTRIES_CRATES_IO_PROTOCOL=git >> $GITHUB_ENV[0m
2025-12-16T23:20:32.4483418Z [36;1m  fi[0m
2025-12-16T23:20:32.4483584Z [36;1mfi[0m
2025-12-16T23:20:32.4511495Z shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
2025-12-16T23:20:32.4511787Z env:
2025-12-16T23:20:32.4511947Z   RUST_BACKTRACE: 1
2025-12-16T23:20:32.4512251Z   GH_TOKEN: ***
2025-12-16T23:20:32.4512448Z   CARGO_HOME: /home/runner/.cargo
2025-12-16T23:20:32.4512673Z   CARGO_INCREMENTAL: 0
2025-12-16T23:20:32.4512867Z   CARGO_TERM_COLOR: always
2025-12-16T23:20:32.4513061Z ##[endgroup]
2025-12-16T23:20:32.4860780Z ##[group]Run : work around spurious network errors in curl 8.0
2025-12-16T23:20:32.4861180Z [36;1m: work around spurious network errors in curl 8.0[0m
2025-12-16T23:20:32.4861706Z [36;1m# https://rust-lang.zulipchat.com/#narrow/stream/246057-t-cargo/topic/timeout.20investigation[0m
2025-12-16T23:20:32.4862258Z [36;1mif rustc +beta --version --verbose | grep -q '^release: 1\.7[01]\.'; then[0m
2025-12-16T23:20:32.4862670Z [36;1m  echo CARGO_HTTP_MULTIPLEXING=false >> $GITHUB_ENV[0m
2025-12-16T23:20:32.4862951Z [36;1mfi[0m
2025-12-16T23:20:32.4894493Z shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
2025-12-16T23:20:32.4894812Z env:
2025-12-16T23:20:32.4894996Z   RUST_BACKTRACE: 1
2025-12-16T23:20:32.4895362Z   GH_TOKEN: ***
2025-12-16T23:20:32.4895568Z   CARGO_HOME: /home/runner/.cargo
2025-12-16T23:20:32.4895796Z   CARGO_INCREMENTAL: 0
2025-12-16T23:20:32.4895997Z   CARGO_TERM_COLOR: always
2025-12-16T23:20:32.4896206Z ##[endgroup]
2025-12-16T23:20:32.5104141Z ##[group]Run rustc +beta --version --verbose
2025-12-16T23:20:32.5104444Z [36;1mrustc +beta --version --verbose[0m
2025-12-16T23:20:32.5135063Z shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
2025-12-16T23:20:32.5135530Z env:
2025-12-16T23:20:32.5135700Z   RUST_BACKTRACE: 1
2025-12-16T23:20:32.5136090Z   GH_TOKEN: ***
2025-12-16T23:20:32.5136281Z   CARGO_HOME: /home/runner/.cargo
2025-12-16T23:20:32.5136524Z   CARGO_INCREMENTAL: 0
2025-12-16T23:20:32.5136713Z   CARGO_TERM_COLOR: always
2025-12-16T23:20:32.5136912Z ##[endgroup]
2025-12-16T23:20:32.5309805Z rustc 1.93.0-beta.2 (eb937a317 2025-12-14)
2025-12-16T23:20:32.5310327Z binary: rustc
2025-12-16T23:20:32.5310718Z commit-hash: eb937a3172395082904b966308c0cb5e7ac3c930
2025-12-16T23:20:32.5311180Z commit-date: 2025-12-14
2025-12-16T23:20:32.5311518Z host: x86_64-unknown-linux-gnu
2025-12-16T23:20:32.5311861Z release: 1.93.0-beta.2
2025-12-16T23:20:32.5312171Z LLVM version: 21.1.5
2025-12-16T23:20:32.5410628Z ##[group]Run jdx/mise-action@v2
2025-12-16T23:20:32.5410862Z with:
2025-12-16T23:20:32.5411054Z   version: 2025.10.11
2025-12-16T23:20:32.5411233Z   install: true
2025-12-16T23:20:32.5411410Z   cache: true
2025-12-16T23:20:32.5411569Z   cache_save: true
2025-12-16T23:20:32.5411758Z   cache_key_prefix: mise-v0
2025-12-16T23:20:32.5411957Z   experimental: false
2025-12-16T23:20:32.5412136Z   log_level: info
2025-12-16T23:20:32.5412464Z   reshim: false
2025-12-16T23:20:32.5412756Z   github_token: ***
2025-12-16T23:20:32.5412944Z   fetch_from_github: true
2025-12-16T23:20:32.5413128Z env:
2025-12-16T23:20:32.5413285Z   RUST_BACKTRACE: 1
2025-12-16T23:20:32.5413535Z   GH_TOKEN: ***
2025-12-16T23:20:32.5413721Z   CARGO_HOME: /home/runner/.cargo
2025-12-16T23:20:32.5413939Z   CARGO_INCREMENTAL: 0
2025-12-16T23:20:32.5414127Z   CARGO_TERM_COLOR: always
2025-12-16T23:20:32.5414316Z ##[endgroup]
2025-12-16T23:20:32.6805948Z ##[group]Restoring mise cache
2025-12-16T23:20:32.7548444Z [command]/usr/bin/ldd --version
2025-12-16T23:20:32.7602454Z ldd (Ubuntu GLIBC 2.39-0ubuntu8.6) 2.39
2025-12-16T23:20:32.7605072Z Copyright (C) 2024 Free Software Foundation, Inc.
2025-12-16T23:20:32.7606046Z This is free software; see the source for copying conditions.  There is NO
2025-12-16T23:20:32.7606902Z warranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
2025-12-16T23:20:32.7607844Z Written by Roland McGrath and Ulrich Drepper.
2025-12-16T23:20:32.9614824Z Cache hit for: mise-v0-linux-x64-99601490c61c013d5a41a2e1a716fbd374482f7e16bfc26d846eeb83e2c77d86-2025.10.11
2025-12-16T23:20:34.0250743Z Received 174786345 of 174786345 (100.0%), 166.5 MBs/sec
2025-12-16T23:20:34.0251961Z Cache Size: ~167 MB (174786345 B)
2025-12-16T23:20:34.0280486Z [command]/usr/bin/tar -xf /home/runner/work/_temp/d83af9b7-d239-40a2-b726-08ddd626197a/cache.tzst -P -C /home/runner/work/coda-api/coda-api --use-compress-program unzstd
2025-12-16T23:20:35.5622310Z Cache restored successfully
2025-12-16T23:20:35.5956421Z mise cache restored from key: mise-v0-linux-x64-99601490c61c013d5a41a2e1a716fbd374482f7e16bfc26d846eeb83e2c77d86-2025.10.11
2025-12-16T23:20:35.5961873Z ##[group]Setting env vars
2025-12-16T23:20:35.5962301Z Setting MISE_LOG_LEVEL=info
2025-12-16T23:20:35.5963788Z Setting GITHUB_TOKEN=***
2025-12-16T23:20:35.5964363Z Setting MISE_TRUSTED_CONFIG_PATHS=/home/runner/work/coda-api/coda-api
2025-12-16T23:20:35.5965722Z Setting MISE_YES=1
2025-12-16T23:20:35.5966772Z Adding /home/runner/.local/share/mise/shims to PATH
2025-12-16T23:20:35.5970104Z ##[group]Running mise --version
2025-12-16T23:20:35.5983987Z [command]/home/runner/.local/share/mise/bin/mise --version
2025-12-16T23:20:35.6070892Z 2025.10.11 linux-x64 (2025-10-18)
2025-12-16T23:20:35.6118758Z ##[endgroup]
2025-12-16T23:20:35.6119396Z ##[group]Running mise install 
2025-12-16T23:20:35.6130511Z [command]/home/runner/.local/share/mise/bin/mise install
2025-12-16T23:20:35.6327888Z [2mmise[0m all tools are installed
2025-12-16T23:20:35.6406679Z ##[endgroup]
2025-12-16T23:20:35.6407124Z ##[group]Running mise ls
2025-12-16T23:20:35.6420481Z [command]/home/runner/.local/share/mise/bin/mise ls
2025-12-16T23:20:35.6580630Z cargo-binstall                                            1.10.15           ~/work/coda-api/coda-api/mise.toml  1.10.15
2025-12-16T23:20:35.6581483Z cargo:cargo-expand                                        1.0.114           ~/work/coda-api/coda-api/mise.toml  1.0.114
2025-12-16T23:20:35.6582296Z cargo:cargo-hack                                          0.6.33            ~/work/coda-api/coda-api/mise.toml  0.6.33
2025-12-16T23:20:35.6582774Z cargo:cargo-machete                                       0.7.0             ~/work/coda-api/coda-api/mise.toml  0.7.0
2025-12-16T23:20:35.6583225Z cargo:cargo-nextest                                       0.9.102           ~/work/coda-api/coda-api/mise.toml  0.9.102
2025-12-16T23:20:35.6583985Z cargo:cargo-progenitor                                    0.11.1            ~/work/coda-api/coda-api/mise.toml  0.11.1
2025-12-16T23:20:35.6584441Z cargo:cargo-sort                                          1.0.9             ~/work/coda-api/coda-api/mise.toml  1.0.9
2025-12-16T23:20:35.6585029Z cargo:https://github.com/DenisGorbachev/cargo-doc2readme  ref:dev           ~/work/coda-api/coda-api/mise.toml  branch:dev
2025-12-16T23:20:35.6585608Z cargo:rumdl                                               0.0.185           ~/work/coda-api/coda-api/mise.toml  0.0.185
2025-12-16T23:20:35.6586145Z cargo:sd                                                  1.0.0             ~/work/coda-api/coda-api/mise.toml  1.0.0
2025-12-16T23:20:35.6586506Z deno                                                      1.46.1            ~/work/coda-api/coda-api/mise.toml  1.46.1
2025-12-16T23:20:35.6586852Z jq                                                        1.8.1             ~/work/coda-api/coda-api/mise.toml  1.8.1
2025-12-16T23:20:35.6587216Z node                                                      22.12.0           ~/work/coda-api/coda-api/mise.toml  22.12.0
2025-12-16T23:20:35.6587819Z npm:@commitlint/cli                                       19.6.0            ~/work/coda-api/coda-api/mise.toml  19.6.0
2025-12-16T23:20:35.6588412Z npm:@commitlint/config-conventional                       19.6.0            ~/work/coda-api/coda-api/mise.toml  19.6.0
2025-12-16T23:20:35.6588901Z npm:@commitlint/types                                     19.5.0            ~/work/coda-api/coda-api/mise.toml  19.5.0
2025-12-16T23:20:35.6589328Z npm:lefthook                                              1.8.5             ~/work/coda-api/coda-api/mise.toml  1.8.5
2025-12-16T23:20:35.6589721Z npm:repomix                                               1.5.0             ~/work/coda-api/coda-api/mise.toml  1.5.0
2025-12-16T23:20:35.6590115Z rust                                                      1.89.0 (symlink)  ~/work/coda-api/coda-api/mise.toml  1.89.0
2025-12-16T23:20:35.6601701Z ##[endgroup]
2025-12-16T23:20:35.6728079Z ##[group]Run # use the rust version that was installed in the previous step
2025-12-16T23:20:35.6728599Z [36;1m# use the rust version that was installed in the previous step[0m
2025-12-16T23:20:35.6728930Z [36;1mmise rm rust[0m
2025-12-16T23:20:35.6764699Z shell: /usr/bin/bash -e {0}
2025-12-16T23:20:35.6764931Z env:
2025-12-16T23:20:35.6765104Z   RUST_BACKTRACE: 1
2025-12-16T23:20:35.6765452Z   GH_TOKEN: ***
2025-12-16T23:20:35.6765658Z   CARGO_HOME: /home/runner/.cargo
2025-12-16T23:20:35.6765898Z   CARGO_INCREMENTAL: 0
2025-12-16T23:20:35.6766101Z   CARGO_TERM_COLOR: always
2025-12-16T23:20:35.6766305Z   MISE_LOG_LEVEL: info
2025-12-16T23:20:35.6766577Z   GITHUB_TOKEN: ***
2025-12-16T23:20:35.6766857Z   MISE_TRUSTED_CONFIG_PATHS: /home/runner/work/coda-api/coda-api
2025-12-16T23:20:35.6767171Z   MISE_YES: 1
2025-12-16T23:20:35.6767349Z ##[endgroup]
2025-12-16T23:20:35.6935599Z [2mmise[0m removed: rust from ~/work/coda-api/coda-api/mise.toml
2025-12-16T23:20:35.6970625Z [2mmise[0m [34mrust[0m@1.89.0                         uninstall
2025-12-16T23:20:35.7042152Z info: no toolchain installed for '1.89.0-x86_64-unknown-linux-gnu'
2025-12-16T23:20:35.7052105Z [2mmise[0m [34mrust[0m@1.89.0                         remove ~/.local/share/mise/installs/rust/1.89.0
2025-12-16T23:20:35.7054483Z [2mmise[0m [34mrust[0m@1.89.0                       [38;5;10m✓[0m [32mdone[0m
2025-12-16T23:20:35.7206837Z ##[group]Run Swatinem/rust-cache@v2.8.0
2025-12-16T23:20:35.7207109Z with:
2025-12-16T23:20:35.7207311Z   prefix-key: v0-rust
2025-12-16T23:20:35.7207528Z   cache-targets: true
2025-12-16T23:20:35.7208357Z   cache-all-crates: false
2025-12-16T23:20:35.7208645Z   cache-workspace-crates: false
2025-12-16T23:20:35.7208887Z   save-if: true
2025-12-16T23:20:35.7209083Z   cache-provider: github
2025-12-16T23:20:35.7209300Z   cache-bin: true
2025-12-16T23:20:35.7209490Z   lookup-only: false
2025-12-16T23:20:35.7209675Z env:
2025-12-16T23:20:35.7209848Z   RUST_BACKTRACE: 1
2025-12-16T23:20:35.7210192Z   GH_TOKEN: ***
2025-12-16T23:20:35.7210410Z   CARGO_HOME: /home/runner/.cargo
2025-12-16T23:20:35.7210653Z   CARGO_INCREMENTAL: 0
2025-12-16T23:20:35.7210864Z   CARGO_TERM_COLOR: always
2025-12-16T23:20:35.7211080Z   MISE_LOG_LEVEL: info
2025-12-16T23:20:35.7211368Z   GITHUB_TOKEN: ***
2025-12-16T23:20:35.7211658Z   MISE_TRUSTED_CONFIG_PATHS: /home/runner/work/coda-api/coda-api
2025-12-16T23:20:35.7212023Z   MISE_YES: 1
2025-12-16T23:20:35.7212208Z ##[endgroup]
2025-12-16T23:20:36.1211451Z ##[group]Cache Configuration
2025-12-16T23:20:36.1212245Z Cache Provider:
2025-12-16T23:20:36.1212575Z     github
2025-12-16T23:20:36.1212876Z Workspaces:
2025-12-16T23:20:36.1213225Z     /home/runner/work/coda-api/coda-api
2025-12-16T23:20:36.1213672Z Cache Paths:
2025-12-16T23:20:36.1213997Z     /home/runner/.cargo/bin
2025-12-16T23:20:36.1214426Z     /home/runner/.cargo/.crates.toml
2025-12-16T23:20:36.1214999Z     /home/runner/.cargo/.crates2.json
2025-12-16T23:20:36.1215606Z     /home/runner/.cargo/registry
2025-12-16T23:20:36.1216293Z     /home/runner/.cargo/git
2025-12-16T23:20:36.1216870Z     /home/runner/work/coda-api/coda-api/target
2025-12-16T23:20:36.1217471Z Restore Key:
2025-12-16T23:20:36.1218318Z     v0-rust-test-Linux-x64-961447e5
2025-12-16T23:20:36.1218868Z Cache Key:
2025-12-16T23:20:36.1219312Z     v0-rust-test-Linux-x64-961447e5-4ec2fb9c
2025-12-16T23:20:36.1219967Z .. Prefix:
2025-12-16T23:20:36.1220476Z   - v0-rust-test-Linux-x64
2025-12-16T23:20:36.1221070Z .. Environment considered:
2025-12-16T23:20:36.1221998Z   - Rust Version: 1.93.0-beta.2 x86_64-unknown-linux-gnu (eb937a3172395082904b966308c0cb5e7ac3c930)
2025-12-16T23:20:36.1222875Z   - CARGO_HOME
2025-12-16T23:20:36.1223344Z   - CARGO_INCREMENTAL
2025-12-16T23:20:36.1223991Z   - CARGO_TERM_COLOR
2025-12-16T23:20:36.1290211Z   - RUST_BACKTRACE
2025-12-16T23:20:36.1290673Z .. Lockfiles considered:
2025-12-16T23:20:36.1291138Z   - /home/runner/work/coda-api/coda-api/Cargo.lock
2025-12-16T23:20:36.1291696Z   - /home/runner/work/coda-api/coda-api/Cargo.toml
2025-12-16T23:20:36.1292538Z ##[endgroup]
2025-12-16T23:20:36.1292737Z 
2025-12-16T23:20:36.1292883Z ... Restoring cache ...
2025-12-16T23:20:36.2253842Z ##[warning]Cache not found for keys: v0-rust-test-Linux-x64-961447e5-4ec2fb9c, v0-rust-test-Linux-x64-961447e5
2025-12-16T23:20:36.2261334Z No cache found.
2025-12-16T23:20:36.2333292Z ##[group]Run lefthook run --force pre-commit
2025-12-16T23:20:36.2333652Z [36;1mlefthook run --force pre-commit[0m
2025-12-16T23:20:36.2366690Z shell: /usr/bin/bash -e {0}
2025-12-16T23:20:36.2366949Z env:
2025-12-16T23:20:36.2367131Z   RUST_BACKTRACE: 1
2025-12-16T23:20:36.2367486Z   GH_TOKEN: ***
2025-12-16T23:20:36.2367893Z   CARGO_HOME: /home/runner/.cargo
2025-12-16T23:20:36.2368147Z   CARGO_INCREMENTAL: 0
2025-12-16T23:20:36.2368390Z   CARGO_TERM_COLOR: always
2025-12-16T23:20:36.2368605Z   MISE_LOG_LEVEL: info
2025-12-16T23:20:36.2368891Z   GITHUB_TOKEN: ***
2025-12-16T23:20:36.2369178Z   MISE_TRUSTED_CONFIG_PATHS: /home/runner/work/coda-api/coda-api
2025-12-16T23:20:36.2369504Z   MISE_YES: 1
2025-12-16T23:20:36.2369688Z   CACHE_ON_FAILURE: false
2025-12-16T23:20:36.2369903Z ##[endgroup]
2025-12-16T23:20:36.2898681Z ╭──────────────────────────────────────╮
2025-12-16T23:20:36.2899352Z │ 🥊 lefthook v1.8.5  hook: pre-commit │
2025-12-16T23:20:36.2900007Z ╰──────────────────────────────────────╯
2025-12-16T23:20:36.2904892Z sync hooks: ✔️ (commit-msg, pre-commit)
2025-12-16T23:20:42.3589280Z ┃  gen:readme ❯ 
2025-12-16T23:20:42.3589569Z 
2025-12-16T23:20:42.3590047Z [0m[33m[gen:readme][0m [1m$ ./README.ts --output README.md[0m
2025-12-16T23:20:42.3590877Z [0m[32mDownload[0m https://deno.land/x/zod@v3.23.8/mod.ts
2025-12-16T23:20:42.3591626Z [0m[32mDownload[0m https://jsr.io/@std/assert/meta.json
2025-12-16T23:20:42.3592351Z [0m[32mDownload[0m https://jsr.io/@std/text/meta.json
2025-12-16T23:20:42.3593026Z [0m[32mDownload[0m https://jsr.io/@std/cli/meta.json
2025-12-16T23:20:42.3593726Z [0m[32mDownload[0m https://jsr.io/@std/toml/meta.json
2025-12-16T23:20:42.3594424Z [0m[32mDownload[0m https://registry.npmjs.org/zx
2025-12-16T23:20:42.3595092Z [0m[32mDownload[0m https://deno.land/x/zod@v3.23.8/index.ts
2025-12-16T23:20:42.3595821Z [0m[32mDownload[0m https://deno.land/x/zod@v3.23.8/external.ts
2025-12-16T23:20:42.3596628Z [0m[32mDownload[0m https://deno.land/x/zod@v3.23.8/errors.ts
2025-12-16T23:20:42.3597444Z [0m[32mDownload[0m https://deno.land/x/zod@v3.23.8/helpers/parseUtil.ts
2025-12-16T23:20:42.3598671Z [0m[32mDownload[0m https://deno.land/x/zod@v3.23.8/helpers/typeAliases.ts
2025-12-16T23:20:42.3599844Z [0m[32mDownload[0m https://deno.land/x/zod@v3.23.8/helpers/util.ts
2025-12-16T23:20:42.3600631Z [0m[32mDownload[0m https://deno.land/x/zod@v3.23.8/types.ts
2025-12-16T23:20:42.3601382Z [0m[32mDownload[0m https://deno.land/x/zod@v3.23.8/ZodError.ts
2025-12-16T23:20:42.3602185Z [0m[32mDownload[0m https://deno.land/x/zod@v3.23.8/locales/en.ts
2025-12-16T23:20:42.3603048Z [0m[32mDownload[0m https://deno.land/x/zod@v3.23.8/helpers/enumUtil.ts
2025-12-16T23:20:42.3603919Z [0m[32mDownload[0m https://deno.land/x/zod@v3.23.8/helpers/errorUtil.ts
2025-12-16T23:20:42.3604815Z [0m[32mDownload[0m https://deno.land/x/zod@v3.23.8/helpers/partialUtil.ts
2025-12-16T23:20:42.3605687Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0_meta.json
2025-12-16T23:20:42.3606460Z [0m[32mDownload[0m https://jsr.io/@std/text/1.0.10_meta.json
2025-12-16T23:20:42.3607194Z [0m[32mDownload[0m https://jsr.io/@std/cli/1.0.13_meta.json
2025-12-16T23:20:42.3608510Z [0m[32mDownload[0m https://jsr.io/@std/toml/1.0.5_meta.json
2025-12-16T23:20:42.3609278Z [0m[32mDownload[0m https://jsr.io/@std/cli/1.0.13/_data.json
2025-12-16T23:20:42.3610002Z [0m[32mDownload[0m https://jsr.io/@std/internal/meta.json
2025-12-16T23:20:42.3610755Z [0m[32mDownload[0m https://jsr.io/@std/collections/meta.json
2025-12-16T23:20:42.3611553Z [0m[32mDownload[0m https://jsr.io/@std/internal/1.0.12_meta.json
2025-12-16T23:20:42.3612381Z [0m[32mDownload[0m https://jsr.io/@std/collections/1.1.3_meta.json
2025-12-16T23:20:42.3613176Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/mod.ts
2025-12-16T23:20:42.3613913Z [0m[32mDownload[0m https://jsr.io/@std/text/1.0.10/mod.ts
2025-12-16T23:20:42.3614954Z [0m[32mDownload[0m https://jsr.io/@std/cli/1.0.13/mod.ts
2025-12-16T23:20:42.3615724Z [0m[32mDownload[0m https://jsr.io/@std/toml/1.0.5/mod.ts
2025-12-16T23:20:42.3616822Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/almost_equals.ts
2025-12-16T23:20:42.3618401Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/array_includes.ts
2025-12-16T23:20:42.3619691Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/equals.ts
2025-12-16T23:20:42.3620760Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/exists.ts
2025-12-16T23:20:42.3621837Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/false.ts
2025-12-16T23:20:42.3622817Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/greater_or_equal.ts
2025-12-16T23:20:42.3623846Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/greater.ts
2025-12-16T23:20:42.3624932Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/instance_of.ts
2025-12-16T23:20:42.3625999Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/is_error.ts
2025-12-16T23:20:42.3627048Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/less_or_equal.ts
2025-12-16T23:20:42.3628400Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/less.ts
2025-12-16T23:20:42.3629388Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/match.ts
2025-12-16T23:20:42.3630414Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/not_equals.ts
2025-12-16T23:20:42.3631509Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/not_instance_of.ts
2025-12-16T23:20:42.3632579Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/not_match.ts
2025-12-16T23:20:42.3633653Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/not_strict_equals.ts
2025-12-16T23:20:42.3634774Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/object_match.ts
2025-12-16T23:20:42.3635779Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/rejects.ts
2025-12-16T23:20:42.3636780Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/strict_equals.ts
2025-12-16T23:20:42.3638375Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/string_includes.ts
2025-12-16T23:20:42.3639473Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/throws.ts
2025-12-16T23:20:42.3640494Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/assert.ts
2025-12-16T23:20:42.3641862Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/assertion_error.ts
2025-12-16T23:20:42.3642857Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/equal.ts
2025-12-16T23:20:42.3643788Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/fail.ts
2025-12-16T23:20:42.3644784Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/unimplemented.ts
2025-12-16T23:20:42.3645856Z [0m[32mDownload[0m https://jsr.io/@std/assert/1.0.0/unreachable.ts
2025-12-16T23:20:42.3646996Z [0m[32mDownload[0m https://jsr.io/@std/text/1.0.10/levenshtein_distance.ts
2025-12-16T23:20:42.3648477Z [0m[32mDownload[0m https://jsr.io/@std/text/1.0.10/closest_string.ts
2025-12-16T23:20:42.3649720Z [0m[32mDownload[0m https://jsr.io/@std/text/1.0.10/compare_similarity.ts
2025-12-16T23:20:42.3650804Z [0m[32mDownload[0m https://jsr.io/@std/text/1.0.10/word_similarity_sort.ts
2025-12-16T23:20:42.3651935Z [0m[32mDownload[0m https://jsr.io/@std/text/1.0.10/to_camel_case.ts
2025-12-16T23:20:42.3652991Z [0m[32mDownload[0m https://jsr.io/@std/text/1.0.10/to_kebab_case.ts
2025-12-16T23:20:42.3654058Z [0m[32mDownload[0m https://jsr.io/@std/text/1.0.10/to_pascal_case.ts
2025-12-16T23:20:42.3655262Z [0m[32mDownload[0m https://jsr.io/@std/text/1.0.10/to_snake_case.ts
2025-12-16T23:20:42.3656237Z [0m[32mDownload[0m https://jsr.io/@std/cli/1.0.13/parse_args.ts
2025-12-16T23:20:42.3657294Z [0m[32mDownload[0m https://jsr.io/@std/cli/1.0.13/prompt_secret.ts
2025-12-16T23:20:42.3658557Z [0m[32mDownload[0m https://jsr.io/@std/cli/1.0.13/unicode_width.ts
2025-12-16T23:20:42.3659715Z [0m[32mDownload[0m https://jsr.io/@std/toml/1.0.5/stringify.ts
2025-12-16T23:20:42.3660627Z [0m[32mDownload[0m https://jsr.io/@std/toml/1.0.5/parse.ts
2025-12-16T23:20:42.3661888Z [0m[32mDownload[0m https://jsr.io/@std/text/1.0.10/_util.ts
2025-12-16T23:20:42.3662917Z [0m[32mDownload[0m https://jsr.io/@std/cli/1.0.13/_run_length.ts
2025-12-16T23:20:42.3663997Z [0m[32mDownload[0m https://jsr.io/@std/toml/1.0.5/_parser.ts
2025-12-16T23:20:42.3668427Z [0m[32mDownload[0m https://jsr.io/@std/internal/1.0.12/format.ts
2025-12-16T23:20:42.3670025Z [0m[32mDownload[0m https://jsr.io/@std/internal/1.0.12/mod.ts
2025-12-16T23:20:42.3670799Z [0m[32mDownload[0m https://jsr.io/@std/internal/1.0.12/styles.ts
2025-12-16T23:20:42.3671655Z [0m[32mDownload[0m https://jsr.io/@std/collections/1.1.3/deep_merge.ts
2025-12-16T23:20:42.3672563Z [0m[32mDownload[0m https://jsr.io/@std/internal/1.0.12/assertion_state.ts
2025-12-16T23:20:42.3673405Z [0m[32mDownload[0m https://jsr.io/@std/internal/1.0.12/build_message.ts
2025-12-16T23:20:42.3674223Z [0m[32mDownload[0m https://jsr.io/@std/internal/1.0.12/diff.ts
2025-12-16T23:20:42.3675030Z [0m[32mDownload[0m https://jsr.io/@std/internal/1.0.12/diff_str.ts
2025-12-16T23:20:42.3675850Z [0m[32mDownload[0m https://jsr.io/@std/internal/1.0.12/os.ts
2025-12-16T23:20:42.3676728Z [0m[32mDownload[0m https://jsr.io/@std/internal/1.0.12/truncate_build_message.ts
2025-12-16T23:20:42.3677610Z [0m[32mDownload[0m https://jsr.io/@std/internal/1.0.12/types.ts
2025-12-16T23:20:42.3678759Z [0m[32mDownload[0m https://jsr.io/@std/collections/1.1.3/_utils.ts
2025-12-16T23:20:42.3679637Z [0m[32mDownload[0m https://jsr.io/@std/internal/1.0.12/_os.ts
2025-12-16T23:20:42.3680489Z [0m[32mDownload[0m https://registry.npmjs.org/@types/fs-extra
2025-12-16T23:20:42.3681255Z [0m[32mDownload[0m https://registry.npmjs.org/@types/node
2025-12-16T23:20:42.3682056Z [0m[32mDownload[0m https://registry.npmjs.org/@types/jsonfile
2025-12-16T23:20:42.3682816Z [0m[32mDownload[0m https://registry.npmjs.org/undici-types
2025-12-16T23:20:42.3683634Z [0m[32mDownload[0m https://registry.npmjs.org/zx/-/zx-8.3.2.tgz
2025-12-16T23:20:42.3684574Z [0m[32mDownload[0m https://registry.npmjs.org/@types/fs-extra/-/fs-extra-11.0.4.tgz
2025-12-16T23:20:42.3685556Z [0m[32mDownload[0m https://registry.npmjs.org/@types/node/-/node-25.0.3.tgz
2025-12-16T23:20:42.3686875Z [0m[32mDownload[0m https://registry.npmjs.org/@types/jsonfile/-/jsonfile-6.1.4.tgz
2025-12-16T23:20:42.3688057Z [0m[32mDownload[0m https://registry.npmjs.org/@types/node/-/node-18.16.19.tgz
2025-12-16T23:20:42.3689105Z [0m[32mDownload[0m https://registry.npmjs.org/undici-types/-/undici-types-7.16.0.tgz
2025-12-16T23:20:42.3690017Z [1m[92m    Blocking[0m waiting for file lock on package cache
2025-12-16T23:20:42.3690778Z INFO  [cargo_doc2readme] Reading /home/runner/work/coda-api/coda-api/src/lib.rs
2025-12-16T23:20:42.3691749Z [33mWarning:[0m Glob use statements can lead to incomplete link generation.
2025-12-16T23:20:42.3692706Z        [38;5;246m╭[0m[38;5;246m─[0m[38;5;246m[[0mlib.rs:49535:14[38;5;246m][0m
2025-12-16T23:20:42.3693513Z        [38;5;246m│[0m
2025-12-16T23:20:42.3694807Z  [38;5;246m49535 │[0m [38;5;249mp[0m[38;5;249mu[0m[38;5;249mb[0m[38;5;249m [0m[38;5;249mu[0m[38;5;249ms[0m[38;5;249me[0m[38;5;249m [0m[38;5;249me[0m[38;5;249mx[0m[38;5;249mt[0m[38;5;249m:[0m[38;5;249m:[0m*[38;5;249m;[0m
2025-12-16T23:20:42.3695963Z  [38;5;240m      │[0m              ┬  
2025-12-16T23:20:42.3696853Z  [38;5;240m      │[0m              ╰── All items imported through this glob use will not be used for link generation
2025-12-16T23:20:42.3697835Z [38;5;246m───────╯[0m
2025-12-16T23:20:42.3698520Z [33mWarning:[0m Glob use statements can lead to incomplete link generation.
2025-12-16T23:20:42.3699386Z        [38;5;246m╭[0m[38;5;246m─[0m[38;5;246m[[0mlib.rs:49539:17[38;5;246m][0m
2025-12-16T23:20:42.3699982Z        [38;5;246m│[0m
2025-12-16T23:20:42.3701644Z  [38;5;246m49539 │[0m [38;5;249mp[0m[38;5;249mu[0m[38;5;249mb[0m[38;5;249m [0m[38;5;249mu[0m[38;5;249ms[0m[38;5;249me[0m[38;5;249m [0m[38;5;249mc[0m[38;5;249ml[0m[38;5;249mi[0m[38;5;249me[0m[38;5;249mn[0m[38;5;249mt[0m[38;5;249m:[0m[38;5;249m:[0m*[38;5;249m;[0m
2025-12-16T23:20:42.3702929Z  [38;5;240m      │[0m                 ┬  
2025-12-16T23:20:42.3703829Z  [38;5;240m      │[0m                 ╰── All items imported through this glob use will not be used for link generation
2025-12-16T23:20:42.3704625Z [38;5;246m───────╯[0m
2025-12-16T23:20:42.3705332Z [33mWarning:[0m Glob use statements can lead to incomplete link generation.
2025-12-16T23:20:42.3706193Z        [38;5;246m╭[0m[38;5;246m─[0m[38;5;246m[[0mlib.rs:49543:18[38;5;246m][0m
2025-12-16T23:20:42.3706777Z        [38;5;246m│[0m
2025-12-16T23:20:42.3749809Z  [38;5;246m49543 │[0m [38;5;249mp[0m[38;5;249mu[0m[38;5;249mb[0m[38;5;249m [0m[38;5;249mu[0m[38;5;249ms[0m[38;5;249me[0m[38;5;249m [0m[38;5;249ml[0m[38;5;249mi[0m[38;5;249mm[0m[38;5;249mi[0m[38;5;249mt[0m[38;5;249me[0m[38;5;249mr[0m[38;5;249m:[0m[38;5;249m:[0m*[38;5;249m;[0m
2025-12-16T23:20:42.3751297Z  [38;5;240m      │[0m                  ┬  
2025-12-16T23:20:42.3752211Z  [38;5;240m      │[0m                  ╰── All items imported through this glob use will not be used for link generation
2025-12-16T23:20:42.3757319Z [38;5;246m───────╯[0m
2025-12-16T23:20:42.3757989Z INFO  [cargo_doc2readme] Writing README to stdout
2025-12-16T23:20:42.3758355Z 
2025-12-16T23:20:42.3758664Z [1;32mSuccess:[0m No issues found in 1 file (0ms)
2025-12-16T23:20:42.3759026Z 
2025-12-16T23:20:48.6875325Z ┃  test ❯ 
2025-12-16T23:20:48.6875769Z 
2025-12-16T23:20:48.6876299Z [0m[31m[test:code][0m [1m$ cargo nextest run --all-features --no-tests warn[0m
2025-12-16T23:20:48.6878447Z [0m[35m[test:docs][0m [1m$ cargo test --doc --all-features --no-fail-fast --quiet[0m
2025-12-16T23:20:48.6879575Z [0m[31m[test:code][0m [1m[92m    Blocking[0m waiting for file lock on package cache
2025-12-16T23:20:48.6880593Z [0m[31m[test:code][0m [1m[92m    Blocking[0m waiting for file lock on package cache
2025-12-16T23:20:48.6881660Z [0m[31m[test:code][0m [1m[92m    Blocking[0m waiting for file lock on package cache
2025-12-16T23:20:48.6882685Z [0m[31m[test:code][0m [1m[92m    Blocking[0m waiting for file lock on package cache
2025-12-16T23:20:48.6884089Z [0m[31m[test:code][0m [1m[92m    Blocking[0m waiting for file lock on artifact directory
2025-12-16T23:20:48.6885165Z [0m[35m[test:docs][0m [1m[91merror[0m[1m: linking with `cc` failed: exit status: 1[0m
2025-12-16T23:20:48.6886047Z [0m[35m[test:docs][0m   [1m[94m|[0m
2025-12-16T23:20:48.6892341Z [0m[35m[test:docs][0m   [1m[94m= [0m[1mnote[0m:  "cc" "-m64" "/tmp/rustcnUpYXj/symbols.o" "<5 object files omitted>" "-Wl,--as-needed" "-Wl,-Bstatic" "<sysroot>/lib/rustlib/x86_64-unknown-linux-gnu/lib/{libstd-*,libpanic_unwind-*,libobject-*,libmemchr-*,libaddr2line-*,libgimli-*,libcfg_if-*,librustc_demangle-*,libstd_detect-*,libhashbrown-*,librustc_std_workspace_alloc-*,libminiz_oxide-*,libadler2-*,libunwind-*,liblibc-*,librustc_std_workspace_core-*,liballoc-*,libcore-*,libcompiler_builtins-*}.rlib" "-Wl,-Bdynamic" "-lgcc_s" "-lutil" "-lrt" "-lpthread" "-lm" "-ldl" "-lc" "-L" "/tmp/rustcnUpYXj/raw-dylibs" "-B<sysroot>/lib/rustlib/x86_64-unknown-linux-gnu/bin/gcc-ld" "-fuse-ld=lld" "-Wl,--eh-frame-hdr" "-Wl,-z,noexecstack" "-L" "<sysroot>/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-o" "/home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74" "-Wl,--gc-sections" "-pie" "-Wl,-z,relro,-z,now" "-nodefaultlibs"
2025-12-16T23:20:48.6898543Z [0m[35m[test:docs][0m   [1m[94m= [0m[1mnote[0m: some arguments are omitted. use `--verbose` to show all linker arguments
2025-12-16T23:20:48.6914177Z [0m[35m[test:docs][0m   [1m[94m= [0m[1mnote[0m: rust-lld: error: undefined hidden symbol: core::panicking::assert_failed::h1eb0b3dc7fadff52
2025-12-16T23:20:48.6915507Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.0
2025-12-16T23:20:48.6918342Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.0.rcgu.o:(build_script_build::rustc_minor_nightly::h1bdcac2e3d3d54ea)
2025-12-16T23:20:48.6920141Z [0m[35m[test:docs][0m           
2025-12-16T23:20:48.6921296Z [0m[35m[test:docs][0m           rust-lld: error: undefined hidden symbol: alloc::slice::_$LT$impl$u20$$u5b$T$u5d$$GT$::join::hf93174dedb652f21
2025-12-16T23:20:48.6922479Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.0
2025-12-16T23:20:48.6924757Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.0.rcgu.o:(build_script_build::main::hed94ce6634b641fd)
2025-12-16T23:20:48.6926356Z [0m[35m[test:docs][0m           
2025-12-16T23:20:48.6927851Z [0m[35m[test:docs][0m           rust-lld: error: undefined hidden symbol: _$LT$core..num..error..ParseIntError$u20$as$u20$core..fmt..Debug$GT$::fmt::he1dd3d96b15d527d
2025-12-16T23:20:48.6929158Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.0
2025-12-16T23:20:48.6931397Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.0.rcgu.o:(.data.rel.ro..Lanon.d584bdb0548f9f4c27800516ec8f04be.129+0x18)
2025-12-16T23:20:48.6933154Z [0m[35m[test:docs][0m           
2025-12-16T23:20:48.6934283Z [0m[35m[test:docs][0m           rust-lld: error: undefined hidden symbol: core::str::validations::next_code_point_reverse::h8dc2f8f3a581251c
2025-12-16T23:20:48.6935479Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.1
2025-12-16T23:20:48.6938577Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.1.rcgu.o:(_$LT$core..str..iter..CharIndices$u20$as$u20$core..iter..traits..double_ended..DoubleEndedIterator$GT$::next_back::hb7dfb87a63283389)
2025-12-16T23:20:48.6941009Z [0m[35m[test:docs][0m           
2025-12-16T23:20:48.6942255Z [0m[35m[test:docs][0m           rust-lld: error: undefined hidden symbol: core::char::convert::from_u32_unchecked::precondition_check::hbe83fccbf70964df
2025-12-16T23:20:48.6943527Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.1
2025-12-16T23:20:48.6946432Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.1.rcgu.o:(_$LT$core..str..iter..CharIndices$u20$as$u20$core..iter..traits..double_ended..DoubleEndedIterator$GT$::next_back::hb7dfb87a63283389)
2025-12-16T23:20:48.6989174Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.1
2025-12-16T23:20:48.6991927Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.1.rcgu.o:(_$LT$core..str..iter..CharIndices$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next::haa4ec5d98f5e67bc)
2025-12-16T23:20:48.6993965Z [0m[35m[test:docs][0m           
2025-12-16T23:20:48.6995177Z [0m[35m[test:docs][0m           rust-lld: error: undefined hidden symbol: core::ptr::non_null::NonNull$LT$T$GT$::offset_from_unsigned::h07afc0ef6ad18ad3
2025-12-16T23:20:48.6996407Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.1
2025-12-16T23:20:48.6999623Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.1.rcgu.o:(_$LT$core..str..iter..CharIndices$u20$as$u20$core..iter..traits..double_ended..DoubleEndedIterator$GT$::next_back::hb7dfb87a63283389)
2025-12-16T23:20:48.7001870Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.1
2025-12-16T23:20:48.7004648Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.1.rcgu.o:(_$LT$core..str..pattern..MultiCharEqSearcher$LT$C$GT$$u20$as$u20$core..str..pattern..ReverseSearcher$GT$::next_back::h49374fc71a72b364)
2025-12-16T23:20:48.7006954Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.1
2025-12-16T23:20:48.7009966Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.1.rcgu.o:(_$LT$core..str..pattern..MultiCharEqSearcher$LT$C$GT$$u20$as$u20$core..str..pattern..ReverseSearcher$GT$::next_back::h49374fc71a72b364)
2025-12-16T23:20:48.7012085Z [0m[35m[test:docs][0m           >>> referenced 6 more times
2025-12-16T23:20:48.7012670Z [0m[35m[test:docs][0m           
2025-12-16T23:20:48.7014943Z [0m[35m[test:docs][0m           rust-lld: error: undefined hidden symbol: alloc::collections::btree::node::Handle$LT$alloc..collections..btree..node..NodeRef$LT$alloc..collections..btree..node..marker..Dying$C$K$C$V$C$NodeType$GT$$C$alloc..collections..btree..node..marker..KV$GT$::drop_key_val::h0eb40ff42ee7f491
2025-12-16T23:20:48.7017082Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.1
2025-12-16T23:20:48.7020447Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.1.rcgu.o:(_$LT$$LT$alloc..collections..btree..map..IntoIter$LT$K$C$V$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$K$C$V$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$::drop::he7cc59f492babb0f)
2025-12-16T23:20:48.7023206Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.1
2025-12-16T23:20:48.7025829Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.1.rcgu.o:(_$LT$alloc..collections..btree..map..IntoIter$LT$K$C$V$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$::drop::hdc89ba6b50778424)
2025-12-16T23:20:48.7027896Z [0m[35m[test:docs][0m           
2025-12-16T23:20:48.7029686Z [0m[35m[test:docs][0m           rust-lld: error: undefined hidden symbol: core::str::traits::_$LT$impl$u20$core..slice..index..SliceIndex$LT$str$GT$$u20$for$u20$core..ops..range..Range$LT$usize$GT$$GT$::get_unchecked::precondition_check::hd36002896fb75411
2025-12-16T23:20:48.7031368Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.1
2025-12-16T23:20:48.7033674Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.1.rcgu.o:(core::str::iter::SplitInternal$LT$P$GT$::next::h94688f1f61a8efd1)
2025-12-16T23:20:48.7035633Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.1
2025-12-16T23:20:48.7038137Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.1.rcgu.o:(core::str::iter::SplitInternal$LT$P$GT$::next::he66ea60e5683ce82)
2025-12-16T23:20:48.7040369Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.1
2025-12-16T23:20:48.7042771Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.1.rcgu.o:(core::str::iter::SplitInternal$LT$P$GT$::get_end::hd65437bf8634df85)
2025-12-16T23:20:48.7044628Z [0m[35m[test:docs][0m           >>> referenced 2 more times
2025-12-16T23:20:48.7045211Z [0m[35m[test:docs][0m           
2025-12-16T23:20:48.7046468Z [0m[35m[test:docs][0m           rust-lld: error: undefined hidden symbol: _$LT$alloc..alloc..Global$u20$as$u20$core..clone..Clone$GT$::clone::h8b0983a69a5af733
2025-12-16T23:20:48.7047910Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.1
2025-12-16T23:20:48.7050381Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.1.rcgu.o:(alloc::collections::btree::map::IntoIter$LT$K$C$V$C$A$GT$::dying_next::h7374ff428d667c4a)
2025-12-16T23:20:48.7052484Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.1
2025-12-16T23:20:48.7054968Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.1.rcgu.o:(alloc::collections::btree::map::IntoIter$LT$K$C$V$C$A$GT$::dying_next::h7374ff428d667c4a)
2025-12-16T23:20:48.7056804Z [0m[35m[test:docs][0m           
2025-12-16T23:20:48.7058663Z [0m[35m[test:docs][0m           rust-lld: error: undefined hidden symbol: alloc::collections::btree::navigate::LazyLeafRange$LT$alloc..collections..btree..node..marker..Dying$C$K$C$V$GT$::deallocating_end::h1e60a35e95693299
2025-12-16T23:20:48.7060326Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.1
2025-12-16T23:20:48.7062793Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.1.rcgu.o:(alloc::collections::btree::map::IntoIter$LT$K$C$V$C$A$GT$::dying_next::h7374ff428d667c4a)
2025-12-16T23:20:48.7064879Z [0m[35m[test:docs][0m           
2025-12-16T23:20:48.7066604Z [0m[35m[test:docs][0m           rust-lld: error: undefined hidden symbol: alloc::collections::btree::navigate::LazyLeafRange$LT$alloc..collections..btree..node..marker..Dying$C$K$C$V$GT$::deallocating_next_unchecked::h67cecb81f6afe503
2025-12-16T23:20:48.7068542Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.1
2025-12-16T23:20:48.7071094Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.1.rcgu.o:(alloc::collections::btree::map::IntoIter$LT$K$C$V$C$A$GT$::dying_next::h7374ff428d667c4a)
2025-12-16T23:20:48.7072940Z [0m[35m[test:docs][0m           
2025-12-16T23:20:48.7074239Z [0m[35m[test:docs][0m           rust-lld: error: undefined hidden symbol: _$LT$char$u20$as$u20$core..slice..cmp..SliceContains$GT$::slice_contains::h22329cd07a61aaaf
2025-12-16T23:20:48.7075497Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.1
2025-12-16T23:20:48.7078192Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.1.rcgu.o:(_$LT$$u5b$char$u3b$$u20$N$u5d$$u20$as$u20$core..str..pattern..MultiCharEq$GT$::matches::h011e48363bda7349)
2025-12-16T23:20:48.7080010Z [0m[35m[test:docs][0m           
2025-12-16T23:20:48.7081768Z [0m[35m[test:docs][0m           rust-lld: error: undefined hidden symbol: _$LT$usize$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$::get_unchecked::precondition_check::hbb4f034f71f377cf
2025-12-16T23:20:48.7083293Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.1
2025-12-16T23:20:48.7086042Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.1.rcgu.o:(_$LT$core..str..pattern..CharSearcher$u20$as$u20$core..str..pattern..Searcher$GT$::next_match::he3a0711944700d15)
2025-12-16T23:20:48.7088231Z [0m[35m[test:docs][0m           
2025-12-16T23:20:48.7089343Z [0m[35m[test:docs][0m           rust-lld: error: undefined hidden symbol: core::str::validations::next_code_point::h85f541b9f3bf2202
2025-12-16T23:20:48.7090525Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.1
2025-12-16T23:20:48.7093295Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.1.rcgu.o:(_$LT$core..str..iter..CharIndices$u20$as$u20$core..iter..traits..iterator..Iterator$GT$::next::haa4ec5d98f5e67bc)
2025-12-16T23:20:48.7095326Z [0m[35m[test:docs][0m           
2025-12-16T23:20:48.7096496Z [0m[35m[test:docs][0m           rust-lld: error: undefined hidden symbol: std::sys::backtrace::__rust_begin_short_backtrace::h842e129c3163c6f9
2025-12-16T23:20:48.7097949Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.3
2025-12-16T23:20:48.7100380Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.3.rcgu.o:(std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h62b7db8c13bda56f)
2025-12-16T23:20:48.7102152Z [0m[35m[test:docs][0m           
2025-12-16T23:20:48.7103479Z [0m[35m[test:docs][0m           rust-lld: error: undefined hidden symbol: _$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$::drop::h9756950768d0e976
2025-12-16T23:20:48.7105075Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.3
2025-12-16T23:20:48.7108174Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.3.rcgu.o:(core::ptr::drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$core..error..Error$u2b$core..marker..Sync$u2b$core..marker..Send$GT$$GT$::h5aff21021d44d078)
2025-12-16T23:20:48.7110461Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.3
2025-12-16T23:20:48.7113303Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.3.rcgu.o:(core::ptr::drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$core..error..Error$u2b$core..marker..Sync$u2b$core..marker..Send$GT$$GT$::h5aff21021d44d078)
2025-12-16T23:20:48.7115401Z [0m[35m[test:docs][0m           
2025-12-16T23:20:48.7116665Z [0m[35m[test:docs][0m           rust-lld: error: undefined hidden symbol: _$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$::drop::h444d4a3918bcddad
2025-12-16T23:20:48.7118435Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.3
2025-12-16T23:20:48.7122008Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.3.rcgu.o:(core::ptr::drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$core..ops..function..FnMut$LT$$LP$$RP$$GT$$u2b$Output$u20$$u3d$$u20$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$u2b$core..marker..Sync$u2b$core..marker..Send$GT$$GT$::h9fd6ad8d2a5a72a7)
2025-12-16T23:20:48.7124944Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.3
2025-12-16T23:20:48.7128555Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.3.rcgu.o:(core::ptr::drop_in_place$LT$alloc..boxed..Box$LT$dyn$u20$core..ops..function..FnMut$LT$$LP$$RP$$GT$$u2b$Output$u20$$u3d$$u20$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$u2b$core..marker..Sync$u2b$core..marker..Send$GT$$GT$::h9fd6ad8d2a5a72a7)
2025-12-16T23:20:48.7131137Z [0m[35m[test:docs][0m           
2025-12-16T23:20:48.7132512Z [0m[35m[test:docs][0m           rust-lld: error: undefined hidden symbol: _$LT$alloc..raw_vec..RawVec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$::drop::hc6b83e612eba7e06
2025-12-16T23:20:48.7133845Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.3
2025-12-16T23:20:48.7137410Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.3.rcgu.o:(core::ptr::drop_in_place$LT$alloc..raw_vec..RawVec$LT$alloc..boxed..Box$LT$dyn$u20$core..ops..function..FnMut$LT$$LP$$RP$$GT$$u2b$Output$u20$$u3d$$u20$core..result..Result$LT$$LP$$RP$$C$std..io..error..Error$GT$$u2b$core..marker..Sync$u2b$core..marker..Send$GT$$GT$$GT$::h4eea0c1fdd5a0355)
2025-12-16T23:20:48.7140341Z [0m[35m[test:docs][0m           
2025-12-16T23:20:48.7141671Z [0m[35m[test:docs][0m           rust-lld: error: undefined hidden symbol: _$LT$std..os..fd..owned..OwnedFd$u20$as$u20$core..ops..drop..Drop$GT$::drop::h26f87972e70dec88
2025-12-16T23:20:48.7142992Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.3
2025-12-16T23:20:48.7145405Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.3.rcgu.o:(core::ptr::drop_in_place$LT$std..os..fd..owned..OwnedFd$GT$::hc394c1137d1482af)
2025-12-16T23:20:48.7147480Z [0m[35m[test:docs][0m           
2025-12-16T23:20:48.7148926Z [0m[35m[test:docs][0m           rust-lld: error: undefined hidden symbol: _$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$::drop::h2e75e268ff52b23c
2025-12-16T23:20:48.7150167Z [0m[35m[test:docs][0m           >>> referenced by build_script_build.b5e3ef27496fc655-cgu.3
2025-12-16T23:20:48.7152525Z [0m[35m[test:docs][0m           >>>               /home/runner/work/coda-api/coda-api/target/debug/build/libc-5c16d4d39f2a8a74/build_script_build-5c16d4d39f2a8a74.build_script_build.b5e3ef27496fc655-cgu.3.rcgu.o:(core::ptr::drop_in_place$LT$alloc..boxed..Box$LT$$u5b$u8$u5d$$GT$$GT$::h333164d64f145c37)
2025-12-16T23:20:48.7154176Z [0m[35m[test:docs][0m           
2025-12-16T23:20:48.7155059Z [0m[35m[test:docs][0m           rust-lld: error: too many errors emitted, stopping now (use --error-limit=0 to see all errors)
2025-12-16T23:20:48.7156067Z [0m[35m[test:docs][0m           collect2: error: ld returned 1 exit status
2025-12-16T23:20:48.7156665Z [0m[35m[test:docs][0m           
2025-12-16T23:20:48.7157085Z [0m[35m[test:docs][0m 
2025-12-16T23:20:48.7158004Z [0m[35m[test:docs][0m [1m[91merror[0m: could not compile `libc` (build script) due to 1 previous error
2025-12-16T23:20:48.7158932Z [0m[31m[test:code][0m [1m[92m   Compiling[0m proc-macro2 v1.0.101
2025-12-16T23:20:48.7159650Z [0m[31m[test:code][0m [1m[92m   Compiling[0m libc v0.2.174
2025-12-16T23:20:48.7160364Z [0m[31m[test:code][0m [1m[92m   Compiling[0m futures-core v0.3.31
2025-12-16T23:20:48.7161280Z [0m[31m[test:code][0m [1m[92m   Compiling[0m bitflags v2.9.1
2025-12-16T23:20:48.7162003Z [0m[31m[test:code][0m [1m[92m   Compiling[0m cc v1.2.29
2025-12-16T23:20:48.7162713Z [0m[31m[test:code][0m [1m[92m   Compiling[0m itoa v1.0.15
2025-12-16T23:20:48.7163443Z [0m[31m[test:code][0m [1m[92m   Compiling[0m pkg-config v0.3.32
2025-12-16T23:20:48.7164142Z [0m[31m[test:code][0m [1m[92m   Compiling[0m quote v1.0.40
2025-12-16T23:20:48.7164792Z [0m[31m[test:code][0m [1m[92m   Compiling[0m syn v2.0.106
2025-12-16T23:20:48.7166539Z [0m[31m[test:code][0m [1m[91merror[0m[1m: failed to build archive at `/home/runner/work/coda-api/coda-api/target/debug/deps/libpkg_config-62b833e36315fb7b.rlib`: failed to map object file: memory map must have a non-zero length[0m
2025-12-16T23:20:48.7168183Z [0m[31m[test:code][0m 
2025-12-16T23:20:48.7168942Z [0m[31m[test:code][0m [1m[91merror[0m: could not compile `pkg-config` (lib) due to 1 previous error
2025-12-16T23:20:48.7170189Z [0m[31m[test:code][0m [1m[33mwarning[0m: build failed, waiting for other jobs to finish...
2025-12-16T23:20:48.7172043Z [0m[31m[test:code][0m [1m[91merror[0m[1m: failed to build archive at `/home/runner/work/coda-api/coda-api/target/debug/deps/libcc-0013882728014204.rlib`: failed to map object file: memory map must have a non-zero length[0m
2025-12-16T23:20:48.7173423Z [0m[31m[test:code][0m 
2025-12-16T23:20:48.7174130Z [0m[31m[test:code][0m [1m[91merror[0m: could not compile `cc` (lib) due to 1 previous error
2025-12-16T23:20:48.7176001Z [0m[31m[test:code][0m [1m[91merror[0m[1m: failed to build archive at `/home/runner/work/coda-api/coda-api/target/debug/deps/libsyn-4eab108667290e6e.rlib`: failed to map object file: memory map must have a non-zero length[0m
2025-12-16T23:20:48.7177420Z [0m[31m[test:code][0m 
2025-12-16T23:20:48.7178328Z [0m[31m[test:code][0m [1m[91merror[0m: could not compile `syn` (lib) due to 1 previous error
2025-12-16T23:20:48.7180254Z [0m[31m[test:code][0m [31;1merror[0m: command `[1m/home/runner/.rustup/toolchains/beta-x86_64-unknown-linux-gnu/bin/cargo test --no-run --message-format json-render-diagnostics --all-features[0m` exited with code [1m101[0m
2025-12-16T23:20:48.7181922Z [2mFinished in 12.35s[0m
2025-12-16T23:20:48.7182422Z [0m[35m[test:docs][0m [31mERROR[0m task failed
2025-12-16T23:20:48.7182766Z 
2025-12-16T23:21:30.7428688Z exit status 101┃  fix ❯ 
2025-12-16T23:21:30.7429007Z 
2025-12-16T23:21:30.7430219Z [0m[33m[fix:code][0m [1m$ mise run fix:code:warnings
2025-12-16T23:21:30.7430771Z mise run fix:code:style[0m
2025-12-16T23:21:30.7431275Z [0m[32m[fix:deps][0m [1m$ mise run fix:deps:usage
2025-12-16T23:21:30.7431852Z mise run fix:deps:order[0m
2025-12-16T23:21:30.7432296Z [0m[34m[fix:docs][0m [1m$ rumdl fmt[0m
2025-12-16T23:21:30.7433036Z [0m[32m[fix:deps][0m [0m[34m[fix:deps:usage][0m [1m$ cargo machete --with-metadata --fix[0m
2025-12-16T23:21:30.7434274Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m$ cargo clippy --all-targets --all-features --fix --allow-dirty --allow-staged[0m
2025-12-16T23:21:30.7435114Z [0m[34m[fix:docs][0m 
2025-12-16T23:21:30.7435647Z [0m[34m[fix:docs][0m Success: No issues found in 5 files (80ms)
2025-12-16T23:21:30.7436114Z [0m[34m[fix:docs][0m Finished in 98.9ms
2025-12-16T23:21:30.7436777Z [0m[32m[fix:deps][0m [0m[34m[fix:deps:usage][0m Analyzing dependencies of crates in this directory...
2025-12-16T23:21:30.7438014Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Blocking[0m waiting for file lock on package cache
2025-12-16T23:21:30.7438866Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Blocking[0m waiting for file lock on package cache
2025-12-16T23:21:30.7439610Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Blocking[0m waiting for file lock on package cache
2025-12-16T23:21:30.7440647Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Blocking[0m waiting for file lock on package cache
2025-12-16T23:21:30.7441756Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m proc-macro2 v1.0.101
2025-12-16T23:21:30.7442483Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m unicode-ident v1.0.18
2025-12-16T23:21:30.7443152Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m libc v0.2.174
2025-12-16T23:21:30.7443732Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m cfg-if v1.0.0
2025-12-16T23:21:30.7444313Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m stable_deref_trait v1.2.0
2025-12-16T23:21:30.7444874Z [0m[32m[fix:deps][0m [0m[34m[fix:deps:usage][0m Done!
2025-12-16T23:21:30.7445536Z [0m[32m[fix:deps][0m [0m[34m[fix:deps:usage][0m cargo-machete didn't find any unused dependencies in this directory. Good job!
2025-12-16T23:21:30.7446289Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m pin-project-lite v0.2.16
2025-12-16T23:21:30.7446870Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m once_cell v1.21.3
2025-12-16T23:21:30.7447422Z [0m[32m[fix:deps][0m [0m[35m[fix:deps:order][0m [1m$ cargo sort[0m
2025-12-16T23:21:30.7448150Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m bitflags v2.9.1
2025-12-16T23:21:30.7448720Z [0m[32m[fix:deps][0m [0m[35m[fix:deps:order][0m Checking coda-api...
2025-12-16T23:21:30.7449239Z [0m[32m[fix:deps][0m [0m[35m[fix:deps:order][0m Finished: Cargo.toml for "coda-api" has been rewritten
2025-12-16T23:21:30.7449770Z [0m[32m[fix:deps][0m Finished in 4.58s
2025-12-16T23:21:30.7450240Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m bytes v1.10.1
2025-12-16T23:21:30.7450810Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m smallvec v1.15.1
2025-12-16T23:21:30.7451463Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m futures-core v0.3.31
2025-12-16T23:21:30.7452306Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m shlex v1.3.0
2025-12-16T23:21:30.7453266Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m cc v1.2.29
2025-12-16T23:21:30.7453899Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m vcpkg v0.2.15
2025-12-16T23:21:30.7454516Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m quote v1.0.40
2025-12-16T23:21:30.7455023Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m syn v2.0.106
2025-12-16T23:21:30.7455630Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m itoa v1.0.15
2025-12-16T23:21:30.7456505Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m pkg-config v0.3.32
2025-12-16T23:21:30.7457098Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m mio v1.0.4
2025-12-16T23:21:30.7458002Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m socket2 v0.5.10
2025-12-16T23:21:30.7458702Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m futures-sink v0.3.31
2025-12-16T23:21:30.7459253Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m fnv v1.0.7
2025-12-16T23:21:30.7459890Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m serde_core v1.0.227
2025-12-16T23:21:30.7460527Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m getrandom v0.3.3
2025-12-16T23:21:30.7461145Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m futures-task v0.3.31
2025-12-16T23:21:30.7461692Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m pin-utils v0.1.0
2025-12-16T23:21:30.7462477Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m writeable v0.6.1
2025-12-16T23:21:30.7463126Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m slab v0.4.10
2025-12-16T23:21:30.7463754Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m litemap v0.8.0
2025-12-16T23:21:30.7464352Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m openssl-sys v0.9.109
2025-12-16T23:21:30.7464935Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m futures-util v0.3.31
2025-12-16T23:21:30.7465565Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m http v1.3.1
2025-12-16T23:21:30.7466236Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m icu_normalizer_data v2.0.0
2025-12-16T23:21:30.7467038Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m icu_properties_data v2.0.1
2025-12-16T23:21:30.7468477Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m http-body v1.0.1
2025-12-16T23:21:30.7469632Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m httparse v1.10.1
2025-12-16T23:21:30.7470653Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m foreign-types-shared v0.1.1
2025-12-16T23:21:30.7471667Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m percent-encoding v2.3.2
2025-12-16T23:21:30.7472617Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m serde v1.0.227
2025-12-16T23:21:30.7473512Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m openssl v0.10.73
2025-12-16T23:21:30.7474459Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m foreign-types v0.3.2
2025-12-16T23:21:30.7475142Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m try-lock v0.2.5
2025-12-16T23:21:30.7475676Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m native-tls v0.2.14
2025-12-16T23:21:30.7476204Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m zerocopy v0.8.28
2025-12-16T23:21:30.7476937Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m synstructure v0.13.2
2025-12-16T23:21:30.7477478Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m tower-service v0.3.3
2025-12-16T23:21:30.7478313Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m want v0.3.1
2025-12-16T23:21:30.7479272Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m futures-channel v0.3.31
2025-12-16T23:21:30.7480318Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m tracing-core v0.1.33
2025-12-16T23:21:30.7481376Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m parking_lot_core v0.9.12
2025-12-16T23:21:30.7481962Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m openssl-probe v0.1.6
2025-12-16T23:21:30.7482617Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m scopeguard v1.2.0
2025-12-16T23:21:30.7483210Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m crossbeam-utils v0.8.21
2025-12-16T23:21:30.7483739Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m log v0.4.27
2025-12-16T23:21:30.7484247Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m tracing v0.1.41
2025-12-16T23:21:30.7484764Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m lock_api v0.4.14
2025-12-16T23:21:30.7485301Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m form_urlencoded v1.2.1
2025-12-16T23:21:30.7486020Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m sync_wrapper v1.0.2
2025-12-16T23:21:30.7486550Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m rustix v1.0.8
2025-12-16T23:21:30.7487060Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m ryu v1.0.20
2025-12-16T23:21:30.7487564Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m utf8_iter v1.0.4
2025-12-16T23:21:30.7488312Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m ipnet v2.11.0
2025-12-16T23:21:30.7488910Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m serde_json v1.0.145
2025-12-16T23:21:30.7489895Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m allocator-api2 v0.2.21
2025-12-16T23:21:30.7490850Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m equivalent v1.0.2
2025-12-16T23:21:30.7491777Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m memchr v2.7.4
2025-12-16T23:21:30.7492674Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m base64 v0.22.1
2025-12-16T23:21:30.7493582Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m autocfg v1.5.0
2025-12-16T23:21:30.7494530Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m zerofrom-derive v0.1.6
2025-12-16T23:21:30.7495497Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m yoke-derive v0.8.0
2025-12-16T23:21:30.7496463Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m zerovec-derive v0.11.1
2025-12-16T23:21:30.7497420Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m displaydoc v0.2.5
2025-12-16T23:21:30.7498557Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m tokio-macros v2.5.0
2025-12-16T23:21:30.7499530Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m serde_derive v1.0.227
2025-12-16T23:21:30.7500473Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m zerofrom v0.1.6
2025-12-16T23:21:30.7501581Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m openssl-macros v0.1.1
2025-12-16T23:21:30.7502480Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m yoke v0.8.0
2025-12-16T23:21:30.7503308Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m tokio v1.46.1
2025-12-16T23:21:30.7504159Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m zerovec v0.11.2
2025-12-16T23:21:30.7505038Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m zerotrie v0.2.2
2025-12-16T23:21:30.7505974Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m tinystr v0.8.1
2025-12-16T23:21:30.7506936Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m icu_locale_core v2.0.0
2025-12-16T23:21:30.7508062Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m potential_utf v0.1.2
2025-12-16T23:21:30.7509055Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m icu_collections v2.0.0
2025-12-16T23:21:30.7509975Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m icu_provider v2.0.0
2025-12-16T23:21:30.7510947Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m icu_properties v2.0.1
2025-12-16T23:21:30.7511923Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m icu_normalizer v2.0.0
2025-12-16T23:21:30.7512813Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m tower-layer v0.3.3
2025-12-16T23:21:30.7513753Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m ppv-lite86 v0.2.21
2025-12-16T23:21:30.7514838Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m idna_adapter v1.2.1
2025-12-16T23:21:30.7515731Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m idna v1.0.3
2025-12-16T23:21:30.7516416Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m num-traits v0.2.19
2025-12-16T23:21:30.7516934Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m hyper v1.6.0
2025-12-16T23:21:30.7517428Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m tower v0.5.2
2025-12-16T23:21:30.7518178Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m tokio-native-tls v0.3.1
2025-12-16T23:21:30.7518742Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m rand_core v0.9.3
2025-12-16T23:21:30.7519723Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m http-body-util v0.1.3
2025-12-16T23:21:30.7520655Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m thiserror v2.0.17
2025-12-16T23:21:30.7521359Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m utf8parse v0.2.2
2025-12-16T23:21:30.7521897Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m iri-string v0.7.8
2025-12-16T23:21:30.7522417Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m zeroize v1.8.1
2025-12-16T23:21:30.7522941Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m hyper-util v0.1.15
2025-12-16T23:21:30.7523477Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m linux-raw-sys v0.9.4
2025-12-16T23:21:30.7524045Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m portable-atomic v1.11.1
2025-12-16T23:21:30.7524635Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m rustls-pki-types v1.12.0
2025-12-16T23:21:30.7525193Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m anstyle-parse v0.2.7
2025-12-16T23:21:30.7525880Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m rand_chacha v0.9.0
2025-12-16T23:21:30.7526405Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m hyper-tls v0.6.0
2025-12-16T23:21:30.7526943Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m serde_urlencoded v0.7.1
2025-12-16T23:21:30.7527482Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m tower-http v0.6.6
2025-12-16T23:21:30.7528213Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m tokio-util v0.7.15
2025-12-16T23:21:30.7528724Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m url v2.5.4
2025-12-16T23:21:30.7529278Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m thiserror-impl v2.0.17
2025-12-16T23:21:30.7529817Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m raw-cpuid v11.6.0
2025-12-16T23:21:30.7530342Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m hashbrown v0.14.5
2025-12-16T23:21:30.7530893Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m is_terminal_polyfill v1.70.1
2025-12-16T23:21:30.7531432Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m foldhash v0.2.0
2025-12-16T23:21:30.7531945Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m foldhash v0.1.5
2025-12-16T23:21:30.7532465Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m unicode-xid v0.2.6
2025-12-16T23:21:30.7532997Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m anstyle-query v1.1.4
2025-12-16T23:21:30.7533652Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m colorchoice v1.0.4
2025-12-16T23:21:30.7534176Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m anstyle v1.0.11
2025-12-16T23:21:30.7534695Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m powerfmt v0.2.0
2025-12-16T23:21:30.7535201Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m fastrand v2.3.0
2025-12-16T23:21:30.7535705Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m deranged v0.5.3
2025-12-16T23:21:30.7536219Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m tempfile v3.20.0
2025-12-16T23:21:30.7536738Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m dashmap v6.1.0
2025-12-16T23:21:30.7537245Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m anstream v0.6.20
2025-12-16T23:21:30.7538061Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m derive_more-impl v2.0.1
2025-12-16T23:21:30.7538616Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m hashbrown v0.15.2
2025-12-16T23:21:30.7539134Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m quanta v0.12.6
2025-12-16T23:21:30.7539641Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m hashbrown v0.16.1
2025-12-16T23:21:30.7540157Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m reqwest v0.12.23
2025-12-16T23:21:30.7540663Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m rand v0.9.2
2025-12-16T23:21:30.7541184Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m parking_lot v0.12.5
2025-12-16T23:21:30.7541710Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m spinning_top v0.3.0
2025-12-16T23:21:30.7542251Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m futures-timer v3.0.3
2025-12-16T23:21:30.7542777Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m num-conv v0.1.0
2025-12-16T23:21:30.7543419Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m clap_lex v0.7.5
2025-12-16T23:21:30.7543924Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m heck v0.5.0
2025-12-16T23:21:30.7544445Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m time-core v0.1.6
2025-12-16T23:21:30.7544966Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m strsim v0.11.1
2025-12-16T23:21:30.7545476Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m web-time v1.1.0
2025-12-16T23:21:30.7546000Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m nonzero_ext v0.3.0
2025-12-16T23:21:30.7546545Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m clap_builder v4.5.48
2025-12-16T23:21:30.7547087Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m   Compiling[0m clap_derive v4.5.47
2025-12-16T23:21:30.7547606Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m time v0.3.43
2025-12-16T23:21:30.7548341Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m governor v0.10.2
2025-12-16T23:21:30.7548870Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m derive_more v2.0.1
2025-12-16T23:21:30.7549423Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m progenitor-client v0.11.1
2025-12-16T23:21:30.7549956Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m chrono v0.4.41
2025-12-16T23:21:30.7550471Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m regress v0.10.4
2025-12-16T23:21:30.7551355Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m error-handling v0.1.0 (https://github.com/DenisGorbachev/error-handling-rs#f26e25b8)
2025-12-16T23:21:30.7552060Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m uuid v1.17.0
2025-12-16T23:21:30.7552558Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m clap v4.5.48
2025-12-16T23:21:30.7553181Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Checking[0m coda-api v0.1.0 (/home/runner/work/coda-api/coda-api)
2025-12-16T23:21:30.7553910Z [0m[33m[fix:code][0m [0m[31m[fix:code:warnings][0m [1m[92m    Finished[0m `dev` profile [unoptimized + debuginfo] target(s) in 53.77s
2025-12-16T23:21:30.7554485Z [0m[33m[fix:code][0m [0m[31m[fix:code:style][0m [1m$ cargo fmt --all[0m
2025-12-16T23:21:30.7554844Z [0m[33m[fix:code][0m Finished in 54.39s
2025-12-16T23:21:30.7555119Z [2mFinished in 54.41s[0m
2025-12-16T23:21:30.7555256Z 
2025-12-16T23:21:30.7555364Z                                       
2025-12-16T23:21:30.7555647Z   ────────────────────────────────────
2025-12-16T23:21:30.7555924Z summary: (done in 54.46 seconds)      
2025-12-16T23:21:30.7556177Z ✔️  gen:readme
2025-12-16T23:21:30.7556359Z ✔️  fix
2025-12-16T23:21:30.7556538Z 🥊  test
2025-12-16T23:21:30.7561111Z ##[error]Process completed with exit code 1.
2025-12-16T23:21:30.7630336Z Post job cleanup.
2025-12-16T23:21:30.8589082Z [command]/usr/bin/git version
2025-12-16T23:21:30.8627924Z git version 2.52.0
2025-12-16T23:21:30.8671837Z Temporarily overriding HOME='/home/runner/work/_temp/50dab926-79f2-4c9d-9403-d8e63bb2edae' before making global git config changes
2025-12-16T23:21:30.8673130Z Adding repository directory to the temporary git global config as a safe directory
2025-12-16T23:21:30.8678425Z [command]/usr/bin/git config --global --add safe.directory /home/runner/work/coda-api/coda-api
2025-12-16T23:21:30.8715870Z [command]/usr/bin/git config --local --name-only --get-regexp core\.sshCommand
2025-12-16T23:21:30.8749656Z [command]/usr/bin/git submodule foreach --recursive sh -c "git config --local --name-only --get-regexp 'core\.sshCommand' && git config --local --unset-all 'core.sshCommand' || :"
2025-12-16T23:21:30.8981239Z [command]/usr/bin/git config --local --name-only --get-regexp http\.https\:\/\/github\.com\/\.extraheader
2025-12-16T23:21:30.9003526Z http.https://github.com/.extraheader
2025-12-16T23:21:30.9016932Z [command]/usr/bin/git config --local --unset-all http.https://github.com/.extraheader
2025-12-16T23:21:30.9048410Z [command]/usr/bin/git submodule foreach --recursive sh -c "git config --local --name-only --get-regexp 'http\.https\:\/\/github\.com\/\.extraheader' && git config --local --unset-all 'http.https://github.com/.extraheader' || :"
2025-12-16T23:21:30.9273103Z [command]/usr/bin/git config --local --name-only --get-regexp ^includeIf\.gitdir:
2025-12-16T23:21:30.9304098Z [command]/usr/bin/git submodule foreach --recursive git config --local --show-origin --name-only --get-regexp remote.origin.url
2025-12-16T23:21:30.9638840Z Cleaning up orphan processes
```
