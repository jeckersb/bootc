# NAME

bootc-install-to-existing-root - Install to the host root filesystem

# SYNOPSIS

**bootc install to-existing-root** [*OPTIONS...*] [*ROOT_PATH*]

# DESCRIPTION

Install to the host root filesystem.

This is a variant of `install to-filesystem` that is designed to
install \"alongside\" the running host root filesystem. Currently, the
host root filesystem\'s `/boot` partition will be wiped, but the
content of the existing root will otherwise be retained, and will need
to be cleaned up if desired when rebooted into the new root.

# OPTIONS

<!-- BEGIN GENERATED OPTIONS -->
**ROOT_PATH**

    Path to the mounted root; this is now not necessary to provide. Historically it was necessary to ensure the host rootfs was mounted at here via e.g. `-v /:/target`

**--replace**=*REPLACE*

    Configure how existing data is treated

    Possible values:
    - wipe
    - alongside

    Default: alongside

**--source-imgref**=*SOURCE_IMGREF*

    Install the system from an explicitly given source

**--target-transport**=*TARGET_TRANSPORT*

    The transport; e.g. oci, oci-archive, containers-storage.  Defaults to `registry`

    Default: registry

**--target-imgref**=*TARGET_IMGREF*

    Specify the image to fetch for subsequent updates

**--enforce-container-sigpolicy**=*ENFORCE_CONTAINER_SIGPOLICY*

    This is the inverse of the previous `--target-no-signature-verification` (which is now a no-op).  Enabling this option enforces that `/etc/containers/policy.json` includes a default policy which requires signatures

    Possible values:
    - true
    - false

**--run-fetch-check**=*RUN_FETCH_CHECK*

    Verify the image can be fetched from the bootc image. Updates may fail when the installation host is authenticated with the registry but the pull secret is not in the bootc image

    Possible values:
    - true
    - false

**--skip-fetch-check**=*SKIP_FETCH_CHECK*

    Verify the image can be fetched from the bootc image. Updates may fail when the installation host is authenticated with the registry but the pull secret is not in the bootc image

    Possible values:
    - true
    - false

**--disable-selinux**=*DISABLE_SELINUX*

    Disable SELinux in the target (installed) system

    Possible values:
    - true
    - false

**--karg**=*KARG*

    Add a kernel argument.  This option can be provided multiple times

**--root-ssh-authorized-keys**=*ROOT_SSH_AUTHORIZED_KEYS*

    The path to an `authorized_keys` that will be injected into the `root` account

**--generic-image**=*GENERIC_IMAGE*

    Perform configuration changes suitable for a "generic" disk image. At the moment:

    Possible values:
    - true
    - false

**--bound-images**=*BOUND_IMAGES*

    How should logically bound images be retrieved

    Possible values:
    - stored
    - skip
    - pull

    Default: stored

**--stateroot**=*STATEROOT*

    The stateroot name to use. Defaults to `default`

**--acknowledge-destructive**=*ACKNOWLEDGE_DESTRUCTIVE*

    Accept that this is a destructive action and skip a warning timer

    Possible values:
    - true
    - false

**--cleanup**=*CLEANUP*

    Add the bootc-destructive-cleanup systemd service to delete files from the previous install on first boot

    Possible values:
    - true
    - false

<!-- END GENERATED OPTIONS -->

# VERSION

<!-- VERSION PLACEHOLDER -->

