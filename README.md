# blobsum

A command line utility for calculating [S5 Blob Identifiers (CIDs)](https://docs.s5.pro/spec/blobs.html)

This tool is forked from `b3sum` (https://github.com/BLAKE3-team/BLAKE3)

```
Usage: blobsum [OPTIONS] [FILE]...

Arguments:
  [FILE]...  Files to hash, or checkfiles to check

Options:
      --num-threads <NUM>     The maximum number of threads to use
      --no-mmap               Disable memory mapping
      --no-names              Omit filenames in the output
      --quiet                 Skip printing OK for each checked file
  -h, --help                  Print help (see more with '--help')
  -V, --version               Print version
```

## Example

```
❯ blobsum LICENSE 
blobb5kykfixjikdxcpnx5dpo26pjjywwuz473hfdsmbxvsvcq5xgbcipmewa  LICENSE
```