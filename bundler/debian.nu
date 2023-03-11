def preview [] {
  "dpkg -l '{}'"
}

def preview_or_install [] {
  $"\(dpkg -l '{}' || sudo apt-get install -y '{}'; (preview) | (pager)\)"
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
    sudo apt-get update; sudo apt-get upgrade
  } else {
    sudo apt-get upgrade $args
  }
}

export def list [pack?: string] {
  if $pack == null {
    dpkg -l | tail +5 | each {str trim 'ii '} | sort -i | str join "\n" | fzf --preview (preview) --layout=reverse --bind $'enter:execute(preview_or_install)'
  } else {
    dpkg -l $pack | tail +5 | each {str trim 'ii '} | sort -i | str join "\n" | fzf --preview (preview) --layout=reverse --bind $'enter:execute(preview_or_install)'
  }
}

export def query [query: string = '.*'] {
  apt-cache search $query | sort -i | each { $in | split row ' - ' | get 0 } | str join "\n" | fzf --preview (preview) --layout=reverse --bind $'enter:execute(preview_or_install)'
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

export alias cleanup = sudo apt-get clean
export alias file    = dpkg -S
export alias install = sudo apt-get install
export alias remove  = sudo apt-get purge

export alias f = bundler file
export alias i = bundler install
export alias l = bundler list
export alias q = bundler query
export alias u = bundler update
export alias x = bundler remove
export alias ? = bundler help
