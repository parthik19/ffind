# ffind

`ffind` is a limited but more performant version of Linux's `find` command. In addition to finding the path of the given file name, `ffind` also supports "fuzzy" searches (hence the additional `f`). This is useful when trying to find a file without knowing it's _exact_ name.

Future versions of `ffind` will attempt to implement many of the useful features of Linux's builtin `find` command.

## Usage

```console
ffind [OPTIONS] <query> [starting-dir]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --fuzzy <fuzzy>    

ARGS:
    <query>           
    <starting-dir>
```

You can generate usage via `ffind --help`.

## Examples

```console
> ffind someFile.txt
/Users/path/to/file/temp/test2Dir/someFile.txt
/Users/path/to/file/temp/test1Dir/someFile.txt

Took 21 msecs.
```


This command will search for and print the path of any file with name `someFile.txt` in the current directory.

```console
> ffind someFile.txt test2Dir
test2Dir/someFile.txt

Took 3 msecs.
```


This command will search for and print the absolute path of any file with name `someFile.txt` in the `myDir` directory.

```console
> ffind someFile.txt -f 80
/Users/path/to/file/temp/test2Dir/someFile.txt
/Users/path/to/file/temp/test1Dir/someFile.txt
/Users/path/to/file/temp/test1Dir/someFilee.txtt

Took 6 msecs.
```


This command will search for and print the absolute path of any file which has a name that is _close_ to `someFile.txt` in the current directory. The `-f` flag indicates the use of "fuzzy" search, and the following integer denotes the "strength" of the comparison. Providing a strength of `0` will match anything, while providing a strength of `100` will check for strict string equality. Behind the scenes, `ffind` uses the [Normalized Levenshtein](https://en.wikipedia.org/wiki/Levenshtein_distance) distance implemented by the [strsim](https://docs.rs/strsim/0.10.0/strsim/) crate.

## Performance

![ffind vs. find](/images/ffind_performance.png "ffind vs. find")

As you can see, on my machine, searching for a file name starting at the root directory is approximately 10x faster than Linux's `find` command.
