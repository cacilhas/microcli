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
    sudo pacman -Syu; sudo pacman - Fy
  } else {
    bundler install $args
  }
}

export def list [pack?: string] {
  if $pack == null {
    pacman -Qq | ^sort -f | fzf --preview (preview) --layout=reverse --bind $'enter:execute(preview_or_install)'
  } else {
    pacman -Qq $pack | ^sort -f | fzf --preview (preview) --layout=reverse --bind $'enter:execute(preview_or_install)'
  }
}

export def query [query?: string] {
  if $query == null {
    pacman -Slq | ^sort -f | fzf --preview (preview) --layout=reverse --bind $'enter:execute(preview_or_install)'
  } else {
    pacman -Slq | grep $query | ^sort -f | fzf --preview (preview) --layout=reverse --bind $'enter:execute(preview_or_install)'
  }
}

export def cleanup [] {
  pacman -Qtdq | sudo pacman -Rns - err> /dev/null
  sudo paccache -ruk0 out> /dev/null err> /dev/null
  sudo pacman -Sc
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

export alias file   = pacman -Fx
export alias remove = sudo pacman -Rcns

export alias f = bundler file
export alias i = bundler install
export alias l = bundler list
export alias q = bundler query
export alias u = bundler update
export alias x = bundler remove
export alias ? = bundler help
