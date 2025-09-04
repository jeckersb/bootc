# NAME

bootc-container-lint - Perform relatively inexpensive static analysis
checks as part of a container build

# SYNOPSIS

**bootc container lint** [*OPTIONS...*]

# DESCRIPTION

Perform relatively inexpensive static analysis checks as part of a
container build.

This is intended to be invoked via e.g. `RUN bootc container lint` as
part of a build process; it will error if any problems are detected.

# OPTIONS

<!-- BEGIN GENERATED OPTIONS -->
**--rootfs**=*ROOTFS*

    Operate on the provided rootfs

    Default: /

**--fatal-warnings**=*FATAL_WARNINGS*

    Make warnings fatal

    Possible values:
    - true
    - false

**--list**=*LIST*

    Instead of executing the lints, just print all available lints. At the current time, this will output in YAML format because it's reasonably human friendly. However, there is no commitment to maintaining this exact format; do not parse it via code or scripts

    Possible values:
    - true
    - false

**--skip**=*SKIP*

    Skip checking the targeted lints, by name. Use `--list` to discover the set of available lints

**--no-truncate**=*NO_TRUNCATE*

    Don't truncate the output. By default, only a limited number of entries are shown for each lint, followed by a count of remaining entries

    Possible values:
    - true
    - false

<!-- END GENERATED OPTIONS -->

# VERSION

<!-- VERSION PLACEHOLDER -->

