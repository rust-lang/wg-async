#!/usr/bin/env bash

# Get a list of all contributors that contributed in some way
# either by directly opening a PR, or by participating in issues.
#
# This script is a helper to fill out the "Contributors" chapter.
# It'll only spit out the names and an user must add them to the book.
#
# This script takes two arguments:
#   - `username`: Your github username used to authenticate the GitHub API
#   - `token`: A github Personal Access Token used to authenticate the GitHub API

set -euo pipefail

# Check if there are `username` and `token` arguments
if [ $# -eq 0 ]
then
  echo "Usage: $0 <username> <token>"
  exit 1
fi

user="$1"
token="$2"

# Check if a command is available, otherwise exit.
function check_bin() {
  if ! command -v $1 &> /dev/null
  then
    echo "'$1' is not installed, but required to run this script."
    exit 1
  fi
}

check_bin curl
check_bin jq

# Get a list of users that participated in issues.
function issue_contributors() {
  local numbers=$(curl -s -u $user:$token -H "Accept: application/vnd.github.v3+json" 'https://api.github.com/repos/rust-lang/wg-async-foundations/issues?state=all&labels=status-quo-story-ideas' | jq '.[].number')

  for num in $numbers; do
    curl -s -u $user:$token -H "Accept: application/vnd.github.v3+json" \
      https://api.github.com/repos/rust-lang/wg-async-foundations/issues/$num/comments | jq -r \
      '.[].user | "[" + .login + "](" + .html_url + ")"'
  done | sort | uniq
}

# Get a list of direct code contributors
function code_contributors() {
  curl -s -u $user:$token -H "Accept: application/vnd.github.v3+json" \
    https://api.github.com/repos/rust-lang/wg-async-foundations/contributors | jq -r \
    '.[] | "[" + .login + "](" + .html_url + ")"' | sort | uniq
}

echo "Issue contributors"
issue_contributors

echo

echo "Code contributors"
code_contributors
