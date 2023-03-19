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
  let brew = ([(ansi cyan_bold) brew (ansi reset)] | str join)
  let r = (ansi reset)
  let c = (ansi yellow)
  let p = (ansi magenta)
  [
    $'($brew) ($c)install($r)|($c)i($r) ($p)<packages>($r)'
    $'($brew) ($c)update($r)|($c)u($r) [($p)<packages>($r)]'
    $'($brew) ($c)file($r)|($c)f($r) ($p)<file>($r)'
    $'($brew) ($c)list($r)|($c)l($r) [($p)<filter>($r)]'
    $'($brew) ($c)query($r)|($c)q($r) [($p)<filter>($r)]'
    $'($brew) ($c)uninstall($r)|($c)x($r) ($p)<packages>($r)'
    $'($brew) ($c)cleanup($r) (ansi red)\(not implemented\)($r)'
    $'($brew) ($c)help($r)|($c)?($r)'
  ] | str join "\n"
}

export alias cleanup   = error make {msg: 'not implemented'}
export alias file      = yum whatprovides
export alias install   = scoop install
export alias uninstall = scoop uninstall
export alias remove    = brew uninstall

export alias f = brew file
export alias i = brew install
export alias l = brew list
export alias q = brew query
export alias u = brew update
export alias x = brew uninstall
export alias ? = brew help
