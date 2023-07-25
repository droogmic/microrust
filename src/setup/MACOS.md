# macOS

All the tools can be install using [Homebrew]:

[Homebrew]: http://brew.sh/

``` shell
$ brew install --cask gcc-arm-embedded
$ brew install minicom openocd
```

Unfortunately gcc-arm-embedded has been [removed from Homebrew casks] in an attempt to force somebody to move it to Homebrew, and so it has to be installed using the cask in an earlier commit.  

[removed from Homebrew casks]: https://github.com/Homebrew/homebrew-cask/pull/56802

That's all! Go to the [next section].

[next section]: ../setup/VERIFY.html
