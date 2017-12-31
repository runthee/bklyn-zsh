setopt NO_HUP
setopt NO_CHECK_JOBS

bklyn_zsh_dir=${0:A:h}

bklyn_zsh_update() {
  for f in color_codes install ostype ssh git prompt; do
    source ${bklyn_zsh_dir}/src/$f.zsh
  done
}

bklyn_zsh_update

up() {
  bklyn_zsh_update
}
