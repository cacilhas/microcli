def preview [] {
  "pacman -Qil '{}'"
}

def preview_or_install [] {
  $"\(pacman -Q '{}' || sudo pacman install; (preview) | (pager)\)"
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
    pacman -Slq | ^ag $query | ^sort -f | fzf --preview (preview) --layout=reverse --bind $'enter:execute(preview_or_install)'
  }
}

export def cleanup [] {
  sudo pacman -Qtdq | pacman -Rns -
  sudo paccache -ruk0 out> /dev/null err> /dev/null
  sudo pacman -Sc
}

export def help [] {
  [
    'bundler install|i <packages>'
    'bundler update|u [<packages>]'
    'bundler file|f <file>'
    'bundler list|l [<filter>]'
    'bundler query|q [<filter>]'
    'bundler remove|x <packages>'
    'bundler cleanup'
    'bundler help|?'
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
