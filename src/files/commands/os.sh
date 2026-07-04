echo "os:" "$(grep '^PRETTY_NAME=' /etc/os-release | cut -d'"' -f2) $(uname -m)"
