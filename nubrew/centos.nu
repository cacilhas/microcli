def preview [] {
  "yum list installed | grep '{}'"
}

def preview_or_install [] {
  $"\(yum list installed | grep -q '{}' || sudo yum install -y '{}'; (preview) | (pager)\)"
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

export def update [...args: string] {
  if ($args | is-empty) {
    sudo yum update
  } else {
    brew install $args
  }
}

export def list [pack?: string] {
  if $pack == null {
    yum list installed | tail +5 | each {str trim 'ii '} | sort -i | str join "\n" | fzf --preview (preview) --layout=reverse --bind $'enter:execute(preview_or_install)'
  } else {
    yum list installed | grep $pack | tail +5 | each {str trim 'ii '} | sort -i | str join "\n" | fzf --preview (preview) --layout=reverse --bind $'enter:execute(preview_or_install)'
  }
}

export def query [query: string = '.*'] {
  yum search $query | sort -i | each { $in | split row ' - ' | get 0 } | str join "\nww" | fzf --preview (preview) --layout=reverse --bind $'enter:execute(preview_or_install)'
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
    $'($brew) ($c)cleanup($r)'
    $'($brew) ($c)help($r)|($c)?($r)'
  ] | str join "\n"
}

export alias cleanup   = sudo yum clean packages
export alias file      = yum whatprovides
export alias install   = sudo yum install
export alias uninstall = sudo yum erase
export alias remove    = brew uninstall

export alias f = brew file
export alias i = brew install
export alias l = brew list
export alias q = brew query
export alias u = brew update
export alias x = brew uninstall
export alias ? = brew help
