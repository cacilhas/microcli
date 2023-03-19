def preview [] {
  "pacman -Qil '{}'"
}

def preview_or_install [] {
  $"\(pacman -Q '{}' || sudo pacman install '{}'; (preview) | (pager)\)"
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

export def install [...args: string] {
  let files = ($args | filter { path exists })
  let packs = ($args | filter { |it| not ($it | path exists)})

  if (not ($packs | is-empty)) {
    pacman -S $packs
  }
  if (not ($files | is-empty)) {
    pacman -U $files
  }
}

export def update [...args: string] {
  if ($args | is-empty) {
    sudo pacman -Syu; sudo pacman -Fy
  } else {
    brew install $args
  }
}

export def list [pack?: string] {
  if $pack == null {
    pacman -Qq | sort -i | str join "\n" | fzf --preview (preview) --layout=reverse --bind $'enter:execute(preview_or_install)'
  } else {
    pacman -Qq $pack | sort -i | str join "\n" | fzf --preview (preview) --layout=reverse --bind $'enter:execute(preview_or_install)'
  }
}

export def query [query?: string] {
  if $query == null {
    pacman -Slq | sort -i | str join "\n" | fzf --preview (preview) --layout=reverse --bind $'enter:execute(preview_or_install)'
  } else {
    pacman -Slq | ^ag $query | sort -i | str join "\n" | fzf --preview (preview) --layout=reverse --bind $'enter:execute(preview_or_install)'
  }
}

export def cleanup [] {
  pacman -Qtdq | sudo pacman -Rns - err> /dev/null
  sudo paccache -ruk0 out> /dev/null err> /dev/null
  sudo pacman -Sc
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

export alias file      = pacman -Fx
export alias uninstall = sudo pacman -Rcns
export alias remove    = brew uninstall

export alias f = brew file
export alias i = brew install
export alias l = brew list
export alias q = brew query
export alias u = brew update
export alias x = brew uninstall
export alias ? = brew help
