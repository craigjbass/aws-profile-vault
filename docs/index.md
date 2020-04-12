---
title: aws-profile-vault
description: Let's you run scripts that use aws-profile when you only have aws-vault
show_downloads: true
---

# (AWS Profile)-Vault

```
USAGE:
    aws-profile <command>... -p <PROFILE>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p <PROFILE>        The AWS profile to use.

ARGS:
    <command>...    
```

## Installing

**Step 1**

- Linux/macOS 64-bit: Download the appropriate [binary from releases](https://github.com/craigjbass/aws-profile-vault/releases).
- Any other OS: [Compile from source](https://github.com/craigjbass/aws-profile-vault#Compiling).
- Windows: Untested. Fairly sure it's broken see [#2](https://github.com/craigjbass/aws-profile-vault/issues/2)

**Step 2**

- Ensure it is on your $PATH
- Ensure it is executable
- Ensure that it is named `aws-profile`
- Ensure you have `aws-vault` setup with profiles that match your `aws-profile` needs.
