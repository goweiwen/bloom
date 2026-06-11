#!/usr/bin/env nix-shell
#!nix-shell --pure -i bash -p bash -p imagemagick -p libwebp

set -euo pipefail

SRC=assets/textures/banners
OUT=${1:-assets/textures/banner_atlas.webp}
W=20
H=40
X=1
Y=1

patterns=(
  base
  border
  bricks
  circle
  creeper
  cross
  curly_border
  diagonal_left
  diagonal_up_right
  diagonal_up_left
  diagonal_right
  flower
  globe
  gradient
  gradient_up
  half_horizontal
  half_horizontal_bottom
  half_vertical
  half_vertical_right
  mojang
  piglin
  rhombus
  skull
  small_stripes
  square_bottom_left
  square_bottom_right
  square_top_left
  square_top_right
  straight_cross
  stripe_bottom
  stripe_center
  stripe_downleft
  stripe_downright
  stripe_left
  stripe_middle
  stripe_right
  stripe_top
  triangle_bottom
  triangle_top
  triangles_bottom
  triangles_top
  flow
  guster
)

colors=(
  "249,255,254"  # White
  "157,157,151"  # LightGray
  "71,79,82"     # Gray
  "29,29,33"     # Black
  "254,216,61"   # Yellow
  "249,128,29"   # Orange
  "176,46,38"    # Red
  "131,84,50"    # Brown
  "128,199,31"   # Lime
  "94,124,22"    # Green
  "58,179,218"   # LightBlue
  "22,156,156"   # Cyan
  "60,68,170"    # Blue
  "243,139,170"  # Pink
  "199,78,189"   # Magenta
  "137,50,184"   # Purple
)

files=()
for p in "${patterns[@]}"; do
  f="$SRC/$p.png"
  [[ -f "$f" ]] || { echo "missing texture: $f" >&2; exit 1; }
  files+=("$f")
done

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

# Take the 20x40 front banner portion and convert to sRGB
magick "${files[@]}" -crop "${W}x${H}+${X}+${Y}" +repage +append \
  -colorspace sRGB -type TrueColorAlpha "$tmp/row.png"

rows=()
for i in "${!colors[@]}"; do
  row="$tmp/row_$i.png"
  # Multiply to convert grayscale base pattern into colored pattern
  magick "$tmp/row.png" \
    \( +clone -alpha off -fill "rgb(${colors[$i]})" -colorize 100 \) \
    -compose Multiply -composite \
    "$tmp/row.png" -compose CopyOpacity -composite \
    "$row"
  rows+=("$row")
done

magick "${rows[@]}" -append "$tmp/atlas.png"
cwebp -z 9 "$tmp/atlas.png" -o "$OUT"
echo "wrote $OUT (${#colors[@]} colors x ${#files[@]} patterns, $(( ${#files[@]} * W ))x$(( ${#colors[@]} * H )))"
