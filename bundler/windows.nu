def preview [] {
  "scoop info '{}'"
}

def preview_or_install [] {
  $"\((preview) || scoop install '{}'; (preview) | (pager)\)"
}

def pager [] {
  try {
    return $nu.PAGER
  }

if (not (which bat | is-empty)) {
  return 'bat'
}

  'more'
}

export def update [...args: string] {
  if ($args | is-empty) {
    scoop update
  } else {
    coop update $args
  }
}

export def list [pack?: string] {
  if $pack == null {
    scoop list | sort -i | str join "\n" | fzf --preview (preview) --layout=reverse --bind $'enter:execute(preview_or_install)'
  } else {
    scoop list | grep $pack | sort -i | str join "\n" | fzf --preview (preview) --layout=reverse --bind $'enter:execute(preview_or_install)'
  }
}

export def query [query: string = '.*'] {
  scoop search $query | sort -i | str join "\n" | fzf --preview (preview) --layout=reverse --bind $'enter:execute(preview_or_install)'
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
    $'($bundler) ($c)cleanup($r) (ansi red)\(not implemented\)($r)'
    $'($bundler) ($c)help($r)|($c)?($r)'
  ] | str join "\n"
}

export alias cleanup = error make {msg: 'not implemented'}
export alias file    = yum whatprovides
export alias install = scoop install
export alias remove  = scoop uninstall

export alias f = bundler file
export alias i = bundler install
export alias l = bundler list
export alias q = bundler query
export alias u = bundler update
export alias x = bundler remove
export alias ? = bundler help
