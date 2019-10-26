# macOS

All the tools can be install using [Homebrew]:

[Homebrew]: http://brew.sh/

``` shell
$ brew cask install https://raw.githubusercontent.com/Homebrew/homebrew-cask/b88346667547cc85f8f2cacb3dfe7b754c8afc8a/Casks/gcc-arm-embedded.rb
$ brew install minicom openocd
```

Unfortunately gcc-arm-embedded has been [removed from Homebrew casks] in an attempt to force somebody to move it to Homebrew, and so it has to be installed using the cask in an earlier commit.  

[removed from Homebrew casks]: https://github.com/Homebrew/homebrew-cask/pull/56802

That's all! Go to the [next section].

[next section]: ../setup/VERIFY.html
