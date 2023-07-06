target="tailwindcss-linux-x64"
curl -sLO "https://github.com/tailwindlabs/tailwindcss/releases/latest/download/${target}"
chmod +x $target
mv $target .bin/tailwindcss

echo "${target} installed"