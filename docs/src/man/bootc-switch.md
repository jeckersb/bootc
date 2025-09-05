# NAME

bootc-switch - Target a new container image reference to boot

# SYNOPSIS

**bootc switch** \[**\--quiet**\] \[**\--apply**\]
\[**\--soft-reboot**\] \[**\--transport**\]
\[**\--enforce-container-sigpolicy**\] \[**\--retain**\]
\[**-h**\|**\--help**\] \<*TARGET*\>

# DESCRIPTION

Target a new container image reference to boot.

This is almost exactly the same operation as \`upgrade\`, but
additionally changes the container image reference instead.

\## Usage

A common pattern is to have a management agent control operating system
updates via container image tags; for example,
\`quay.io/exampleos/someuser:v1.0\` and
\`quay.io/exampleos/someuser:v1.1\` where some machines are tracking
\`:v1.0\`, and as a rollout progresses, machines can be switched to
\`v:1.1\`.

# OPTIONS

**\--quiet**

:   Don\'t display progress

**\--apply**

:   Restart or reboot into the new target image.

    Currently, this option always reboots. In the future this command
    will detect the case where no kernel changes are queued, and perform
    a userspace-only restart.

**\--soft-reboot** *\<SOFT_REBOOT\>*

:   Configure soft reboot behavior.

    \'required\' will fail if soft reboot is not available. \'auto\'
    will use soft reboot if available, otherwise fall back to regular
    reboot.\

    \
    *Possible values:*

    -   required: Require a soft reboot; fail if not possible

    -   auto: Automatically use soft reboot if possible, otherwise use
        regular reboot

**\--transport** *\<TRANSPORT\>* \[default: registry\]

:   The transport; e.g. oci, oci-archive, containers-storage. Defaults
    to \`registry\`

**\--enforce-container-sigpolicy**

:   This is the inverse of the previous
    \`\--target-no-signature-verification\` (which is now a no-op).

    Enabling this option enforces that \`/etc/containers/policy.json\`
    includes a default policy which requires signatures.

**\--retain**

:   Retain reference to currently booted image

**-h**, **\--help**

:   Print help (see a summary with \'-h\')

\<*TARGET*\>

:   Target image to use for the next boot

# VERSION

v1.8.0
