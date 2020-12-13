# gitignore-cli
Tool to generate gitignore files for my projects. Existing oh-my-zsh plugin makes HTTP request on each `Tab` press.
It's no so cool and fast. This implementation saves all template ids to file in `.cache` directory and on `Tab` press reads ids from file.

To install this plugin: 

```bash
make install
```

And then add `gitignore-rust` to `plugins` in `.zsh`

Also you can run updating of ids with schedule. You must add `gitignore_teplates --save` to yours crontab.

To remove this just run:

```bash
make uninstall
```
