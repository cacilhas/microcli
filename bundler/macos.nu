def preview [] {
  "brew info '{}'"
}

def preview_or_install [] {
  $"\((preview) || brew install '{}'; (preview) | (pager)\)"
}

def pager [] {
  try {
    return $nu.PAGER
  }

  for it in [bat lv less] {
    if (not (which $it | is-empty)) {
      return $it
    }
  }

  'more'
}

export def file [file: string] {
  let real = (^which $file)
  if $real == null {
    return
  }
  let name = (ls $real | get 0.name)

  if ($name | str starts-with /Applications/) {
    return ($name | path split | get 2)
  }

  if ($name | str contains /homebrew/Cellar/) {
    let pack = ($name | str replace '^.*homebrew/Cellar/' '')
    let pack = ($pack | path.split)
    return $'($pack | get 0)@($pack | get 1)'
  }

  'core'
}

export def list [pack?: string] {
  if $pack == null {
    brew list | ^sort -f | fzf --preview (preview) --layout=reverse --bind $'enter:execute(preview_or_install)'
  } else {
    brew list | ag $pack | ^sort -f | fzf --preview (preview) --layout=reverse --bind $'enter:execute(preview_or_install)'
  }
}

export def query [query: string = '*'] {
  brew search $query | ^sort -f | fzf --preview (preview) --layout=reverse --bind $'enter:execute(preview_or_install)'
}

export def help [] {
  let bundler = ([(ansi cyan_bold) bundler (ansi reset)] | str join)
  let r = (ansi reset)
  let c = (ansi yellow)
  let p = (ansi magenta)
  [
    $'($bundler) ($c)install($r)|($c)i($r) ($p)<packages>($r)'
    $'($bundler) ($c)update($r)|($c)u($r) [($p)<packages>($r)]'
    $'($bundler) ($c)file($r)|($c)f($r) ($p)<file>($r)'
    $'($bundler) ($c)list($r)|($c)l($r) [($p)<filter>($r)]'
    $'($bundler) ($c)query($r)|($c)q($r) [($p)<filter>($r)]'
    $'($bundler) ($c)remove($r)|($c)x($r) ($p)<packages>($r)'
    $'($bundler) ($c)cleanup($r)'
    $'($bundler) ($c)help($r)|($c)?($r)'
  ] | str join "\n"
}

export alias cleanup = brew cleanup
export alias install = brew install
export alias remove  = brew uninstall
export alias update  = (brew update; brew upgrade)

export alias f = bundler file
export alias i = bundler install
export alias l = bundler list
export alias q = bundler query
export alias u = bundler update
export alias x = bundler remove
export alias ? = bundler help
