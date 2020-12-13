function gi() { curl -fLw '\n' https://www.gitignore.io/api/"${(j:,:)@}" }

_gitignoreio_get_command_list() {
    gitignore_templates
}

_gitignoreio () {
  compset -P '*,'
  compadd -S '' `_gitignoreio_get_command_list`
}

compdef _gitignoreio gi
